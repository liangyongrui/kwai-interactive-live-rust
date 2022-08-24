//! 快手互动直播 bevy plugin

use async_compat::Compat;
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use bevy_tasks::{IoTaskPool, Task};
use futures_lite::{future, StreamExt};
use kwai_interactive_live::EventStream;
pub use kwai_interactive_live::{ConnectParams, ConnectResp, DisconnectParams, Event};

/// 快手互动直播 bevy plugin
#[allow(clippy::restriction)]
pub struct KwaiPlugin;

impl Plugin for KwaiPlugin {
    #[inline]
    fn build(&self, app: &mut App) {
        app.add_event::<ConnectErrEvent>()
            .init_resource::<EventReceiver>()
            .add_system(frame_loop);
    }
}

/// 建立连接的返回事件
///
/// - `Err(e)` 连接失败的原因
/// - `Ok(resp)` 连接成功的返回值
pub struct ConnectErrEvent(pub anyhow::Error);

/// 直播事件接收器
#[derive(Default)]
pub struct EventReceiver(Option<crossbeam_channel::Receiver<kwai_interactive_live::Event>>);

impl EventReceiver {
    /// 接收直播事件
    ///
    /// 每一帧渲染的时候一直`recv`到`None`为止
    #[must_use]
    #[inline]
    pub fn recv(&self) -> Option<Event> {
        self.0.as_ref().and_then(|rx| rx.try_recv().ok())
    }
}

#[derive(Component)]
struct KwaiConnectResp(Task<anyhow::Result<(ConnectResp, EventStream)>>);

/// 建立连接
///
/// 如果失败了，可以收到 `EventReader<ConnectErrEvent>` bevy event
#[inline]
pub fn connect(commands: &mut Commands, p: ConnectParams) {
    let task = IoTaskPool::get().spawn(Compat::new(async {
        kwai_interactive_live::connect(p).await
    }));
    commands.spawn().insert(KwaiConnectResp(task));
}

/// 断开连接
#[inline]
pub fn disconnect(p: DisconnectParams) {
    IoTaskPool::get()
        .spawn(Compat::new(async move {
            kwai_interactive_live::disconnect(&p).await
        }))
        .detach();
}

fn frame_loop(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut KwaiConnectResp)>,
    mut connect_resp_event: EventWriter<ConnectErrEvent>,
) {
    // 理论上这里只有一个
    for (e, mut task) in &mut tasks {
        if let Some(resp) = future::block_on(future::poll_once(&mut task.0)) {
            match resp {
                Ok((resp, stream)) => {
                    let (tx, rx) = crossbeam_channel::unbounded();
                    IoTaskPool::get()
                        .spawn(Compat::new(stream.into_stream().for_each(move |s| {
                            if let Err(e) = tx.send(s) {
                                log::error!("send event error: {e}");
                            }
                        })))
                        .detach();
                    commands.insert_resource(EventReceiver(Some(rx)));
                    commands.insert_resource(resp);
                }
                Err(e) => connect_resp_event.send(ConnectErrEvent(e)),
            }
            // 释放内存
            commands.entity(e).despawn();
        }
    }
}
