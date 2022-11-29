/*
 * Env variables:
 * DB_PASS -- postgresql db password for gw2_wallet database
 * GW2_API_TOKEN -- self explanatory
 */

use postgres::{Client, NoTls};
use std::env;



fn get_stats() -> Result<String, reqwest::Error> {
	let token = env::var("GW2_API_TOKEN").unwrap();

    let url = "https://api.guildwars2.com/v2/account/wallet";
    let cli = reqwest::blocking::Client::new();

    let curr = cli
        .get(url)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .unwrap()
        .text()
        .unwrap();

    Ok(curr)
}

fn store_to_db(curr: &String) {
	let db_pass = env::var("DB_PASS").unwrap();
	let connstring = format!("postgresql://kristjan:{}@192.168.1.113:5432/webscrape", db_pass);

    let mut cli = Client::connect(&connstring, NoTls).unwrap();
    cli.batch_execute("CREATE TABLE IF NOT EXISTS gw2_wallet (
        id  SERIAL PRIMARY KEY,
		time BIGINT,
        data TEXT
    )").unwrap();

	let now = std::time::SystemTime::now()
		.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
	cli.execute(
		"INSERT INTO gw2_wallet (time, data) VALUES ($1, $2)",
		&[&now, curr]
	).unwrap();
}


fn main() {

	println!("[*] Fetching data from gw2 API...");
    let curr = get_stats().unwrap();
    println!("{:?}", curr); 

	println!("[*] Storing to DB"); 
    store_to_db(&curr);
	println!("[*] Done");
}
