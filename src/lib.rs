use log::*;
use regex::Regex;
use structopt::StructOpt;

pub type BotResult<T> = std::result::Result<T, String>;

pub trait StructBotOpt: StructOpt {
    fn from_bot_args() -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn parse_bot_args(msg: &str) -> Option<BotResult<Self>>
    where
        Self: Sized,
    {
        parse_msg(msg)
    }
}

impl<T> StructBotOpt for T where T: StructOpt {}

fn parse_msg<T: StructOpt + Sized>(message: &str) -> Option<BotResult<T>> {
    let message = message.replace("　", " ");
    let mut tokens = message
        .lines()
        .map(|s| s.split(" "))
        .flatten()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .peekable();

    let tokens = match tokens.peek() {
        Some(head) if *head == T::clap().get_name() => tokens,
        _ => return None,
    };

    {
        let tt = tokens.clone();
        for (i, t) in tt.enumerate() {
            info!("token {}: {}", i, t);
        }
    }

    enum Section {
        Usage,
        Flags,
        SubCommands,
        Else(String),
        Args,
        None,
    }

    match T::from_iter_safe(tokens) {
        Ok(opt) => Some(Ok(opt)),
        Err(e) => {
            let h = e.message.clone();
            let mut out = String::new();
            let mut sec = Section::None;
            let mut lines = h.lines();
            let mut fst = false;

            while let Some(line) = lines.next() {
                let re = Regex::new(r"\x1B\[[^m]+m").unwrap();
                let line = re.replace_all(&line, "`");
                warn!("line: \"{}\"", line);
                let re = Regex::new(r"^(?P<title>[A-Z]+):").unwrap();

                match re.captures(&line) {
                    Some(cap) => {
                        sec = match cap.get(1).map(|c| c.as_str()) {
                            Some("USAGE") => Section::Usage,
                            Some("FLAGS") => Section::Flags,
                            Some("SUBCOMMANDS") => Section::SubCommands,
                            Some("ARGS") => Section::Args,
                            e => Section::Else(e.map(|s| s.into()).unwrap_or_else(|| "".into())),
                        };

                        let title = re.replace_all(&line, "\n##### $title");
                        out.push_str(&format!("{}\n\n", title));

                        match sec {
                            Section::Usage => {
                                fst = true;
                            }
                            _ => {}
                        }
                    }
                    None => {
                        let line = match &sec {
                            Section::Usage => {
                                if fst {
                                    fst = false;
                                    format!("```\n{}\n```\n", line.trim())
                                } else {
                                    format!("{}\n", line.trim())
                                }
                            }
                            Section::SubCommands | Section::Args => {
                                let mut tokens = line.trim().split(" ");
                                let cmd = tokens.next().unwrap_or("").trim();
                                let desc: Vec<_> = tokens.collect();
                                format!("* `{}`: {}\n", cmd, desc.join(" "))
                            }
                            Section::Flags if line.trim().is_empty() => String::new(),
                            Section::Flags => {
                                let re = Regex::new(r"(?P<option>--?[a-zA-Z-]+)").unwrap();
                                let opts = re.replace_all(&line, "`$option`");
                                if opts.trim().ends_with("`") {
                                    let desc = lines.next().unwrap_or("");
                                    format!("* {}: {}\n", opts.trim(), desc.trim())
                                } else {
                                    format!("* {}\n", opts)
                                }
                            }
                            Section::None => continue,
                            _ => format!("{}\n", line),
                        };

                        out.push_str(&line);
                    }
                }
            }

            Some(Err(format!("{}", out)))
        }
    }
}
