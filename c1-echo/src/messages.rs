use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub src: String,
    pub dest: String,
    pub body: InputBody,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InputBody {
    #[serde(rename = "init")]
    Init(Init),
    #[serde(rename = "echo")]
    Echo(Echo),
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    pub src: String,
    pub dest: String,
    pub body: OutputBody,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OutputBody {
    #[serde(rename = "init_ok")]
    InitOk(InitOk),
    #[serde(rename = "echo_ok")]
    EchoOk(EchoOk),
    #[serde(rename = "error")]
    Error(ErrorMsg),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorMsg {
    pub in_reply_to: usize,
    pub code: usize,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct Init {
    pub msg_id: usize,
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct InitOk {
    pub msg_id: usize,
    pub in_reply_to: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Echo {
    pub msg_id: usize,
    pub echo: String,
}

#[derive(Serialize, Deserialize)]
pub struct EchoOk {
    pub msg_id: usize,
    pub in_reply_to: usize,
    pub echo: String,
}
