//! Fully counted source negation-rho on Tian's fixed published cubic instance.
use five_leads_study::three_leads::{is_prime, solve_rho, Elliptic, Field, Point, StudyInstance};
use serde_json::json;
fn pack(c0: u64, c1: u64, c2: u64) -> u64 {
    c0 + 523 * c1 + 523 * 523 * c2
}
fn main() {
    let f = Field::new(523, 3, 2); // w^3+2=0
    for j in 1..=1024u64 {
        let value = (j * 1_000_003) % f.size();
        if value != 0 {
            assert_eq!(f.mul(value, f.inv(value)), 1, "cubic norm inverse");
        }
    }
    f.reset();
    let e = Elliptic {
        f,
        c: [pack(243, 339, 359), pack(140, 92, 82), 0, 1],
        binary: false,
    };
    let p = Point::affine(7, pack(310, 306, 155));
    let q = Point::affine(pack(38, 393, 311), pack(33, 167, 418));
    let r = 143_069_629;
    assert!(is_prime(r));
    assert!(e.valid(p) && e.valid(q));
    assert!(e.mul(p, r).inf);
    assert_eq!(e.mul(p, 1987), q);
    let q3 = 523u64.pow(3);
    let delta = (r as i64 - q3 as i64 - 1).unsigned_abs();
    assert!(delta * delta <= 4 * q3 && r > 4 * (q3 as f64).sqrt() as u64 + 2);
    let fixture = StudyInstance {
        curve: Elliptic {
            f: e.f.fresh(),
            c: e.c,
            binary: false,
        },
        order: r,
        r,
        h: 1,
        generator: p,
        family: "published_tian_cubic".into(),
        parameter: 3,
        evidence: json!({"prime_order":true,"hasse_unique_multiple":true,"fixed_published_fixture":true}),
    };
    for seed in [20261020u64, 20261021, 20261022, 20261023, 20261024] {
        let out = solve_rho(&fixture, q, seed, 100 * r + 10_000, true, false);
        let verifier = Elliptic {
            f: e.f.fresh(),
            c: e.c,
            binary: false,
        };
        let recovered = out.recovered.expect("source rho exhausted");
        assert_eq!(verifier.mul(p, recovered), q);
        let counts = out.counts + verifier.f.counts();
        println!(
            "{}",
            json!({"schema":"tian-cubic-rho/v1","seed":seed,"r":r,"p":523,"extension_degree":3,
            "source_point":[p.x,p.y],"target_point":[q.x,q.y],"recovered":recovered,"verified":true,
            "counts":counts,"phases":out.phases,"native":out.native,"fixture_validation_excluded_both_arms":true})
        );
    }
}
