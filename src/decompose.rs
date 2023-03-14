pub fn decompose(command: &str) -> (String, Vec<String>) {
    let mut command = String::from(command);
    if command.starts_with('`') && command.ends_with('`') {
        command.remove(0);
        command.pop();
    }
    let mut parts = command.split_whitespace();
    let command_name = parts.next().unwrap_or_default().to_string();
    let command_args: Vec<String> = command.split_whitespace().skip(1).map(|part| part.to_string()).collect();
    if command_args.contains(&"|".to_string()) {
        let remaining_parts: Vec<String> = command.split_whitespace().map(|part| part.to_string()).collect();
        return (String::from("bash"), vec!["-c".to_string(), remaining_parts.join(" ")]);
    }
    (command_name, command_args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompose_ls() {
        assert_eq!(
            decompose("ls"),
            ("ls".to_string(), Vec::<String>::new())
        );
    }

    #[test]
    fn test_decompose_ls_la() {
        assert_eq!(
            decompose("ls -la"),
            ("ls".to_string(), vec!["-la".to_string()])
        );
    }

    #[test]
    fn test_decompose_strip_code_marks() {
        assert_eq!(
            decompose("`ls -l`"),
            ("ls".to_string(), vec!["-l".to_string()])
        );
    }

    #[test]
    fn test_decompose_graphic_cards() {
        assert_eq!(
            decompose("lspci | grep VGA"),
            ("bash".to_string(), vec!["-c".to_string(), "lspci | grep VGA".to_string()])
        );
    }
}