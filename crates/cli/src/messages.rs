use std::io::{self, Write};

use colored::Colorize;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum MessageLevel {
    Info,
    Warning,
    Error,
    Success,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub level: MessageLevel,
    pub text: String,
}

#[derive(Default)]
pub struct MessageSink {
    messages: Vec<Message>,
}

impl MessageSink {
    pub fn push(&mut self, level: MessageLevel, text: impl Into<String>) {
        self.messages.push(Message {
            level,
            text: text.into(),
        });
    }

    pub fn drain(&mut self) -> Vec<Message> {
        std::mem::take(&mut self.messages)
    }
}

pub fn render_messages(messages: &[Message]) -> io::Result<()> {
    if messages.is_empty() {
        return Ok(());
    }

    let mut stderr = io::BufWriter::new(io::stderr());
    writeln!(stderr)?;
    for msg in messages {
        let line = match msg.level {
            MessageLevel::Info => format!("info: {}", msg.text).blue(),
            MessageLevel::Warning => format!("warning: {}", msg.text).yellow(),
            MessageLevel::Error => format!("error: {}", msg.text).red(),
            MessageLevel::Success => msg.text.clone().green(),
        };
        writeln!(stderr, "{line}")?;
    }
    stderr.flush()
}
