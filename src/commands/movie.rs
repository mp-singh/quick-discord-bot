use std::collections::HashMap;
use rand::prelude::*;
use titlecase::titlecase;

use crate::{MOVIE_CONTENTS, MOVIE1, MOVIE_SYNOPSIS1, MOVIE_SYNOPSIS2};
use crate::models::*;

fn pick(template: String, picked: &mut HashMap<String, Vec<String>>) -> String {
    let v = MOVIE_CONTENTS.get(&template).unwrap();
    let r: usize = rand::thread_rng().gen_range(0..v.len());
    let pickedval = v.get(r).unwrap().clone();
    if !picked.contains_key(&template) {
        picked.insert(template.clone(), Vec::new());
    }
    picked.get_mut(&template).unwrap().push(String::from(pickedval.clone()));
    return pickedval;
}

fn populate_template(tmpl: String, picked: &mut HashMap<String, Vec<String>>) -> String {
    let mut template = pick(String::from(tmpl), picked);
    while template.find('<').is_some() {
        template = MOVIE1.replace_all(&template, |caps: &regex::Captures| {
            let tmpl = caps.get(1).unwrap().as_str().to_string();
            if caps.get(2).is_none()  {
                return pick(tmpl, picked);
            }
            let num = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
            let list = picked.get(&tmpl);
            if list.is_some() {
                let found = list.unwrap().get(num);
                if found.is_some() {
                    return found.unwrap().clone();
                }
            }
            return pick(tmpl, picked);
        } ).to_mut().clone();
    }
    return template;
}

pub fn generate_movie() -> Movie {
    let mut picked = HashMap::new();
    let mut synopsis = populate_template(String::from("synopsis"), &mut picked);
    let title = populate_template(String::from("title"), &mut picked);
    synopsis = MOVIE_SYNOPSIS1.replace(&synopsis, "an $1").to_mut().clone();
    synopsis = MOVIE_SYNOPSIS2.replace_all(&synopsis, |caps: &regex::Captures| {
        caps.get(1).unwrap().as_str().to_owned() + &caps.get(2).unwrap().as_str().to_uppercase()
    }).to_mut().clone();
    return Movie {
        title: titlecase(&title),
        synopsis: synopsis
    };
}