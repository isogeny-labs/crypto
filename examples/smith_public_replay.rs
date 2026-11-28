//! Focused public-source Smith-to-Diem DLP replay on Smith's Example 1.
//! Run with `cargo run --example smith_public_replay --offline`.

use crypto_lib::cryptanalysis::quartic_ladder::{QArm, QIcParams};
use crypto_lib::cryptanalysis::smith_isogeny::SmithIso;
use crypto_lib::cryptanalysis::smith_prop24::{EXAMPLE1_F, EXAMPLE1_ND, EXAMPLE1_P};
use crypto_lib::cryptanalysis::smith_trigonal::Trigonal;

fn main() {
    let (n1, n0, d1, d0) = EXAMPLE1_ND;
    let tr = Trigonal::new(&EXAMPLE1_F, EXAMPLE1_P, n1, n0, d1, d0)
        .expect("Smith Example 1 trigonal model");
    let iso = SmithIso::from_trigonal_map(tr).expect("Smith Example 1 transfer");
    assert_eq!(iso.order, 55_666);
    let planted = iso.planted(32, 3).expect("admitted prime subgroup");
    assert_eq!((planted.h, planted.r), (26, 2141));

    let public = iso
        .public_instance(&planted.g_h, &planted.t_h, 5)
        .expect("public source classes must transfer");
    assert_eq!(public.t_secret, 0);
    assert_eq!(public.r, planted.r);
    assert!(iso
        .public_instance(&iso.jac.zero(), &planted.t_h, 5)
        .is_err());

    let result = iso
        .solve_public(
            &planted.g_h,
            &planted.t_h,
            5,
            QIcParams {
                arm: QArm::Full,
                nb: (3.0 * f64::from(public.q).sqrt()).ceil() as usize,
                seed: 5,
                max_lines: 1 << 22,
                verify_relations: true,
            },
        )
        .expect("public Smith solve");
    assert!(result.quartic.certified);
    assert_eq!(result.scalar, Some(planted.t));
    assert!(iso.certify(&planted, result.scalar));
    println!(
        "Smith Example 1 public-source Diem replay: q={} r={} recovered={} source_verified=true",
        public.q,
        public.r,
        result.scalar.unwrap()
    );
}
