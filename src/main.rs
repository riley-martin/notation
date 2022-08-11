use chrono::prelude::{DateTime, Local};
use clap::{Arg, Command, SubCommand};
// use std::ffi::OsString;
use std::io::prelude::Write;
use std::{fs, path};

fn main() {
    let app = Command::new("Notation")
        .version("0.1.0")
        .author("Riley Martin")
        .about("Take and edit notes in the terminal")
        .subcommand(
            SubCommand::with_name("new")
                .about("Create a new note. If neither a title or body are given, opens in editor")
                .arg(Arg::with_name("title").help("Title of the note").index(1))
                .arg(Arg::with_name("body").help("Body of the note").index(2)),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .about("Edit a note")
                .arg(Arg::with_name("note").help("Note to edit").index(1)),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete a note")
                .arg(Arg::with_name("note").help("Note to delete").index(1)),
        )
        .arg(
            Arg::with_name("editor")
                .short('e')
                .long("editor")
                .takes_value(true)
                .help("which editor to use (uses $EDITOR by default)"),
        )
        .get_matches();

    match app.subcommand_name() {
        Some("new") => {
            let local_time: DateTime<Local> = Local::now();
            let mut note_file = new_note(local_time).expect("Couldn't create note");
            let subcmd_matches = app.subcommand().unwrap().1;
            if subcmd_matches.contains_id("title") {
                note_file
                    .write_all(
                        format!(
                            "# {}\n",
                            subcmd_matches.get_one::<String>("title").expect("no title")
                        )
                        .as_bytes(),
                    )
                    .expect("Couldn't write file");
                if subcmd_matches.contains_id("body") {
                    note_file
                        .write_all(
                            format!(
                                "{}\n",
                                subcmd_matches.get_one::<String>("body").expect("no body")
                            )
                            .as_bytes(),
                        )
                        .expect("Couldn't write file");
                }
            }
        }
        Some("edit") => {
            let subcmd_matches = app.subcommand().unwrap().1;
            let editor = std::env::var("EDITOR").expect("Set $EDITOR to a valid value");
            println!("{}", editor);
            let mut child = std::process::Command::new(editor)
                .arg(if subcmd_matches.contains_id("note") {
                    subcmd_matches
                        .get_one::<String>("note")
                        .expect("What note shall I edit")
                        .clone()
                } else {
                    find_note().expect("couldn't find note")
                })
                .spawn()
                .expect("Couln't execute $EDITOR");
            child.wait().expect("Failed to wait for editor");
        }
        Some("delete") => {
            let subcmd_matches = app.subcommand().unwrap().1;
            if subcmd_matches.contains_id("note") {
                rm_note(
                    subcmd_matches
                        .get_one::<String>("note")
                        .expect("What note shall I delete?")
                        .to_string(),
                )
                .expect("Couldn't delete file");
            }
        }
        _ => {
            println!("Unrecognized command")
        }
    }
}

fn new_note(time: DateTime<Local>) -> Result<fs::File, std::io::Error> {
    let pathfmt = format!("{}", time.format("notes/%Y/%m/%d/%H.%M.md"));
    let path = path::Path::new(&pathfmt);
    let prefix = path.parent().unwrap();
    fs::create_dir_all(prefix).unwrap();
    let result = if path.exists() {
        Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Note exists, perhaps try editing existing note?",
        ))
    } else {
        Ok(fs::File::create(&path)?)
    };
    result
}

fn rm_note(loc: String) -> Result<(), std::io::Error> {
    fs::remove_file(loc)?;
    Ok(())
}

fn find_note() -> Result<String, std::io::Error> {
    let mut selection: String = "notes".into();
    for word in ["year", "month", "day"] {
        selection = format!("{selection}/{}", find_directory(&*selection, word)?);
    }
    selection = format!("{selection}/{}", find_file(&*selection)?);
    Ok(selection)
}

fn find_directory(path: &str, name: &str) -> Result<String, std::io::Error> {
    // Get year
    print!("\x1b[2J");
    println!("Select {name} note was written in");
    let mut paths = fs::read_dir(path)?;
    // let a = paths.clone();
    for (index, path) in paths.by_ref().enumerate() {
        println!("({})  {}", index, path?.file_name().to_str().unwrap());
    }
    print!("In what {name} was the note created? Enter a number: ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let selection = format!(
        "{}",
        fs::read_dir(path)?
            .nth(
                input
                    .trim()
                    .parse::<usize>()
                    .expect("Invalid input; enter a number in the list"),
            )
            .unwrap_or_else(|| {
                panic!("path.nth({input}) == None");
            })
            .expect("Invalid input; enter a number in the list")
            .file_name()
            .to_str()
            .unwrap()
    );
    Ok(selection)
}

fn find_file(path: &str) -> Result<String, std::io::Error> {
    // TODO: Use title instead of filename
    print!("\x1b[2J");
    println!("Select note");
    let mut paths = fs::read_dir(path)?;
    // let a = paths.clone();
    for (index, path) in paths.by_ref().enumerate() {
        println!("({})  {}", index, path?.file_name().to_str().unwrap());
    }
    print!("What note should I edit? Enter a number: ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let selection = format!(
        "{}",
        fs::read_dir(path)?
            .nth(
                input
                    .trim()
                    .parse::<usize>()
                    .expect("Invalid input; enter a number in the list"),
            )
            .unwrap_or_else(|| {
                panic!("path.nth({input}) == None");
            })
            .expect("Invalid input; enter a number in the list")
            .file_name()
            .to_str()
            .unwrap()
    );
    Ok(selection)
}
