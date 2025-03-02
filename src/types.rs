#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct Part {
	pub text: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct Content {
	pub parts: Vec<Part>,
	pub role: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct CitationSource {
	#[serde(rename = "startIndex")]
	pub start_index: i32,
	#[serde(rename = "endIndex")]
	pub end_index: i32,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct CitationMetadata {
	#[serde(rename = "citationSources")]
	pub citation_sources: Vec<CitationSource>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct Candidate {
	pub content: Content,
	#[serde(rename = "finishReason")]
	pub finish_reason: Option<String>,
	#[serde(rename = "citationMetadata")]
	pub citation_metadata: Option<CitationMetadata>,
	#[serde(rename = "avgLogprobs")]
	pub avg_logprobs: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct TokenDetail {
	pub modality: String,
	#[serde(rename = "tokenCount")]
	pub token_count: i32,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct UsageMetadata {
	#[serde(rename = "promptTokenCount")]
	pub prompt_token_count: i32,
	#[serde(rename = "candidatesTokenCount")]
	pub candidates_token_count: Option<i32>,
	#[serde(rename = "totalTokenCount")]
	pub total_token_count: i32,
	#[serde(rename = "promptTokensDetails")]
	pub prompt_tokens_details: Option<Vec<TokenDetail>>,
	#[serde(rename = "candidatesTokensDetails")]
	pub candidates_tokens_details: Option<Vec<TokenDetail>>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct GeminiError {
	pub code: i32,
	pub message: String,
	pub status: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct GeminiApiResponse {
	pub candidates: Option<Vec<Candidate>>,
	#[serde(rename = "usageMetadata")]
	pub usage_metadata: Option<UsageMetadata>,
	#[serde(rename = "modelVersion")]
	pub model_version: Option<String>,
	pub error: Option<GeminiError>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct GeminiApiErrorResponse {
	pub error: GeminiError,
}
