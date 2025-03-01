use serde_json::json;
use std::collections::HashMap;

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct Part {
	pub text: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct Content {
	pub parts: Vec<Part>,
	pub role: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct CitationSource {
	#[serde(rename = "startIndex")]
	pub start_index: i32,
	#[serde(rename = "endIndex")]
	pub end_index: i32,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct CitationMetadata {
	#[serde(rename = "citationSources")]
	pub citation_sources: Vec<CitationSource>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct Candidate {
	pub content: Content,
	#[serde(rename = "finishReason")]
	pub finish_reason: String,
	#[serde(rename = "citationMetadata")]
	pub citation_metadata: CitationMetadata,
	#[serde(rename = "avgLogprobs")]
	pub avg_logprobs: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct TokenDetail {
	pub modality: String,
	#[serde(rename = "tokenCount")]
	pub token_count: i32,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct UsageMetadata {
	#[serde(rename = "promptTokenCount")]
	pub prompt_token_count: i32,
	#[serde(rename = "candidatesTokenCount")]
	pub candidates_token_count: i32,
	#[serde(rename = "totalTokenCount")]
	pub total_token_count: i32,
	#[serde(rename = "promptTokensDetails")]
	pub prompt_tokens_details: Vec<TokenDetail>,
	#[serde(rename = "candidatesTokensDetails")]
	pub candidates_tokens_details: Vec<TokenDetail>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct GeminiRequest {
	pub candidates: Vec<Candidate>,
	#[serde(rename = "usageMetadata")]
	pub usage_metadata: UsageMetadata,
	#[serde(rename = "modelVersion")]
	pub model_version: String,
}

/// テスト その1
#[allow(unused)]
fn test1() -> Result<(), Box<dyn std::error::Error>> {
	let client = reqwest::blocking::Client::new();
	let res = client.get("https://www.rust-lang.org").send()?.text()?;
	println!("{}", res);
	return Ok(());
}

/// 環境変数を取得します。
#[allow(unused)]
fn getenv(name: &str) -> String {
	return std::env::var(name).unwrap_or_default();
}

/// テスト その2
#[allow(unused)]
fn test2() -> Result<(), Box<dyn std::error::Error>> {
	let gemini_api_key = getenv("GEMINI_API_KEY");

	// curl "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key=GEMINI_API_KEY" \
	// -H 'Content-Type: application/json' \
	// -X POST \
	// -d '{
	//   "contents": [{
	//     "parts":[{"text": "Explain how AI works"}]
	//     }]
	//    }'

	let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent";

	let mut q = HashMap::<&str, String>::new();
	q.insert("key", gemini_api_key);

	let mut headers = reqwest::header::HeaderMap::new();
	headers.insert("Content-Type", "application/json".parse().unwrap());

	let body = json!({
		"contents": [{
			"parts":[{"text": "Explain how AI works"}]
		}]
	})
	.to_string();

	let client = reqwest::blocking::Client::new();
	let res = client
		.post(url)
		.query(&q)
		.headers(headers)
		// .json(&json!({
		.body(body)
		.send()?
		.text()?;

	println!("{}", res);

	let gemini_response: GeminiRequest = serde_json::from_str(&res)?;

	println!("{:?}", gemini_response);

	return Ok(());
}

/// Rust アプリケーションのエントリーポイント
#[tokio::main]
async fn main() {
	// let result = test1();
	// if result.is_err() {
	// 	println!("Error: {:?}", result.err());
	// 	return;
	// }

	let result = test2();
	if result.is_err() {
		println!("Error: {:?}", result.err());
		return;
	}
}
