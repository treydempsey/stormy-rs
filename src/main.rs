// vim: et sw=4 ts=4

extern crate chrono;
extern crate hyper;
extern crate hyper_tls;
extern crate irc;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

mod dark_sky;
mod serde_utils;
//mod formatters;

use std::rc::Rc;

//use formatters::standard;
use futures::Future;
use futures::IntoFuture;
use irc::client::prelude::*;
use irc::client::Client;
use tokio_core::reactor;

//use open_weather_map::response::CityResponse;

fn process_message(reactor_handle: Rc<reactor::Handle>, irc_client: Rc<IrcClient>, message: Message) -> Result<(), irc::error::IrcError>
{
    println!("{:?}", message);
    if let Command::PRIVMSG(ref _target, ref msg) = message.command {
        if msg.starts_with("!w ") {
            let irc_client = Rc::clone(&irc_client);

            match irc_client.config().get_option("darksky_key") {
                Some(key) => {
                    let target = message.response_target().unwrap().to_owned();
                    let (latitude, longitude) = (29.7594,-95.3594);
                    let response = dark_sky::forecast(key, latitude, longitude)
                        .and_then(move |forecast| {
                            println!("Forecast: {:?}", forecast);
                            irc_client.send_privmsg(target, format!("{:?}", forecast)).unwrap();
                            Ok(())
                        })
                        .map_err(|e| {
                            println!("Error requesting API response: {:?}", e);
                        });

                    reactor_handle.spawn(response);
                }
                None => irc_client.send_privmsg(message.response_target().unwrap(), format!("Error: I can't find my Open Weather Map API Key. Set openweathermap_appid in my config.toml.")).unwrap(),
            }
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
