use actix::prelude::{Message, Recipient};
use uuid::Uuid;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;



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
    pub typ: RTCSessionDescription,
    // pub sdp: u32,
    pub self_id: Uuid,
    pub room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct IceCandidate{
    pub candidate: String,
    pub self_id: Uuid,
    pub room_id: Uuid,
}


#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Event {
    IceCandidate{candidate: String, self_id: Uuid, room_id: Uuid},
    OfferAnswer{typ: RTCSessionDescription, self_id: Uuid, room_id: Uuid}
}
