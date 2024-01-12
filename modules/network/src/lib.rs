use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};

use bevy::prelude::*;
use bevy_quinnet::{client::QuinnetClientPlugin, server::QuinnetServerPlugin, shared::ClientId};
use serde::{Deserialize, Serialize};

pub use bevy_quinnet::client::Client;
pub use bevy_quinnet::server::Server;

mod client;
mod server;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NetworkEvent>();
        app.add_plugins((
            QuinnetServerPlugin::default(),
            QuinnetClientPlugin::default(),
        ));
        app.add_systems(Startup, server::start_server);
        app.add_systems(
            Update,
            (
                server::handle_client_messages,
                server::handle_disconnect_events,
                client::handle_server_messages,
                client::handle_connection_events,
                handle_internal_client_events,
            ),
        );
        app.add_systems(PostUpdate, client::on_app_exit);
        app.insert_resource(Users::default());
    }
}

#[derive(Event)]
pub enum NetworkEvent {
    ClientConnect(String, u16),
}

fn handle_internal_client_events(
    mut client: ResMut<Client>,
    mut events: EventReader<NetworkEvent>,
) {
    for event in events.read() {
        match event {
            NetworkEvent::ClientConnect(host, port) => {
                let server_addr = IpAddr::V4(host.parse::<Ipv4Addr>().unwrap());
                let local_addr = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
                client
                    .open_connection(
                        bevy_quinnet::client::connection::ConnectionConfiguration::from_ips(server_addr, *port, local_addr, 0),
                        bevy_quinnet::client::certificate::CertificateVerificationMode::SkipVerification,
                    )
                    .unwrap();
                // TODO handle errors
            } // _ => {}
        }
    }
}

#[derive(Resource, Debug, Clone, Default)]
struct Users {
    names: HashMap<ClientId, String>,
    self_id: ClientId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Join { name: String },
    Disconnect {},
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    ClientConnected {
        client_id: ClientId,
        username: String,
    },
    ClientDisconnected {
        client_id: ClientId,
    },
    InitClient {
        client_id: ClientId,
        usernames: HashMap<ClientId, String>,
    },
}
