use std::collections::HashMap;

use reqwest::blocking::Client;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    message: HashMap<String, String>,
    finish_reason: String,
    index: u32,
}
#[derive(Serialize, Deserialize, Debug)]
struct GPTResp {
    id: String,
    object: String,
    created: u128,
    model: String,
    usage: HashMap<String, u32>,
    choices: Vec<Choice>,
}

#[allow(unused_variables)]
pub fn summarize(key: &str, info: String) -> Result<String, Box<dyn std::error::Error>> {
    println!("Summarizing...");
    let prompt = "You are a professional code reviewer who can make a breif summary for commit message from a `git diff` output,
    breifly state changes in bullet way to make a git commit message. Remember to ignore package importing and debug information.
    Heres my `git diff` output:
    ";
    let client = Client::new();
    let data = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{
            "role": "assistant",
            "content": format!("{} {}", prompt, &info)
        }],
        "temperature": 0.7
    });
    let req = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", key))
        .json(&data);
    let resp = req.send()?;
    let resp_json: GPTResp = resp.json()?;
    let content = resp_json.choices.get(0).expect("Network error").message["content"].clone();
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::{summarize, GPTResp};

    #[test]
    #[allow(unused_variables)]
    fn test_summarize() {
        let key = std::fs::read_to_string("./key.txt").expect("Unable to read file");
        let info = std::fs::read_to_string("./demo.txt").expect("Unable to read file");
        match summarize(&key, info) {
            Ok(msg) => println!("{}", msg),
            Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn text_ser() {
        let resp_text = std::fs::read_to_string("./resp.txt").expect("Unable to read file");
        // println!("{}", resp_text);
        let resp_json: Result<GPTResp, serde_json::Error> = serde_json::from_str(&resp_text);
        // println!("{:?}", resp_json);
        match resp_json {
            Ok(data) => println!(
                "{}",
                data.choices.get(0).expect("Network error").message["content"]
            ),
            _ => {}
        }
    }
}
