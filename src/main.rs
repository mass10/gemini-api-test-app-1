use serde_json::json;
use std::collections::HashMap;

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct Part {
	text: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct Content {
	pub parts: Vec<Part>,
	pub role: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct CitationSource {
    #[serde(rename = "startIndex")]
	start_index: i32,
    #[serde(rename = "endIndex")]
	end_index: i32,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct CitationMetadata {
    #[serde(rename = "citationSources")]
	citation_sources: Vec<CitationSource>,
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
	modality: String,
    #[serde(rename = "tokenCount")]
	token_count: i32,
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

#[allow(unused)]
fn analyze_gemini_response(response: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
	let model_version = &response["modelVersion"];
	println!("modelVersion: {}", model_version);

	let usage_metadata = &response["usageMetadata"];
	println!("usageMetadata: {}", usage_metadata);

	let candidates = &response["candidates"];
	if !candidates.is_array() {
		return Err("candidates is not an array".into());
	}

	for candidate in candidates.as_array().unwrap() {
		let content = &candidate["content"];
		let parts = &content["parts"];
		if !parts.is_array() {
			return Err("parts is not an array".into());
		}

		let role = content["role"].as_str().unwrap_or_default();
		println!("  role: [{}]", role);

		for part in parts.as_array().unwrap() {
			let text = part["text"].as_str().unwrap_or_default();
			println!("  text: {}", text);
		}
		let finish_reason = &candidate["finishReason"];
		let citation_metadata = &candidate["citationMetadata"];
		let citation_sources = &citation_metadata["citationSources"];
		if !citation_sources.is_array() {
			return Err("citation_sources is not an array".into());
		}
		for citation_source in citation_sources.as_array().unwrap() {
			println!("    source: {:?}", citation_source["source"]);
		}
	}

	return Ok(());
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

	// let mut headers = HashMap::new();
	// headers.insert("Content-Type", "application/json");
	// let headermap1: reqwest::header::HeaderMap = headers.into();

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

	let object_tree: serde_json::Value = serde_json::from_str(&res)?;

	// println!("{:?}", object_tree);

	// analyze_gemini_response(&object_tree)?;

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
