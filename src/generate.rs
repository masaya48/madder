use std::fs;
use std::fs::File;
use regex::Regex;
use handlebars::{Handlebars, to_json};
use serde_json::value::Map;
use std::error::Error;
use madder::configuration;

fn make_dir(path: &str) {
    fs::create_dir_all(path).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}

pub fn generate (name: String, path: String) -> Result<(), Box<dyn Error>> {
    println!("Using file name: {}", name);
    let mut file_path = path;
    let reg = Regex::new(r"/$").unwrap();
    if !reg.is_match(&file_path) {
        file_path = format!("{}/", file_path)
    }
    file_path = format!("{}{}/", file_path, name);

    make_dir(&file_path);
    println!("generated dir: {}", file_path);

    let template_dir = configuration::get_templates_dir();
    let replace_key = configuration::get_replace_key();

    let mut data = Map::new();
    data.insert(replace_key, to_json(&name));

    let mut handlebars = Handlebars::new();

    for entry in fs::read_dir(template_dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = &name;
        let template_file_name = entry.file_name().into_string().unwrap();
        let file_name = template_file_name
            .replace("template", &name)
            .replace(".hbs", "");
        let file_path = format!("{}{}", file_path, file_name);
        handlebars.register_template_file(&file_name, path)?;
        let mut output_file = File::create(file_path)?;
        handlebars.render_to_write(&file_name, &data, &mut output_file)?;
        println!("Successfully created: {}", file_name);
    }
    println!("completed generating components.");
    Ok(())
}
