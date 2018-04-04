use websocket::async::futures::stream::{SplitSink,SplitStream};
use websocket::client::async::Client;


/// sink half of a websocket client.
pub type Sender<S> = SplitSink<Client<S>>;


/// stream half of a websocket client.
pub type Receiver<S> = SplitStream<Client<S>>;


