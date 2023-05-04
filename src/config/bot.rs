use teloxide::{prelude::*, utils::command::BotCommands};

use crate::scraper::ferry;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported commands:")]
pub enum Command {
    #[command(description = "Check sliema ferry service status")]
    Sliema,
    #[command(description = "Avaialable options")]
    Help,
}

pub async fn command_handler(msg: Message, bot: Bot, command: Command) -> ResponseResult<()> {
    match command {
        Command::Sliema => {
            bot.send_message(msg.chat.id, ferry::get_ferry_status())
                .await?
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
    };
    Ok(())
}

pub async fn run() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    bot.set_my_commands(Command::bot_commands())
        .await
        .expect("Failed to set bot commands");

    Command::repl(bot, command_handler).await;
}
