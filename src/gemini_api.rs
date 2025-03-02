use serde_json::json;
use std::collections::HashMap;

use crate::types;
use crate::util;

/// テスト: Gemini API に質問を投げる
#[allow(unused)]
pub fn ask_gemini(question: &str) -> Result<(), Box<dyn std::error::Error>> {
	println!("============================================");
	println!("▼QUESTION:");
	let gemini_api_key = util::getenv("GEMINI_API_KEY");

	let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent";

	let mut query = HashMap::<&str, String>::new();
	query.insert("key", gemini_api_key);

	let mut headers = reqwest::header::HeaderMap::new();
	headers.insert("Content-Type", "application/json".parse().unwrap());

	let body = json!({
		"contents": [{
			"parts":[{"text": question}]
		}]
	})
	.to_string();

	let client = reqwest::blocking::Client::new();
	let res = client.post(url).query(&query).headers(headers).body(body).send()?.text()?;

	println!("============================================");
	println!("▼RESPONSE:");
	println!("{}", res);

	println!("============================================");
	println!("▼Gemini✨:");
	let gemini_response: types::GeminiApiResponse = serde_json::from_str(&res)?;

	if gemini_response.error.is_some() {
		let error = gemini_response.error.unwrap();
		println!("Error: {:?}", error);
		return Ok(());
	}

	if gemini_response.candidates.is_some() {
		let candidates = gemini_response.candidates.as_ref().unwrap();
		for e in candidates {
			let content = &e.content;
			for content in &content.parts {
				println!("> {}", content.text);
			}
		}
	}
	// println!("{:?}", gemini_response);

	return Ok(());
}

/// テスト: キャッシュされたコンテンツを利用する(WIP)
#[allow(unused)]
pub fn ask_gemini_cached_stream(question: &str) -> Result<(), Box<dyn std::error::Error>> {
	println!("============================================");
	println!("▼QUESTION:");
	let gemini_api_key = util::getenv("GEMINI_API_KEY");

	let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:streamGenerateContent";

	let mut query = HashMap::<&str, String>::new();
	query.insert("key", gemini_api_key);

	let mut headers = reqwest::header::HeaderMap::new();
	headers.insert("Content-Type", "application/json".parse().unwrap());

	// TODO: キャッシュ利用の手続きが必要
	let body = json!({
		"contents": [{
			"parts":[{"text": question}]
		}]
	})
	.to_string();

	let client = reqwest::blocking::Client::new();
	let res = client.post(url).query(&query).headers(headers).body(body).send()?.text()?;

	println!("============================================");
	println!("▼RESPONSE:");
	println!("{}", res);

	println!("============================================");
	println!("▼Gemini✨:");

	// TODO: レスポンスの型が少し違うので、struct は分割したほうがよい
	let result  = serde_json::from_str::<types::GeminiApiErrorResponse>(&res);
	if result.is_ok() {
		let error = result.unwrap();
		println!("Error: {:?}", error);
		return Ok(());
	}

	// println!("{:?}", gemini_response);

	let gemini_response  = serde_json::from_str::<Vec<types::GeminiApiResponse>>(&res)?;
	for e in &gemini_response {
		if e.candidates.is_some() {
			let candidates = e.candidates.as_ref().unwrap();
			for e in candidates {
				let content = &e.content;
				for content in &content.parts {
					println!("> {}", content.text);
				}
			}
		}
	}

	return Ok(());
}
