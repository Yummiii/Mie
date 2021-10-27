use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tecla {
    pub pressed: bool,
    pub key: u16,
    pub special: bool
}