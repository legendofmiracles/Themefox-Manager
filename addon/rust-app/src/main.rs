mod lib;
use std::io;
use std::process::Command;
fn main() {
    let json_val = match lib::read_input(io::stdin()) {
        Err(why) => panic!("\"error\": \"{}\"", why.to_string()),
        Ok(json_val) => json_val,
    };
    if json_val == "ping" {
        // your code here
        let response = "{{\"msg\": \"pong\" }}";
        match lib::write_output(io::stdout(), response.to_string()) {
            Err(why) => panic!("\"error\": \"{}\"", why.to_string()),
            Ok(_) => (),
        };
    } else if json_val.as_str().unwrap().starts_with("DO") {
        let string = json_val.as_str().unwrap();
        let string: Vec<&str> = string.split_ascii_whitespace().collect();
        let output = Command::new("themefox-manager")
            .arg(format!("{}", string[1]))
            .output();
        if output.is_err() {
            let response = format!("{{\"error\": {:?}}}", output.err());
            match lib::write_output(io::stdout(), response) {
                Err(why) => panic!("\"error\": \"{}\"", why.to_string()),
                Ok(_) => (),
            };
        }
        let stdout = std::str::from_utf8(&output.unwrap().stdout.as_slice()).unwrap();
        let stderr = std::str::from_utf8(&output.unwrap().stdout.as_slice()).unwrap();
        let response = format!(
            "{{ \"output\": {}, \"error\": {}, \"status\": {:?}}}",
            stdout,
            stderr,
            output.unwrap().status.code()
        );
        match lib::write_output(io::stdout(), response) {
            Err(why) => panic!("\"error\": \"{}\"", why.to_string()),
            Ok(_) => (),
        };
    } else if json_val.as_str().unwrap().starts_with("git") {
        let string = json_val.as_str().unwrap();
        let string: Vec<&str> = string.split_ascii_whitespace().collect();
        let output = Command::new("themefox-manager")
            .arg("-g")
            .arg(format!("{}", string[1]))
            .output();
        if output.is_err() {
            let response = format!("{{\"error\": {:?}}}", output.err());
            match lib::write_output(io::stdout(), response) {
                Err(why) => panic!("\"error\": \"{}\"", why.to_string()),
                Ok(_) => (),
            }
        }
        let stdout = std::str::from_utf8(output.unwrap().stdout.as_slice()).unwrap();
        let stderr = std::str::from_utf8(output.unwrap().stderr.as_slice()).unwrap();
        let response = format!(
            "{{ \"output\": {}, \"error\": {}, \"status\": {:?}}}",
            stdout,
            stderr,
            output.unwrap().status.code()
        );
        match lib::write_output(io::stdout(), response) {
            Err(why) => panic!("\"error\": \"{}\"", why.to_string()),
            Ok(_) => (),
        };
    } else {
        match lib::write_output(
            io::stdout(),
            "{{\"output\": \"umm, no known signal\"}})".to_string(),
        ) {
            Err(why) => panic!("\"error\": \"{}\"", why.to_string()),
            Ok(_) => (),
        };
    }
}
