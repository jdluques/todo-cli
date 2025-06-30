pub mod prompts {
    use inquire::{Select, Text};

    pub fn text_input(prompt: &str) -> Option<String> {
        Text::new(prompt).prompt().ok()
    }

    pub fn select_option<'a>(prompt: &str, options: &[&'a str]) -> Option<&'a str> {
        Select::new(prompt, options.to_vec()).prompt().ok()
    }
}
