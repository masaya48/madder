use std::{fs::{File, self}, error::Error, io::ErrorKind};
use config::Config;
use dialoguer::Input;

fn get_config() -> Config {
    let config = Config::builder()
        .add_source(config::File::with_name("madder"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();
    config
}

pub fn get_templates_dir() -> String {
    let config = get_config();
    config.get_string("templates_dir").unwrap()
}

pub fn get_replace_key() -> String {
    let config = get_config();
    config.get_string("replace_key").unwrap()
}

pub fn create_config() -> Result<String, Box<dyn Error>> {
    let config = "madder.yml";
    let c = File::open(config);
    match c {
        Ok(_) => {
            Ok("Existed".to_string())
        },
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create(config) {
                Ok(_) => {
                    println!("There is no config file, so created config file.");
                    write_config();
                    Ok("Created".to_string())
                },
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    }
}

 /**
  * 使用ライブラリの選択
  * template内のリプレイスキーの設定を行う
  */
fn write_config() {
    let templates_dir : String = Input::new()
        .with_prompt("Where do you place the templates directory?")
        .with_initial_text("__templates")
        .default("__templates".into())
        .interact_text().unwrap();
    
    let replace_key : String = Input::new()
        .with_prompt("What key do you want to convert?")
        .with_initial_text("name")
        .default("name".into())
        .interact_text().unwrap();
    fs::write("madder.yml", format!("templates_dir: {}\nreplace_key: {}", templates_dir, replace_key)).unwrap();
}
