use std::collections::HashMap;

use actix::{Recipient, Actor, Context, Handler};
use serde::Serialize;
use uuid::Uuid;
use webrtc::{ice_transport::ice_candidate::RTCIceCandidate, peer_connection::sdp::session_description::RTCSessionDescription};
use super::messages::{WsMessage, Disconnect, Connect, OfferAnswer, IceCandidate};



type Socket = Recipient<WsMessage>;

#[derive(Debug)]
pub struct User {
    pub socket: Socket,
    sdp: Option<RTCSessionDescription>, 
    ice_candidate: Option<RTCIceCandidate>
}

impl User {
    pub fn new(socket: Socket) -> Self {
        Self{
            socket,
            sdp: None,
            ice_candidate: None
        }
    }

    pub fn set_sd(&mut self, sdp: &RTCSessionDescription) {
        self.sdp = Some(sdp.clone());
    }

    pub fn set_ice_candidate(&mut self, ice_candidate: &RTCIceCandidate) {
        self.ice_candidate = Some(ice_candidate.clone())
    }
}


#[derive(Debug)]
pub struct Room {
    pub participants: HashMap<Uuid, User>
}

impl Default for Room {
    fn default() -> Self {
        Self {
            participants: HashMap::new(),
        }
    }
}






#[derive(Debug)]
pub struct Lobby {
    pub sessions: HashMap<Uuid, Room>,
}

impl Default for Lobby {
    fn default() -> Self {
        Self { 
            sessions: HashMap::new(),
        }
    }
}

impl Lobby {
    fn send_connect(&mut self, message: &str, room_id: &Uuid, id_to: &Uuid) {
        if let Some(room) = self.sessions.get(room_id) {
            if let Some(socket_recipient) = room.participants.get(id_to) {
                let _ = socket_recipient.socket
                .do_send(
                    WsMessage(message.to_string())
                );
            }

        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }

    fn send_session_description(&mut self, usr: &Uuid, message: &OfferAnswer) {
        self.sessions.get(&message.room_id).unwrap()
            .participants.get(usr).unwrap()
            .socket.do_send(
                WsMessage(serde_json::to_string(message).unwrap())
            );
    }

    fn send_ice_candidate(&mut self, usr: &Uuid, message: &IceCandidate) {
        self.sessions.get(&message.room_id).unwrap()
        .participants.get(usr).unwrap()
        .socket.do_send(
            WsMessage(serde_json::to_string(message).unwrap())
        );
    }
}


impl Actor for Lobby {
    type Context = Context<Self>;
}


impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Self::Result {
        if  let Some(room) = self.sessions.get_mut(&msg.room_id) {
            room.participants.remove(&msg.self_id);
            if room.participants.is_empty() {
                self.sessions.remove(&msg.room_id);
            }
        }
        // TODO add remove peer for all participants of room 
    }
}


impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        if self.sessions.get(&msg.room_id).is_none() {
            self.sessions.insert(msg.room_id, Room::default());
        }
        if let Some(room) = self.sessions.get_mut(&msg.room_id) {
            room.participants.insert(msg.self_id, User::new(msg.addr));
        }
        let result = ConnectionResponse::new(msg.self_id, msg.room_id);
        self.send_connect(serde_json::to_string(&result).unwrap().as_str(), &msg.room_id, &msg.self_id)
    }
}


impl Handler<WsMessage> for Lobby { 
    type Result = ();

    fn handle(&mut self, _msg: WsMessage, _ctx: &mut Self::Context) -> Self::Result {
        
    }
}

impl Handler<OfferAnswer> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: OfferAnswer, _ctx: &mut Self::Context) -> Self::Result {
        // keys for sending the offer
        let mut keys: Vec<Uuid> = Vec::new();

        // for all users in room => send offer
        if let Some(room) = self.sessions.get_mut(&msg.room_id) {
            if let Some(usr) = room.participants.get_mut(&msg.self_id) {
                usr.set_sd(&msg.typ);
                for key in room.participants.keys() {
                    if *key != msg.self_id {
                        keys.push(*key)
                    }
                }
            }
        }
        for key in keys {
            self.send_session_description(&key, &msg);
        }

    }
}


impl Handler<IceCandidate> for Lobby {
    type Result = ();
    fn handle(&mut self, msg: IceCandidate, _ctx: &mut Self::Context) -> Self::Result {
        let mut keys: Vec<Uuid> = Vec::new();
        if let Some(room) = self.sessions.get_mut(&msg.room_id) {
            if let Some(usr) = room.participants.get_mut(&msg.self_id) {
                usr.set_ice_candidate(&msg.candidate);
                for key in room.participants.keys() {
                    if *key != msg.self_id {
                        keys.push(*key)
                    }
                }
            }
        }
        for key in keys {
            self.send_ice_candidate(&key, &msg)
        }
    }
}

#[derive(Serialize, Debug)]
struct ConnectionResponse {
    pub user_id: Uuid,
    pub room_id: Uuid,
}

impl ConnectionResponse {
    fn new(user_id: Uuid, room_id: Uuid) -> Self {
        Self { user_id, room_id }
    }
}
