use serde_json::Value;

pub fn exec() {
    println!("hello world!!")
}

pub fn render(text: &str, setting: Value) -> String {
    let setting_map = setting.as_object();
    if setting_map.is_none() {
        return text.to_string();
    }

    let result = match setting.as_object() {
        Some(setting_map) => {
            let mut tmp_text = text.to_string();
            // TODO Parse and render, not replace
            for (key, value) in setting_map {
                tmp_text = match value.as_str() {
                    Some(val) => replace_string(tmp_text, key, val),
                    None => tmp_text
                };
            }
            return tmp_text;
        }
        None => text.to_string(),
    };

    return result;
}

fn replace_string(text: String, key: &str, value: &str) -> String {
    let pattern = &(["{{", key, "}}"].concat());
    return text.replace(pattern, value);
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::render;

    #[test]
    fn test_render_no_setting() {
        let setting = json!("{}");
        assert_eq!("hello rtpl!!".to_string(), render("hello rtpl!!", setting))
    }

    #[test]
    fn test_render_only_one_word() {
        let setting = json!({"name": "rtpl"});
        assert_eq!("hello rtpl!!".to_string(), render("hello {{name}}!!", setting));
    }

    #[test]
    fn test_render_two_word() {
        let setting = json!({
            "name": "rtpl",
            "greeting": "good morning"
            });
        assert_eq!(
            "good morning rtpl!!".to_string(),
            render("{{greeting}} {{name}}!!", setting)
        );
    }
}
