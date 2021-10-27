use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tecla {
    pub pressed: bool,
    pub key: u8,
    pub special: bool
}