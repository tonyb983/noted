// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use noted::types::{api::Ordering, Note};

fn main() {
    // check_ordering();
    date_ordering();
}

fn create_notes() -> Vec<Note> {
    vec![
            Note::create((
                "A Title",
                "The content for the note goes here.",
                vec!["tag1", "tag2"],
            )),
            Note::create((
                "Some Title",
                "Here is content for the note the for content is Here.",
                vec!["1tag", "2tag"],
            )),
            Note::create((
                "This is Title",
                "Whoa, how about this note content.",
                vec!["tag1", "tag2"],
            )),
            Note::create((
                "Gooooo Title",
                "My my, how amazing this note is!",
                vec!["whoa", "dude"],
            )),
            Note::create((
                "Last Title",
                "Once upon a time, there was a note. It was beautiful.",
                vec!["tag1", "tag2"],
            )),
            Note::create((
                "Title Goes Here",
                "This note has a title that does not end in the word Title, what a fkin rebel this guy is.",
                vec!["what", "a", "rebel"],
            )),
        ]
}

fn apply_order(notes: &[Note], order: Ordering) -> Vec<Note> {
    let mut notes = notes.to_vec();
    notes.sort_unstable_by(order.comparison());
    notes
}

fn print_titles(notes: &[Note], linebreak: bool) {
    let titles = notes.iter().map(Note::title).collect::<Vec<_>>();
    if linebreak {
        println!("{}", titles.join("\n"));
    } else {
        println!("{}", titles.join(" "));
    }
}

fn print_contents(notes: &[Note], linebreak: bool) {
    let titles = notes.iter().map(Note::content).collect::<Vec<_>>();
    if linebreak {
        println!("{}", titles.join("\n"));
    } else {
        println!("{}", titles.join(" "));
    }
}

fn print_concise(notes: &[Note]) {
    println!("--------------------------------------------------------------------------------");
    for (i, note) in notes.iter().enumerate() {
        println!("Note #{}", i + 1);
        println!("Title: {}", note.title());
        println!("Content: {}", note.content());
        println!("Tags: [{}]", note.tags().join(", "));
        println!("Created: {} \t Updated: {}", note.created(), note.updated());
        println!(
            "--------------------------------------------------------------------------------"
        );
    }
}

fn check_ordering() {
    use noted::types::api::OrderBy;

    let notes = create_notes();
    assert_eq!(notes.len(), 6, "create_notes should create 6 notes");

    let title_asc = Ordering::ascending(OrderBy::Title);
    let content_desc = Ordering::descending(OrderBy::Content);
    let ordered_ta = apply_order(&notes, title_asc);
    let ordered_cd = apply_order(&notes, content_desc);

    println!("Starting Titles:");
    print_titles(&notes, true);
    println!();

    println!("Ascending Titles:");
    print_titles(&ordered_ta, true);
    println!();

    println!("Starting Contents:");
    print_contents(&notes, true);
    println!();

    println!("Descending Contents:");
    print_contents(&ordered_cd, true);
    println!();
}

fn date_ordering() {
    use noted::types::api::OrderBy;
    let notes = create_notes();
    print_concise(&notes);

    let created_desc = Ordering::descending(OrderBy::Created);
    let ordered = apply_order(&notes, created_desc);
    print_concise(&ordered);
}
