use crate::Arg;

pub struct Builder {
    user_args: Vec<Arg>
}

impl Builder {
    pub fn arg(&mut self, arg: Arg) -> &mut Builder {
        self.user_args.push(arg);

        self
    }
}

pub fn new() -> Builder {
    Builder { 
        user_args: Vec::new()
    }
}