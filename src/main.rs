// vim: et sw=4 ts=4

extern crate hyper;
extern crate irc;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;

mod open_weather_map;

use futures::Future;
use futures::IntoFuture;
use irc::client::prelude::*;
use irc::client::Client;
use tokio_core::reactor;

use std::rc::Rc;

fn _format_message(_response: open_weather_map::response::City) {
}

fn process_message(reactor_handle: Rc<reactor::Handle>, irc_client: Rc<IrcClient>, message: Message) -> Result<(), irc::error::IrcError>
{
    println!("Message: {:?}", message);
    if let Command::PRIVMSG(ref _target, ref msg) = message.command {
        if msg.contains("pickles") {
            let irc_client = Rc::clone(&irc_client);
            let target = message.response_target().unwrap().to_owned();
            let response = open_weather_map::query::city(msg.clone())
                .and_then(move |body| {
                    match serde_json::from_str::<open_weather_map::response::City>(&body) {
                        Ok(data) => println!("Data: {:?}", data),
                        Err(e) => println!("Error: {:?}", e),
                    }
                    Rc::clone(&irc_client).send_privmsg(target, body).unwrap();
                    Ok(())
                });

            reactor_handle.spawn(response);
        }
    }
    Ok(())
}

fn main() {
    let config = Config::load("config.toml").expect("Error loading config file");
    let mut irc_reactor = IrcReactor::new().unwrap();
    let client = Rc::new(irc_reactor.prepare_client_and_connect(&config).unwrap());
    client.identify().unwrap();

    let reactor_handle = Rc::new(irc_reactor.inner_handle());
    let processor = client.stream().for_each(move |message| {
        reactor_handle.spawn(process_message(Rc::clone(&reactor_handle), Rc::clone(&client), message).into_future().map_err(|_| (())));
        Ok(())
    });
    irc_reactor.register_future(processor);
    irc_reactor.run().unwrap();
}
