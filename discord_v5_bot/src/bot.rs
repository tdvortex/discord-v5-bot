use rand::prelude::*;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::interactions::application_command::{
    ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
};
use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::prelude::*;
use std::env;

mod roll_dice;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => "pong".to_string(),
                "smile" => ":smile:".to_string(),
                "d10" => {
                    let face: u8 = thread_rng().gen_range(1..=10);
                    face.to_string()
                }
                "roll" => {
                    let black_dice = match command
                        .data
                        .options
                        .get(0)
                        .expect("Expected number of black dice")
                        .resolved
                        .as_ref()
                        .expect("Expected number of black dice")
                    {
                        ApplicationCommandInteractionDataOptionValue::Integer(black_dice) => {
                            *black_dice as u8
                        }
                        _ => 0,
                    };

                    let red_dice = match command
                        .data
                        .options
                        .get(1)
                        .expect("Expected number of red dice")
                        .resolved
                        .as_ref()
                        .expect("Expected number of red dice")
                    {
                        ApplicationCommandInteractionDataOptionValue::Integer(red_dice) => {
                            *red_dice as u8
                        }
                        _ => 0,
                    };

                    roll_dice::roll_dice(black_dice, red_dice)
                }
                _ => "Not implemented".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in the environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command.name("smile").description("Bot smiles back")
                })
                .create_application_command(|command| {
                    command.name("d10").description("Roll a ten-sided die")
                })
                .create_application_command(|command| {
                    command
                        .name("roll")
                        .description("Roll black and red dice together")
                        .create_option(|option| {
                            option
                                .name("black dice")
                                .description("Number of black dice to roll")
                                .kind(ApplicationCommandOptionType::Integer)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("red dice")
                                .description("Number of red dice to roll")
                                .kind(ApplicationCommandOptionType::Integer)
                                .required(true)
                        })
                })
        })
        .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );
    }
}

pub async fn run_bot() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}