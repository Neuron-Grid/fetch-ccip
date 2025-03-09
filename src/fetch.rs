use rand::{Rng, thread_rng};
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

/// RIRファイルを取得してテキストとして返す。リトライ+指数バックオフ付き
pub async fn fetch_with_retry(
    client: &Client,
    url: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let retry_attempts = 10;
    for i in 0..retry_attempts {
        match client.get(url).send().await {
            Ok(resp) => match resp.text().await {
                Ok(text) => return Ok(text),
                Err(e) => eprintln!("レスポンスのテキスト取得でエラー: {}", e),
            },
            Err(e) => eprintln!("HTTP取得エラー: {}", e),
        }

        // 指数バックオフ+ランダムスリープ
        let sleep_time = (2u64.pow(i) as f64) + thread_rng().gen_range(0.0..1.0);
        sleep(Duration::from_secs_f64(sleep_time)).await;
    }

    Err(format!(
        "{}回試みてもデータを取得できませんでした: {}",
        retry_attempts, url
    )
    .into())
}
