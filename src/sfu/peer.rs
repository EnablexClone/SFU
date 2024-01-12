use webrtc::{api::setting_engine::SettingEngine, peer_connection::configuration::RTCConfiguration, rtp_transceiver::rtp_codec::{RTCRtpHeaderExtensionCapability, RTPCodecType}, ice_transport::ice_server::RTCIceServer};



pub struct PeerConfig {
    pub setting_engine: SettingEngine,
    pub rtc_config: RTCConfiguration,
    pub header_extensions: Vec<(RTCRtpHeaderExtensionCapability, RTPCodecType)>,
}


impl Default for PeerConfig {
    fn default() -> PeerConfig {
        PeerConfig {
            setting_engine: SettingEngine::default(),
            rtc_config: RTCConfiguration {
                ice_servers: vec![RTCIceServer {
                    urls: vec!["stun:stun.l.google.com:19302".to_owned()],
                    ..Default::default()
                }],
                ..Default::default()
            },
            header_extensions: vec![],
        }
    }
}