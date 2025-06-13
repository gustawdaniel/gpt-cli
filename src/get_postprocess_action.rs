use std::env;

#[derive(Debug, PartialEq)]
pub enum PostprocessAction {
    // default
    Confirm,
    Copy,
    Out,
}

pub fn get_postprocess_action(answer_text: &str) -> PostprocessAction {
    let action_by_env = match env::var("GPT_POST") {
        Ok(val) => match val.as_str() {
            "confirm" => PostprocessAction::Confirm,
            "copy" => PostprocessAction::Copy,
            "out" => PostprocessAction::Out,
            _ => PostprocessAction::Confirm,
        },
        Err(_) => PostprocessAction::Confirm,
    };

    if (answer_text.contains('$') || answer_text.starts_with("export"))
        && action_by_env == PostprocessAction::Confirm
    {
        return PostprocessAction::Copy;
    }

    action_by_env
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_postprocess_action_confirm() {
        unsafe {
            env::remove_var("GPT_POST");
        }
        let answer = "This is a normal answer.".to_string();
        let action = get_postprocess_action(&answer);
        assert_eq!(action, PostprocessAction::Confirm);
    }

    #[test]
    fn test_get_postprocess_action_copy() {
        unsafe {
            env::remove_var("GPT_POST");
        }
        let answer = "This is an answer containing $variable.".to_string();
        let action = get_postprocess_action(&answer);
        assert_eq!(action, PostprocessAction::Copy);
    }

    #[test]
    fn test_get_postprocess_action_export() {
        unsafe {
            env::remove_var("GPT_POST");
        }
        let answer = "export MY_VARIABLE=value".to_string();
        let action = get_postprocess_action(&answer);
        assert_eq!(action, PostprocessAction::Copy);
    }

    #[test]
    fn test_get_postprocess_action_env_confirm() {
        unsafe {
            env::set_var("GPT_POST", "confirm");
        }
        let answer = "This is a normal answer.".to_string();
        let action = get_postprocess_action(&answer);
        assert_eq!(action, PostprocessAction::Confirm);
    }

    #[test]
    fn test_get_postprocess_action_env_copy() {
        unsafe {
            env::set_var("GPT_POST", "copy");
        }
        let answer = "This is a normal answer.".to_string();
        let action = get_postprocess_action(&answer);
        assert_eq!(action, PostprocessAction::Copy);
    }

    #[test]
    fn test_get_postprocess_action_env_out() {
        let answer = "This is a normal answer.".to_string();
        unsafe {
            env::set_var("GPT_POST", "out");
        }
        let action = get_postprocess_action(&answer);
        assert_eq!(action, PostprocessAction::Out);
    }

    #[test]
    fn test_get_postprocess_action_env_invalid() {
        unsafe {
            env::set_var("GPT_POST", "invalid");
        }
        let answer = "This is a normal answer.".to_string();
        let action = get_postprocess_action(&answer);
        assert_eq!(action, PostprocessAction::Confirm);
    }
}
