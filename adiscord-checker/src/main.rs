use headless_chrome::Browser;
use std::{
    fs::{create_dir, read_to_string, write},
    path::Path,
};

const BASE_URL: &str = "https://discord.com/developers/docs/resources/";
const RESOURCES_NAME: [&str; 15] = [
    "application",
    "application-role-connection-metadata",
    "audit-log",
    "auto-moderation",
    "channel",
    "emoji",
    "guild",
    "guild-scheduled-event",
    "guild-template",
    "invite",
    "stage-instance",
    "sticker",
    "user",
    "voice",
    "webhook",
];

fn main() {
    let mut exists = true;
    if !Path::new("checks").exists() {
        create_dir("checks").unwrap();

        exists = false
    }

    let browser = Browser::default().unwrap();
    for name in RESOURCES_NAME {
        let tab = browser.new_tab().unwrap();
        tab.navigate_to(&format!("{BASE_URL}{name}").to_string())
            .unwrap();

        let elem = tab.wait_for_element("#app-mount").unwrap();
        let remote_object = elem
            .call_js_fn(&read_to_string("script.js").unwrap(), vec![], false)
            .unwrap();

        let json = goldilocks_json_fmt::format(
            &serde_json::from_value::<String>(remote_object.value.unwrap()).unwrap(),
        )
        .expect("Invalid JSON");

        if exists {
            if read_to_string(format!("checks/{name}.json")).unwrap() == json {
                println!("{name} good");
            } else {
                panic!("{name} not good");
            }
        } else {
            write(format!("checks/{name}.json"), json).unwrap();
        }
    }
}
