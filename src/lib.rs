use std::{borrow::{Borrow, BorrowMut}, env, error::Error, fmt};

/// enum с вариантами ошибок
#[derive(Debug)]
pub enum ArgError {
    InvalidName,        // Не корректное имя аргумента (или его отсутствие)
    // InvalidFlagValue(String),
    NoneValue(String),  // Отсутствие значение аргумента, когда олно ожидается
}

/// Реализаия Display для вывода понятного сообщения пользователю
impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgError::InvalidName => write!(f, "Не корректное имя аргумента"),
            // ArgError::InvalidFlagValue(arg_name) => write!(f, "Ожидалось значение bool для аргумента {}", arg_name),
            ArgError::NoneValue(arg_name) => write!(f, "Ожидалось значение для аргумента {}", arg_name),
        }
    }
}

/// Реализаия пустого трейта Error
impl Error for ArgError {}

/// Структура токенов, для парсинга аргументов
#[derive(Debug)]
enum Token {
    Short(char),      // Передаем символов для поддержки -x
    Long(String),     // Цельное имя
    Value(String),    // Значение или позиционный аргумент
}

/// Структура для хранения вектора токенов
struct PrepareTokens {
    tokens: Vec<PreparedToken>
}


/// Структура для хранения токена и флага об его обработке
struct PreparedToken {
    token: Token,
    is_prepared: bool
}

/// Реализаия операий с токенами
impl PrepareTokens {
    fn get_value_arg(&mut self, arg: &Arg) -> Result<Option<String>, ArgError> {
        return match arg.arg_kind.borrow() {
            ArgKind::Short(name) => {
                self.prepared_short_token(name, arg.is_flag)
            },
            ArgKind::Long(name) => {
                self.prepared_long_token(name, arg.is_flag)
            },
            ArgKind::ShortLong(name) => {
                let long_result = self.prepared_long_token(name, arg.is_flag)?;
                match long_result {
                    Some(_) => Ok(long_result),
                    None => self.prepared_short_token(&name.chars().nth(0).unwrap(), arg.is_flag),
                }
            },
            ArgKind::Position => {
                self.prepared_position_token()
            },
        }
    }

    /// Метод присваивает значения позиционным аргументам из свободных value токенов 
    fn prepared_position_token(&mut self) -> Result<Option<String>, ArgError> {
        for index in 0..self.tokens.len() {
        
            // Проверяем текущий токен
            if self.tokens[index].is_prepared {
                continue;
            }

            // Проверяем, является ли он значением
            if let Token::Value(v) = &self.tokens[index].token {
                let result_str = v.to_owned();
                self.tokens[index].is_prepared = true;

                return Ok(Some(result_str))
            }
        }

        Ok(None)
    }

    ///  Метод присваивает значения аргументов из short(flag) токенов
    fn prepared_short_token(&mut self, name: &char, is_flag: bool) -> Result<Option<String>, ArgError> {
        let mut token_iter = self.tokens.iter_mut();

        while let Some(prepared_token) = token_iter.next() {
            match prepared_token.token {
                Token::Short(token_name) => {
                    if name == &token_name {
                        prepared_token.is_prepared = true;

                        return PrepareTokens::take_value_by_name(is_flag, token_iter.next(), &token_name.to_string());
                    }
                },
                _ => continue
            } 
        }

        Ok(None)
    }

    ///  Метод присваивает значения аргументов из long(flag) токенов
    fn prepared_long_token(&mut self, name: &str, is_flag: bool) -> Result<Option<String>, ArgError> {
        let mut token_iter = self.tokens.iter_mut();

        while let Some(prepared_token) = token_iter.next() {
            match prepared_token.token {
                Token::Long(ref token_name) => {
                    if name.eq(token_name) {
                        prepared_token.is_prepared = true;

                        return PrepareTokens::take_value_by_name(is_flag, token_iter.next(), token_name);
                    }
                },
                _ => continue,
            }
        }

        Ok(None)
    }

    fn take_value_by_name(is_flag: bool, next_token: Option<&mut PreparedToken>, token_name: &str) -> Result<Option<String>, ArgError> {
        if is_flag {
            Ok(Some(true.to_string()))
        } else {
            if let Some(next_prepared_token) = next_token {
                return match next_prepared_token.token {
                    Token::Value(ref v) => return {     
                        next_prepared_token.is_prepared = true;
                        Ok(Some(v.to_owned()))
                    } ,
                    _ => Err(ArgError::NoneValue(token_name.to_string()))
                }
            }

            Err(ArgError::NoneValue(token_name.to_string()))
        }


    }
}

/// Струкатура для хранения типов аргумента
#[derive(Debug)]
pub enum ArgKind {
    Short(char),
    Long(String),
    ShortLong(String),
    Position
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

/// Функия присваивает пользовательским переменным значения из аргументов приложения.
/// Основная функция парсинга данных.
pub fn parse<T, I>(mut user_args: T) -> Result<(), ArgError>
where 
    T: AsMut<[I]>,
    I: BorrowMut<Arg> + Borrow<Arg>,
{
    let mut tokens = PrepareTokens { 
        tokens: convert_to_tokens(env::args().skip(1).collect())
            .into_iter()
            .map(|t| PreparedToken { token: t, is_prepared: false })
            .collect()
    };

    let mut_user_arg = user_args.as_mut();

    // Сортировка: ArgKind::Position уходит в самый конец
    mut_user_arg.sort_by_key(|item| {
        let arg: &Arg = item.borrow();
        
        // Используем match для определения приоритета (ключа сортировки)
        match arg.arg_kind {
            ArgKind::Position => 1, // Самый большой приоритет — улетят в конец
            _ => 0,                 // Все остальные аргументы — останутся в начале
        }
    });

    for arg in mut_user_arg.iter_mut() {
        let argument = arg.borrow_mut();
        // Передаем в парсер токенов ссылку, но значение присваиваем через мутабельный доступ
        argument.value = tokens.get_value_arg(argument)?;
    }

    Ok(())
}

fn convert_to_tokens(args: Vec<String>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut stop_parse = false;

    for arg in args.into_iter() {
        if arg.eq("--") {
            // tokens.push(Token::DoubleDash);
            stop_parse = true;
            continue;
        }

        if arg.starts_with("--") && !stop_parse {
            if let Some(split_sub_str) = arg.split_once('=') {
                tokens.push(Token::Long(split_sub_str.0[2..].to_owned()));
                tokens.push(Token::Value(split_sub_str.1.to_owned()));
            } else {
                tokens.push(Token::Long(arg[2..].to_owned()));
            }

            continue;
        }

        if arg.starts_with('-') && arg.len() > 1 && !stop_parse {
            if let Some(split_sub_str) = arg.split_once('=') {
                add_short_tokens(&mut tokens, split_sub_str.0);
                tokens.push(Token::Value(split_sub_str.1.to_owned()));
            } else {
                add_short_tokens(&mut tokens, &arg);
            }            
        } 

        tokens.push(Token::Value(arg));
    }

    tokens
}

fn add_short_tokens(tokens: &mut Vec<Token>, arg: &str) {
    for c in arg.chars().skip(1) {
        tokens.push(Token::Short(c));
    }
}

