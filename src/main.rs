use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "main.pest"]
struct SystemParser;

#[derive(Debug)]
enum FolderMode {
    Append,
    Speak,
}

#[derive(Debug)]
struct Folder<'a> {
    mode: FolderMode,
    buttons: Vec<&'a str>,
}

#[derive(Debug)]
struct System<'a> {
    name: &'a str,
    description: &'a str,
    default: &'a str,
    folders: HashMap<&'a str, Folder<'a>>,
}

fn parse(system: &str) -> Result<System, &'static str> {
    let pairs = SystemParser::parse(Rule::program, system).unwrap();

    let mut metadata: HashMap<&str, &str> = HashMap::new();
    let mut folders: HashMap<&str, Folder> = HashMap::new();

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        match pair.as_rule() {
            Rule::assignment => {
                let mut inner = pair.into_inner();
                let key = inner.next().expect("Got Rule::assignment with no inner items?").as_str();
                let value = inner.next().expect("Got Rule::assignment with only one inner item?").as_str();

                metadata.insert(key, value);
            },
            Rule::folder => {
                let mut inner = pair.into_inner();
                let name = inner.next().expect("Got Rule::folder with no name?").as_str();
                let mode = match inner.next().expect("Got Rule::folder with no mode?").as_str() {
                    "append" => FolderMode::Append,
                    "speak" => FolderMode::Speak,
                    _ => panic!("Valid modes are 'append' and 'speak'"),
                };

                let buttons: Vec<&str> = inner.map(|x| x.as_str()).collect();
                folders.insert(name, Folder { mode, buttons });
            },
            _ => panic!("Should only get an assignment or a folder."),
        }
    }

    let name = metadata.get("name").ok_or("Expected 'name' to be specified.")?;
    let description = metadata.get("description").ok_or("Expected 'description' to be specified.")?;
    let default = metadata.get("default").ok_or("Expected 'default' to be specified.")?;

    Ok(System { name, description, default, folders })
}

fn main() {
    let system = r#"
name = "Example system."
description = "This is an example system."
default = "Example Folder"

folder "foo" (append) "asdf";

folder "Example Folder" (append)
        "1"     "2"     "3"
        "4"     "5"     "6"
;
    "#;

    match parse(system) {
        Ok(system) => println!("{:?}", system),
        Err(msg) => panic!("{}", msg),
    }
}


#[test]
fn test_parser() {
    let system = r#"
        #sgs

        name = "Example system."
        description = "This is an example system."

        rows = 2
        cols = 3

        :Example Folder (append)
                "1"     "2"     "3"
                "4"     "5"     "6"
        .
    "#;

    println!("{}",system);
}
