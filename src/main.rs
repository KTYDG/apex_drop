use std::collections::HashMap;

use {
    chrono::Local,
    dotenv,
    rand::Rng,
    teloxide::{
        prelude::*,
        types::{InputFile, KeyboardButton, KeyboardMarkup},
    },
    tokio::{
        fs::OpenOptions,
        io::AsyncWriteExt
    },
};

#[tokio::main]
async fn main() {
    // 1) Create .env file
    // 2) Paste here: TELOXIDE_TOKEN="<your token>"
    // 3) ..
    // 4) PROFIT
    dotenv::dotenv().expect("Could not set env variables");
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |mes: Message, bot: AutoSend<Bot>| async move {
        match mes.text() {
            Some("/start") => {
                let mut file = OpenOptions::new().write(true).append(true).open("users.txt").await.unwrap();
                let user = mes.from().unwrap();
                let info = format!(
                    "\nid: {}\nfirst_name: {}\nlast_name: {}\nusername: {}\n##############################\n",
                    user.id,
                    user.first_name,
                    user.last_name.as_ref().unwrap_or(&"None".to_string()),
                    user.username.as_ref().unwrap_or(&"None".to_string())
                );
                file.write_all(info.as_bytes()).await.expect("can't find file");

                bot.send_message(mes.chat.id, "Choose map:")
                    .reply_markup(make_keyboard())
                    .await?
            }
            Some("Kings Canyon") => {
                let (path, place) = kings_canyon_rand();
                bot.send_photo(mes.chat.id, path).caption(place).await?
            }
            Some("Worlds Edge") => {
                bot.send_photo(
                    mes.chat.id,
                    teloxide::types::InputFile::file("./maps/WorldsEdge/WorldsEdge.webp"),
                )
                .await?
            }
            Some("Olympus") => {
                bot.send_photo(
                    mes.chat.id,
                    teloxide::types::InputFile::file("./maps/Olympus/Olympus.webp"),
                )
                .await?
            }
            Some("Storm Point") => {
                bot.send_photo(
                    mes.chat.id,
                    teloxide::types::InputFile::file("./maps/StormPoint/StormPoint.webp"),
                )
                .await?
            }
            _ => bot.send_message(mes.chat.id, "Unknown text").await?,
        };
        respond(())
    })
    .await;
}

fn make_keyboard() -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    let debian_versions = ["Kings Canyon", "Worlds Edge", "Olympus", "Storm Point"];

    for versions in debian_versions.chunks(1) {
        let row = versions
            .iter()
            .map(|&version| KeyboardButton::new(version.to_owned()))
            .collect();
        keyboard.push(row);
    }

    KeyboardMarkup::new(keyboard)
}

fn kings_canyon_rand() -> (InputFile, String) {
    let mut places = HashMap::new();

    places.insert(1, "Airbase");
    places.insert(2, "ARES Capacitor");
    places.insert(3, "Artillery Battery");

    places.insert(4, "Basin");
    places.insert(5, "Broken Coast Overlook");
    places.insert(6, "Bunker Pass");

    places.insert(7, "Cage");
    places.insert(8, "Caustic Treatment");
    places.insert(9, "Crash Site");
    places.insert(10, "Creature Containment");
    places.insert(11, "Crypto's Map Room");

    places.insert(12, "High Desert");
    places.insert(13, "Hillside Outpost");
    places.insert(14, "Hydro Dam");

    places.insert(15, "Marketplace");

    places.insert(16, "Octane's Gauntlet");
    places.insert(17, "Offshore Rig");

    places.insert(18, "Reclaimed Forest");
    places.insert(19, "Relic");
    places.insert(20, "Repulsor");
    places.insert(21, "River Center");
    places.insert(22, "Runoff");

    places.insert(23, "Singh Labs");
    places.insert(24, "Singh Labs Interior");
    places.insert(25, "Spotted Lake");
    places.insert(26, "Swamps");

    places.insert(27, "The Pit");
    places.insert(28, "Two Spines Outpost");

    places.insert(29, "Verdant Crossing");

    places.insert(30, "Watchtower North");
    places.insert(31, "Watchtower South");

    let number = rand::thread_rng().gen_range(1..31);
    let place = places.get(&number).unwrap_or(&"Relic").to_string();
    let filepath = format!("./maps/KingsCanyon/{}.png", place.replace(" ", ""));

    let date_time = Local::now();
    println!(
        "[{}] map: {number}, path: {filepath}",
        date_time.format("%d-%m-%Y | %H:%M:%S").to_string()
    );

    (teloxide::types::InputFile::file(filepath), place)
}
