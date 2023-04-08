pub fn summarize(info: String) -> String{
    println!("Summarizig...");
    println!("{:?}", info.get(0..4));
    String::from(info.get(0..4).unwrap_or(""))
}