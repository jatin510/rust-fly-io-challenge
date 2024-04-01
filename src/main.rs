use serde::{self, Deserialize, Serialize};
use std::io::{self, Error, Read};

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Debug, Serialize, Deserialize)]
struct Body {
    msg_id: Option<u64>,
    in_reply_to: Option<u64>,

    echo: String,
    #[serde(rename = "type")]
    body_type: String,

    node_id: String,
    node_ids: Vec<String>,
}

// nodes prefix  - n
// client prefix - p

// if body type = echo
// response body type = echo_ok
// in_reply_to message id
//

fn handle_message(msg: Message) -> Message {
    let response_body_type = match msg.body.body_type.as_str() {
        "echo" => "echo_ok".to_string(),
        "init" => "init_ok".to_string(),
        _ => "unknown".to_string(),
    };

    let new_message = Message {
        src: msg.dest,
        dest: msg.src,
        body: Body {
            msg_id: msg.body.msg_id,
            in_reply_to: msg.body.msg_id,
            echo: msg.body.echo,
            body_type: response_body_type,
            node_id: msg.body.node_id,
            node_ids: msg.body.node_ids,
        },
    };

    return new_message;
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

        println!("{:?}", output_message);
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
    use super::*; // Import everything from the outer module

    #[test]
    fn test_handle_message_echo() {
        // Setup a sample message with `body_type` set to "echo"
        let test_message = Message {
            src: "c1".to_string(),
            dest: "n1".to_string(),
            body: Body {
                echo: "Test echo message".to_string(),
                body_type: "echo".to_string(),
                msg_id: Some(123),
                in_reply_to: None,
                node_id: "node_1".to_string(),
                node_ids: vec!["node_2".to_string(), "node_3".to_string()],
            },
        };

        // Call `handle_message` with the test message
        let response_message = handle_message(test_message);

        // Assertions to verify the response message's properties
        assert_eq!(response_message.src, "n1");
        assert_eq!(response_message.dest, "c1");
        assert_eq!(response_message.body.body_type, "echo_ok");
        assert_eq!(response_message.body.msg_id, Some(123));
        assert_eq!(response_message.body.in_reply_to, Some(123));
        assert_eq!(response_message.body.echo, "Test echo message");
        assert_eq!(response_message.body.node_id, "node_1");
        assert_eq!(response_message.body.node_ids, vec!["node_2", "node_3"]);
    }

    #[test]
    fn test_handle_message_init() {
        // Setup a sample message with `body_type` set to "init"
        let test_message = Message {
            src: "c1".to_string(),
            dest: "n1".to_string(),
            body: Body {
                echo: "Test init message".to_string(),
                body_type: "init".to_string(),
                msg_id: Some(456),
                in_reply_to: None,
                node_id: "node_x".to_string(),
                node_ids: vec!["node_y".to_string(), "node_z".to_string()],
            },
        };

        // Call `handle_message` with the test message
        let response_message = handle_message(test_message);

        // Assertions to verify the response message's properties
        assert_eq!(response_message.src, "n1");
        assert_eq!(response_message.dest, "c1");
        assert_eq!(response_message.body.body_type, "init_ok");
        assert_eq!(response_message.body.msg_id, Some(456));
        assert_eq!(response_message.body.in_reply_to, Some(456));
        assert_eq!(response_message.body.echo, "Test init message");
        assert_eq!(response_message.body.node_id, "node_x");
        assert_eq!(response_message.body.node_ids, vec!["node_y", "node_z"]);
    }
}
