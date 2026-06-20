use std::collections::HashMap;

pub struct Translations {
    data: HashMap<String, String>,
    pub lang: String,
}

impl Translations {
    pub fn load(lang: &str) -> Self {
        let content = match lang {
            "en" => include_str!("../locales/en.toml"),
            _    => include_str!("../locales/fr.toml"),
        };
        let data: HashMap<String, String> = toml::from_str(content)
            .expect("invalid translation file");
        Self { data, lang: lang.to_string() }
    }

    pub fn t<'a>(&'a self, key: &'a str) -> &'a str {
        self.data.get(key).map(|s| s.as_str()).unwrap_or(key)
    }

    pub fn tf(&self, key: &str, args: &[(&str, &str)]) -> String {
        let mut s = self.t(key).to_string();
        for (k, v) in args {
            s = s.replace(&format!("{{{}}}", k), v);
        }
        s
    }
}
