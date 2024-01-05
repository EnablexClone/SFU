#[derive(serde::Deserialize, serde::Serialize)]
pub struct offer_answer{
typ:string,
sdp:u32,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct iceCandidate{
    candidate:string
}