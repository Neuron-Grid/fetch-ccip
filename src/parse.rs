use crate::parse_ipv4::largest_ipv4_block;
use ipnet::{IpNet, Ipv4Net, Ipv6Net};
use std::net::Ipv4Addr;

/// テキスト全体を行ごとに分解し、指定された `country_code` に合致するIPv4/IPv6のリストを返す。
pub fn parse_ip_lines(
    text: &str,
    country_code: &str,
) -> Result<(Vec<IpNet>, Vec<IpNet>), Box<dyn std::error::Error + Send + Sync>> {
    let mut ipv4_list = Vec::new();
    let mut ipv6_list = Vec::new();
    for line in text.lines() {
        // コメント行やreserved行をスキップ
        if line.starts_with('#') || line.contains('*') || line.contains("reserved") {
            continue;
        }
        let params: Vec<&str> = line.split('|').collect();
        if params.len() < 5 {
            continue;
        }
        // 国コードフィルタ
        if params[1] != country_code {
            continue;
        }
        let ip_type = params[2];
        if ip_type == "ipv4" || ip_type == "ipv6" {
            match parse_ip_params(&params) {
                Ok(nets) => {
                    if ip_type == "ipv4" {
                        ipv4_list.extend(nets);
                    } else {
                        ipv6_list.extend(nets);
                    }
                }
                Err(e) => {
                    eprintln!("parse_ip_paramsでエラー: {}", e);
                }
            }
        }
    }
    Ok((ipv4_list, ipv6_list))
}

/// ip_typeを判別し、対応するパース関数を呼び出す
fn parse_ip_params(
    params: &[&str],
) -> Result<Vec<IpNet>, Box<dyn std::error::Error + Send + Sync>> {
    let ip_type = params[2];
    let start_str = params[3];
    let value_str = params[4];
    if ip_type == "ipv4" {
        parse_ipv4(start_str, value_str)
    } else if ip_type == "ipv6" {
        parse_ipv6(start_str, value_str)
    } else {
        Ok(vec![])
    }
}

/// IPv4をパースし、必要に応じてCIDRブロックに細分化する
fn parse_ipv4(
    start_str: &str,
    value_str: &str,
) -> Result<Vec<IpNet>, Box<dyn std::error::Error + Send + Sync>> {
    let start_v4 = start_str.parse::<Ipv4Addr>()?;
    let width = value_str.parse::<u64>()?;
    let start_num = u32::from(start_v4);

    let end_num = start_num
        .checked_add(width as u32)
        .ok_or("範囲が大きすぎます")?
        .checked_sub(1)
        .ok_or("計算エラー")?;

    let mut cidrs = Vec::new();
    let mut current = start_num;
    while current <= end_num {
        let max_size = largest_ipv4_block(current, end_num);
        let net = Ipv4Net::new(Ipv4Addr::from(current), max_size)?;
        cidrs.push(IpNet::V4(net));

        let block_size = 1u32 << (32 - max_size);
        current = current.saturating_add(block_size);
    }
    Ok(cidrs)
}

/// IPv6をパースする
fn parse_ipv6(
    start_str: &str,
    value_str: &str,
) -> Result<Vec<IpNet>, Box<dyn std::error::Error + Send + Sync>> {
    // RIRのフォーマット上、IPv6は「prefix/length」形式で丸ごとを扱う
    let cidr_str = format!("{}/{}", start_str, value_str);
    let net = cidr_str.parse::<Ipv6Net>()?;
    Ok(vec![IpNet::V6(net)])
}
