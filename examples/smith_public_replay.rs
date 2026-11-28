//! Focused public-source Smith-to-Diem DLP replay on Smith's Example 1.
//! Run with `cargo run --example smith_public_replay --offline`.

use crypto_lib::cryptanalysis::quartic_ladder::{QArm, QIcParams};
use crypto_lib::cryptanalysis::smith_canonical::Mumford;
use crypto_lib::cryptanalysis::smith_isogeny::SmithIso;
use crypto_lib::cryptanalysis::smith_prop24::{SplitMix64, EXAMPLE1_F, EXAMPLE1_ND, EXAMPLE1_P};
use crypto_lib::cryptanalysis::smith_trigonal::Trigonal;

fn main() {
    let (n1, n0, d1, d0) = EXAMPLE1_ND;
    let tr = Trigonal::new(&EXAMPLE1_F, EXAMPLE1_P, n1, n0, d1, d0)
        .expect("Smith Example 1 trigonal model");
    let iso = SmithIso::from_trigonal_map(tr).expect("Smith Example 1 transfer");
    assert_eq!(iso.order, 55_666);
    let planted = iso.planted(32, 3).expect("admitted prime subgroup");
    assert_eq!((planted.h, planted.r), (26, 2141));
    assert!(iso.jac.is_reduced_mumford(&planted.g_h));
    assert!(iso.jac.is_reduced_mumford(&planted.t_h));
    let mut off_curve = None;
    'points: for x in 0..iso.p {
        for y in 0..iso.p {
            let d = iso.jac.point(x, y);
            if !iso.jac.is_reduced_mumford(&d) {
                off_curve = Some(d);
                break 'points;
            }
        }
    }
    let malformed = [
        Mumford {
            u: vec![],
            v: vec![],
        },
        Mumford {
            u: vec![1, 2],
            v: vec![],
        },
        Mumford {
            u: vec![iso.p, 1],
            v: vec![],
        },
        Mumford {
            u: vec![1, 1],
            v: vec![0, 1],
        },
        off_curve.expect("off-curve point over the toy field"),
    ];
    for bad in malformed {
        assert!(!iso.jac.is_reduced_mumford(&bad));
        assert!(iso.image(&bad, &mut SplitMix64(5)).is_err());
        assert!(iso.public_instance(&bad, &planted.t_h, 5).is_err());
        assert!(iso.public_instance(&planted.g_h, &bad, 5).is_err());
    }

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
