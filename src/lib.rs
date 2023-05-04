mod config;
mod scraper;

use config::bot;

pub async fn start_bot() {
    bot::run().await;
}
