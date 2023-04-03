use crate::{
    models::{ChuckNorris, Excuse, Trump, NASAPicOfTheDay, Joke},
    utils::interactions::Interaction, lazy_statics::NASA_API_KEY,
};
use rand::Rng;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{interaction::application_command::ApplicationCommandInteraction},
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
    let mut interaction = Interaction::new(ctx, command, false);

    let Ok(response) = REQUEST
        .get(format!(
            "https://api.nasa.gov/planetary/apod?api_key={}",
            NASA_API_KEY.as_str()
        ))
        .send()
    .await else {
        interaction.reply("Unable to grab nasa pic of the day").await;
        return;
    };

    let Ok(pic) = response
    .json::<NASAPicOfTheDay>()
    .await else {
        interaction.reply("Unable to grab nasa pic of the day").await;
        return;
    };
    
    let Ok(_) = command.channel_id
    .send_message(ctx, |message| {
        message.embed(|embed| {
            embed.title(pic.title);
            embed.image(pic.url.as_str());
            embed.footer(|f| f.text(format!("© {}", &pic.copyright)));
            embed
        });
        message
    })
    .await else {
        interaction.reply("Unable to grab nasa pic of the day").await;
        return;
    };
    
}

pub fn register_flip(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("flip")
        .description("flip a coin and get heads or tails")
}

pub async fn run_flip(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut interaction = Interaction::new(ctx, command, false);

    let response = match rand::thread_rng().gen_range(0..2) {
        0 => "Heads",
        1 => "Tails",
        _ => "Shit's broken yo!",
    };

    interaction.reply(response).await;
    
}

pub fn register_face(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("face")
        .description("Generates a face that doesn't exist")
}

pub async fn run_face(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut interaction = Interaction::new(ctx, command, false);
    
    let Ok(_) = command.channel_id
    .send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("This face doesn't exist")
                .image("https://thispersondoesnotexist.com/image")
        })
    })
    .await else {
        interaction.reply("Unable to grab nasa pic of the day").await;
        return;
    };
}
