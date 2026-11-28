mod qualified;
use std::{fs, io::Write};
fn prime(p: u64) -> bool {
    p >= 2 && (2..).take_while(|d| d * d <= p).all(|d| p % d != 0)
}
fn main() {
    let root = "research/hec_residual_span_20261116/strength";
    let mut receipt = fs::File::create_new(format!("{root}/FIXTURES.tsv")).unwrap();
    writeln!(receipt, "g\tp\tc\tstatus").unwrap();
    println!("g\tp\tstatus\tc\tell\tupper\twhole_cofactor_upper\tm\torbits\th");
    let (mut classified, mut positives) = (0, 0);
    for (g, p) in [
        (2usize, 101u64),
        (2, 251),
        (2, 401),
        (2, 601),
        (2, 1021),
        (3, 113),
        (3, 211),
        (3, 421),
    ] {
        assert!(prime(p));
        assert_eq!((p - 1) % (2 * g + 1) as u64, 0);
        let upper = qualified::upper_bound(g, p);
        eprintln!("screen_start g={g} p={p} upper={upper}");
        match qualified::fixture(g, p, &mut receipt) {
            Some(x) => {
                assert!((x.ell as u128) * 4 >= upper as u128);
                assert!(x.ell <= upper);
                assert!(!x.c.is_zero(&x.source));
                assert!(x.c.is_zero(&x.c.mul(&x.source, x.ell)));
                if (g, p) == (2, 101) {
                    assert_eq!(x.ell, 7481);
                    positives += 1
                }
                if (g, p) == (3, 421) {
                    assert_eq!(x.ell, 77152909);
                    positives += 1
                }
                println!(
                    "{g}\t{p}\tselected\t{}\t{}\t{upper}\t{}\t{}\t{}\t{}",
                    x.constant,
                    x.ell,
                    upper / x.ell,
                    x.fb.len(),
                    x.reps.len(),
                    x.h
                )
            }
            None => {
                println!("{g}\t{p}\tno_eligible_model\tnone\tnone\t{upper}\tnone\tnone\tnone\tnone")
            }
        }
        classified += 1;
    }
    assert_eq!(classified, 8);
    assert_eq!(positives, 2);
    eprintln!("classified={classified} positive_controls={positives} no_target_solves=true");
}
