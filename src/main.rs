//
//Copyright © 2020-22 Maestro Creativescape
//
// SPDX-License-Identifier: GPL-3.0
//
// Simple Mail Deleter Script
//

use imap;
use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    fetch_inbox_top().unwrap();
}

fn fetch_inbox_top() -> imap::error::Result<Option<String>> {
    let domain = std::env::var("IMAP_DOMAIN").unwrap_or("none".to_string());
    let username = std::env::var("IMAP_USERNAME").unwrap_or("none".to_string());
    let password = std::env::var("IMAP_PASSWORD").unwrap_or("none".to_string());
    let client = imap::ClientBuilder::new(domain, 993).native_tls()?;
    let mut imap_session = client.login(username, password).map_err(|e| e.0)?;
    let inbox = imap_session.select("Inbox")?;
    let subject = Regex::new(r"Subject: (.*)").unwrap();
    let file = File::open("patterns.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let re = Regex::new(format!("({})+", line?).as_str()).unwrap();
        println!("Working on Pattern: {}", re.as_str());
    for i in 1 as u32..inbox.exists {
        let messages = imap_session.fetch((inbox.exists - i).to_string(), "RFC822")?;
        let message = if let Some(m) = messages.iter().next() {
            m
        } else {
            return Ok(None);
        };
        let body = message.body().unwrap_or("NULL".as_bytes());
        let body = std::str::from_utf8(body).unwrap_or("NULL").to_string();
        if re.is_match(&body) {
            imap_session
                .store(format!("{}", message.message), "+FLAGS (\\Deleted)")
                .unwrap();
            let subject_re = subject.captures(&body).unwrap();
            println!("Deleted Mail with Subject: {}", &subject_re[1]);
        }
    }
    }
    imap_session.expunge().unwrap();
    imap_session.logout()?;
    println!("\n\n\n********PURGED ALL REQUESTED MAIL********");
    Ok(Some("logout".to_string()))
}
