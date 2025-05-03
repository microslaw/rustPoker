use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientMessageTypes {
    Handshake1,
    Handshake3,
    JoinGameAcknowledgement,
    NextTrunAcknowledgement,
    SpectatorNextTurnAcknowledgement,
    GameResultAcknowledgement,
    PlayCard,
}

impl fmt::Display for ClientMessageTypes{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let txt: String = match self {
            ClientMessageTypes::Handshake1 => "Handshake1".to_string(),
            ClientMessageTypes::Handshake3 => "Handshake3".to_string(),
            ClientMessageTypes::JoinGameAcknowledgement => "JoinGameAcknowledgement".to_string(),
            ClientMessageTypes::NextTrunAcknowledgement => "NextTrunAcknowledgement".to_string(),
            ClientMessageTypes::SpectatorNextTurnAcknowledgement => "SpectatorNextTrunAcknowledgement".to_string(),
            ClientMessageTypes::GameResultAcknowledgement => "GameResultAcknowledgement".to_string(),
            ClientMessageTypes::PlayCard => "PlayCard".to_string(),
        };

        write!(f,"<{}>", txt)
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerMessageTypes {
    Handshake2,
    SpectatorNextTurn,
    JoinGame,
    NextTurn,
    GameResult,
}

impl fmt::Display for ServerMessageTypes{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let txt: String = match self {
            ServerMessageTypes::Handshake2 => "Handshake2".to_string(),
            ServerMessageTypes::SpectatorNextTurn => "SpectatorNextTurn".to_string(),
            ServerMessageTypes::JoinGame => "JoinGame".to_string(),
            ServerMessageTypes::NextTurn => "NextTurn".to_string(),
            ServerMessageTypes::GameResult => "GameResult".to_string(),
        };

        write!(f,"<{}>", txt)
    }
}
