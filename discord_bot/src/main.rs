use std::time::Duration;

use delver_sim::{database::DatabaseManager, delver_display::{DisplayConstruct, ToDisplayConstruct}, game::Game};
use poise::serenity_prelude::{self as serenity, CreateMessage, MessageBuilder};
use chronobase::{DirectConnection, EntityID};
use tokio::time::sleep;
use uuid::Uuid;

struct UserData(DatabaseManager); // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, UserData, Error>;

async fn display_construct(ctx: Context<'_>, construct: DisplayConstruct) -> Result<(), Error> {
    match construct {
        a @ DisplayConstruct::ParentChildren(..) | a @ DisplayConstruct::Single(_) | a @ DisplayConstruct::List(_) | a @ DisplayConstruct::Multi(_) => {
            let message = MessageBuilder::new()
                .push_codeblock(a.to_string(), Some("ansi"))
                .build();
            let message = CreateMessage::new().content(message);
            ctx.channel_id().send_message(ctx, message).await?;            
        }
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn print_out_game(
    ctx: Context<'_>,
    game_id: Uuid,
    n: usize
) -> Result<(), Error> {
    let mut game = ctx.data().0.0.load(EntityID::<Game>::from(game_id), None, n)?;
    game.reverse();
    for state in game {
        let events = DisplayConstruct::Multi(state.latest_events.iter().map(|event| event.longform(&state, &ctx.data().0)).collect());
        display_construct(ctx, events).await?;
        sleep(Duration::from_millis(500)).await;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN should be present in .env");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![print_out_game()],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                let commands: Vec<serenity::CreateCommand> = framework.options().commands.iter().flat_map(|command| command.create_as_slash_command()).collect();
                for guild in &ready.guilds {
                    guild.id.set_commands(ctx, commands.clone()).await.unwrap();
                    ctx.online();
                }
                
                Ok(UserData(DatabaseManager::new(Box::new(DirectConnection::new("temp.db".to_string())))))
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
