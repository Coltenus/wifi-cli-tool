use rustyline::{Helper, hint::Hinter, completion::Completer};
use rustyline::validate::{Validator, ValidationResult, ValidationContext};
use std::borrow::Cow;
use rustyline::highlight::Highlighter;

pub struct MaskingHighlighter {
    masking: char,
}

impl MaskingHighlighter {
    pub fn new(masking: char) -> Self {
        MaskingHighlighter { masking }
    }
}

impl Highlighter for MaskingHighlighter {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        if self.masking != '\0' {
            let masked = line.chars().map(|c| if c.is_whitespace() { c } else { '*' }).collect::<String>();
            Cow::Owned(masked)
        } else {
            Cow::Borrowed(line)
        }
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool
    ) -> Cow<'b, str> {
        Cow::Borrowed(prompt)
    }

    fn highlight_char(&self, _line: &str, _pos: usize) -> bool {
        self.masking != '\0'
    }
}

impl Helper for MaskingHighlighter {}

impl Validator for MaskingHighlighter {
    fn validate(&self, _ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        Ok(ValidationResult::Valid(None))
    }
}

impl Hinter for MaskingHighlighter {
    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<String> {
        None // No hint is provided in this example
    }

    type Hint = String;
}

impl Completer for MaskingHighlighter {
    type Candidate = String;
}