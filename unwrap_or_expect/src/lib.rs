pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_string(),
        Security::Message => server.expect("ERROR: program stops").to_string(),
        Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),
        Security::NotFound => server.map_or_else(
            |err| format!("Not found: {}", err),
            |url| url.to_string(),
        ),
        Security::UnexpectedUrl => server.map_or_else(
            |err| err.to_string(),
            |url| panic!("{}", url),
        ),
    }
}
/*
unwrap_or_expect
Instructions
Create a function named fetch_data with two arguments:

server: A Result<String, String>, with either a server URL or an error message inside, respectively.
security_level: An enum instance representing the desired behavior of the function in case of errors.
The security_level enum should be defined as follows:

Unknown: Returns the server URL or panics.
Message: Returns the server URL or panics with the error message ERROR: program stops.
Warning: Returns the server URL or the message WARNING: check the server.
NotFound: Returns the server URL or the message Not found: [MESSAGE], where [MESSAGE] represents the server's error message.
UnexpectedUrl: Returns the error message or panics with the error message being the server URL.
Expected Functions
pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    todo!()
}
Usage
Here is a program to test your function:

use unwrap_or_expect::*;

fn main() {
    println!("{}", fetch_data(Ok("server1.com"), Security::Warning));
    println!("{}", fetch_data(Err("server.com"), Security::Warning));
    println!("{}", fetch_data(Err("server2.com"), Security::NotFound));

    // Panics with no custom message
    // fetch_data(Err("ERROR CRITICAL"), Security::Unknown);

    // Panics with the message "ERROR: program stops"
    // fetch_data(Err("server.com"), Security::Message);

    // Panics with the message "malicious_server.com"
    // fetch_data(Ok("malicious_server.com"), Security::UnexpectedUrl);
}
And its output:

$ cargo run
server1.com
WARNING: check the server
Not found: server2.com
$
Notions
Error Handling
Unwrap keywords
*/