use crate::{Arg, ArgKind};

#[derive(Debug)]
pub struct Matches {
    args: Vec<Arg>
}

impl Matches {
    pub(crate) fn new(args: Vec<Arg>) -> Matches {
        Matches { args }
    } 

    pub fn get_value(&self, arg: &str) -> Option<&String> {
        if let Some(argument) = self.find_arg(Matches::strip_arg(arg)) {
            return argument.value();
        }

        None
    }

    pub fn get_flag(&self, arg: &str) -> bool {
        if let Some(argument) = self.find_arg(Matches::strip_arg(arg)) {
            return argument.value().is_some();
        }

        false
    }

    pub fn get_position_at(&self, index: u32) -> Option<&String> {
        let mut idx: u32 = 1;
        for arg in self.args.iter() {
            match arg.arg_kind {
                ArgKind::Position => {
                    if idx == index {
                        return arg.value()
                    }

                    idx += 1;
                },
                _ => continue
            } 
        }

        None
    }

    fn find_arg(&self, name: &str) -> Option<&Arg> {
        for arg in self.args.iter() {
            if arg.arg_kind.name().as_deref() == Some(name) {
                return Some(arg);
            }
        }

        None
    }

    fn strip_arg(name: &str) -> &str {
        let mut str = name;
        while str.starts_with('-') {
            str = &str[1..];
        }

        str
    }
}