#[allow(dead_code)]
mod good {
    include!("../hec_calibration631_20261115/good.rs");include!("../hec_calibration631_20261115/base_control.rs");
    include!("section_api.rs");
}
#[allow(dead_code)]
pub mod target {
    include!("target.rs");
    include!("../hec_calibration631_20261115/target_api631.rs");
}
use target::{QClass, QuarticJacobian};
fn proportional(a: [u64; 8], b: [u64; 8]) -> bool {
    (0..8).all(|i| (0..8).all(|j| a[i] * b[j] % 631 == a[j] * b[i] % 631))
}
static CROSS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static SWAPPED: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
fn coord(
    j: &QuarticJacobian,
    s: &good::Sections,
    x: QClass,
    shift: QClass,
) -> Option<([u64; 8], usize)> {
    let x = j.add_raw(&x, &shift);
    let a = target::graph_chart(j, x, false).and_then(|(u, v)| {
        s.coords_chart(u, v, false)
            .map(|c| (c, good::root_degree(u)))
    });
    let b = target::graph_chart(j, x, true).and_then(|(u, v)| {
        s.coords_chart(u, v, true)
            .map(|c| (c, good::root_degree(u)))
    });
    if let (Some((p, d)), Some((q, e))) = (a, b) {
        assert_eq!(d, e);
        assert!(proportional(p, q), "projective chart covariance");
        CROSS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    if a.is_none() && b.is_some() {
        SWAPPED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    println!(
        "TARGET_CHART original={} swapped={}",
        a.is_some(),
        b.is_some()
    );
    a.or(b)
}

pub fn special_control(){good::special_control();}
pub fn counts(){good::counts();}
