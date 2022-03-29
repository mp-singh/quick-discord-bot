use rand::prelude::SliceRandom;
use rand::Rng;
use serenity::client::Context;
use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::utils::Content;
use serenity::utils::ContentModifier::Spoiler;

use crate::{REGEX_DICE, REQESUT};

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
        .await?
        .text()
        .await?;

    msg.reply(ctx, ip).await?;
    Ok(())
}

#[command]
#[usage(": ~joke")]
#[description("Get a random joke")]
pub async fn joke(ctx: &Context, msg: &Message) -> CommandResult {
    let joke = REQESUT
        .get("https://icanhazdadjoke.com/")
        .header("Accept", "text/plain")
        .send()
        .await?
        .text()
        .await?;
    msg.reply(ctx, joke).await?;
    Ok(())
}

#[command]
#[usage(": ~yomama")]
#[description("Get a random yomama joke")]
pub async fn yomama(ctx: &Context, msg: &Message) -> CommandResult {
    let yomama = REQESUT
        .get("https://api.yomomma.info/")
        .header("Accept", "text/plain")
        .send()
        .await?
        .json::<Joke>()
        .await?;
    msg.reply(ctx, yomama.joke).await?;
    Ok(())
}

#[command]
#[usage(": ~trivia")]
#[description("Get a random trivia question")]
pub async fn trivia(ctx: &Context, msg: &Message) -> CommandResult {
    let trivia = REQESUT
        .get("https://opentdb.com/api.php?amount=1")
        .header("Accept", "application/json; charset=utf-8")
        .send()
        .await?
        .json::<TriviaQuestions>()
        .await?
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
#[usage(": ~excuse")]
#[description("Generate a random excuse for not joining the fun!")]
pub async fn excuse(ctx: &Context, msg: &Message) -> CommandResult {
    let excuse = REQESUT
        .get("https://excuser.herokuapp.com/v1/excuse")
        .send()
        .await?
        .json::<Vec<Excuse>>()
        .await?
        .into_iter()
        .next();

    match excuse {
        Some(excuse) => msg.reply(ctx, excuse.excuse).await?,
        None => msg.reply(ctx, "Shit's broken yo!").await?,
    };
    Ok(())
}

#[command]
#[usage(": ~chuck")]
#[description("Get your daily Chuck Norris fact!")]
#[aliases("chuck", "chucknorris")]
pub async fn chuck_norris(ctx: &Context, msg: &Message) -> CommandResult {
    let chuck_norris = REQESUT
        .get("https://api.chucknorris.io/jokes/random")
        .send()
        .await?
        .json::<ChuckNorris>()
        .await?;

    msg.reply(ctx, chuck_norris.value).await?;
    Ok(())
}

#[command]
#[description("Generates a face that doesn't exist")]
pub async fn face(ctx: &Context, msg: &Message) -> CommandResult {
    // attach the image to the message
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.image("https://thispersondoesnotexist.com/image");
                e
            })
        })
        .await?;
    Ok(())
}

#[command]
#[usage(": ~count test")]
#[description("Counts the number of occurance of a phrase in a messages")]
pub async fn count(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    //filter out the message content that starts with "~"
    let phrase = args.message().to_string();
    let count = msg
        .channel_id
        .messages(&ctx.http, |m| m.limit(100))
        .await?
        .into_iter()
        .filter(|m| !m.author.bot && !m.content.starts_with('~'))
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

#[command]
#[usage(": ~flip")]
#[description("Flip a coin")]
pub async fn flip(ctx: &Context, msg: &Message) -> CommandResult {
    // flip a coin
    let response = match rand::thread_rng().gen_range(0..2) {
        0 => "Heads",
        1 => "Tails",
        _ => "Shit's broken yo!",
    };
    msg.reply(ctx, response).await?;
    Ok(())
}

#[command]
#[usage(": ~haphazardly item1,item2,item3...itemN")]
#[description("Choose a random item from a provided list")]
pub async fn haphazardly(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.reply(ctx, "You need to provide a list of items to choose from!")
            .await?;
        return Ok(());
    }

    let list = args.message().to_string();
    let list_items: Vec<&str> = list
        .split(',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect();
    let random_item = list_items.choose(&mut rand::thread_rng());
    let response = match random_item {
        Some(item) => item.trim().to_string(),
        None => "Shit's broken yo!".to_string(),
    };
    msg.reply(ctx, response).await?;
    Ok(())
}

#[command]
#[usage(": ~roll | ~roll <number> | ~roll <number>d<number>")]
#[max_args(1)]
#[description("Roll a dice")]
pub async fn roll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        let response = rand::thread_rng().gen_range(1..7);
        msg.reply(ctx, response.to_string()).await?;
        return Ok(());
    }

    if !args.is_empty() {
        match REGEX_DICE.captures(args.message()) {
            Some(captures) => {
                let dice_count = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
                let dice_sides = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
                let mut dice_rolls: Vec<u64> = Vec::new();
                for _ in 0..dice_count {
                    dice_rolls.push(rand::thread_rng().gen_range(1..dice_sides + 1));
                }
                let response = format!(
                    "Rolled {}d{}!\n[{}]\nTotal: {}",
                    dice_count,
                    dice_sides,
                    dice_rolls
                        .iter()
                        .map(|r| r.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    dice_rolls.iter().sum::<u64>()
                );
                msg.reply(ctx, response).await?;
                return Ok(());
            }
            None => match args.message().parse::<u64>() {
                Ok(number) => {
                    let response = rand::thread_rng().gen_range(1..number + 1).to_string();
                    msg.reply(ctx, response).await?;
                    return Ok(());
                }
                Err(_) => {
                    msg.reply(ctx, "Don't be a smart ass and pick a valid input!")
                        .await?;
                    return Ok(());
                }
            },
        }
    };
    Ok(())
}

#[command]
#[usage(": ~trump")]
#[description("Generate a random, completely factual, Donald Trump quote.")]
pub async fn trump(ctx: &Context, msg: &Message) -> CommandResult {
    let trump = REQESUT
        .get("https://api.whatdoestrumpthink.com/api/v1/quotes/random")
        .send()
        .await?
        .json::<Trump>()
        .await?;

    msg.reply(ctx, trump.message).await?;
    Ok(())
}
