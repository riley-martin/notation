use chrono::prelude::*;
use clap::{Arg, Command, SubCommand};
use std::io::prelude::*;
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
    let local_time: DateTime<Local> = Local::now();
    if app.subcommand_name() == Some("new") {
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
    return result;
}
