//!
//! Rust + Gemini API
//!

mod gemini_api;
mod types;
mod util;

/// 環境変数を取得します。
#[allow(unused)]
fn getenv(name: &str) -> String {
	return std::env::var(name).unwrap_or_default();
}

/// Rust アプリケーションのエントリーポイント
#[tokio::main]
async fn main() {
	let result = gemini_api::ask_gemini("Explain how AI works？ in Japanese please.");
	if result.is_err() {
		println!("Error: {:?}", result.err());
		return;
	}
}
