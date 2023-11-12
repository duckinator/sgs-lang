use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "main.pest"]
struct SystemParser;

#[derive(Debug, PartialEq)]
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
    rows: u8,
    cols: u8,
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

    println!("{:?}", metadata);
    let rows = metadata.get("rows").ok_or("Expected 'rows' to be specified.")?.parse().unwrap();
    let cols = metadata.get("cols").ok_or("Expected 'cols' to be specified.")?.parse().unwrap();

    Ok(System { name, description, default, rows, cols, folders })
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

    let system = parse(system).unwrap();
    println!("{:?}", system);
}


#[test]
fn test_parser() {
    let system = r#"
        name = "Example System"
        description = "This is an example system."
        default = "Example Folder"

        rows = 2
        cols = 3

        folder "Example Folder" (append)
                "0"     "1"     "2"
                "3"     "4"     "5"
        ;
    "#;

    let system = parse(system).unwrap();

    assert_eq!("Example System", system.name);
    assert_eq!("This is an example system.", system.description);
    assert_eq!("Example Folder", system.default);

    assert_eq!(2, system.rows);
    assert_eq!(3, system.cols);

    let folder = &system.folders["Example Folder"];
    assert_eq!(FolderMode::Append, folder.mode);
    assert_eq!(6, folder.buttons.len());
    assert_eq!("0", folder.buttons[0]);
    assert_eq!("1", folder.buttons[1]);
    assert_eq!("2", folder.buttons[2]);
    assert_eq!("3", folder.buttons[3]);
    assert_eq!("4", folder.buttons[4]);
    assert_eq!("5", folder.buttons[5]);
}
