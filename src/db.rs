pub struct Text {
    pub source: String,
    pub text: String,
}

pub fn random_string() -> Text {
    Text {
        source: String::from("Random"),
        text: String::from("This is a random string to type"),
    }
}

pub fn random_wikipedia() -> Text {
    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();

    let page_title = wiki.random();

    if page_title.is_err() {
        return random_string();
    }

    let safe_page_title = page_title.unwrap();

    if safe_page_title.is_none() {
        return random_string();
    }

    let page = wiki.page_from_title(safe_page_title.clone().unwrap());
    let text = page.get_summary().unwrap().split('\n').collect::<String>();

    Text {
        source: format!("{} ({})", "Wikipedia", safe_page_title.unwrap()),
        text: cutoff(transliterate(text), 80),
    }
}

fn transliterate(text: String) -> String {
    text.chars().filter(|c| c.is_ascii()).collect::<String>()
}

fn cutoff(text: String, max_len: usize) -> String {
    let mut result = String::new();

    for word in text.split_whitespace() {
        if result.len() + word.len() > max_len {
            break;
        }

        result.push_str(word);
        result.push(' ');
    }

    result.trim().to_string()
}
