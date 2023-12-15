use std::ops::Range;

use codespan_derive::{IntoDiagnostic, IntoLabel, Label, LabelStyle};
use codespan_reporting::{
    files::SimpleFiles,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};

/// A source span to store a `file:byte range`
struct Span {
    file_id: usize,
    range: Range<usize>,
}

impl IntoLabel for Span {
    type FileId = usize;

    fn into_label(&self, style: LabelStyle) -> Label<Self::FileId> {
        Label::new(style, self.file_id, self.range.clone())
    }
}

#[derive(IntoDiagnostic)]
#[file_id(usize)]
enum Error {
    #[message = "This is an error: {message}"]
    Example {
        message: &'static str,

        #[primary = "This is a primary span"]
        primary_span: Span,

        #[secondary = "This is a secondary span"]
        secondary_span: Span,
    },
}

fn main() {
    let mut files: SimpleFiles<&'static str, &'static str> = SimpleFiles::new();
    let file_id = files.add("example.txt", "Test Case");

    let err = Error::Example {
        message: "This is a stored message",
        primary_span: Span {
            file_id,
            range: 5..9,
        },
        secondary_span: Span {
            file_id,
            range: 0..4,
        },
    };

    // Basic codespan-diagnostic printing to terminal
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();
    term::emit(&mut writer.lock(), &config, &files, &err.into_diagnostic())
        .expect("Failed to show diagnostic");
}