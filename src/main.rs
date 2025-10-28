use crate::app::config::Config;
use crate::app::handlers::Handlers;
use crate::app::repositories::Repositories;
use crate::app::services::Services;
use crate::metrics::PrometheusMetrics;
use crate::servers::http::server::run_http_server;
use crate::utils::db::init_primary_db;
use dotenvy::dotenv;
use std::sync::Arc;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, utils::command::BotCommands};
use uuid::Uuid;
mod app;
mod bot;
mod domain;
mod feature;
mod metrics;
mod servers;
mod swagger;
mod utils;

#[cfg(not(target_os = "windows"))]
use jemallocator::Jemalloc as GlobalAlloc;

use crate::feature::url::service::UrlServiceTrait;
use crate::utils::url::extract_first_valid_url_from_message;
#[cfg(target_os = "windows")]
use mimalloc::MiMalloc as GlobalAlloc;
use teloxide::types::{BotCommand, BotCommandScope};

#[global_allocator]
static GLOBAL: GlobalAlloc = GlobalAlloc;
type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Start")]
    Start,
    #[command(description = "All urls")]
    AllUrls,
    #[command(description = "help command")]
    Help,
}
#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveFullUrl,
}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_env();
    let token = std::env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN env var not set");
    log::info!("Starting throw dice bot...");
    log::info!("tk token: {}", token);
    let bot = Bot::from_env();
    let config = Config::new().await;
    let http_server = config.server.clone().unwrap_or_else(|| {
        panic!("HTTP server configuration not found");
    });

    let pool = init_primary_db(&config).await.expect("Count not init db");
    let repo = Arc::new(Repositories::new(pool.clone()));
    let services = Arc::new(Services::new(repo));
    let metrics = Arc::new(PrometheusMetrics::new().expect("Failed to create Prometheus metrics"));
    let handlers = Arc::new(Handlers::new(services.clone(), metrics.clone()));
    let metrics_clone = metrics.clone();
    let http_task = async {
        run_http_server(
            &http_server.host,
            http_server.port,
            handlers,
            pool,
            metrics_clone,
        )
        .await;
    };
    set_bot_commands(&bot)
        .await
        .expect("Failed to set commands");
    let bot_task = async {
        Dispatcher::builder(
            bot,
            Update::filter_message()
                .enter_dialogue::<Message, InMemStorage<State>, State>()
                .branch(dptree::case![State::Start].endpoint(start))
                .branch(dptree::case![State::ReceiveFullUrl].endpoint(receive_full_url)),
        )
        .dependencies(dptree::deps![
            InMemStorage::<State>::new(),
            Arc::clone(&services),
            Arc::clone(&metrics)
        ])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
        log::warn!("Bot task ended!");
    };

    tokio::join!(bot_task, http_task);

    Ok(())
}
async fn command_handler(
    bot: Bot,
    msg: Message,
    cmd: Command,
    dialogue: MyDialogue,
    metrics: Arc<PrometheusMetrics>,
) -> anyhow::Result<()> {
    match cmd {
        Command::Start => {
            start(bot, dialogue, msg, metrics)
                .await
                .expect("TODO: panic message");
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::AllUrls => {
            bot.send_message(msg.chat.id, format!("Получен URL: {}", "asdsad"))
                .await?;
            dialogue.update(State::Start).await?;
        }
    }
    Ok(())
}
async fn set_bot_commands(bot: &Bot) -> anyhow::Result<()> {
    let commands = Command::bot_commands()
        .into_iter()
        .map(|cmd| BotCommand {
            command: cmd.command,
            description: cmd.description.to_string(),
        })
        .collect::<Vec<_>>();

    bot.set_my_commands(commands)
        .scope(BotCommandScope::Default)
        .await?;
    Ok(())
}

async fn start(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    metrics: Arc<PrometheusMetrics>,
) -> HandlerResult {
    if let Some(txt) = msg.text() {
        if txt != "/start" {
            bot.send_message(msg.chat.id, "Please, write /start")
                .await?;
            return Ok(());
        }
    }
    bot.send_message(msg.chat.id, "Let's start! What's your full url?")
        .await?;
    dialogue.update(State::ReceiveFullUrl).await?;
    Ok(())
}
async fn receive_full_url(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    services: Arc<Services>,
    metrics: Arc<PrometheusMetrics>,
) -> HandlerResult {
    metrics.inc_telegram_messages();

    if let Some(valid_url) = extract_first_valid_url_from_message(&msg) {
        let user_id = if let Some(user) = &msg.from() {
            let username = user.username.as_deref().unwrap_or("<no username>");
            println!("valid url from {} ({}): {}", username, user.id, valid_url);
            user.id
        } else {
            bot.send_message(msg.chat.id, "❌ Unable to identify user.")
                .await?;
            return Ok(());
        };
        let user_id_uuid = user_id_to_uuid(user_id);
        match services
            .url_service
            .create_url(valid_url.to_string(), user_id_uuid)
            .await
        {
            Ok(created_url) => {
                metrics.inc_url_shortening();
                bot.send_message(msg.chat.id, format!("✅ Saved url: {:?}", created_url))
                    .await?;
            }
            Err(e) => {
                metrics.inc_errors("url_creation_error", "telegram_bot");
                bot.send_message(msg.chat.id, "❌ Failed to save URL.")
                    .await?;
                log::error!("Failed to create url: {:?}", e);
            }
        }
        dialogue.update(State::ReceiveFullUrl).await?;
    } else {
        bot.send_message(
            msg.chat.id,
            "❌ Не удалось найти корректный URL в сообщении.",
        )
        .await?;
    }
    Ok(())
}
fn init_env() {
    #[cfg(debug_assertions)]
    {
        dotenvy::dotenv().ok();
    }

    pretty_env_logger::init();
}
fn user_id_to_uuid(user_id: UserId) -> Uuid {
    // UserId - это обертка над i64
    let id_bytes = user_id.0.to_be_bytes();
    let mut uuid_bytes = [0u8; 16];

    uuid_bytes[..8].copy_from_slice(&id_bytes);

    Uuid::from_bytes(uuid_bytes)
}
