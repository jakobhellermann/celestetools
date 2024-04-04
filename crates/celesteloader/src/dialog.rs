use std::{
    borrow::Cow,
    collections::HashMap,
    io::{BufRead, BufReader},
};

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
            input.replace(['/', '-', '+', ' '], "_").into()
        } else {
            input.into()
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.dict
            .iter()
            .map(|(key, val)| (key.as_str(), val.as_str()))
    }
}

impl Dialog {
    pub fn from_export(_bytes: &[u8]) {
        todo!()
    }

    pub fn from_txt(text: &str) -> Dialog {
        enum Never {}
        Dialog::from_lines::<Never>(text.lines().map(Result::Ok)).unwrap_or_else(|e| match e {})
    }

    pub fn from_lines<E>(
        lines: impl Iterator<Item = Result<impl AsRef<str>, E>>,
    ) -> Result<Dialog, E> {
        let mut dict = HashMap::new();

        let mut last_key = String::new();
        let mut builder = String::new();

        for line in lines {
            let line = line?;
            let line: &str = line.as_ref();
            let line = line.trim_start_matches('\u{feff}');
            let line = line.trim();

            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            // TODO portrait
            // let line = line.replace("\\#", "#");

            let variable_pattern = line
                .split_once('=')
                .filter(|&(key, _)| key.chars().all(|c| c.is_alphanumeric() || c == '_'));

            // clippy doesn't see that the value is moved into the dict
            #[allow(clippy::assigning_clones)]
            if let Some((key, value)) = variable_pattern {
                if !last_key.is_empty() {
                    dict.insert(UniCase::new(last_key), std::mem::take(&mut builder));
                }

                let key = key.trim();
                let value = value.trim();

                last_key = key.to_owned();

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
        Ok(Dialog { dict })
    }

    pub fn from_read<R: std::io::Read>(read: R) -> Result<Dialog, std::io::Error> {
        let reader = BufReader::new(read);
        Dialog::from_lines(reader.lines())
    }
}
