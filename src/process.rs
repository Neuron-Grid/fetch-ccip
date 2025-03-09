use crate::output::sort_and_write;
use crate::parse::parse_ip_lines;
use std::collections::BTreeSet;

/// 国コードと、すでにダウンロード済みのRIRファイル文字列を受け取り、
/// パースしてファイル書き込みまで行う。
pub async fn process_country_code(
    country_code: &str,
    rir_texts: &[String],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // BTreeSetで重複排除+ソート(内部の順序付け)を行う
    let mut ipv4_list = BTreeSet::new();
    let mut ipv6_list = BTreeSet::new();

    // 1回ダウンロードしたテキストを順次パースし、該当する国コードのIPをセットに追加
    for text in rir_texts {
        let (v4, v6) = match parse_ip_lines(text, country_code) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!("パースでエラー: {}", e);
                continue;
            }
        };
        ipv4_list.extend(v4);
        ipv6_list.extend(v6);
    }

    // BTreeSetにより既にソートされているので、ここでは二度目のソートは不要
    sort_and_write(country_code, &ipv4_list, &ipv6_list)?;

    Ok(())
}
