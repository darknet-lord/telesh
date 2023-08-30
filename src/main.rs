use std::process::Command as StdCommand;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;


#[tokio::main]
async fn main() {
    let bot = Bot::from_env();
    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "help")]
    Help,
    #[command(description = "shell command")]
    Sh(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Sh(cmd_str) => {
            let parts: Vec<&str> = cmd_str.split(" ").collect();
            let prog = parts[0];
            let mut sh_cmd = StdCommand::new(prog);
            for part in parts.iter().skip(1) {
                sh_cmd.arg(part);
            }
            let output = sh_cmd.output().expect("Command failed");
            let str = String::from_utf8_lossy(&output.stdout);
            bot.send_message(msg.chat.id, str).await?
        },
    };
    Ok(())
}
