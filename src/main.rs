// use imap::Session;
// use native_tls::TlsConnector;
use std::net::TcpStream;

fn main() -> imap::error::Result<()> {
    let username = "testuser01@example.org";
    let password = "secret";
    let host = "localhost"; //could also be an IP address
    let port = 10143;
    // let domain = "localhost"; //if host is given by IP address, this should be the actual domain name (needed for example for TLS domain name verification)

    // Connect w/o TLS
    let stream = TcpStream::connect((host, port)).unwrap();
    let client = imap::Client::new(stream);

/*
    // Connect with TLS
    let tls = TlsConnector::builder().build().unwrap();
    let client = imap::connect((host, port), domain, &tls)?;
*/

    // Start IMAP session
    let mut imap_session = client
        .login(username, password)
        .map_err(|e| e.0)?;

    // Select INBOX
    imap_session.select("INBOX")?;


    let mut cnt = 1;
    loop {
        // Read mails in FIFO manner from INBOX
        while imap_session.select("INBOX")?.exists > 0 {
            let messages = imap_session.fetch("1", "BODY[]")?;
            for message in messages.iter() {
                if let Some(email) = message.body() {
                    println!(
                        "[{cnt}]--------------------------------------------------------------------------------------"
                    );
                    cnt = cnt + 1;
                    let email = String::from_utf8_lossy(email).to_string();
                    println!("{email}");
                    println!(
                        "--------------------------------------------------------------------------------------\n\n\n"
                    );
                }
            }

            // Delete + Expunge mail
            imap_session.store("1", "+FLAGS (\\Deleted)")?;
            imap_session.expunge()?;
        }

        // No more mails in INBOX
        // IDLE wait
        let idle_result = imap_session.idle()?.wait_keepalive(); //keep connection open (use default keep alive of 29 minutes)
        if idle_result.is_err() {
            eprintln!("[ERROR] IDLE: {}", idle_result.err().unwrap());
            break;
        }
    }

    imap_session.logout()?;
    Ok(())
}
