#[allow(dead_code)]
mod previous {
    include!("../hec_weil13_20261115/main.rs");
    include!("../hec_theta13_20261115/gate.rs");
    include!("../hec_kummer13_20261115/coordinates.rs");
    include!("../hec_image13_20261115/relations.rs");
    include!("../hec_geometry13_20261115/jets.rs");
    include!("../hec_sourcejet13_20261115/source.rs");
    include!("map.rs");
}
#[allow(dead_code)]
mod target {
    include!("../hec_transport13_20261115/reused.rs");
    fn splitmix64(mut x: u64) -> u64 {
        x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = x;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn is_prime_u64(p: u64) -> bool {
        p >= 2 && !(2..).take_while(|d| d * d <= p).any(|d| p % d == 0)
    }
    pub fn scalar(j: &QuarticJacobian, x: QClass, mut n: u64) -> QClass {
        let mut a = j.zero();
        let mut b = x;
        while n != 0 {
            if n & 1 != 0 {
                a = j.add_raw(&a, &b)
            }
            n >>= 1;
            if n != 0 {
                b = j.add_raw(&b, &b)
            }
        }
        a
    }
    pub fn difference(
        j: &QuarticJacobian,
        p: [u64; 3],
        q: [u64; 3],
        points: &[[u64; 3]],
    ) -> QClass {
        let a: Vec<_> = points
            .iter()
            .copied()
            .filter(|r| *r != p && *r != q)
            .take(2)
            .collect();
        let cp = j.from_points([p, a[0], a[1]]).unwrap();
        let cq = j.from_points([q, a[0], a[1]]).unwrap();
        j.add_raw(&cp, &j.neg_raw(&cq))
    }
}
fn main() {
    previous::run_publicmap()
}
