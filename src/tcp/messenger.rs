use std::net::TcpStream;
use crate::tcp::message_types::{ClientMessageTypes, ServerMessageTypes};


pub struct Messenger{
    stream: TcpStream,
}


impl Messenger{
    fn message_client(self, message_type: ServerMessageTypes, ){

    }

}

