use imap;
use native_tls;
use regex::Regex;

fn main() {
    // To connect to the gmail IMAP server with this you will need to allow unsecure apps access.
    // See: https://support.google.com/accounts/answer/6010255?hl=en
    fetch_inbox_top().unwrap();
}

fn fetch_inbox_top() -> imap::error::Result<Option<String>> {
    let domain = "imap.mail.com";
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect((domain, 993), domain, &tls).unwrap();
    let mut imap_session = client
        .login("mail username", "mail pwd")
        .map_err(|e| e.0)?;
    let inbox = imap_session.select("Spam")?;
    for i in 1 as u32..inbox.exists {

    let messages = imap_session.fetch( (inbox.exists-i).to_string(), "RFC822")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(None);
    };
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();
    let re = Regex::new(r"(insert string in message body, even 1 matching would do)+").unwrap();
    if re.is_match(&body) {
        imap_session.store(format!("{}", message.message), "+FLAGS (\\Deleted)").unwrap();
        println!("DELETED smth!");
    }
    else {
        println!("Doesnt match");
    }
    }
    imap_session.expunge().unwrap();
    imap_session.logout()?;
    Ok(Some("logout".to_string()))
}