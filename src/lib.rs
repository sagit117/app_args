use crate::token::PreparedToken;
use crate::token::PrepareTokens;
use crate::err::ArgError;
use crate::token::convert_to_tokens;
use std::{borrow::{Borrow, BorrowMut}, env};

mod err;
mod token;
pub mod builder;
pub mod matches;

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
        
        argument.value = tokens.get_value_arg(&argument.arg_kind, argument.is_flag)?;
    }

    Ok(())
}

