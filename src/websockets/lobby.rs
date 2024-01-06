use std::collections::HashMap;

use actix::{Recipient, Actor, Context, Handler};
use serde::Serialize;
use uuid::Uuid;
use warp::filters::ws::Ws;

use super::messages::{WsMessage, Disconnect, Connect};



type Socket = Recipient<WsMessage>;
#[derive(Debug)]
pub struct Room {
    pub participants: HashMap<Uuid, Socket>
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
                let _ = socket_recipient
                .do_send(
                    WsMessage(message.to_string())
                );
            }

        } else {
            println!("attempting to send message but couldn't find user id.");
        }
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
    }
}


impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        if self.sessions.get(&msg.room_id).is_none() {
            self.sessions.insert(msg.room_id, Room::default());
        }
        if let Some(room) = self.sessions.get_mut(&msg.room_id) {
            room.participants.insert(msg.self_id, msg.addr);
        }
        let result = ConnectionResponse::new(msg.self_id, msg.room_id);
        self.send_connect(serde_json::to_string(&result).unwrap().as_str(), &msg.room_id, &msg.self_id)
    }
}


impl Handler<WsMessage> for Lobby { 
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
        
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
