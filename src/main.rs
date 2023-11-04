use std::env;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

mod helpers;

#[group]
#[commands(ping, vitals, servers, fireball, coinflip, up, down)]
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
    let mut up = vitals.uptime;
    let days = up / 86400;
    up -= days * 86400;
    let hours = up / 3600;
    up -= hours * 3600;
    let minutes = up / 60;

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Machine Vitals 🧙‍♂️");
            e.field("", "", false);
            e.colour(499252);
            e.thumbnail("https://i.imgur.com/IMZQqfP.png");
            e.field("Memory Available", format!("{} MiB", vitals.mem_free / 1024 / 1024), false);
            e.field("", "", false);
            e.field("Memory Used", format!("{} MiB", vitals.mem_used / 1024 / 1024), false);
            e.field("", "", false);
            e.field("Total CPU Usage", format!("{:.1}%", vitals.cpu_usage), false);
            e.field("", "", false);
            e.footer(|f| {
                f.text(format!("Up for {} days, {} hours, {} minutes 🖥️", days, hours, minutes))
            })
        })
    }).await?;
    Ok(())
}

#[command]  
async fn servers(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx.http).await?;

    let servers = helpers::get_servers();
    // Avoided using map because async closures are unstable and i didn't wanna mess with that
    let mut server_statuses: Vec<bool> = Vec::new();

    for server in &servers {
        let is_online = helpers::probe_port(&server.port, &server.endpoint).await;

        server_statuses.push(is_online);
    }

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Server Status");
            e.thumbnail("https://i.imgur.com/IMZQqfP.png");
            e.colour(499252);

            for (idx, server) in servers.iter().enumerate() {
                let text = format!("{}:", server.name);
                let is_online = server_statuses[idx];

                if is_online {
                    e.field("", "", false);
                    e.field(text, "Online ✔️", false);
                    e.field("", format!("Can be accessed at {}:{}", "127.0.0.1", server.port), false);
                } else {
                    e.field("", "", false);
                    e.field(text, "Offline ❌", false);
                }
            }

            e.field("", "", false)
        })
    }).await?;

    Ok(())
}

#[command]
async fn fireball(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "https://i.imgur.com/66cTj4C.gif").await?;
    
    Ok(())
}

#[command]
async fn coinflip(ctx: &Context, msg: &Message) -> CommandResult {
    match helpers::generate_random_number(0, 2).await {
        0 => {
            msg.reply(&ctx.http, "On it, boss!").await?;
        }

        1 => {
            msg.reply(&ctx.http, "Sure thing, boss!").await?;
        }
    }

    msg.channel_id.broadcast_typing(&ctx.http).await?;

    match helpers::generate_random_number(0, 2).await {
        0 => {
            msg.reply(&ctx.http, "We gots a heads, boss!").await?;
        }

        1 => {
            msg.reply(&ctx.http, "It's a tails, boss!").await?;
        }
        
        2_u64..=u64::MAX => {

        }
    }
    Ok(())
}

#[command]
async fn up(ctx: &Context, msg: &Message) -> CommandResult {
    let content = helpers::get_argument_from_command(&msg.content);
    let res = helpers::up_server(content).await;

    match res {
        Ok(resp) => {
            if resp.status() == 200 {
                msg.reply(&ctx.http, "Server has been successfully started!").await?;
            } else {
                msg.reply(&ctx.http, "Something went wrong while starting the server!").await?;
            }
        }

        Err(err) => {
            println!("Error: {}", err);
            msg.reply(&ctx.http, "Something went wrong while starting the server!").await?;
        }
    }
    Ok(())
}

#[command]
async fn down(ctx: &Context, msg: &Message) -> CommandResult {
    let content = helpers::get_argument_from_command(&msg.content);
    let res = helpers::down_server(content).await;

    match res {
        Ok(resp) => {
            if resp.status() == 200 {
                msg.reply(&ctx.http, "Server has been successfully stopped!").await?;
            } else {
                msg.reply(&ctx.http, "Something went wrong while stopping the server!").await?;
            }
        }

        Err(err) => {
            println!("Error: {}", err);
            msg.reply(&ctx.http, "Something went wrong while stopping the server!").await?;
        }
    }
    Ok(())
}
