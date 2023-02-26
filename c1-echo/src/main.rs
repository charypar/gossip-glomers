mod messages;

use anyhow::Result;
use std::io::BufRead;

use messages::{EchoOk, InitOk, Input, InputBody, Output, OutputBody};

struct State {
    next_msg_id: usize,
}

fn main() {
    let stdin = std::io::stdin();

    let mut state = State { next_msg_id: 0 };

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => return,
        };

        match handle(&line, &mut state) {
            Ok(response) => println!("{response}"),
            Err(error) => eprintln!("{error}"),
            _ => {}
        }
    }
}

fn handle(msg: &str, state: &mut State) -> Result<String> {
    eprintln!("Handling: {}", msg);
    let message: Input = serde_json::from_str(msg)?;

    let response = match message.body {
        InputBody::Init(init) => Output {
            src: message.dest,
            dest: message.src,
            body: OutputBody::InitOk(InitOk {
                msg_id: state.next_msg_id,
                in_reply_to: init.msg_id,
            }),
        },
        InputBody::Echo(echo) => Output {
            src: message.dest,
            dest: message.src,
            body: OutputBody::EchoOk(EchoOk {
                msg_id: state.next_msg_id,
                in_reply_to: echo.msg_id,
                echo: echo.echo,
            }),
        },
    };

    state.next_msg_id += 1;
    let response = serde_json::to_string(&response)?;

    eprintln!("Responding with: {}", response);

    Ok(response)
}
