#[allow(unused_variables)]
pub fn summarize(key:&str, info: String) -> String{
    println!("Summarizig...");
    // String::from(info.get(0..4).unwrap_or(""))
    println!("api-key: {}", key);
    String::from("A test commit message from rustdiff")
}