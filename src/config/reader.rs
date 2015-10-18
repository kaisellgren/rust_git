use super::config::KeyValue;
use super::config::ConfigEntries;
use std::collections::hash_map::HashMap;

pub fn read(data: &str) -> ConfigEntries {
    let mut entries = HashMap::new();

    #[deriving(PartialEq)]
    enum State {
        Nothing,
        Category,  // [core]
        Parameter, // symlinks
        Value,     // false
    }

    let mut state             = State::Nothing;
    let mut current_category  = String::new();
    let mut current_parameter = String::new();
    let mut current_value     = String::new();

    fn ensureExtraLineFeedAtTheEnd(data: &str) -> String {
        data.into_string() + "\n".into_string()
    }

    for c in ensureExtraLineFeedAtTheEnd(data).chars() {
        match c {
            '[' => {
                state = State::Category;
                current_category = String::new();
            },
            ']' => {
                state = State::Parameter;
                entries.insert(current_category.clone(), Vec::new());
            },
            '=' => state = State::Value,
            '\n' if state == State::Value => {
                state = State::Parameter;

                let entry = &mut entries[current_category.trim().into_string()];
                entry.push(KeyValue(
                    current_parameter.trim().into_string(),
                    current_value.trim().into_string()));

                current_parameter = String::new();
                current_value = String::new();
            },
            _ if state == State::Category  => current_category.push(c),
            _ if state == State::Parameter => current_parameter.push(c),
            _ if state == State::Value     => current_value.push(c),
            _ => unreachable!()
        };
    };

    entries
}