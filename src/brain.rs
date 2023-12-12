use crate::*;

type Token = String;
type TokenId = usize;
type TokenFrequency = usize;

const MAX_CONTENT_SIZE: usize = 5;

#[derive(Default)]
pub struct Brain {
    token_to_id: HashMap<Token, TokenId>,
    id_to_token: HashMap<TokenId, Token>,
    next_id: TokenId,
    frequencies: HashMap<Vec<TokenId>, HashMap<TokenId, TokenFrequency>>,
}

impl Brain {
    pub fn train(&mut self, text: &str) {
        self.train_tokens(Self::tokenize(text));
    }

    fn train_tokens<'a>(&mut self, tokens: impl Iterator<Item = &'a str>) {
        let mut context: Vec<usize> = Vec::new();

        for token in tokens {
            if !self.token_to_id.contains_key(token) {
                self.add_token(token);
            }
            let token = *self.token_to_id.get(token).unwrap();

            for cs in 1..=context.len() {
                let context: Vec<_> = context[(context.len() - cs)..context.len()]
                    .iter()
                    .map(|token_id| token_id.clone())
                    .collect();
                *self
                    .frequencies
                    .entry(context)
                    .or_default()
                    .entry(token)
                    .or_default() += 1;
            }

            context.push(token);

            if context.len() > MAX_CONTENT_SIZE {
                context.remove(0);
            }
        }
    }

    pub fn prompt(&self, prompt: &str, length: usize) -> String {
        let mut out: Vec<_> = Self::tokenize(prompt)
            .map(|token| {
                self.token_to_id
                    .get(token)
                    .expect("unknown token in prompt")
                    .clone()
            })
            .collect();

        let mut rng = rand::thread_rng();

        while out.len() < length {
            let mut next_token_id = None;

            for cs in (1..=MAX_CONTENT_SIZE).rev() {
                if cs > out.len() {
                    continue;
                }

                let context: Vec<_> = out[(out.len() - cs)..out.len()]
                    .iter()
                    .map(|token_id| token_id.clone())
                    .collect();

                if let Some(next_tokens_ids) = self.frequencies.get(&context) {
                    let next_token_ids: Vec<_> = next_tokens_ids.iter().collect();

                    next_token_id = Some(
                        next_token_ids
                            .choose_weighted(&mut rng, |(_token_id, frequency)| *frequency)
                            .unwrap()
                            .0
                            .clone(),
                    );

                    break;
                }
            }

            if let Some(next_token_id) = next_token_id {
                out.push(next_token_id);
            } else {
                break;
            }
        }

        let out: Vec<String> = out
            .into_iter()
            .map(|id| self.id_to_token.get(&id).unwrap().clone())
            .collect();

        out.join("")
    }

    fn tokenize(s: &str) -> impl Iterator<Item = &str> {
        let mut chars = s.char_indices().peekable();

        iter::from_fn(move || loop {
            let (index, char) = chars.next()?;

            if char.is_alphanumeric() {
                let idx_from = index;
                let mut idx_to = index + char.len_utf8();
                while let Some((_index, char)) = chars.peek() {
                    if char.is_alphanumeric() {
                        idx_to += char.len_utf8();
                        chars.next();
                    } else {
                        break;
                    }
                }

                return Some(&s[idx_from..idx_to]);
            } else {
                let idx_from = index;
                let idx_to = index + char.len_utf8();

                return Some(&s[idx_from..idx_to]);
            }
        })
    }

    fn add_token(&mut self, token: &str) {
        self.token_to_id.insert(token.to_string(), self.next_id);
        self.id_to_token.insert(self.next_id, token.to_string());
        self.next_id += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        "Hello, World!",
        &["Hello", ",", " ", "World", "!"]
    )]
    #[test_case(
        "#include <coffee.h>",
        &["#", "include", " ", "<", "coffee", ".", "h", ">"]
    )]
    #[test_case(
        "123 + 234 = 0xCAFEBABE",
        &["123", " ", "+", " ", "234", " ", "=", " ", "0xCAFEBABE"]
    )]
    fn test_tokenize(s: &str, expected: &[&str]) {
        let actual: Vec<_> = Brain::tokenize(s).collect();
        let expected: Vec<_> = expected.iter().map(|token| token.to_string()).collect();

        assert_eq!(actual, expected);
    }
}
