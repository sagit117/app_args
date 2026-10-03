use std::{borrow::Borrow, env, error::Error, fmt};

/// enum с вариантами ошибок
#[derive(Debug)]
pub enum ArgError {
    InvalidName,
}

/// Реализаия Display для вывода понятного сообщения пользователю
impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgError::InvalidName => write!(f, "Не корректное имя аргумента"),
        }
    }
}

/// Реализаия пустого трейта Error
impl Error for ArgError {}

#[derive(Eq, Hash, PartialEq, Debug)]
enum Token {
    Short(char),      // Передаем символов для поддержки -x
    Long(String),     // Цельное имя
    Value(String),    // Значение или позиционный аргумент
    DoubleDash,       // Сигнал остановиться, все следующие аргументы позиционные
}

/// Струкатура для хранения типов аргумента
#[derive(Debug)]
pub enum ArgKind {
    Short(char),
    Long(String),
    ShortLong(String),
    Position
}

impl ArgKind {
    /// Вывод имени аргумента с символами --
    fn name(&self) -> Option<String> {
        match self {
            ArgKind::Short(c) => Some(format!("-{}", c)),
            ArgKind::Long(s) => Some(format!("--{}", s)),
            ArgKind::ShortLong(s) => Some(format!("--{}", s)),
            ArgKind::Position => None,
        }
    }

    fn is_short_long(&self) -> bool {
        match self {
            ArgKind::ShortLong(_) => true,
            _ => false
        }
    }
}

/// Структура для хранения данных аргумента
#[derive(Debug)]
pub struct Arg {
    arg_kind: ArgKind,
    is_flag: bool,
    description: String,
    value: Option<String>
}

impl Arg {
    fn new(arg_kind: ArgKind, is_flag: bool, description: impl Into<String>) -> Arg {
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

    // Хелпер для создания Short аргументов
    pub fn short(name: char, is_flag: bool, description: impl Into<String>) -> Result<Arg, ArgError> {
        if name == ' ' || name == '\0' {
            return Err(ArgError::InvalidName)
        }

        Ok(Arg::new(ArgKind::Short(name), is_flag, description))
    }

    // Хелпер для создания ShortLong аргументов
    pub fn short_long(name: impl Into<String>, is_flag: bool, description: impl Into<String>) -> Result<Arg, ArgError> {
        let name_str = name.into();

        if name_str.is_empty() {
            return Err(ArgError::InvalidName)
        }

        Ok(Arg::new(ArgKind::ShortLong(name_str), is_flag, description))
    }

    // Хелпер для создания Long аргументов
    pub fn long(name: impl Into<String>, is_flag: bool, description: impl Into<String>) -> Result<Arg, ArgError> {
        let name_str = name.into();

        if name_str.is_empty() {
            return Err(ArgError::InvalidName)
        }

        Ok(Arg::new(ArgKind::Long(name_str), is_flag, description))
    }

    // Хелпер для создания Position аргументов
    pub fn position(description: impl Into<String>) -> Arg {
        Arg::new(ArgKind::Position, false, description)
    }
}

pub fn parse<T, I>(user_args: T) 
where 
    T: AsRef<[I]>,
    I: Borrow<Arg>, 
{
    let tokens = get_tokens(env::args().skip(1).collect());


    println!("{:?}", tokens);
}

fn get_tokens(args: Vec<String>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut is_double_dash = false;

    for arg in args.into_iter() {
        if is_double_dash { // если был аргумент --, значит все следующие аргементы позиционные
            tokens.push(Token::Value(arg));
            continue;
        }

        if arg.eq("--") {
            tokens.push(Token::DoubleDash);
            is_double_dash = true;
            continue;
        }

        if arg.starts_with("--") {
            if let Some(split_sub_str) = arg.split_once("=") {
                tokens.push(Token::Long(split_sub_str.0.to_owned()));
                tokens.push(Token::Value(split_sub_str.1.to_owned()));
                continue;
            }

            tokens.push(Token::Long(arg.to_owned()));
            continue;
        }

        if arg.starts_with("-") && arg.len() > 1 {
            if let Some(split_sub_str) = arg.split_once("=") {
                add_short_tokens(&mut tokens, split_sub_str.0);
                tokens.push(Token::Value(split_sub_str.1.to_owned()));
                continue;
            }

            add_short_tokens(&mut tokens, &arg);
        } else {
            tokens.push(Token::Value(arg));
        }
    }

    tokens
}

fn add_short_tokens(tokens: &mut Vec<Token>, arg: &str) {
    for c in arg.chars().skip(1) {
        tokens.push(Token::Short(c));
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         parse();
//     }
// }
