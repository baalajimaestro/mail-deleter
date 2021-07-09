//
//Copyright © 2020 Maestro Creativescape
//
// SPDX-License-Identifier: GPL-3.0
//
// Simple Mail Deleter Script
//

use imap;
use regex::Regex;

fn main() {
    // To connect to the gmail IMAP server with this you will need to allow unsecure apps access.
    // See: https://support.google.com/accounts/answer/6010255?hl=en
    fetch_inbox_top().unwrap();
}

fn fetch_inbox_top() -> imap::error::Result<Option<String>> {
    let domain = "imap.mail.com";
    let client = imap::ClientBuilder::new(domain, 993).native_tls()?;
    let mut imap_session = client
        .login("mail username", "mail pwd")
        .map_err(|e| e.0)?;
    let inbox = imap_session.select("Inbox")?;
    let re = Regex::new(r"(insert string in message body, even 1 matching would do)+").unwrap();
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
    else {
        println!("Mail Doesnt match");
    }
    }
    imap_session.expunge().unwrap();
    imap_session.logout()?;
    Ok(Some("logout".to_string()))
}
