pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => {
            match server {
                Ok(message) => message.to_string(),
                Err(msg) =>  panic!("called `Result::unwrap()` on an `Err` value: \"{}\"",msg)
            }
        }
        Security::Warning => {
            match server {
                Ok(message) => message.to_string(),
                Err(_) => "WARNING: check the server".to_string(),
            }
        }
        Security::NotFound => {
            match server {
                Ok(message) => message.to_string(),
                Err(message) => format!("Not found: {}", message),
            }
        }
        Security::Message => {
            match server {
                Ok(message) => message.to_string(),
                Err(_) => panic!("ERROR: program stops"),
            }
        }
        Security::UnexpectedUrl => {
            match server {
                Err(msg) => msg.to_string(),
                Ok(url) => panic!("{}", url),
            }
        }
    }
}
