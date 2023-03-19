use colored::{ColoredString, Colorize};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct ShouldExit {
    pub exit: bool,
    pub is_error: bool,
    pub messages: Vec<ColoredString>,
}

pub fn should_exit(args: &Vec<String>) -> ShouldExit {
    if args.len().eq(&0) {
        return ShouldExit { exit: true, messages: vec!["Please add description, which command you want to execute.".red(), "eg.: cargo run -- show calendar".white()], is_error: true };
    } else if args.len().eq(&1) && args.first().unwrap().eq("--version") {
        return ShouldExit { exit: true, messages: vec![VERSION.into()], is_error: false };
    }
    ShouldExit { exit: false, messages: vec![], is_error: false }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Color;

    #[test]
    fn test_should_exit_empty_args() {
        let args: Vec<String> = vec![];
        let result = should_exit(&args);

        assert_eq!(result.exit, true);
        assert_eq!(result.is_error, true);
        assert_eq!(result.messages.len(), 2);
        assert_eq!(result.messages[0].clone().clear().to_string().as_str(), "Please add description, which command you want to execute.");
        assert_eq!(result.messages[0].fgcolor(), Some(Color::Red));
        assert_eq!(result.messages[1].clone().clear().to_string().as_str(), "eg.: cargo run -- show calendar");
        assert_eq!(result.messages[1].fgcolor(), Some(Color::White));
    }

    #[test]
    fn test_should_exit_version() {
        let args: Vec<String> = vec![String::from("--version")];
        let result = should_exit(&args);

        assert_eq!(result.exit, true);
        assert_eq!(result.is_error, false);
        assert_eq!(result.messages.len(), 1);
        assert_eq!(result.messages[0].to_string().as_str(),VERSION);
    }

    #[test]
    fn test_should_exit_no_exit() {
        let args: Vec<String> = vec![String::from("show"), String::from("calendar")];
        let result = should_exit(&args);

        assert_eq!(result.exit, false);
        assert_eq!(result.is_error, false);
        assert_eq!(result.messages.len(), 0);
    }
}