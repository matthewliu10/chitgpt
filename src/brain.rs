use crate::*;

#[derive(Default)]
pub struct Brain {
    words: HashMap<String, HashMap<String, usize>>,
}

impl Brain {
    pub fn train(&mut self, text: &str) {
        let mut prev_word = None;

        for word in text.split(' ') {
            if let Some(prev_word) = prev_word.replace(word) {
                *self
                    .words
                    .entry(prev_word.to_string())
                    .or_default()
                    .entry(word.to_string())
                    .or_default() += 1;
            }
        }
    }

    pub fn prompt(&self, prompt: &str, length: usize) -> String {
        let mut out: Vec<_> = prompt.split(' ').map(|word| word.to_string()).collect();

        let mut rng = rand::thread_rng();

        while out.len() < length {
            let last_word = out.last().unwrap();

            if let Some(next_words) = self.words.get(last_word) {
                let next_words: Vec<_> = next_words.iter().collect();

                let next_word = next_words.choose_weighted(&mut rng, |(_word, frequency)| *frequency).unwrap();

                out.push(next_word.0.to_string());
            } else {
                break;
            }
        }

        out.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_train() {
        let mut brain = Brain::default();
        brain.train("a b a c a b");

        let expected = vec![
            (
                String::from("a"),
                vec![(String::from("b"), 2), (String::from("c"), 1)],
            ),
            (String::from("b"), vec![(String::from("a"), 1)]),
            (String::from("c"), vec![(String::from("a"), 1)]),
        ];

        let expected: HashMap<String, HashMap<_, usize>> = expected
            .into_iter()
            .map(|(word, data)| (word, data.into_iter().collect()))
            .collect();
        assert_eq!(brain.words, expected);
    }
}
