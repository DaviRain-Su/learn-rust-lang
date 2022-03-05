use std::io::{self, BufRead};

use feed_rs::parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        print!("{}  ... ", line);
        let xml = reqwest::blocking::get(&line)?.bytes()?;

        match parser::parse_with_uri(xml.as_ref(), Some(&line)) {
            Ok(feed) => {
                println!("entry len = {}", feed.entries.len());
                for entriey in feed.entries.iter() {
                    println!();
                    println!("mirror Link = {}", entriey.id);
                    println!("mirror Title = {}", entriey.title.as_ref().unwrap().content);
                    println!("mirror UpdateTime = {:?}", entriey.updated);
                    println!("mirror Content = {:?}", entriey.content.as_ref().unwrap().body);
                    // println!("{:?}", entriey);
                }
                // println!("feed = {:?}", feed);
            }
            Err(error) => println!("failed: {:?}\n{:?}\n-------------------------------------------------------------", error, xml),
        }
    }

    Ok(())
}