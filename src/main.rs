//
//Copyright © 2020-22 Maestro Creativescape
//
// SPDX-License-Identifier: GPL-3.0
//
// Simple Mail Deleter Script
//

use imap;
use regex::Regex;

fn main() {
    fetch_inbox_top().unwrap();
}

fn fetch_inbox_top() -> imap::error::Result<Option<String>> {
    let domain = std::env::var("IMAP_DOMAIN").unwrap_or("none".to_string());
    let username = std::env::var("IMAP_USERNAME").unwrap_or("none".to_string());
    let password = std::env::var("IMAP_PASSWORD").unwrap_or("none".to_string());
    let pattern =  std::env::var("DELETE_PATTERN").unwrap_or("none".to_string());
    let client = imap::ClientBuilder::new(domain, 993).native_tls()?;
    let mut imap_session = client
        .login(username, password)
        .map_err(|e| e.0)?;
    let inbox = imap_session.select("Inbox")?;
    let re = Regex::new(format!("({})+", &pattern).as_str()).unwrap();
    for i in 1 as u32..inbox.exists {

    let messages = imap_session.fetch( (inbox.exists-i).to_string(), "RFC822")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(None);
    };
    let body = message.body().unwrap_or("NULL".as_bytes());
    let body = std::str::from_utf8(body)
        .unwrap_or("NULL")
        .to_string();
    if re.is_match(&body) {
        imap_session.store(format!("{}", message.message), "+FLAGS (\\Deleted)").unwrap();
        println!("DELETED SPAM Target!");
    }
    }
    imap_session.expunge().unwrap();
    imap_session.logout()?;
    Ok(Some("logout".to_string()))
}
