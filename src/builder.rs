use crate::{Arg, err::ArgError};

pub struct Builder {
    user_args: Vec<Arg>
}

impl Builder {
    pub fn arg(&mut self, arg: Result<Arg, ArgError>) -> &mut Builder {
        self.user_args.push(arg.unwrap());

        self
    }
}

pub fn new() -> Builder {
    Builder { 
        user_args: Vec::new()
    }
}