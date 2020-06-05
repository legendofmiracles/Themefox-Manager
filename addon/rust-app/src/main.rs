mod lib;
use std::io;
use std::process::Command;
fn main() {
    let json_val = match lib::read_input(io::stdin()) {
        Err(why) => panic!("{}", why.to_string()),
        Ok(json_val) => json_val,
    };
    if json_val == "ping" {
        // your code here
        let response = serde_json::json!({ "msg": "pong" });
        match lib::write_output(io::stdout(), &response) {
            Err(why) => panic!("{}", why.to_string()),
            Ok(_) => (),
        };
    } else if json_val.as_str().unwrap().starts_with("DO") {
        let string = json_val.as_str().unwrap();
        let string: Vec<&str> = string.split_ascii_whitespace().collect();
        let output = Command::new("themefox-manager")
            .arg(format!("{}", string[1]))
            .output()
            .expect("Failed to execute command");
        let stdout = std::str::from_utf8(output.stdout.as_slice()).unwrap();
        let stderr = std::str::from_utf8(output.stderr.as_slice()).unwrap();
        let response =
            serde_json::json!({ "output": stdout, "error": stderr, "status": output.status.code()});
        match lib::write_output(io::stdout(), &response) {
            Err(why) => panic!("{}", why.to_string()),
            Ok(_) => (),
        };
    } else if json_val.as_str().unwrap().starts_with("git") {
        /*
        let response =
            serde_json::json!({ "output": "LOLSU", "error": "", "status": "TEST"});
        match lib::write_output(io::stdout(), &response) {
            Err(why) => panic!("{}", why.to_string()),
            Ok(_) => (),
        };
        */
        
        let string = json_val.as_str().unwrap();
        let string: Vec<&str> = string.split_ascii_whitespace().collect();
        let output = Command::new("themefox-manager")
            .arg("-g")
            .arg(format!("{}", string[1]))
            .output()
            .expect("Failed to execute command");
        let stdout = std::str::from_utf8(output.stdout.as_slice()).unwrap();
        let stderr = std::str::from_utf8(output.stderr.as_slice()).unwrap();
        let response =
            serde_json::json!({ "output": stdout, "error": stderr, "status": output.status.code()});
        match lib::write_output(io::stdout(), &response) {
            Err(why) => panic!("{}", why.to_string()),
            Ok(_) => (),
        };
        
    } else {
        match lib::write_output(
            io::stdout(),
            &serde_json::json!({"output": "umm, no known signal"}),
        ) {
            Err(why) => panic!("{}", why.to_string()),
            Ok(_) => (),
        };
    }
}
