use std::fs;

use image_conv::{conv, Filter, PaddingType};
use photon_rs::native::{open_image_from_bytes, save_image};
use rand::{prelude::SliceRandom, Rng};
use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::{application::component::ButtonStyle, channel::Message};
use serenity::utils::{Content, ContentModifier::Spoiler};

use crate::commands::movie;
use crate::lazy_statics::{NASA_API_KEY, REGEX_DICE, REQUEST, TRANSFORMATION_TYPES};

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
    let ip = REQUEST
        .get("https://api.ipify.org")
        .send()
        .await?
        .text()
        .await?;

    msg.reply(ctx, ip).await?;
    Ok(())
}

#[command]
#[example(": ~joke")]
#[description("Get a random joke")]
pub async fn joke(ctx: &Context, msg: &Message) -> CommandResult {
    let joke = REQUEST
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
#[example(": ~yomama")]
#[description("Get a random yomama joke")]
pub async fn yomama(ctx: &Context, msg: &Message) -> CommandResult {
    let yomama = REQUEST
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
    let trivia = REQUEST
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
#[description("Generate a random excuse why dev work isn't complete!")]
pub async fn excuse(ctx: &Context, msg: &Message) -> CommandResult {
    let excuse = REQUEST
        .get("https://api.devexcus.es")
        .send()
        .await?
        .json::<Excuse>()
        .await
        .unwrap();

    msg.reply(ctx, excuse.text).await?;
    Ok(())
}

#[command]
#[usage(": ~chuck_norris")]
#[example(": ~chuck, ~chuck_norris, ~chucknorris")]
#[description("Get your daily Chuck Norris fact!")]
#[aliases("chuck", "chucknorris")]
pub async fn chuck_norris(ctx: &Context, msg: &Message) -> CommandResult {
    let chuck_norris = REQUEST
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
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("This face doesn't exist")
                    .image("https://thispersondoesnotexist.com/image")
            })
        })
        .await;
    Ok(())
}

#[command]
#[usage(": ~count <phrase>")]
#[example(": ~count turkey, ~count Turkey Tuesdays")]
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
#[example(": ~haphazardly 1,2,test,nickel,im having too much fun,8")]
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
#[example(": ~roll | ~roll 69 | ~roll 4d10")]
#[max_args(1)]
#[description("Roll a dice")]
pub async fn roll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        let response = rand::thread_rng().gen_range(1..7);
        msg.reply(ctx, response.to_string()).await?;
        return Ok(());
    }

    match REGEX_DICE.captures(args.message()) {
        Some(captures) => {
            let dice_count = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
            let dice_sides = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
            let mut dice_rolls: Vec<u128> = Vec::new();
            for _ in 0..dice_count {
                dice_rolls.push(rand::thread_rng().gen_range(1..dice_sides + 1).into());
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
                dice_rolls.iter().sum::<u128>()
            );
            if response.len() > 2000 {
                msg.reply(ctx, "Too many dice to display!").await?;
            } else {
                msg.reply(ctx, response).await?;
            }
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
}

#[command]
#[usage(": ~trump")]
#[description("Generate a random, completely factual, Donald Trump quote.")]
pub async fn trump(ctx: &Context, msg: &Message) -> CommandResult {
    let trump = REQUEST
        .get("https://api.whatdoestrumpthink.com/api/v1/quotes/random")
        .send()
        .await?
        .json::<Trump>()
        .await?;

    msg.reply(ctx, trump.message).await?;
    Ok(())
}

#[command]
#[usage(": ~pirate")]
#[min_args(1)]
#[description("Translate english into something piratey could say in a drunken pirate voice.")]
pub async fn pirate(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let translated = REQUEST
        .get("https://pirate.monkeyness.com/api/translate")
        .query(&[("english", args.message().to_string())])
        .header("Accept", "text/plain")
        .send()
        .await?
        .text()
        .await?;

    msg.reply(ctx, translated).await?;
    Ok(())
}

#[command]
#[usage(": ~cv")]
#[min_args(1)]
#[max_args(1)]
#[description("")]
pub async fn cv(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if msg.attachments.is_empty() {
        msg.reply(ctx, "You need to provide an image to translate!")
            .await?;
        return Ok(());
    }
    let transformation = match TRANSFORMATION_TYPES.contains_key(&args.message()) {
        true => TRANSFORMATION_TYPES.get(&args.message()).unwrap(),
        false => {
            msg.reply(ctx, "You need to provide a valid transformation!")
                .await?;
            return Ok(());
        }
    };

    let api_image = REQUEST
        .get(&msg.attachments.first().unwrap().url)
        .send()
        .await?
        .bytes()
        .await?
        .to_vec();

    let img = open_image_from_bytes(&api_image).expect("No such file found");

    let filter = Filter::from(transformation.to_vec(), 3, 3);

    // Apply convolution
    let transformed = conv::convolution(&img, filter, 1, PaddingType::UNIFORM(1));
    let filename = format!("transformed-{}.png", chrono::Utc::now().timestamp());
    //TODO: figure out a way to not save the image to disk and then removing it later
    save_image(transformed, &filename);
    let file_path = format!("./{}", &filename);

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Image after transformation")
                    .image(format!("attachment://{}", &filename))
                // .footer(|f| f.text("Note this image message will be deleted in 24 hours."))
            })
            .add_file(file_path.as_str())
        })
        .await;

    //cleanup file
    fs::remove_file(file_path)?;
    Ok(())
}

#[command]
#[usage(": ~lucky")]
#[description("Links to Googles im feeling lucky link when you search for something")]
pub async fn lucky(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let response = REQUEST
        .get("https://www.google.com/search")
        .query(&[
            ("q", args.message().to_string()),
            ("btnI", "I'm Feeling Lucky".to_string()),
        ])
        .send()
        .await
        .unwrap();

    let url = response
        .headers()
        .get("Location")
        .unwrap()
        .to_str()
        .unwrap();

    msg.reply(ctx, url.split("q=").collect::<Vec<&str>>()[1])
        .await?;
    Ok(())
}

#[command]
#[usage(": ~now")]
#[description("Returns the time")]
pub async fn now(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, format!("<t:{}:F>", chrono::Utc::now().timestamp()))
        .await?;
    Ok(())
}

#[command]
#[usage(": ~movie")]
#[description("Generates a random movie")]
pub async fn movie(ctx: &Context, msg: &Message) -> CommandResult {
    let movie = movie::generate_movie();
    msg.reply(
        ctx,
        format!("__**{}**__\n\n{}", movie.title, movie.synopsis),
    )
    .await?;
    Ok(())
}

#[command]
#[usage(": ~xkcd | ~xkcd random | ~xkcd <comic_num>")]
#[example(": ~xkcd | ~xkcd random | ~xkcd 69")]
#[description(
    "Get an xkcd comic. If you provide an invalid comic number, you will get a latest comic."
)]
async fn xkcd(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let latest = REQUEST
        .get("https://xkcd.com/info.0.json")
        .send()
        .await?
        .json::<XkcdComic>()
        .await?;
    let mut comic_num = latest.num;
    if args.message() == "random" {
        comic_num = rand::thread_rng().gen_range(1..comic_num + 1)
    }

    comic_num = args.single::<u16>().unwrap_or(comic_num);
    let xkcd_url = format!("https://xkcd.com/{}/info.0.json", comic_num);
    let response = REQUEST.get(xkcd_url).send().await?;

    if response.status() == 404 {
        msg.reply(ctx, "Please provide a valid xkcd comic ID!")
            .await?;
        return Ok(());
    }

    let comic = response.json::<XkcdComic>().await?;
    let title = comic.title;
    let alt = comic.alt;
    let num = comic.num;
    let page = format!("https://xkcd.com/{}", num);
    let wiki = format!("https://explainxkcd.com/wiki/index.php/{}", num);

    msg.channel_id
        .send_message(ctx, |message| {
            message.embed(|embed| {
                embed.title(title);
                embed.description(alt);
                embed.image(comic.img.as_str());
                embed.footer(|f| f.text(format!("xkcd comic no. {}", &num)));
                embed
            });
            message.components(|c| {
                c.create_action_row(|row| {
                    row.create_button(|b| {
                        b.label("View xkcd image page")
                            .style(ButtonStyle::Link)
                            .url(page)
                    })
                });
                c.create_action_row(|row| {
                    row.create_button(|b| {
                        b.label("View xkcd explanation")
                            .style(ButtonStyle::Link)
                            .url(wiki)
                    })
                });
                c
            });
            message
        })
        .await?;

    Ok(())
}

#[command]
#[usage(": ~nasa")]
#[description("Displays a random image from NASA's Astronomy Picture of the Day")]
async fn nasa(ctx: &Context, msg: &Message) -> CommandResult {
    let pic = REQUEST
        .get(format!(
            "https://api.nasa.gov/planetary/apod?api_key={}",
            NASA_API_KEY.as_str()
        ))
        .send()
        .await?
        .json::<NASAPicOfTheDay>()
        .await?;

    msg.channel_id
        .send_message(ctx, |message| {
            message.embed(|embed| {
                embed.title(pic.title);
                embed.image(pic.url.as_str());
                embed.footer(|f| f.text(format!("© {}", &pic.copyright)));
                embed
            });
            message
        })
        .await?;

    Ok(())
}
