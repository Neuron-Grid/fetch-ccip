use clap::Parser;
use fetch_ccip::fetch::fetch_with_retry;
use fetch_ccip::process::process_country_code;
use futures::future::join_all;
use reqwest::Client;
use tokio::task::JoinHandle;

/// コマンドライン引数を処理するための構造体
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "This tool can be used to obtain IP addresses by country."
)]
struct Cli {
    #[arg(
        short = 'c',
        long = "country",
        required = true,
        num_args = 1..,
        help = "Specify the code in this argument.\nExample: jp br us"
    )]
    country_codes: Vec<String>,

    #[arg(
        short = 'm',
        long = "mode",
        default_value = "overwrite",
        required = true,
        hide_default_value = true,
        help = "Select file output mode: 'append' or 'overwrite'\ndefault: overwrite"
    )]
    mode: String,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // コマンドライン引数解析
    let args = Cli::parse();

    // 小文字入力を大文字へ変換
    let country_codes: Vec<String> = args
        .country_codes
        .iter()
        .map(|code| code.to_uppercase())
        .collect();

    // HTTPクライアント生成
    let client = Client::new();

    // RIRファイルをすべてダウンロード
    let rir_texts = download_all_rir_files(&client, &RIR_URLS).await?;

    // 各国コードごとに処理を並行実行
    let mut tasks: Vec<JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>> = vec![];
    for code in country_codes {
        let rir_texts_clone = rir_texts.clone();
        let mode_clone = args.mode.clone();
        let handle = tokio::spawn(async move {
            // 国コードごとにパース→ファイル出力
            if let Err(e) = process_country_code(&code, &rir_texts_clone, &mode_clone).await {
                eprintln!("エラー (国コード: {}): {}", code, e);
            }
            Ok(())
        });
        tasks.push(handle);
    }

    // 全タスク終了を待機
    for t in tasks {
        let _ = t.await?;
    }

    Ok(())
}

/// RIRファイルをすべてダウンロードしてメモリ上に文字列ベクタとして返す
async fn download_all_rir_files(
    client: &Client,
    urls: &[&str],
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut handles = Vec::new();

    for url in urls {
        let url_owned = url.to_string();
        let client_ref = client.clone();

        // fetch_with_retry関数をspawnして並行ダウンロードする
        handles.push(tokio::spawn(async move {
            fetch_with_retry(&client_ref, &url_owned).await
        }));
    }

    // 並行ダウンロード結果を収集
    let results = join_all(handles).await;
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

// ダウンロード対象のRIRファイル一覧
const RIR_URLS: &[&str] = &[
    "https://ftp.afrinic.net/pub/stats/afrinic/delegated-afrinic-extended-latest",
    "https://ftp.lacnic.net/pub/stats/lacnic/delegated-lacnic-extended-latest",
    "https://ftp.ripe.net/pub/stats/ripencc/delegated-ripencc-extended-latest",
    "https://ftp.apnic.net/pub/stats/apnic/delegated-apnic-extended-latest",
    "https://ftp.arin.net/pub/stats/arin/delegated-arin-extended-latest",
];
