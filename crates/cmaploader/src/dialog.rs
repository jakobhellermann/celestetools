use std::{borrow::Cow, collections::HashMap};

use unicase::UniCase;

#[derive(Debug)]
pub struct Dialog {
    pub dict: HashMap<UniCase<String>, String>,
}

impl Dialog {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.dict
            .get(&UniCase::new(Dialog::dialog_keyify(key).into_owned()))
            .map(String::as_str)
    }

    pub fn dialog_keyify(input: &str) -> Cow<'_, str> {
        if input.contains(|c| c == '/' || c == '-' || c == '+' || c == ' ') {
            input
                .replace('/', "_")
                .replace('-', "_")
                .replace('+', "_")
                .replace(' ', "_")
                .into()
        } else {
            input.into()
        }
    }
}

impl Dialog {
    pub fn from_export(_bytes: &[u8]) {
        todo!()
    }

    pub fn from_txt(text: &str) -> Dialog {
        let mut dict = HashMap::new();

        let mut last_key = "";
        let mut builder = String::new();

        for line in text.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            // TODO portrait
            // let line = line.replace("\\#", "#");

            let variable_pattern = line
                .split_once('=')
                .filter(|&(key, _)| key.chars().all(|c| c.is_alphanumeric() || c == '_'));

            if let Some((key, value)) = variable_pattern {
                if !last_key.is_empty() {
                    dict.insert(
                        UniCase::new(last_key.to_owned()),
                        std::mem::take(&mut builder),
                    );
                }

                let key = key.trim();
                let value = value.trim();

                last_key = key;

                builder.clear();
                builder.push_str(value);
            } else {
                builder.push_str(line);
            }
        }

        if !builder.is_empty() {
            dict.insert(
                UniCase::new(last_key.to_owned()),
                std::mem::take(&mut builder),
            );
        }

        // TODO: resolve variables

        Dialog { dict }
    }
}
