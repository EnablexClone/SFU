use std::collections::HashMap;



pub struct peerConnection<T>{
 peer:HashMap<String,T>,
}

trait base{
    fn reinitialiseHashmap<T>(&self)->peerConnection<T>;
}


impl peerConnection<String>{
    fn shareRTC(){
    let participants:HashMap<String,String>=HashMap::new();
    let WebsocketUUID=String::from("");
    for member in participants{
     let nameOfpeer=member.0;
     let RTCbridge=member.1;
     match nameOfpeer {
         WebSocketUUID=> Some("No"),
         _=> sharePeer()
     }
    }
    }
}


