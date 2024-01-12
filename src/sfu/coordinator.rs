use webrtc::peer_connection::RTCPeerConnection;


pub struct Coordinator {
    pub sessions: Session
}


pub struct Session {
    pub participants: Vec<RTCPeerConnection>
}