//! bevy plugin demo

#![allow(clippy::restriction)]
use bevy::prelude::*;
use kwai_interactive_live_bevy_plugin::{
    connect, ConnectErrEvent, ConnectParams, EventReceiver, KwaiPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(KwaiPlugin)
        .add_startup_system(connect_kwai)
        .add_system(connect_err_system)
        .add_system(event_resource)
        .run();
}

fn connect_kwai(mut commands: Commands) {
    let p = ConnectParams {
        host: "xxxxxxxxx.com".to_string(),
        app_id: "app_id".to_string(),
        code: "code".to_string(),
        ..Default::default()
    };
    connect(&mut commands, p);
}

fn event_resource(event: Res<EventReceiver>) {
    while let Some(e) = event.recv() {
        log::info!("receive {:?}", e);
    }
}

fn connect_err_system(mut event: EventReader<ConnectErrEvent>) {
    for e in event.iter() {
        log::error!("连接失败，需要重试：{}", e.0);
    }
}
