/// largest_ipv4_block は、IPv4の範囲を部分的にサブネットへ分割する際、
/// どこまで大きなCIDRブロックが取れるかを求める関数
pub fn largest_ipv4_block(current: u32, end: u32) -> u8 {
    let tz = current.trailing_zeros();
    let span = (end - current + 1).ilog2_sub1();
    let max_block = tz.min(span);
    (32 - max_block) as u8
}

pub trait ILog2Sub1 {
    fn ilog2_sub1(&self) -> u32;
}

impl ILog2Sub1 for u32 {
    fn ilog2_sub1(&self) -> u32 {
        if *self == 0 {
            0
        } else {
            // 2のべき乗分だけ考慮するときに使うヘルパー
            31 - self.leading_zeros()
        }
    }
}
