use std::io::{self, Error};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

use rand::Rng;
use std::io::BufRead;

mod protocol;
use protocol::*;

// fn handle_message(
//     msg: Message,
//     messages: &mut Vec<u64>,
//     this_node_id: &String,
//     cluster_nodes: &mut Vec<String>,
// ) {
//     // let reply =
//     match msg.body.payload {
//         Payload::Init { node_id, node_ids } => {
//             this_node_id = &node_id;

//             let output_message = Message {
//                 src: msg.dest,
//                 dest: msg.src,
//                 body: Body {
//                     msg_id: Some(1),
//                     in_reply_to: msg.body.msg_id,
//                     payload: Payload::InitOk {},
//                 },
//             };

//             print_and_flush(output_message);
//         }
//         Payload::Echo { echo } => {
//             let output_message = Message {
//                 src: msg.dest,
//                 dest: msg.src,
//                 body: Body {
//                     msg_id: Some(1),
//                     in_reply_to: msg.body.msg_id,
//                     payload: Payload::EchoOk { echo },
//                 },
//             };

//             print_and_flush(output_message);
//         }
//         Payload::Generate {} => {
//             let mut rng = rand::thread_rng();
//             let random_id: u64 = rng.gen();

//             let output_message = Message {
//                 src: msg.dest,
//                 dest: msg.src,
//                 body: Body {
//                     msg_id: Some(1),
//                     in_reply_to: msg.body.msg_id,
//                     payload: Payload::GenerateOk { id: random_id },
//                 },
//             };

//             print_and_flush(output_message);
//         }
//         Payload::Broadcast { message } => {
//             messages.push(message);

//             for cluster_node in &cluster_nodes.to_vec() {
//                 if *cluster_node != this_node_id.to_string() {
//                     // internal communication
//                     let internal_message: Message = Message {
//                         src: this_node_id.clone(),
//                         dest: (*cluster_node).clone(),
//                         body: Body {
//                             in_reply_to: msg.body.in_reply_to,
//                             msg_id: msg.body.msg_id,
//                             payload: Payload::InternalMessage {
//                                 new_message: message,
//                             },
//                         },
//                     };

//                     print_and_flush(internal_message);
//                 }
//             }

//             let output_message = Message {
//                 src: msg.dest,
//                 dest: msg.src,
//                 body: Body {
//                     msg_id: Some(1),
//                     in_reply_to: msg.body.msg_id,
//                     payload: Payload::BroadcastOk {},
//                 },
//             };

//             print_and_flush(output_message);
//         }
//         Payload::Read {} => {
//             let output_message = Message {
//                 src: msg.dest,
//                 dest: msg.src,
//                 body: Body {
//                     msg_id: Some(1),
//                     in_reply_to: msg.body.msg_id,
//                     payload: Payload::ReadOk {
//                         messages: messages.to_vec(),
//                     },
//                 },
//             };

//             print_and_flush(output_message)
//         }
//         Payload::Topology { topology } => {
//             let output_message = Message {
//                 src: msg.dest,
//                 dest: msg.src,
//                 body: Body {
//                     msg_id: Some(1),
//                     in_reply_to: msg.body.msg_id,
//                     payload: Payload::TopologyOk {},
//                 },
//             };

//             print_and_flush(output_message)
//         }
//         Payload::InternalMessage { new_message } => messages.push(new_message),
//         _ => {
//             let output_message = Message {
//                 src: msg.dest,
//                 dest: msg.src,
//                 body: Body {
//                     msg_id: Some(1),
//                     in_reply_to: msg.body.msg_id,
//                     payload: Payload::InitOk {},
//                 },
//             };

//             print_and_flush(output_message)
//         } // return reply;
//     }
// }

fn print_and_flush(output_message: Message) {
    // let mut stdout = io::stdout();

    let reply = serde_json::to_string(&output_message).expect("the message to be serializable");
    println!("{}", reply);
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let buffer = io::BufReader::new(reader);

    let mut messages: Vec<u64> = Vec::new();
    let mut this_node_id = String::new();
    let mut cluster_nodes = Vec::<String>::new();

    let (msg_sender, msg_receiver): (Sender<Message>, Receiver<Message>) = channel();

    let handler = std::thread::spawn(move || -> anyhow::Result<()> {
        loop {
            for msg in msg_receiver.iter().take(50) {
                let serialized_output = serde_json::to_string(&msg)?;
                println!("{}", serialized_output);
            }
            std::thread::sleep(Duration::from_millis(400));
        }
        Ok(())
    });

    for line in buffer.lines() {
        let line = line.expect("Failed to read line");
        // let msg: Message = serde_json::from_str(&line).expect("Failed to parse message");
        let msg: Message = serde_json::from_str(&line)
            .unwrap_or_else(|_| panic!("Failed to parse message: {}", &line));

        // let output_message =
        // handle_message(msg, &mut messages, &mut this_node_id, &mut cluster_nodes);
        {
            // let reply =
            match msg.body.payload {
                Payload::Init { node_id, node_ids } => {
                    this_node_id = node_id;
                    cluster_nodes = node_ids;

                    let output_message = Message {
                        src: msg.dest,
                        dest: msg.src,
                        body: Body {
                            msg_id: Some(1),
                            in_reply_to: msg.body.msg_id,
                            payload: Payload::InitOk {},
                        },
                    };

                    print_and_flush(output_message);
                }
                Payload::Echo { echo } => {
                    let output_message = Message {
                        src: msg.dest,
                        dest: msg.src,
                        body: Body {
                            msg_id: Some(1),
                            in_reply_to: msg.body.msg_id,
                            payload: Payload::EchoOk { echo },
                        },
                    };

                    print_and_flush(output_message);
                }
                Payload::Generate {} => {
                    let mut rng = rand::thread_rng();
                    let random_id: u64 = rng.gen();

                    let output_message = Message {
                        src: msg.dest,
                        dest: msg.src,
                        body: Body {
                            msg_id: Some(1),
                            in_reply_to: msg.body.msg_id,
                            payload: Payload::GenerateOk { id: random_id },
                        },
                    };

                    print_and_flush(output_message);
                }
                Payload::Broadcast { message } => {
                    messages.push(message);

                    for cluster_node in &cluster_nodes.to_vec() {
                        if *cluster_node != this_node_id.to_string() {
                            // internal communication
                            let internal_message: Message = Message {
                                src: this_node_id.clone(),
                                dest: (*cluster_node).clone(),
                                body: Body {
                                    in_reply_to: msg.body.in_reply_to,
                                    msg_id: msg.body.msg_id,
                                    payload: Payload::InternalMessage {
                                        new_message: message,
                                    },
                                },
                            };

                            msg_sender.send(internal_message)?;
                        }
                    }

                    let output_message = Message {
                        src: msg.dest,
                        dest: msg.src,
                        body: Body {
                            msg_id: Some(1),
                            in_reply_to: msg.body.msg_id,
                            payload: Payload::BroadcastOk {},
                        },
                    };

                    print_and_flush(output_message);
                }
                Payload::Read {} => {
                    let output_message = Message {
                        src: msg.dest,
                        dest: msg.src,
                        body: Body {
                            msg_id: Some(1),
                            in_reply_to: msg.body.msg_id,
                            payload: Payload::ReadOk {
                                messages: messages.to_vec(),
                            },
                        },
                    };

                    print_and_flush(output_message)
                }
                Payload::Topology { topology } => {
                    let output_message = Message {
                        src: msg.dest,
                        dest: msg.src,
                        body: Body {
                            msg_id: Some(1),
                            in_reply_to: msg.body.msg_id,
                            payload: Payload::TopologyOk {},
                        },
                    };

                    print_and_flush(output_message)
                }
                Payload::InternalMessage { new_message } => messages.push(new_message),
                _ => {
                    let output_message = Message {
                        src: msg.dest,
                        dest: msg.src,
                        body: Body {
                            msg_id: Some(1),
                            in_reply_to: msg.body.msg_id,
                            payload: Payload::InitOk {},
                        },
                    };

                    print_and_flush(output_message)
                } // return reply;
            }
        }

        // let reply = serde_json::to_string(&output_message).expect("the message to be serializable");
        // println!("{}", reply);
    }

    let _ = handler.join().unwrap();

    return Ok(());
}
