extern crate env_logger;
#[macro_use]
extern crate serenity;
#[macro_use]
extern crate serde_derive;
use serenity::{
    client::Client, client::Context, framework::StandardFramework, model::event::ResumedEvent,
    model::gateway::Ready, prelude::EventHandler,
};

mod commands;
mod generator;
mod utils;


struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _resume: ResumedEvent) {
        println!("Resumed");
    }
}

fn main() {
    env_logger::init();

    let config : utils::Config = utils::get_config();

    let mut client =
        Client::new(&config.discord_bot_token.as_str(), Handler).expect("Error creating client");

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix(&config.prefix.as_str()))
            .cmd("start", commands::wordcloud::give_wordcloud),
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
