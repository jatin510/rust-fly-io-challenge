use serde::{self, Deserialize, Serialize};
use std::io::{self, Error, Read};

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

fn handle_message(msg: Message) -> Message {
    let reply = match msg.body.payload {
        Payload::Init {
            node_id: _,
            node_ids: _,
        } => Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: Some(1),
                in_reply_to: msg.body.msg_id,
                payload: Payload::InitOk {},
            },
        },
        Payload::Echo { echo } => Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: Some(1),
                in_reply_to: msg.body.msg_id,
                payload: Payload::EchoOk { echo },
            },
        },
        _ => Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: Some(1),
                in_reply_to: msg.body.msg_id,
                payload: Payload::InitOk {},
            },
        },
    };

    return reply;
}

fn main() -> Result<(), Error> {
    let stdin = io::stdin(); // Get the standard input handle
    let mut handle = stdin.lock(); // Lock stdin for efficient reading

    let mut input = String::new();
    handle.read_to_string(&mut input)?; // Read all data into input

    // println!("input {:?}", input);
    // Deserialize the input string into a Vec<Message>
    let messages: Vec<Message> = serde_json::from_str(&input.trim())?;
    //

    for message in messages {
        let output_message = handle_message(message);

        let reply = serde_json::to_string(&output_message).expect("the message to be serializable");
        println!("{}", reply);
        // println!("{:?}", output_message);
    }

    // let stdin = io::stdin().lock();
    // let messages = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();
    // println!("hello world3");

    // Print the deserialized Vec<Message> for verification
    // println!("Deserialized messages: {:?}", messages);

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_message_init() {
        let init_msg = Message {
            src: "client1".to_string(),
            dest: "server1".to_string(),
            body: Body {
                msg_id: Some(10),
                in_reply_to: None,
                payload: Payload::Init {
                    node_id: "node123".to_string(),
                    node_ids: vec!["node456".to_string()],
                },
            },
        };

        let reply = handle_message(init_msg);
        assert_eq!(reply.src, "server1");
        assert_eq!(reply.dest, "client1");
        // assert_matches!(reply.body.payload, Payload::InitOk {});
    }

    #[test]
    fn handle_message_echo() {
        let echo_msg = Message {
            src: "client2".to_string(),
            dest: "server2".to_string(),
            body: Body {
                msg_id: Some(20),
                in_reply_to: None,
                payload: Payload::Echo {
                    echo: "Hello!".to_string(),
                },
            },
        };

        let reply = handle_message(echo_msg);
        assert_eq!(reply.src, "server2");
        assert_eq!(reply.dest, "client2");
        if let Payload::EchoOk { echo } = reply.body.payload {
            assert_eq!(echo, "Hello!");
        } else {
            panic!("Expected Payload::EchoOk");
        }
    }
}
