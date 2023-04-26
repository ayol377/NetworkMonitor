use std::fs;
use keyring::{Entry};

use reqwest::{Client, StatusCode};
use serde_json::json;

const API_KEY: &str = "AIzaSyDdYD48HQc89yrgxuvDB1agDroSFkCCd-c";


pub async fn signin(email: String, passwd: String) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}",
        API_KEY
    );
    let body = json!({
        "email" : email,
        "password": passwd,
        "returnSecureToken": true
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await?;

    let okstatus = StatusCode::from_u16(200).unwrap();
    let existcode = StatusCode::from_u16(400).unwrap();

    println!("{}", response.status());

    if response.status() == okstatus{
        let response = response.json::<serde_json::Value>().await?;
        let id_token =  response["idToken"].as_str().unwrap();
        println!("id token: {}", id_token);
        let cred = Entry::new("NetSecure", "User").unwrap();
        cred.set_password(&passwd).unwrap();

        let path = platform_dirs::AppDirs::new(Option::Some("NetSecure"), false).unwrap();
        let mut path = path.data_dir;
        path.push("settings.json");
        let data = fs::read_to_string(&path).unwrap();
        let data = data.as_str();
        let mut settings = json::parse(data).unwrap();
        settings["email"] = email.into();
        fs::write(path, json::stringify_pretty(settings, 1)).unwrap();
        return Ok("Ok".to_string())

    }else if response.status() == existcode{
        return Ok("Login Failed".to_string())
    }

    return Ok("Other Error".to_string())
}

pub async fn create_user (email: String, passwd: String) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signUp?key={}",
        API_KEY
    );
    let body = json!({
        "email" : email,
        "password": passwd,
        "returnSecureToken": true
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await?;

    let okstatus = StatusCode::from_u16(200).unwrap();
    let existcode = StatusCode::from_u16(400).unwrap();

    if response.status() == okstatus{
        let response = response.json::<serde_json::Value>().await?;
        let id_token =  response["idToken"].as_str().unwrap();
        println!("id token: {}", id_token);
        let cred = Entry::new("NetSecure", "User").unwrap();
        cred.set_password(&passwd).unwrap();
        verify_email(id_token.to_string()).await?;

        let path = platform_dirs::AppDirs::new(Option::Some("NetSecure"), false).unwrap();
        let mut path = path.data_dir;
        path.push("settings.json");
        let data = fs::read_to_string(&path).unwrap();
        let data = data.as_str();
        let mut settings = json::parse(data).unwrap();
        settings["email"] = email.into();
        fs::write(path, json::stringify_pretty(settings, 1)).unwrap();
        return Ok("Ok".to_string())

    }else if response.status() == existcode{
        println!("Email Exists");
        return Ok("Email Exists".to_string())
    }

    return Ok("Other Error".to_string())

}

pub async fn verify_email(token: String) -> Result<(), reqwest::Error>{
    let client = Client::new();
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:sendOobCode?key={}",
        API_KEY
    );
    let body = json!({
        "requestType" : "VERIFY_EMAIL",
        "idToken": token.as_str(),
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await?;

    let okstatus = StatusCode::from_u16(200).unwrap();
    let existcode = StatusCode::from_u16(400).unwrap();
    
    if response.status() == okstatus{
        let response = response.json::<serde_json::Value>().await?;
        let id_token =  response["email"].as_str().unwrap();
        println!("id token: {}", id_token);
    }else if response.status() == existcode{
        let response = response.json::<serde_json::Value>().await?;
        println!("{}", response);
    }

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn logout(){
    let cred = Entry::new("NetSecure", "User").unwrap();
    cred.delete_password().unwrap();

    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure"), false).unwrap();
    let mut path = path.data_dir;
    path.push("settings.json");
    let data = fs::read_to_string(&path).unwrap();
    let data = data.as_str();
    let mut settings = json::parse(data).unwrap();
    settings["email"] = "".into();
    fs::write(path, json::stringify_pretty(settings, 1)).unwrap();
}