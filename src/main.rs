use std::io::{Write, stdout};

use markov::Chain;
use rand::{Rng, RngCore};

use crate::{
    conf::read_multiplier,
    filters::{check_post, get_filter_configuration},
    posts::{Post, control_follows_for_user, get_followers_ids, get_posts},
};

mod conf;
mod filters;
mod posts;

fn main() {
    println!("collabmarkovpages  Copyright (C) 2025  Lunya
This program comes with ABSOLUTELY NO WARRANTY.
This is free software, and you are welcome to redistribute it under certain conditions. Please check LICENSE for more infomation.");

    let followers_ids = get_followers_ids();

    for i in &followers_ids {
        control_follows_for_user(i);
    }

    let mut all_posts: Vec<Post> = vec![];

    for i in 0..followers_ids.len() {
        let follower_i = followers_ids[i].clone();
        println!(
            "processing {} ({}/{})",
            follower_i,
            i + 1,
            followers_ids.len()
        );

        let mut posts = get_posts(follower_i);
        all_posts.append(&mut posts);
    }

    println!("now feeding chain");

    let mut total_corpus_length = 0;
    let mut total_post_count = 0;

    let mut chain = Chain::new();

    let filter_config = get_filter_configuration();

    println!("starting to go through posts");

    for post in all_posts {
        if post.text.is_none() {
            continue;
        }

        if !check_post(&filter_config, &post) {
            continue;
        }

        total_corpus_length += post.text.clone().unwrap().len();
        total_post_count += 1;
        chain.feed_str(post.text.unwrap().as_str());

        if total_post_count % 500 == 0 {
            print!("..{}", total_post_count);
            stdout().flush().ok();
        }
    }

    println!();
    println!("total users: {}", followers_ids.len());
    println!("total posts: {}", total_post_count);
    println!("total corpus length: {}", total_corpus_length);

    let mut rng = rand::rng();

    let title = loop {
        let a = chain.generate_str();
        if a.len() > 4 && a.len() < 16 {
            break a;
        }
    };

    println!("title..{}", title);

    let mut body: Vec<String> = vec![];

    print!("body");
    for i in 0..(rng.random_range(2..8) * read_multiplier()) {
        let body_el = loop {
            let a = chain.generate_str();
            if a.len() > 250 {
                break a;
            }
        };

        body.push(body_el);
        print!("..{}", i);
        stdout().flush().ok();
    }

    println!();

    let summary = loop {
        let a = chain.generate_str();
        if a.len() > 200 && a.len() < 256 {
            break a;
        }
    };

    posts::create_post(title, summary, body);
}
