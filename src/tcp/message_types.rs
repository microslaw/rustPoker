use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};


#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum ClientMessageTypes {
    JoinGameAcknowledgement,
    NextTrunAcknowledgement,
    SpectatorNextTurnAcknowledgement,
    GameResultAcknowledgement,
    PlayCard,
}

impl fmt::Display for ClientMessageTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let txt: String = match self {
            ClientMessageTypes::JoinGameAcknowledgement => "JoinGameAcknowledgement".to_string(),
            ClientMessageTypes::NextTrunAcknowledgement => "NextTrunAcknowledgement".to_string(),
            ClientMessageTypes::SpectatorNextTurnAcknowledgement => {
                "SpectatorNextTrunAcknowledgement".to_string()
            }
            ClientMessageTypes::GameResultAcknowledgement => {
                "GameResultAcknowledgement".to_string()
            }
            ClientMessageTypes::PlayCard => "PlayCard".to_string(),
        };

        write!(f, "<{}>", txt)
    }
}


#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum ServerMessageTypes {
    SpectatorNextTurn,
    JoinGame,
    NextTurn,
    GameResult,
}

impl fmt::Display for ServerMessageTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let txt: String = match self {
            ServerMessageTypes::SpectatorNextTurn => "SpectatorNextTurn".to_string(),
            ServerMessageTypes::JoinGame => "JoinGame".to_string(),
            ServerMessageTypes::NextTurn => "NextTurn".to_string(),
            ServerMessageTypes::GameResult => "GameResult".to_string(),
        };

        write!(f, "<{}>", txt)
    }
}

