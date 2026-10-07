use crate::Arg;

#[derive(Debug)]
pub struct Matches {
    arg: Vec<Arg>
}

impl Matches {
    pub(crate) fn new(arg: Vec<Arg>) -> Matches {
        Matches { arg }
    } 

    pub fn get_value(arg: &str) -> Option<String> {
        None
    }

    pub fn get_flag(arg: &str) -> bool {
        false
    }

    pub fn get_position_at(index: u32) -> Option<String> {
        None
    }
}