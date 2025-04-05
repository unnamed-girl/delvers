use delver_sim::entities::{BaseCharacter, Stats};
use poise::serenity_prelude as serenity;
use chronobase::database_connection::{AsyncTypebase, Chronobase, HTTPConnection};

struct UserData(HTTPConnection); // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, UserData, Error>;

#[poise::command(slash_command, prefix_command)]
async fn reserve_temporal_index(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let response = ctx.data().0.async_reserve_temporal_index().await;
    let ping = format!("<@{}>", &ctx.author().id);
    let response = match response {
        Ok(response) => format!("{} Next temporal index is {}", ping, response),
        Err(_) => format!("Error retrieving next temporal index")
    };
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn create_character(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let character = BaseCharacter::roll("Character".to_string(), Stats::example());
    ctx.data().0.async_save(character.id(), &character).await.map_err(|_| "error saving character")?;
    let character = ctx.data().0.async_load_latest(character.id(), None).await;

    let response = match character {
        Ok(character) => format!("{:?}", character),
        Err(_) => format!("Error retrieving character after creation")
    };
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN should be present in .env");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![reserve_temporal_index(), create_character()],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                let commands: Vec<serenity::CreateCommand> = framework.options().commands.iter().flat_map(|command| command.create_as_slash_command()).collect();
                for guild in &ready.guilds {
                    guild.id.set_commands(ctx, commands.clone()).await.unwrap();
                    ctx.online();
                }
                
                Ok(UserData(HTTPConnection::new("http://localhost:8000")))
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
