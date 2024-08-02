use std::{collections::HashMap, iter::Peekable};
use lettre::transport::smtp::authentication::Credentials; 
use lettre::{Message, SmtpTransport, Transport}; 

use reqwest::Error;
use serde_json::{Value};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct PeggedAsset {
    id: String,
    name: String,
    symbol: String,
    gecko_id: String,
    pegType: String,
    priceSource: String,
    pegMechanism: String,
    circulating: Circular,
    circulatingPrevDay: Circular,
    circulatingPrevWeek: Circular,
    circulatingPrevMonth: Circular,
    chainCirculating:  HashMap<String,Value>,
    chains: Vec<Value>,
}

#[derive(Debug, Deserialize)]
struct Circular {
    peggedUSD: f64,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let response = reqwest::get("https://stablecoins.llama.fi/stablecoins")
        .await?
        .json::<Value>()
        .await?;

    // let pegged_assets: Vec<PeggedAsset> = serde_json::from_value(response).unwrap();
    // println!("{:?}", pegged_assets[0].name);
    let ps = response["peggedAssets"].as_array().unwrap();
    // println!("{:?}", ps.len());

    let mut Ua:Vec<Value>  =vec![] ;
    for  i in 0..ps.len() {
        if Ua.len() >=2 {
            break;
        }
        if ps[i]["symbol"] == "USDT".to_string() {
            Ua.push(ps[i].clone());
        }
        if ps[i]["symbol"] == "USDC".to_string() {
            Ua.push(ps[i].clone());
        }
    }


    let USDT = Ua[0]["circulating"]["peggedUSD"].as_f64().unwrap() ;
    let USDC = Ua[1]["circulating"]["peggedUSD"].as_f64().unwrap();


    let u1 =  Ua[0]["circulating"]["peggedUSD"].as_f64().unwrap();
    let u1d =  Ua[0]["circulatingPrevDay"]["peggedUSD"].as_f64().unwrap();
    let u1w =  Ua[0]["circulatingPrevWeek"]["peggedUSD"].as_f64().unwrap();
    let u1m =  Ua[0]["circulatingPrevMonth"]["peggedUSD"].as_f64().unwrap();
   
    let u2 =  Ua[1]["circulating"]["peggedUSD"].as_f64().unwrap();
    let u2d =  Ua[1]["circulatingPrevDay"]["peggedUSD"].as_f64().unwrap();
    let u2w =  Ua[1]["circulatingPrevWeek"]["peggedUSD"].as_f64().unwrap();
    let u2m =  Ua[1]["circulatingPrevMonth"]["peggedUSD"].as_f64().unwrap();

    let USD = u1 + u2 ;

    let ud = USD-u1d-u2d ;
    let uw = USD-u1w-u2w; 
    let um = USD-u1m-u2m ;
    println!("------------------------------------------------------------------");

    println!("{}          {}  {}",Ua[0]["symbol"],"circulating", Ua[0]["circulating"]["peggedUSD"].as_f64().unwrap());
    println!("{}   {}  {}",Ua[0]["symbol"],"circulatingPrevDay", Ua[0]["circulatingPrevDay"]["peggedUSD"].as_f64().unwrap());
    println!("{}  {}  {}",Ua[0]["symbol"],"circulatingPrevWeek", Ua[0]["circulatingPrevWeek"]["peggedUSD"].as_f64().unwrap());
    println!("{} {}  {}",Ua[0]["symbol"],"circulatingPrevMonth", Ua[0]["circulatingPrevMonth"]["peggedUSD"].as_f64().unwrap());
    println!("------------------------------------------------------------------");
   
    println!("{}          {}  {}",Ua[1]["symbol"],"circulating", Ua[1]["circulating"]["peggedUSD"].as_f64().unwrap());
    println!("{}   {}  {}",Ua[1]["symbol"],"circulatingPrevDay", Ua[1]["circulatingPrevDay"]["peggedUSD"].as_f64().unwrap());
    println!("{}  {}  {}",Ua[1]["symbol"],"circulatingPrevWeek", Ua[1]["circulatingPrevWeek"]["peggedUSD"].as_f64().unwrap());
    println!("{} {}  {}",Ua[1]["symbol"],"circulatingPrevMonth", Ua[1]["circulatingPrevMonth"]["peggedUSD"].as_f64().unwrap());

    println!("------------------------------------------------------------------");

    println!("USD :{:.4} 亿 一天增量 {:.4} 亿  一周增量 {:.4} 亿 一月增量 {:.4} 亿",USD/100000000.0, ud/100000000.0,uw/100000000.0,um/100000000.0);


    // let email = Message::builder() 
    // .from("Sender <sender@gmail.com>".parse().unwrap()) 
    // .to("Receiver <receiver@gmail.com>".parse().unwrap()) 
    // .subject("Sending email with Rust") 
    // .body(String::from("This is my first email")) 
    // .unwrap(); 
  
    // let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string()); 
    
    // // Open a remote connection to gmail 
    // let mailer = SmtpTransport::relay("smtp.gmail.com") 
    //     .unwrap() 
    //     .credentials(creds) 
    //     .build(); 
    
    // // Send the email 
    // match mailer.send(&email) { 
    //     Ok(_) => println!("Email sent successfully!"), 
    //     Err(e) => panic!("Could not send email: {:?}", e), 
    // }
    Ok(())
}