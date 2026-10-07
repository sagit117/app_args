use std::{error::Error, fmt};

/// enum с вариантами ошибок
#[derive(Debug)]
pub enum ArgError {
    InvalidName,        // Не корректное имя аргумента (или его отсутствие)
    NoneValue(String),  // Отсутствие значение аргумента, когда олно ожидается
}

/// Реализаия Display для вывода понятного сообщения пользователю
impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgError::InvalidName => write!(f, "Не корректное имя аргумента"),
            ArgError::NoneValue(arg_name) => write!(f, "Ожидалось значение для аргумента {}", arg_name),
        }
    }
}

/// Реализаия пустого трейта Error
impl Error for ArgError {}