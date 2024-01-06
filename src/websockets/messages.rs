use actix::prelude::{Message, Recipient};
use uuid::Uuid;



#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub self_id: Uuid,
    pub room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug)]
pub struct Disconnect {
    pub self_id: Uuid,
    pub room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Debug)]
pub struct OfferAnswer{
    typ: String,
    sdp: u32,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct IceCandidate{
    candidate: String
}