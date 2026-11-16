#[allow(dead_code)]
mod weil_recovery {
    include!("weil_recovery.rs");
}
#[allow(dead_code)]
mod previous {
    include!("../hec_weil13_20261115/main.rs");
    include!("../hec_theta13_20261115/gate.rs");
    include!("../hec_kummer13_20261115/coordinates.rs");
    include!("../hec_image13_20261115/relations.rs");
    include!("../hec_geometry13_20261115/jets.rs");
    include!("../hec_coldbuild13_20261115/source_dynamic.rs");
    include!("../hec_coldbuild13_20261115/map_dynamic.rs");
    include!("../hec_coldbuild13_20261115/pipeline_dynamic.rs");
    include!("../hec_coldbuild13_20261115/pair_dynamic.rs");
    include!("builder.rs");
    include!("cold_api.rs");
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
    include!("../hec_coldbuild13_20261115/source_counts.rs");
}
fn main() {
    previous::run_coldprep()
}
mod descent {
    use super::target::{QClass, QuarticJacobian};
    pub fn effective(j: &QuarticJacobian, x: &QClass, pts: &[[u64; 3]]) -> Option<[usize; 3]> {
        let ann: Vec<Vec<u64>> =
            x.0.chunks(10)
                .map(|r| r.iter().map(|a| *a as u64).collect())
                .collect();
        let mut roots = Vec::new();
        for (i, p) in pts.iter().enumerate() {
            let mut v = j.eval_row3(*p);
            for r in &ann {
                let pivot = r.iter().position(|a| *a != 0).expect("rank3class");
                let a = v[pivot];
                for k in pivot..10 {
                    v[k] = (v[k] + 127 - a * r[k] % 127) % 127
                }
            }
            if v.iter().all(|a| *a == 0) {
                roots.push(i)
            }
        }
        if roots.len() != 3 {
            return None;
        }
        let ids = [roots[0], roots[1], roots[2]];
        (j.from_points(ids.map(|i| pts[i])).ok()? == *x).then_some(ids)
    }
}
#[allow(dead_code)]
mod lp_reference {
    include!("../hec_pairic13_20261115/lp_reference.rs");
}
#[allow(dead_code)]
mod lp_trace {
    include!("../hec_pairic13_20261115/lp_trace.rs");
}
#[allow(dead_code)]
mod orders_dynamic {
    include!("../hec_coldbuild13_20261115/orders_dynamic.rs");
}
#[allow(dead_code)]
#[path = "../hec_anchor_relations_20261114/anchor.rs"]
mod anchor;
#[allow(dead_code)]
#[path = "../../crates/hec-native-ic/src/table_factor.rs"]
mod table_factor;
