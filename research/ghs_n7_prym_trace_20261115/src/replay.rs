use std::collections::BTreeSet;
use std::fs;

fn mul(a: u16, b: u16) -> u16 {
    let mut product = 0_u16;
    for bit in 0..7 {
        if b & (1 << bit) != 0 { product ^= a << bit; }
    }
    for bit in (7..=12).rev() {
        if product & (1 << bit) != 0 { product ^= 0x83 << (bit - 7); }
    }
    assert!(product < 128);
    product
}

fn order(a: u16, b: u16) -> u16 {
    let mut n = 1; // infinity
    for x in 0..128 {
        let x2 = mul(x, x);
        let rhs = mul(x2, x) ^ mul(a, x2) ^ b;
        for y in 0..128 {
            if mul(y, y) ^ mul(x, y) == rhs { n += 1; }
        }
    }
    n
}

fn main() {
    let csv = fs::read_to_string("research/ghs_n7_prym_trace_20261115/reports/run-20261115T0821Z/curves.csv").unwrap();
    let mut lines = csv.lines();
    assert_eq!(lines.next(), Some("b,a,order,t,classical,place7,exceptional_subfield_trace"));
    let mut seen = BTreeSet::new();
    for line in lines {
        let f: Vec<&str> = line.split(',').collect();
        assert_eq!(f.len(), 7);
        let b: u16 = f[0].parse().unwrap();
        let a: u16 = f[1].parse().unwrap();
        let expected_n: u16 = f[2].parse().unwrap();
        let expected_t: i16 = f[3].parse().unwrap();
        let expected_place7: i16 = f[5].parse().unwrap();
        assert!(seen.insert((a, b)));
        let n = order(a, b);
        assert_eq!((a, b, n), (a, b, expected_n));
        let t = 129 - n as i16;
        assert_eq!(t, expected_t);
        assert_eq!(18 - t, expected_place7);
    }
    assert_eq!(seen.len(), 112);
    println!("direct_equation_replay_rows={} all_orders_match=true", seen.len());
}
