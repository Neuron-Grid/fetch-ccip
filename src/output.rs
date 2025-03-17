use chrono::{Datelike, Local, Timelike};
use ipnet::IpNet;
use std::collections::BTreeSet;
use std::fs;

/// IPv4/IPv6リストをファイルに書き出す。
/// BTreeSetにより既にソート済みなので、ここでは再ソートしない。
pub fn sort_and_write(
    country_code: &str,
    ipv4_list: &BTreeSet<IpNet>,
    ipv6_list: &BTreeSet<IpNet>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ipv4_file = format!("IPv4_{}.txt", country_code);
    let ipv6_file = format!("IPv6_{}.txt", country_code);

    // 実際の書き込みを行うサブ関数呼び出し
    write_file(&ipv4_file, ipv4_list)?;
    write_file(&ipv6_file, ipv6_list)?;

    Ok(())
}

/// 実際にBTreeSetをテキストに変換し、ファイル書き込みする
fn write_file(
    path: &str,
    nets: &BTreeSet<IpNet>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let now = Local::now();
    let formatted_header = format!(
        "# {}年{}月{}日 {}時{}分\n",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute()
    );

    // BTreeSetの順序をそのまま利用
    let lines: Vec<String> = nets.iter().map(|net| net.to_string()).collect();
    let content = format!("{}{}", formatted_header, lines.join("\n"));

    fs::write(path, content)?;
    println!("ファイルに書き込みました: {}", path);

    Ok(())
}
