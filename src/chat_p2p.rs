use chat::message::Message;
use webrtc::api::media_engine::MediaEngine;
use webrtc::api::APIBuilder;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::error::Result;

pub struct ChatP2P {
    peer_connection: RTCPeerConnection,
}

impl ChatP2P {
    pub async fn new() -> Result<Self> {
        let mut media_engine = MediaEngine::default();
        let api = APIBuilder::new().with_media_engine(media_engine).build();
        let config = RTCConfiguration::default();
        
        // Use the create_peer_connection function to create the RTCPeerConnection
        let peer_connection = api.new_peer_connection(config).await?;

        Ok(Self { peer_connection })
    }

    pub async fn set_remote_description(&self, sdp: &str) -> Result<()> {
        let description = RTCSessionDescription::offer(sdp.to_string())?;
        self.peer_connection.set_remote_description(description).await
    }

    pub async fn create_offer(&self) -> Result<String> {
        let offer = self.peer_connection.create_offer(None).await?;
        Ok(offer.sdp)
    }

    pub fn send_message(&self, message: &Message) {
        // Send message using WebRTC data channel
    }
}
