extern crate alloc;

use alloc::collections::VecDeque;
use core::time::Duration;

use async_stream::stream;
use futures_lite::stream::Stream;
use reqwest::Url;
use tokio::time::{Interval, MissedTickBehavior};

use crate::event::Event;
use crate::kwai;

/// 用于构建事件流的状态
///
/// 获取后，只能用于一件事 `.into_stream()`
#[derive(Debug)]
pub struct EventStream {
    http_client: reqwest::Client,
    url: Url,
    buffer: VecDeque<Event>,
    interval: Option<Interval>,
    /// 休眠时间（毫秒）
    ///
    /// sleep 字段用于优化比较, 和 interval 一样
    sleep: u64,
    token: String,
    p_cursor: String,
}

impl EventStream {
    pub(crate) fn new(
        http_client: reqwest::Client,
        host: &str,
        token: String,
    ) -> anyhow::Result<Self> {
        let mut poll_url = Url::parse("https://example.com/openapi/sdk/v1/poll")?;
        poll_url.set_host(Some(host))?;
        Ok(EventStream {
            http_client,
            url: poll_url,
            buffer: VecDeque::with_capacity(200),
            interval: None,
            sleep: 0,
            token,
            p_cursor: String::new(),
        })
    }

    /// 转换成异步流
    #[inline]
    pub fn into_stream(mut self) -> impl Stream<Item = Event> {
        stream! {
            loop {
                if let Some(t) = self.buffer.pop_back() {
                    yield t;
                }
                if let Some(t) = &mut self.interval {
                    t.tick().await;
                }
                let resp = kwai::poll(&self.http_client, self.url.clone(), &self.token, &self.p_cursor, 200).await;
                match resp {
                    // disconnect
                    Ok(None) => break,
                    Err(e) => {
                        log::error!("receive event error: {e}");
                    }
                    Ok(Some(resp)) => {
                        if resp.sleep != self.sleep {
                            log::debug!("update sleep: {} ms", resp.sleep);
                            self.sleep = resp.sleep;
                            if resp.sleep > 0 {
                                let mut t = tokio::time::interval(Duration::from_millis(resp.sleep));
                                t.set_missed_tick_behavior(MissedTickBehavior::Delay);
                                t.tick().await;
                                self.interval = Some(t);
                            } else {
                                self.interval = None;
                            }
                        }
                        if resp.data.is_empty() {
                            continue
                        }
                        self.p_cursor = resp.p_cursor;
                        let mut data = resp.data.into_iter();
                        let res = data.next().unwrap();
                        for e in data {
                            self.buffer.push_front(e)
                        }
                        yield res
                    }
                }
            }
        }
    }
}
