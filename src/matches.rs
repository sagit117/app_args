use crate::Arg;

#[derive(Debug)]
pub struct Matches {
    arg: Vec<Arg>
}

impl Matches {
    pub(crate) fn new(arg: Vec<Arg>) -> Matches {
        Matches { arg }
    } 
}