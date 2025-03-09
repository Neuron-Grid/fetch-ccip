use chrono::{Datelike, Local, Timelike};
use ipnet::IpNet;
use std::collections::BTreeSet;
use std::fs;

/// BTreeSetにより既にソート済みなので、ここでは再ソートしない。
/// そのままファイルへ書き出す。
pub fn sort_and_write(
    country_code: &str,
    ipv4_list: &BTreeSet<IpNet>,
    ipv6_list: &BTreeSet<IpNet>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    write_file(&format!("IPv4_{}.txt", country_code), ipv4_list)?;
    write_file(&format!("IPv6_{}.txt", country_code), ipv6_list)?;
    Ok(())
}

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

    // BTreeSetの順序をそのまま使用し、再度のソートをしない
    let lines: Vec<String> = nets.iter().map(|net| net.to_string()).collect();

    let content = format!("{}{}", formatted_header, lines.join("\n"));
    fs::write(path, content)?;
    println!("ファイルに書き込みました: {}", path);
    Ok(())
}
