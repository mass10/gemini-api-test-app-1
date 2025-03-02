use serde_json::json;
use std::collections::HashMap;

use crate::types;
use crate::util;

/// テスト その2
#[allow(unused)]
pub fn ask_gemini(question: &str) -> Result<(), Box<dyn std::error::Error>> {
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

	println!("{}", res);

	let gemini_response: types::GeminiApiResponse = serde_json::from_str(&res)?;

	if gemini_response.error.is_some() {
		let error = gemini_response.error.unwrap();
		println!("Error: {:?}", error);
		return Ok(());
	}

	println!("Gemini✨:");
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
