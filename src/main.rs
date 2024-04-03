use std::io::{self, Error};

use rand::Rng;
use std::io::BufRead;

mod protocol;
use protocol::*;

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
        Payload::Generate {} => {
            let mut rng = rand::thread_rng();
            let random_id: u64 = rng.gen();

            return Message {
                src: msg.dest,
                dest: msg.src,
                body: Body {
                    msg_id: Some(1),
                    in_reply_to: msg.body.msg_id,
                    payload: Payload::GenerateOk { id: random_id },
                },
            };
        }
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
    let stdin = io::stdin();
    let reader = stdin.lock();
    let buffer = io::BufReader::new(reader);

    for line in buffer.lines() {
        let line = line.expect("Failed to read line");
        let msg: Message = serde_json::from_str(&line).expect("Failed to parse message");

        let output_message = handle_message(msg);

        let reply = serde_json::to_string(&output_message).expect("the message to be serializable");
        println!("{}", reply);
    }

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
