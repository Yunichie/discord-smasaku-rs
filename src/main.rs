mod commands;
mod capitalizer;

use std::env;
use serenity::{
    async_trait,
    model::{
        application::{
            interaction::{
                Interaction,
                //InteractionResponseType
            },
            command::{
                Command
            },
        },
        gateway::{
            Ready
        },
        Timestamp
    },
    prelude::{
        *
    }
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let _content = match command.data.name.as_str() {
                "perkenalan" => commands::perkenalan_slash::run(&command, &ctx).await,
                "bantuan" => commands::bantuan::run(&ctx, &command).await,
                _ => {
                    let msg = command.channel_id
                    .send_message(&ctx.http, |msg| {
                        msg.embed(|e| {
                            e
                            .title("Error")
                            .description("Not implemented")
                            .timestamp(Timestamp::now())
                        })
                    }).await;
                    
                    if let Err(why) = msg {
                        println!("Error sending message: {:?}", why);
                    }
                }
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::perkenalan_slash::register(command)
        })
        .await;

        println!("I now have the following guild slash commands: {:#?}", guild_command);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}