

pub struct peerConnection<T>{
 peer:HashMap<String,T>,
}

trait base{
    fn reinitialiseHashmap<T>(&self)->peerConnection<T>;
}
