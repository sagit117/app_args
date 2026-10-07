use std::env;

use crate::{Arg, matches::Matches, token::{PreparedToken, convert_to_tokens}};

pub struct Builder {
    user_args: Vec<Arg>,
    prepared_tokens: Vec<PreparedToken>
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
        user_args: Vec::new(),
        prepared_tokens: convert_to_tokens(env::args().skip(1).collect())
            .into_iter()
            .map(|t| PreparedToken { token: t, is_prepared: false })
            .collect()
    }
}

