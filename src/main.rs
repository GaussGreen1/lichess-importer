use reqwest;
use std::io;

#[tokio::main]
async fn main()  -> Result<(), reqwest::Error> {
	let mut username = String::new();
	println!("Please enter the username of the player whose games you wish to import:");
	
	io::stdin()
       .read_line(&mut username)
	   .expect("Failed to read line");
	
	let date_endpoint = str::replace("https://api.chess.com/pub/player/username", "username", &username);
    println!("{}", date_endpoint);
	
	let res = reqwest::get(&date_endpoint).await?;
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    Ok(())
}