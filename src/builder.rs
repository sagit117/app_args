use std::env;

use crate::{Arg, err::ArgError, matches::Matches, sort_args_by_priority, token::{PrepareTokens, PreparedToken, convert_to_tokens}};

pub struct Builder {
    user_args: Vec<Arg>,
    prepared_tokens: PrepareTokens
}

impl Builder {
    pub fn arg(mut self, arg: Arg) -> Builder {
        self.user_args.push(arg);

        self
    }

    pub fn parse(self) -> Result<Matches, ArgError> {
        // Разделяем self на независимые переменные.
        // Теперь user_args и prepared_tokens никак не связаны друг с другом в памяти!
        let mut user_args = self.user_args;
        let mut prepared_tokens = self.prepared_tokens;

        // Сортируем локальный вектор аргументов
        user_args.sort_by_key(sort_args_by_priority);

        // Используем обычный цикл for для обновления значений
        for arg in &mut user_args {
            arg.value = prepared_tokens.get_value_arg(&arg.arg_kind, arg.is_flag)?;
        }

        // Передаем обновленный вектор во владение Matches
        Ok(Matches::new(user_args))
    }
}

pub fn new() -> Builder {
    Builder { 
        user_args: Vec::new(),
        prepared_tokens: PrepareTokens {
            tokens: convert_to_tokens(env::args().skip(1).collect())
                .into_iter()
                .map(|t| PreparedToken { token: t, is_prepared: false })
                .collect()
        }
    }
}

