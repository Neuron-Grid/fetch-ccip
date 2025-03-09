mod fetch;
mod output;
mod parse;
mod parse_ipv4;
mod process;

use fetch::fetch_with_retry;
use futures::future::join_all;
use process::process_country_code;
use reqwest::Client;
use tokio::task::JoinHandle;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let rir_urls = vec![
        "https://ftp.afrinic.net/pub/stats/afrinic/delegated-afrinic-extended-latest",
        "https://ftp.lacnic.net/pub/stats/lacnic/delegated-lacnic-extended-latest",
        "https://ftp.ripe.net/pub/stats/ripencc/delegated-ripencc-extended-latest",
        "https://ftp.apnic.net/pub/stats/apnic/delegated-apnic-extended-latest",
        "https://ftp.arin.net/pub/stats/arin/delegated-arin-extended-latest",
    ];

    // 複数国コードが指定されてもRIRファイルは1回ずつしかダウンロードしない
    let country_codes = vec!["JP", "US", "BR"];

    let client = Client::new();

    // RIRファイルをまとめて一度だけダウンロード
    let rir_texts = download_all_rir_files(&client, &rir_urls).await?;

    // 各国コードごとに処理を並行実行する
    let mut tasks: Vec<JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>> = vec![];
    for code in country_codes {
        let code_owned = code.to_string();
        let rir_texts_clone = rir_texts.clone(); // 参照カウントのあるStringなら複製可能

        let handle = tokio::spawn(async move {
            // RIRのテキストを国コード別にパースしてファイル出力まで行う
            if let Err(e) = process_country_code(&code_owned, &rir_texts_clone).await {
                eprintln!("エラー (国コード: {}): {}", code_owned, e);
            }
            Ok(())
        });

        tasks.push(handle);
    }

    for t in tasks {
        let _ = t.await?;
    }

    Ok(())
}

/// RIRファイルをすべてダウンロードしてメモリ上の文字列ベクタとして返す
async fn download_all_rir_files(
    client: &Client,
    urls: &[&str],
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut handles = vec![];

    for url in urls {
        let url_owned = url.to_string();
        let client_ref = client.clone();
        // fetch_with_retry関数をspawnして並行ダウンロード
        handles.push(tokio::spawn(async move {
            fetch_with_retry(&client_ref, &url_owned).await
        }));
    }

    let results = join_all(handles).await;

    // 全ダウンロード結果をまとめる
    let mut rir_texts = Vec::new();
    for res in results {
        match res {
            Ok(Ok(text)) => {
                rir_texts.push(text);
            }
            Ok(Err(e)) => {
                eprintln!("HTTP取得エラー: {}", e);
            }
            Err(e) => {
                eprintln!("タスク失敗: {}", e);
            }
        }
    }
    Ok(rir_texts)
}
