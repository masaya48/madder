use std::error::Error;
use std::io::Write;
use std::{fs};
use std::fs::File;
use dialoguer::{
    Select,
    theme::ColorfulTheme
};
use dialoguer::console::Term;

use madder::configuration;

const REACT_COMPONENT_TEMPLATE: &'static str = r#"import React, {FC} from 'react'

export const {{name}}: FC = () => (
  <>{{key}} is rendered.</>
)
"#;

const REACT_STORY_TEMPLATE: &'static str = r#"import React from 'react'
import {ComponentStory, ComponentMeta} from '@storybook/react'
import {{{key}}} from './{{key}}'

export default {
  component: {{key}},
} as ComponentMeta<typeof {{key}}>

const Template: ComponentStory<typeof {{key}}> = args => <{{key}} {...args} />

export const Default = Template.bind({})
Default.args = {
}
"#;

const INDEX_TEMPLATE: &'static str = r#"export * from './{{key}}'
"#;

#[derive(Debug)]
struct Template {
    name: String,
    template: String,
}

struct LibraryTemplates {
    react: Vec<Template>,
}

fn create_template(t: Template) -> Result<(), Box<dyn Error>>{
    let templates_dir = configuration::get_templates_dir();
    let replace_key = configuration::get_replace_key();

    let mut f = File::create(format!("{}/{}", templates_dir, t.name))?;

    let replaced = t.template.replace("key", &replace_key);
    f.write_all(replaced.as_bytes())?;
    println!("{} was generated.", t.name);
    Ok(())
}

/**
 * component templatesを作成
 * react or vue
 */
fn create_templates() -> Result<(), Box<dyn Error>> {
    let templates_dir = configuration::get_templates_dir();
    fs::create_dir(templates_dir)?;
    let templates = LibraryTemplates {
        react: vec![
            Template { name: "template.tsx.hbs".to_string(), template: REACT_COMPONENT_TEMPLATE.to_string() },
            Template { name: "template.stories.tsx.hbs".to_string(), template: REACT_STORY_TEMPLATE.to_string() },
            Template { name: "index.ts.hbs".to_string(), template: INDEX_TEMPLATE.to_string() }
        ]
    };
    let items = vec!["react"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which library do you want?")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        ?;

    match selection {
        Some(index) => {
            let s = items[index];
            match s {
                "react" => {
                    let react_templates = templates.react;
                    for t in react_templates.into_iter() {
                        create_template(t).unwrap();
                    };
                }
                &_ => print!("nothing match.")
            }
        },
        None => println!("User did not select anything")
    }
    Ok(())
}

pub fn init() {
    println!("Initialize the Component Generator.");
    match configuration::create_config() {
        Ok(s) => println!("{} config file", s),
        Err(e) => panic!("There was a problem checking the config file: {:?}", e), 
    }
    match create_templates() {
        Ok(_) => println!("Successfully created templates."),
        Err(e) => panic!("There was a problem creating the templates: {:?}", e),
    }
}
