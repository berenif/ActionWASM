use bevy::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    RtcPeerConnection, RtcDataChannel, RtcDataChannelEvent,
    RtcPeerConnectionIceEvent, RtcIceCandidate, RtcSessionDescription,
    RtcSessionDescriptionInit, MessageEvent, WebSocket, BinaryType,
};
use js_sys::{Object, Reflect, JSON};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkState>()
            .init_resource::<PeerConnections>()
            .add_systems(Startup, setup_networking)
            .add_systems(Update, (
                handle_signaling_messages,
                handle_peer_messages,
                broadcast_game_state,
            ));
    }
}

#[derive(Resource)]
pub struct NetworkState {
    pub player_id: String,
    pub room_id: Option<String>,
    pub signaling_socket: Option<WebSocket>,
    pub is_connected: bool,
}

impl Default for NetworkState {
    fn default() -> Self {
        Self {
            player_id: generate_player_id(),
            room_id: None,
            signaling_socket: None,
            is_connected: false,
        }
    }
}

#[derive(Resource, Default)]
pub struct PeerConnections {
    pub connections: HashMap<String, RtcPeerConnection>,
    pub data_channels: HashMap<String, RtcDataChannel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum SignalingMessage {
    Join { room_id: String, player_id: String },
    Leave { player_id: String },
    Offer { from: String, to: String, sdp: String },
    Answer { from: String, to: String, sdp: String },
    IceCandidate { from: String, to: String, candidate: String },
    RoomUpdate { players: Vec<String> },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum GameMessage {
    PlayerUpdate {
        player_id: String,
        position: [f32; 2],
        velocity: [f32; 2],
    },
    GameStateSync {
        state: crate::game::GameState,
    },
    PlayerJoined {
        player_id: String,
    },
    PlayerLeft {
        player_id: String,
    },
}

fn generate_player_id() -> String {
    format!("player_{}", js_sys::Math::random().to_string().chars().skip(2).take(8).collect::<String>())
}

fn setup_networking(
    mut network_state: ResMut<NetworkState>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        // Connect to signaling server
        let ws_url = get_signaling_url();
        match WebSocket::new(&ws_url) {
            Ok(ws) => {
                ws.set_binary_type(BinaryType::Arraybuffer);
                
                // Set up WebSocket event handlers
                let onopen = Closure::wrap(Box::new(move || {
                    web_sys::console::log_1(&"Connected to signaling server".into());
                }) as Box<dyn FnMut()>);
                
                let onerror = Closure::wrap(Box::new(move |e: web_sys::ErrorEvent| {
                    web_sys::console::error_1(&format!("WebSocket error: {:?}", e.message()).into());
                }) as Box<dyn FnMut(web_sys::ErrorEvent)>);
                
                let onclose = Closure::wrap(Box::new(move |e: web_sys::CloseEvent| {
                    web_sys::console::log_1(&format!("WebSocket closed: code={}, reason={}", e.code(), e.reason()).into());
                }) as Box<dyn FnMut(web_sys::CloseEvent)>);
                
                ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
                ws.set_onerror(Some(onerror.as_ref().unchecked_ref()));
                ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
                
                onopen.forget();
                onerror.forget();
                onclose.forget();
                
                network_state.signaling_socket = Some(ws);
                network_state.is_connected = true;
            }
            Err(e) => {
                web_sys::console::error_1(&format!("Failed to create WebSocket: {:?}", e).into());
                network_state.is_connected = false;
            }
        }
    }
}

fn get_signaling_url() -> String {
    // Check if we're running locally or on GitHub Pages
    let window = match web_sys::window() {
        Some(w) => w,
        None => {
            web_sys::console::error_1(&"Failed to get window object".into());
            return "ws://localhost:8080".to_string();
        }
    };
    
    let location = window.location();
    let hostname = match location.hostname() {
        Ok(h) => h,
        Err(_) => {
            web_sys::console::error_1(&"Failed to get hostname".into());
            return "ws://localhost:8080".to_string();
        }
    };
    
    if hostname == "localhost" || hostname == "127.0.0.1" {
        "ws://localhost:8080".to_string()
    } else {
        // Use environment-based URL or fallback to a configurable server
        // Check for a meta tag or global config first
        if let Some(url) = get_signaling_url_from_config() {
            url
        } else {
            // Default to secure WebSocket with proper domain
            format!("wss://{}/ws", hostname.replace("github.io", "signaling.example.com"))
        }
    }
}

fn get_signaling_url_from_config() -> Option<String> {
    // Try to get signaling URL from a meta tag or global config
    if let Some(window) = web_sys::window() {
        if let Ok(document) = window.document().ok_or("no document") {
            // Look for meta tag with signaling server URL
            if let Ok(meta) = document.query_selector("meta[name='signaling-server']") {
                if let Some(meta_element) = meta {
                    if let Some(content) = meta_element.get_attribute("content") {
                        return Some(content);
                    }
                }
            }
        }
        
        // Check for global config object
        if let Ok(config) = js_sys::Reflect::get(&window, &"GAME_CONFIG".into()) {
            if let Ok(url) = js_sys::Reflect::get(&config, &"signalingUrl".into()) {
                if let Some(url_str) = url.as_string() {
                    return Some(url_str);
                }
            }
        }
    }
    None
}

fn handle_signaling_messages(
    network_state: Res<NetworkState>,
    mut peer_connections: ResMut<PeerConnections>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(ws) = &network_state.signaling_socket {
            // Handle incoming WebSocket messages
            // This would typically be done with event handlers
        }
    }
}

fn handle_peer_messages(
    peer_connections: Res<PeerConnections>,
    mut game_state: ResMut<crate::game::GameState>,
) {
    // Handle incoming messages from peer data channels
    for (peer_id, channel) in peer_connections.data_channels.iter() {
        // Process messages from each peer
    }
}

fn broadcast_game_state(
    peer_connections: Res<PeerConnections>,
    game_state: Res<crate::game::GameState>,
    player_query: Query<&crate::game::Player, With<crate::game::LocalPlayer>>,
) {
    if game_state.is_changed() {
        // Broadcast game state to all connected peers
        let message = GameMessage::GameStateSync {
            state: game_state.clone(),
        };
        
        if let Ok(json_str) = serde_json::to_string(&message) {
            for (_, channel) in peer_connections.data_channels.iter() {
                if channel.ready_state() == web_sys::RtcDataChannelState::Open {
                    let _ = channel.send_with_str(&json_str);
                }
            }
        }
    }
    
    // Broadcast local player updates
    for player in player_query.iter() {
        let message = GameMessage::PlayerUpdate {
            player_id: player.id.clone(),
            position: [player.position.x, player.position.y],
            velocity: [player.velocity.x, player.velocity.y],
        };
        
        if let Ok(json_str) = serde_json::to_string(&message) {
            for (_, channel) in peer_connections.data_channels.iter() {
                if channel.ready_state() == web_sys::RtcDataChannelState::Open {
                    let _ = channel.send_with_str(&json_str);
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn create_peer_connection(peer_id: String) -> Result<RtcPeerConnection, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("Failed to get window"))?;
    let rtc_peer_connection = Reflect::get(&window, &JsValue::from_str("RTCPeerConnection"))?;
    let rtc_peer_connection = rtc_peer_connection.dyn_into::<js_sys::Function>()?;
    
    let ice_servers = js_sys::Array::new();
    let stun_server = Object::new();
    Reflect::set(&stun_server, &JsValue::from_str("urls"), &JsValue::from_str("stun:stun.l.google.com:19302"))?;
    ice_servers.push(&stun_server);
    
    let config = Object::new();
    Reflect::set(&config, &JsValue::from_str("iceServers"), &ice_servers)?;
    
    let peer_connection = rtc_peer_connection.construct_with_args(&js_sys::Array::of1(&config))?;
    let peer_connection: RtcPeerConnection = peer_connection.dyn_into()?;
    
    Ok(peer_connection)
}

#[cfg(target_arch = "wasm32")]
pub fn create_data_channel(peer_connection: &RtcPeerConnection, label: &str) -> RtcDataChannel {
    let data_channel_init = web_sys::RtcDataChannelInit::new();
    data_channel_init.set_ordered(true);
    
    peer_connection.create_data_channel_with_data_channel_dict(label, &data_channel_init)
}