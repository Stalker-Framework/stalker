pub fn hex(value: &[u8]) -> String {
    let mut s = String::new();
    for b in value {
        s.push_str(&format!("{:02x}", b));
    }
    s
}
