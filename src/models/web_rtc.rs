
#[derive(serde::Deserialize, serde::Serialize)]
pub struct OfferAnswer{
    typ: String,
    sdp: u32,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct IceCandidate{
    candidate: String
}