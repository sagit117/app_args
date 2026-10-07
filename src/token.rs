use crate::{ArgKind, err::ArgError};

/// Структура токенов, для парсинга аргументов
#[derive(Debug)]
pub enum Token {
    Short(char),      // Передаем символов для поддержки -x
    Long(String),     // Цельное имя
    Value(String),    // Значение или позиционный аргумент
}

/// Структура для хранения вектора токенов
pub struct PrepareTokens {
    pub(crate) tokens: Vec<PreparedToken>
}


/// Структура для хранения токена и флага об его обработке
pub struct PreparedToken {
    pub(crate) token: Token,
    pub(crate) is_prepared: bool
}

/// Реализаия операий с токенами
impl PrepareTokens {
    pub(crate) fn get_value_arg(&mut self, arg: &ArgKind, is_flag: bool) -> Result<Option<String>, ArgError> {
        return match arg {
            ArgKind::Short(name) => {
                self.prepared_short_token(name, is_flag)
            },
            ArgKind::Long(name) => {
                self.prepared_long_token(name, is_flag)
            },
            ArgKind::ShortLong(name) => {
                let long_result = self.prepared_long_token(name, is_flag)?;
                match long_result {
                    Some(_) => Ok(long_result),
                    None => self.prepared_short_token(&name.chars().nth(0).unwrap(), is_flag),
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
