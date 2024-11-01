use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Options {
    pub user_prompt: String,
    //pub root_prompt: String,
    pub greet: String,
    //pub time: bool,
}
