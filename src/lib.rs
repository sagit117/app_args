use std::{borrow::Borrow, env};

#[derive(Debug)]
enum Token {
    Short(Vec<char>), // Передаем массив символов для поддержки -xvf
    Long(String),     // Цельное имя
    Value(String),    // Значение или позиционный аргумент
    DoubleDash,       // Сигнал остановиться
}

pub enum ArgKind {
    Short(char),
    Long(String),
    ShortLong(String),
    Position
}

pub struct Arg {
    arg_kind: ArgKind,
    is_flag: bool,
    description: String,
    value: Option<String>
}

impl Arg {
    pub fn new(arg_kind: ArgKind, is_flag: bool, description: impl Into<String>) -> Arg {
        Arg { 
            arg_kind, 
            is_flag,
            description: description.into(),
            value: None
        }
    }

    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    // Хелпер для создания ShortLong аргументов из &str
    pub fn short_long(name: impl Into<String>, is_flag: bool, description: impl Into<String>) -> Arg {
        Arg::new(ArgKind::ShortLong(name.into()), is_flag, description)
    }

    //  Хелпер для создания Long аргументов из &str
    pub fn long(name: impl Into<String>, is_flag: bool, description: impl Into<String>) -> Arg {
        Arg::new(ArgKind::Long(name.into()), is_flag, description)
    }
}

pub fn parse<T, I>(user_args: T) 
where 
    T: AsRef<[I]>,
    I: Borrow<Arg>, 
{
    let _args: Vec<String> = env::args().skip(1).collect();
    
}

// fn parse_args(args: Vec<String>) -> Vec<Token> {
//     let mut tokens: Vec<Token> = Vec::new();
//     let mut is_double_dash = false;
    

//     for (i, sub_str) in args.into_iter().enumerate() {
//         if is_double_dash {
//             tokens.push(Token::Value(sub_str));
//             continue;
//         }

//         if sub_str.eq("--") {
//             tokens.push(Token::DoubleDash);
//             is_double_dash = true;
//             continue;
//         }

//         if sub_str.starts_with("--") {
//             if let Some(split_sub_str) = sub_str.split_once("=") {
//                 tokens.push(Token::Long(split_sub_str.0.to_owned()));
//                 tokens.push(Token::Value(split_sub_str.1.to_owned()));
//                 continue;
//             }

//             tokens.push(Token::Long(sub_str.to_owned()));
//             continue;
//         }
//     }

//     tokens
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         parse();
//     }
// }
