#[derive(Debug, Serialize, Deserialize)]
struct Message {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Payload {
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
}

#[derive(Debug, Serialize, Deserialize)]
struct Body {
    msg_id: Option<u64>,
    in_reply_to: Option<u64>,

    #[serde(flatten)]
    payload: Payload,
}

// nodes prefix  - n
// client prefix - p

// if body type = echo
// response body type = echo_ok
// in_reply_to message id
//
