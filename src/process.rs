use crate::output::sort_and_write;
use crate::parse::parse_ip_lines;
use ipnet::IpNet;
use std::collections::BTreeSet;

/// 国コードと、すでにダウンロード済みのRIRファイル文字列を受け取り、
/// パースしてファイル書き込みまで行う。
pub async fn process_country_code(
    country_code: &str,
    rir_texts: &[String],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // パース結果をまとめる（BTreeSetで重複排除+自動ソート）
    let (ipv4_set, ipv6_set) = parse_and_collect_ips(country_code, rir_texts)?;
    // 結果をファイルに書き出す
    sort_and_write(country_code, &ipv4_set, &ipv6_set)?;

    Ok(())
}

/// 指定国コードに合致するIPアドレスをすべて集約し、BTreeSetとして返す
fn parse_and_collect_ips(
    country_code: &str,
    rir_texts: &[String],
) -> Result<(BTreeSet<IpNet>, BTreeSet<IpNet>), Box<dyn std::error::Error + Send + Sync>> {
    let mut ipv4_list = BTreeSet::new();
    let mut ipv6_list = BTreeSet::new();

    for text in rir_texts {
        let (v4, v6) = match parse_ip_lines(text, country_code) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!("パースでエラー (国コード: {}): {}", country_code, e);
                continue;
            }
        };
        ipv4_list.extend(v4);
        ipv6_list.extend(v6);
    }

    Ok((ipv4_list, ipv6_list))
}
