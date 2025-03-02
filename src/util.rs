/// 環境変数を取得します。
pub fn getenv(name: &str) -> String {
	return std::env::var(name).unwrap_or_default();
}
