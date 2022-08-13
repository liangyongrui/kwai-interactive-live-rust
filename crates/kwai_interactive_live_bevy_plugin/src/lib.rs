use async_compat::Compat;
use bevy::prelude::{App, Commands, Component, Deref, Entity, EventWriter, Plugin, Query};
use bevy::tasks::{IoTaskPool, Task};
use futures_lite::{future, StreamExt};
use kwai_interactive_live::EventStream;
pub use kwai_interactive_live::{ConnectParams, ConnectResp, DisconnectParams};

/// 建立连接的返回事件
///
/// - `Err(e)` 连接失败的原因
/// - `Ok(resp)` 连接成功的返回值
#[derive(Deref)]
pub struct ConnectEvent(pub anyhow::Error);
pub type EventReceiver = crossbeam_channel::Receiver<kwai_interactive_live::Event>;

pub struct KwaiPlugin;

impl Plugin for KwaiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConnectEvent>().add_system(frame_loop);
    }
}

#[derive(Component, Deref)]
struct KwaiConnectResp(Task<anyhow::Result<(ConnectResp, EventStream)>>);

pub fn connect(commands: &mut Commands, p: ConnectParams) {
    let task = IoTaskPool::get().spawn(Compat::new(async {
        kwai_interactive_live::connect(p).await
    }));
    commands.spawn().insert(KwaiConnectResp(task));
}

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
    mut connect_resp_event: EventWriter<ConnectEvent>,
) {
    // 理论上这里只有一个
    for (e, mut task) in &mut tasks {
        if let Some(resp) = future::block_on(future::poll_once(&mut task.0)) {
            match resp {
                Ok((resp, stream)) => {
                    let (tx, rx) = crossbeam_channel::unbounded();
                    IoTaskPool::get()
                        .spawn(Compat::new(stream.into_stream().for_each(move |s| {
                            let _ = tx.send(s);
                        })))
                        .detach();
                    commands.insert_resource(rx);
                    commands.insert_resource(resp);
                }
                Err(e) => connect_resp_event.send(ConnectEvent(e)),
            }
            // 释放内存
            commands.entity(e).despawn()
        }
    }
}
