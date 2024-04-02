use std::collections::HashMap;

use serde::{self, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {},
    Generate {},
    GenerateOk {
        id: u64,
    },
    Broadcast {
        message: u64,
    },
    BroadcastOk {},
    Read {},
    ReadOk {
        messages: Vec<u64>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk {},
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    pub msg_id: Option<u64>,
    pub in_reply_to: Option<u64>,

    #[serde(flatten)]
    pub payload: Payload,
}

// nodes prefix  - n
// client prefix - p

// if body type = echo
// response body type = echo_ok
// in_reply_to message id
//
