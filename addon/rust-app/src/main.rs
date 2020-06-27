mod lib;
use std::io;
use std::process::Command;
fn main() {
    #[derive(Debug, Clone)]
    struct Output<'a> {
        field: &'a std::result::Result<std::process::Output, std::io::Error>,
    };
    let json_val = match lib::read_input(io::stdin()) {
        Err(why) => panic!("{{\"error\": \"{}\"}}", why.to_string()),
        Ok(json_val) => json_val,
    };
    if json_val["message"] == "ping" {
        // your code here
        let response = "{{\"message\": \"pong\" }}";
        std::fs::write("/home/legendofmiracles/foo.text", response);
        match lib::write_output(io::stdout(), response.to_string()) {
            Err(why) => panic!("{{\"error\": \"{}\"}}", why.to_string()),
            Ok(_) => (),
        };
    } else if json_val.as_str().unwrap().starts_with("DO") {
        let string = json_val.as_str().unwrap();
        let string: Vec<&str> = string.split_ascii_whitespace().collect();

        let output = Output {
            field: &Command::new("themefox-manager")
                .arg(format!("{}", string[1]))
                .output(),
        };

        if output.field.is_err() {
            let response = format!("{{\"error\": {:?}}}", output.field.as_ref().err());
            match lib::write_output(io::stdout(), response) {
                Err(why) => panic!("{{\"error\": \"{}\"}}", why.to_string()),
                Ok(_) => (),
            };
        }
        let test = output.field.as_ref().unwrap();
        let stdout = std::str::from_utf8(test.stdout.as_slice()).unwrap();
        let stderr = std::str::from_utf8(test.stderr.as_slice()).unwrap();
        let response = format!(
            "{{ \"output\": {}, \"error\": {}, \"status\": {:?}}}",
            stdout,
            stderr,
            output.field.as_ref().unwrap().status.code()
        );
        match lib::write_output(io::stdout(), response) {
            Err(why) => panic!("{{\"error\": \"{}\"}}", why.to_string()),
            Ok(_) => (),
        };
    } else if json_val.as_str().unwrap().starts_with("git") {
        let string = json_val.as_str().unwrap();
        let string: Vec<&str> = string.split_ascii_whitespace().collect();
        let output = Output {
            field: &Command::new("themefox-manager")
                .arg("-g")
                .arg(format!("{}", string[1]))
                .output(),
        };
        if output.field.as_ref().is_err() {
            let response = format!("{{\"error\": {:?}}}", output.field.as_ref().err());
            match lib::write_output(io::stdout(), response) {
                Err(why) => panic!("{{\"error\": \"{}\"}}", why.to_string()),
                Ok(_) => (),
            }
        }
        let test = &output.field.as_ref().unwrap().stdout;
        let test2 = &output.field.as_ref().unwrap().stderr;
        let stdout = std::str::from_utf8(test.as_slice()).unwrap();
        let stderr = std::str::from_utf8(test2.as_slice()).unwrap();
        let response = format!(
            "{{ \"output\": {}, \"error\": {}, \"status\": {:?}}}",
            stdout,
            stderr,
            output.field.as_ref().unwrap().status.code()
        );
        match lib::write_output(io::stdout(), response) {
            Err(why) => panic!("{{\"error\": \"{}\"}}", why.to_string()),
            Ok(_) => (),
        };
    } else {
        match lib::write_output(
            io::stdout(),
            "{{\"output\": \"umm, no known signal\"}})".to_string(),
        ) {
            Err(why) => panic!("{{\"error\": \"{}\"}}", why.to_string()),
            Ok(_) => (),
        };
    }
}
