# structbotopt

[StructOpt](https://docs.rs/structopt/0.3.15/structopt/) extension for chatbot.

```rust
use structbotopt::StructBotOpt;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Cmd {
    /// Suspend bot
    Suspend,
    /// Resume bot
    Resume,
    /// Add target channel
    AddChannel {
        /// Channel name
        #[structopt(name = "channel_name")]
        name: String,
    },
    /// Delete target channel
    DeleteChannel {
        /// Channel name
        #[structopt(name = "channel_name")]
        name: String,
    },
    /// Add target user
    AddUser {
        /// Channel name
        #[structopt(name = "user_name")]
        name: String,
    },
    /// Delete taret user
    DeleteUser {
        /// Channel name
        #[structopt(name = "user_name")]
        name: String,
    },
    /// List users and channels
    List,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "@bot")]
struct BotOpt {
    /// Verbose print
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
    #[structopt(subcommand)]
    cmd: Cmd,
}

fn main() {
    let message = "@bot --help";

    // In actual usecase, call this in the event handler for a message.
    match BotOpt::parse_bot_args(&message) {
        Some(Ok(opt)) => {
            println!("{:?}", opt);
        }
        Some(Err(e)) => {
            // The error contains a help message in markdown.
            println!("{}", e);
        }
        None => {
            println!("Not to me");
        }
    }
}
```
