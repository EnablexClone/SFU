use std::time::{Duration, Instant};
use actix::dev::ContextFutureSpawner;
use actix::{fut, ActorContext, StreamHandler, Handler, WrapFuture, ActorFutureExt};
use actix::{Addr, Actor, AsyncContext};
use actix_web_actors::ws;
use uuid::Uuid;


use super::lobby::Lobby;
use super::messages::{Connect, Disconnect, WsMessage, IceCandidate, OfferAnswer, Event, RemovePeer, AddPeer};


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsConn {
    lobby_addr: Addr<Lobby>,
    heartbeat: Instant,
    session_id: Uuid,
    room_id: Uuid,
}


impl WsConn {
    pub fn new(lobby: Addr<Lobby>, session_id: Uuid, room_id: Uuid) -> WsConn {
        WsConn { 
            lobby_addr: lobby,
            heartbeat: Instant::now(),
            session_id: session_id,
            room_id: room_id
        }
    }

    pub fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.heartbeat) > CLIENT_TIMEOUT {
                println!("Disconnsecting failed heartbeat");
                ctx.stop();
                return;
            }

            ctx.ping(b"hi");
        });
    }

}


impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;


    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);

        let addr = ctx.address();
        self.lobby_addr
            .send(
                Connect {
                    addr: addr.recipient(),
                    self_id: self.session_id,
                    room_id: self.room_id
                }
            )
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }


    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        self.lobby_addr.do_send(Disconnect {
            self_id: self.session_id,
            room_id: self.room_id
        });
        actix::Running::Stop
    }

    
}




impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Ping(msg)) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heartbeat= Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => {
                ctx.binary(bin)
            },
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(ws::Message::Text(s)) => {
                let parse: Event = serde_json::from_str(s.to_string().as_str()).expect("cannot convert");
                match parse {
                    Event::IceCandidate{candidate, self_id, room_id} => {
                        self.lobby_addr.do_send(IceCandidate{candidate, self_id, room_id})
                    },
                    Event::OfferAnswer { typ, self_id, room_id } => {
                        self.lobby_addr.do_send(OfferAnswer{typ, self_id, room_id})
                    },
                    Event::RemovePeer { self_id, room_id } => {
                        self.lobby_addr.do_send(RemovePeer{self_id, room_id})
                    },
                    Event::AddPeer { self_id, room_id } => {
                        self.lobby_addr.do_send(AddPeer{self_id, room_id})
                    }
                }
                let x = WsMessage(s.to_string());
                self.lobby_addr.do_send(x);
            }
            Err(e) => std::panic::panic_any(e),
        }
    }
}



impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}


impl Handler<IceCandidate> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: IceCandidate, ctx: &mut Self::Context) -> Self::Result {
        let body = serde_json::to_string(&msg);
        match body {
            Ok(val) => ctx.text(val),
            Err(e) => { println!("Failed to parse: {:?}\nAdress: {:?}", e, ctx.address())}
        }
    }
}

impl Handler<OfferAnswer> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: OfferAnswer, ctx: &mut Self::Context) -> Self::Result {
        let body = serde_json::to_string(&msg);
        match body {
            Ok(val) => ctx.text(val),
            Err(e) => { println!("Failed to parse: {:?}\nAdress: {:?}", e, ctx.address())}
        }
    }
}


impl Handler<RemovePeer> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: RemovePeer, ctx: &mut Self::Context) -> Self::Result {
        let body = serde_json::to_string(&msg);
        match body {
            Ok(val) => ctx.text(val),
            Err(e) => { println!("Failed to parse: {:?}\nAdress: {:?}", e, ctx.address())}
        }
    }
}


impl Handler<AddPeer> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: AddPeer, ctx: &mut Self::Context) -> Self::Result {
        let body = serde_json::to_string(&msg);
        match body {
            Ok(val) => ctx.text(val),
            Err(e) => { println!("Failed to parse: {:?}\nAdress: {:?}", e, ctx.address())}
        }
    }
}

