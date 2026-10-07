use crate::{Arg, err::ArgError};

pub struct Builder {
    user_args: Vec<Arg>
}

pub struct Matches {

}

impl Builder {
    pub fn arg(&mut self, arg: Arg) -> &mut Builder {
        self.user_args.push(arg);

        self
    }

    pub fn parse(&self) -> Matches {
        Matches {  }
    }
}

pub fn new() -> Builder {
    Builder { 
        user_args: Vec::new()
    }
}