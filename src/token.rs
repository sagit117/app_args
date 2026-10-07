use crate::{ArgKind, err::ArgError};

/// Структура токенов, для парсинга аргументов
#[derive(Debug)]
pub(crate) enum Token {
    Short(char),      // Передаем символов для поддержки -x
    Long(String),     // Цельное имя
    Value(String),    // Значение или позиционный аргумент
}

/// Структура для хранения вектора токенов
pub(crate) struct PrepareTokens {
    pub(crate) tokens: Vec<PreparedToken>
}


/// Структура для хранения токена и флага об его обработке
pub(crate) struct PreparedToken {
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
                    Some(_) => {
                        _ = self.prepared_short_token(&name.chars().nth(0).unwrap(), is_flag);
                        Ok(long_result)
                    },
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

pub(crate) fn convert_to_tokens(args: Vec<String>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::with_capacity(args.len() * 2);
    let mut stop_parse = false;

    for mut arg in args {
        if arg.eq("--") {
            stop_parse = true;
            continue;
        }

        if arg.starts_with("--") && !stop_parse {
            if let Some(pos) = arg.find('=') {
                // Разделяем String на две части без выделения новой памяти под первую часть
                let value = arg.split_off(pos + 1); 
                arg.truncate(pos); // Теперь в arg осталось "--flag"
                
                tokens.push(Token::Long(arg[2..].to_string()));
                tokens.push(Token::Value(value));
            } else {
                tokens.push(Token::Long(arg[2..].to_string()));
            }

            continue;
        }

        // Обработка коротких флагов: -f или -abc или -abc=value
        if arg.starts_with('-') && arg.len() > 1 {
            if let Some(pos) = arg.find('=') {
                let value = arg.split_off(pos + 1);
                
                // Добавляем все символы флагов, кроме первого ('-') и знака '='
                for c in arg[1..pos].chars() {
                    tokens.push(Token::Short(c));
                }

                tokens.push(Token::Value(value));
            } else {
                for c in arg[1..].chars() {
                    tokens.push(Token::Short(c));
                }
            }

            continue; 
        } 

        tokens.push(Token::Value(arg));
    }

    tokens
}
