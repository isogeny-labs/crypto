//! Constructor attribution/control only, not a DLP or speed measurement.
use crypto_lib::cryptanalysis::{gaudry_quartic_bin as narrow, gaudry_quartic_bin_wide as wide};
use serde_json::json;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 2, "narrow|wide|both M");
    let m: u32 = args[1].parse().unwrap();
    let mode = args[0].as_str();
    assert!(["narrow", "wide", "both"].contains(&mode));
    let old = if mode != "wide" {
        Some(narrow::Fq4::new(m))
    } else {
        None
    };
    let new = if mode != "narrow" {
        Some(wide::Fq4::new(m))
    } else {
        None
    };
    if let (Some(a), Some(b)) = (&old, &new) {
        assert_eq!(a.f.modulus, b.f.modulus);
        assert_eq!(a.c.map(u32::from), b.c);
        for i in 0..64u16 {
            for j in 0..64u16 {
                if u32::from(i) < a.f.q && u32::from(j) < a.f.q {
                    assert_eq!(
                        u32::from(a.f.mul_nc(i, j)),
                        b.f.mul_nc(u32::from(i), u32::from(j))
                    );
                }
            }
        }
    }
    let (modulus, coefficients) = if let Some(f) = new {
        (f.f.modulus, f.c)
    } else {
        let f = old.unwrap();
        (f.f.modulus, f.c.map(u32::from))
    };
    println!(
        "{}",
        json!({"schema":"ic-quartic-constructor-probe/v1","mode":mode,"m":m,"base_modulus":modulus,"extension_coefficients":coefficients,"control_parity_checked":mode=="both","scope":"Constructor attribution only; no throughput, complete DLP or full-pipeline claim."})
    );
}
