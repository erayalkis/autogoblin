use std::env;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

mod helpers;

#[group]
#[commands(ping, vitals, servers)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(">"))
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn vitals(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx.http).await?;

    let vitals = helpers::get_vitals();

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Machine Vitals");
            e.field("Memory Available", vitals.mem_free, true);
            e.field("Memory Used", vitals.mem_used, false);
            e.field("CPU Usage", vitals.cpu_usage, false)
        })
    }).await?;
    Ok(())
}

#[command]
async fn servers(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx.http).await?;

    let servers = helpers::get_servers();

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Server Status");

            for server in servers {
                let is_online = helpers::probe_port(server.port);

                let text = format!("{}:", server.image);
                if is_online {
                    e.field(text, "Online", false);
                } else {
                    e.field(text, "Offline", false);
                }
            }

            e.field("", "", false)
        })
    }).await?;

    Ok(())
}