use crate::{
    lazy_statics::NASA_API_KEY,
    models::{ChuckNorris, Excuse, Joke, NASAPicOfTheDay, Trump},
    utils::interactions::Interaction,
};
use serenity::{
    builder::{CreateApplicationCommand, CreateMessage},
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

use crate::lazy_statics::REQUEST;

pub fn register_ping(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}

pub async fn run_ping(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut interaction = Interaction::new(ctx, command, false);
    interaction.reply("Pong Baby!").await;
}

pub fn register_now(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("now").description("Get your current time")
}

pub async fn run_now(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut interaction = Interaction::new(ctx, command, false);
    interaction
        .reply(format!("<t:{}:F>", chrono::Utc::now().timestamp()))
        .await;
}

pub fn register_ip(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ip")
        .description("Get the IP address of the bot")
}

pub async fn run_ip(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut interaction = Interaction::new(ctx, command, false);
    let Ok(ip) = REQUEST
    .get("https://api.ipify.org")
    .send()
    .await
    .unwrap()
    .text()
    .await else {
        interaction.reply("Unable to get IP address").await;
        return;
    };
    interaction.reply(&ip).await;
}

pub fn register_joke(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("joke").description("Get a random dad joke")
}

pub async fn run_joke(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut interaction = Interaction::new(ctx, command, false);

    let Ok(response) = REQUEST
    .get("https://icanhazdadjoke.com/")
    .header("Accept", "text/plain")
    .send()
    .await else {
        interaction.reply("Unable to get joke").await;
        return;
    };

    let Ok(joke) = response
    .text()
    .await else {
        interaction.reply("Unable to get joke").await;
        return;
    };
    interaction.reply(&joke).await;
}

pub fn register_yomama(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("yomama")
        .description("Get a random yo mama joke")
}

pub async fn run_yomama(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut interaction = Interaction::new(ctx, command, false);

    let Ok(response) = REQUEST
        .get("https://api.yomomma.info/")
        .send()
    .await else {
        interaction.reply("Unable to get yo mama joke").await;
        return;
    };

    let Ok(joke) = response
    .json::<Joke>()
    .await else {
        interaction.reply("Unable to get yo mama joke").await;
        return;
    };
    interaction.reply(&joke.joke).await;
}

pub fn register_excuse(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("excuse")
        .description("Get a random developer excuse")
}

pub async fn run_excuse(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut interaction = Interaction::new(ctx, command, false);

    let Ok(response) = REQUEST
        .get("https://api.devexcus.es")
        .send()
    .await else {
        interaction.reply("Unable to get an excuse").await;
        return;
    };

    let Ok(excuse) = response
    .json::<Excuse>()
    .await else {
        interaction.reply("Unable to get an excuse").await;
        return;
    };
    interaction.reply(&excuse.text).await;
}

pub fn register_chuck_norris(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("chucknorris")
        .description("Get a random Chuck Norris fact")
}

pub async fn run_chuck_norris(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut interaction = Interaction::new(ctx, command, false);

    let Ok(response) = REQUEST
        .get("https://api.chucknorris.io/jokes/random")
        .send()
    .await else {
        interaction.reply("Unable to get a Chuck Norris fact").await;
        return;
    };

    let Ok(chuck_norris) = response
    .json::<ChuckNorris>()
    .await else {
        interaction.reply("Unable to get a Chuck Norris fact").await;
        return;
    };
    interaction.reply(&chuck_norris.value).await;
}

pub fn register_trump(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("trump")
        .description("Generate a random, completely factual, Donald Trump quote")
}

pub async fn run_trump(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut interaction = Interaction::new(ctx, command, false);

    let Ok(response) = REQUEST
        .get("https://api.whatdoestrumpthink.com/api/v1/quotes/random")
        .send()
    .await else {
        interaction.reply("Unable to get a trump quote").await;
        return;
    };

    let Ok(trump) = response
    .json::<Trump>()
    .await else {
        interaction.reply("Unable to get a trump quote").await;
        return;
    };
    interaction.reply(&trump.message).await;
}

pub fn register_nasa(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("nasa")
        .description("Displays a random image from NASA's Astronomy Picture of the Day")
}

pub async fn run_nasa(ctx: &Context, command: &ApplicationCommandInteraction) {
    println!("Getting nasa pic of the day");
    let mut interaction = Interaction::new(ctx, command, false);

    let Ok(response) = REQUEST
        .get(format!(
            "https://api.nasa.gov/planetary/apod?api_key={}",
            NASA_API_KEY.as_str()
        ))
        .send()
    .await else {
        interaction.reply("Unable to get nasa pic of the day, bad request").await;
        return;
    };
    println!("Got nasa pic of the day");

    let pic = match response.json::<NASAPicOfTheDay>().await {
        Ok(pic) => pic,
        Err(e) => {
            println!("{:?}", e);
            interaction
                .reply("Unable to get nasa pic of the day, bad response")
                .await;
            return;
        }
    };
    
    println!("{:?}", pic);
    let Ok(_) = command.channel_id.send_message(&ctx.http, |message| {
        message.embed(|embed| {
            embed.title(pic.title);
            embed.image(pic.url.as_str());
            embed.footer(|f| {
                if pic.copyright.is_none() {
                    return f.text("© NASA");
                } else {
                    f.text(format!("© {:?}", &pic.copyright.unwrap()))
                }
            });
            embed
        });
        message
    }).await else {
        interaction.reply("Unable to get nasa pic of the day").await;
        return;
    };
}
