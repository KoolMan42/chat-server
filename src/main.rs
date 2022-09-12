// use log::{debug, error, log_enabled, info, Level};
use std::{net::SocketAddr, time::Duration};

use futures_util::{SinkExt, StreamExt};
use futures_util::future::ok;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use tokio_tungstenite::tungstenite::{Message, Result};

#[derive(Serialize)]
struct User {
    username: String,
    color: String,
}

#[derive(Serialize)]
struct ChatMessage {
    user: User,
    message: String,
}


impl User {
    fn new() -> User {
        let name = get_username();
        let color = get_color();
        return User {
            username: name.parse().unwrap(),
            color: color.parse().unwrap(),
        };
    }
}

impl ChatMessage {
    fn new() -> ChatMessage {
        ChatMessage {
            message: get_message().parse().unwrap(),
            user: User::new(),
        }
    }
}

fn get_color() -> &'static str {
    let color = vec![
        "#EDFF21",
        "#1E5945",
        "#F80000",
        "#464531",
        "#E4A010",
    ];
    let value = color.choose(&mut thread_rng());
    return match value {
        Some(x) => {
            x
        }
        None => {
            ""
        }
    };
}

fn get_username() -> &'static str {
    let usernames = vec!["newspaperttv83",
                         "hotdogttv52",
                         "lipstickttv73",
                         "ttvpager82",
                         "joystickttv69",
                         "ttvseedling44",
                         "ttvvolcano70",
                         "beersttv37",
                         "stationttv29",
                         "ttvchipmunk35",
                         "capricornttv93",
                         "tramttv65",
                         "womensttv89",
                         "diamondsttv27",
                         "ttvmotorboat16",
                         "ttvbangbang20",
                         "speedboatttv48",
                         "ttvminibus18",
                         "bathttv39",
                         "ttvgrinning48",
                         "restroomttv35",
                         "ttvhaircut10",
                         "ttvlevitate36",
                         "kiwittv76",
                         "ramttv95",
                         "balloonttv1",
                         "sailboatttv59",
                         "cowboyttv31",
                         "churchttv54",
                         "peacettv93",
                         "ttvhospital82",
                         "ttvrofl59",
                         "fuelpumpttv14",
                         "ttvcouch84",
    ];
    let value = usernames.choose(&mut thread_rng());
    return match value {
        Some(x) => {
            x
        }
        None => {
            ""
        }
    };
}

fn get_message() -> &'static str {
    let messages = vec!["Testing", "rust lul", "Imagine using TypeScript lmao", "rust is best lang", "Svelte ❤"];

    let value = messages.choose(&mut thread_rng());

    return match value {
        Some(x) => {
            x
        }
        None => {
            ""
        }
    };
}


async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => println!("Error processing connection: {:?}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    println!("New WebSocket connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let mut interval = tokio::time::interval(Duration::from_millis(1500));

    // Echo incoming WebSocket messages and send a message periodically every second.

    loop {
        tokio::select! {
            msg = ws_receiver.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_text() ||msg.is_binary() {
                            ws_sender.send(msg).await?;
                        } else if msg.is_close() {
                            break;
                        }
                    }
                    None => break,
                }
            }
            _ = interval.tick() => {
                ws_sender.send(Message::Text(serde_json::to_string(&ChatMessage::new()).unwrap())).await?;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        println!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream));
    }

}