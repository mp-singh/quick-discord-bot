use crate::{BLACK_LIST, HARDLY};
use serenity::{model::channel::Message, prelude::Mentionable};

pub fn hardly(msg: &str) -> Option<String> {
    for cap in HARDLY.captures_iter(msg) {
        let word = cap.get(1).unwrap().as_str();
        if !BLACK_LIST.contains(word) {
            return Some(format!(
                "{}{}? I hardly know her!",
                word[0..1].to_uppercase(),
                &word[1..]
            ));
        }
    }
    None
}

pub fn shirley(msg: &str) -> Option<String> {
    match msg.to_lowercase().contains("surely") {
        true => Some("Don't call me Shirley!".to_string()),
        false => None,
    }
}

pub fn is_thanks(msg: &Message) -> Option<String> {
    let content = msg.content.to_lowercase();
    match content.contains("thank") || content.contains("thx") {
        true => Some(format!("No, thank you {}!", msg.author.mention())),
        false => None,
    }
}
