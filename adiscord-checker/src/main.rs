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

const SCRIPT: &str = r"function send() {
    let myTable = []
    for (let i = 0; i < document.getElementsByTagName('table').length; i++) {
      myTable[i] = {}
  
      const childrens = document.getElementsByTagName('table')[i].childNodes
      childrens.forEach((children) => {
        switch (children.nodeName) {
          case 'TBODY': {
            myTable[i].childrens = {}
  
            const childrensBody = children.childNodes
            childrensBody.forEach((children, index) => {
              myTable[i].childrens[index] = []
  
              const childrensTd = children.childNodes
              childrensTd.forEach((children) => {
                myTable[i].childrens[index].push(children.textContent)
              })
            })
  
            return
          }
  
          case 'THEAD': {
            myTable[i].fields = []
  
            const childrensBody = children.childNodes
            childrensBody.forEach((children) => {
              const childrensTh = children.childNodes
              childrensTh.forEach((children) => {
                myTable[i].fields.push(children.textContent)
              })
            })
  
            return
          }
        }
      })
    }
  
    return JSON.stringify(myTable)
  }
  ";

fn main() {
    let mut exists = true;
    if !Path::new("checks").exists() {
        create_dir("checks").unwrap();

        exists = false
    }

    let browser = Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();
    for name in RESOURCES_NAME {
        tab.navigate_to(&format!("{BASE_URL}{name}").to_string())
            .unwrap();

        let elem = tab.wait_for_element("#app-mount").unwrap();
        let remote_object = elem.call_js_fn(SCRIPT, vec![], false).unwrap();
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
