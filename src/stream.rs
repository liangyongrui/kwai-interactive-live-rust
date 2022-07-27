use std::collections::VecDeque;
use std::time::Duration;

use async_stream::stream;
use futures::Stream;
use reqwest::Url;
use tokio::time::{Interval, MissedTickBehavior};

use crate::event::Event;
use crate::kwai;

#[derive(Debug)]
pub(crate) struct EventStream {
    url: Url,
    buffer: VecDeque<Event>,
    interval: Option<Interval>,
    token: String,
    p_cursor: String,
}

impl EventStream {
    pub(crate) fn new(host: &str, token: String) -> anyhow::Result<Self> {
        let mut poll_url = Url::parse("https://example.com/openapi/sdk/v1/poll")?;
        poll_url.set_host(Some(host))?;
        Ok(EventStream {
            url: poll_url,
            buffer: VecDeque::with_capacity(200),
            interval: None,
            token,
            p_cursor: "".to_string(),
        })
    }

    pub(crate) fn into_stream(mut self) -> impl Stream<Item = Event> {
        stream! {
            loop {
                if let Some(t) = self.buffer.pop_back() {
                    yield t;
                }
                if let Some(t) = &mut self.interval {
                    t.tick().await;
                }
                let resp = kwai::poll(self.url.clone(), &self.token, &self.p_cursor, 200).await;
                match resp {
                    // disconnect
                    Ok(None) => break,
                    Err(e) => {
                        log::error!("receive event error: {e}");
                    }
                    Ok(Some(resp)) => {
                        if resp.sleep != self.interval.as_ref().map(|t| t.period().as_millis()).unwrap_or(0) as u64 {
                            log::info!("update sleep: {} ms", resp.sleep);
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
