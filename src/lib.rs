use std::{borrow::Borrow, collections::HashMap, env, error::Error, fmt};

// 1. Объявляем enum с вариантами ошибок
#[derive(Debug)]
pub enum ArgError {
    InvalidName,
}

// 2. Реализуем Display для вывода понятного сообщения пользователю
impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgError::InvalidName => write!(f, "Не корректное имя аргумента"),
        }
    }
}

// 3. Реализуем пустой трейт Error
impl Error for ArgError {}

#[derive(Debug)]
enum Token {
    Short(Vec<char>), // Передаем массив символов для поддержки -xvf
    Long(String),     // Цельное имя
    Value(String),    // Значение или позиционный аргумент
    DoubleDash,       // Сигнал остановиться
}

#[derive(Debug)]
pub enum ArgKind {
    Short(char),
    Long(String),
    ShortLong(String),
    Position
}

impl ArgKind {
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
    let args: Vec<String> = env::args().skip(1).collect();
    let mut map: HashMap<String, &I> = HashMap::new();

    let mut pos_i = 0;
    for arg in user_args.as_ref() {
        if let Some(name) = arg.borrow().arg_kind.name() {
            map.insert(name.clone(), arg);

            if arg.borrow().arg_kind.is_short_long() {
                map.insert(format!("-{}", name.chars().nth(2).expect("Пустое имя параметра")), arg);
            }
        } else {
            map.insert(format!("pos{}", pos_i), arg);
            pos_i +=1;
        }

        
    }

    for (i, sub_str) in args.into_iter().enumerate() {
        
    }
    
    
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
