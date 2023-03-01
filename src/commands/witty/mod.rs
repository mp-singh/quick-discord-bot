use serenity::{model::channel::Message, prelude::Mentionable};

use crate::{
    lazy_statics::{BLACK_LIST, HARDLY, IM_REGEX},
    utils::syllables::count_syllables,
};

// #[allow(dead_code)]
pub fn hardly(msg: &Message) -> Option<String> {
    if let Some(value) = check_guild(msg) {
        return value;
    }
    for cap in HARDLY.captures_iter(msg.content.as_str()) {
        let word = cap.get(1).unwrap().as_str();
        if count_syllables(word) > 1 && !BLACK_LIST.contains(word.to_lowercase().as_str()) {
            return Some(format!(
                "{}{}? I hardly know her!",
                word[0..1].to_uppercase(),
                &word[1..]
            ));
        }
    }
    None
}

pub fn im_response(msg: &Message) -> Option<String> {
    if let Some(value) = check_guild(msg) {
        return value;
    }
    IM_REGEX
        .captures(msg.content.to_lowercase().as_str())
        .map(|cap| {
            if cap.get(1).unwrap().as_str() == "horny" || cap.get(1).unwrap().as_str() == "corny" {
                "You should get that checked out.".to_string()
            } else {
                format!("Hi {}, I'm corny!", cap.get(1).unwrap().as_str())
            }
        })
}

fn check_guild(msg: &Message) -> Option<Option<String>> {
    if msg.guild_id.unwrap() != 691497006027505697 {
        return Some(None);
    }
    None
}

pub fn shirley(msg: &str) -> Option<String> {
    match msg.to_lowercase().contains("surely") {
        true => Some("Don't call me Shirley!".to_string()),
        false => None,
    }
}

pub fn thanks(msg: &Message) -> Option<String> {
    let content = msg.content.to_lowercase();
    match content.contains("thank") || content.contains("thx") {
        true => Some(format!("No, thank you {}!", msg.author.mention())),
        false => None,
    }
}
