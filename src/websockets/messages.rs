use actix::prelude::{Message, Recipient};
use uuid::Uuid;
use webrtc::{peer_connection::sdp::session_description::RTCSessionDescription, ice_transport::ice_candidate::RTCIceCandidate};



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
    pub self_id: Uuid,
    pub room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct IceCandidate{
    pub candidate: RTCIceCandidate,
    pub self_id: Uuid,
    pub room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct RemovePeer {
    pub self_id: Uuid,
    pub room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct AddPeer {
    pub self_id: Uuid,
    pub room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Event {
    IceCandidate{candidate: RTCIceCandidate, self_id: Uuid, room_id: Uuid},
    OfferAnswer{typ: RTCSessionDescription, self_id: Uuid, room_id: Uuid},
    RemovePeer{self_id: Uuid, room_id: Uuid},
    AddPeer{self_id: Uuid, room_id: Uuid}
}
