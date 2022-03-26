use serenity::client::Context;
use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::utils::Content;
use serenity::utils::ContentModifier::Spoiler;

use crate::REQESUT;

use crate::models::*;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong").await?;
    Ok(())
}

#[command]
#[description("Get the IP address of the bot")]
#[help_available(false)]
pub async fn ip(ctx: &Context, msg: &Message) -> CommandResult {
    let ip = REQESUT
        .get("https://api.ipify.org")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    msg.reply(ctx, ip).await?;
    Ok(())
}

#[command]
#[description("Get a random joke")]
pub async fn joke(ctx: &Context, msg: &Message) -> CommandResult {
    let joke = REQESUT
        .get("https://icanhazdadjoke.com/")
        .header("Accept", "text/plain")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    msg.reply(ctx, joke).await?;
    Ok(())
}

#[command]
#[description("Get a random yomama joke")]
pub async fn yomama(ctx: &Context, msg: &Message) -> CommandResult {
    let yomama = REQESUT
        .get("https://api.yomomma.info/")
        .header("Accept", "text/plain")
        .send()
        .await
        .unwrap()
        .json::<Joke>()
        .await
        .unwrap();
    msg.reply(ctx, yomama.joke).await?;
    Ok(())
}

#[command]
#[description("Get a random trivia question")]
pub async fn trivia(ctx: &Context, msg: &Message) -> CommandResult {
    let trivia = REQESUT
        .get("https://opentdb.com/api.php?amount=1")
        .header("Accept", "application/json; charset=utf-8")
        .send()
        .await
        .unwrap()
        .json::<TriviaQuestions>()
        .await
        .unwrap()
        .results
        .into_iter()
        .next()
        .unwrap();

    let answer: Content = Spoiler + &trivia.correct_answer + Spoiler;

    msg.reply(
        ctx,
        html_escape::decode_html_entities(&format!(
            "{}\nAnswer: {}",
            trivia.question,
            answer.to_string()
        )),
    )
    .await?;
    Ok(())
}

#[command]
#[description("Generate a random excuse for not joining the fun!")]
pub async fn excuse(ctx: &Context, msg: &Message) -> CommandResult {
    let excuse = REQESUT
        .get("https://excuser.herokuapp.com/v1/excuse")
        .send()
        .await
        .unwrap()
        .json::<Vec<Excuse>>()
        .await
        .unwrap()
        .into_iter()
        .next()
        .unwrap();

    msg.reply(ctx, excuse.excuse).await?;
    Ok(())
}

#[command]
#[description("Get your daily Chuck Norris fact!")]
#[aliases("chuck", "chucknorris")]
pub async fn chuck_norris(ctx: &Context, msg: &Message) -> CommandResult {
    let chuck_norris = REQESUT
        .get("https://api.chucknorris.io/jokes/random")
        .send()
        .await
        .unwrap()
        .json::<ChuckNorris>()
        .await
        .unwrap();

    msg.reply(ctx, chuck_norris.value).await?;
    Ok(())
}

// #[command]
// #[description("Get your daily Chuck Norris fact!")]
// #[aliases("chuck", "chucknorris")]
// pub async fn face(ctx: &Context, msg: &Message) -> CommandResult {
//     let face = REQESUT
//         .get("https://thispersondoesnotexist.com/image")
//         .header("Accept", "image/jpeg")
//         .send()
//         .await
//         .unwrap()
//         .text()
//         .await
//         .unwrap();

//     println!("{}", face);
//     let chuck_norris = REQESUT
//         .get("https://api.chucknorris.io/jokes/random")
//         .send()
//         .await
//         .unwrap()
//         .json::<ChuckNorris>()
//         .await
//         .unwrap();

//     msg.reply(ctx, chuck_norris.value).await?;
//     Ok(())
// }

#[command]
#[description("Counts the number of occurance of a phrase in a messages")]
pub async fn count(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let phrase = args.message().to_string();
    let count = msg
        .channel_id
        .messages(&ctx.http, |m| m.limit(100))
        .await?
        .into_iter()
        .filter(|m| m.content.to_lowercase().contains(&phrase.to_lowercase()))
        .count();

    let response = match count {
        0 => format!("No messages found containing the phrase: \"{}\".", phrase),
        1 => format!("1 message found containing the phrase: \"{}\".", phrase),
        _ => format!(
            "{} messages found containing phrase: \"{}\".",
            count, phrase
        ),
    };
    msg.reply(ctx, response).await?;
    Ok(())
}
