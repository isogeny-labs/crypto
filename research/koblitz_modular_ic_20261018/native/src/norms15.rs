//! New coupled Hecke norms, with an exact product norm control.
use super::*;
const N15: usize = 1024;
const NAMES15: [&str; 3] = ["y0*yinf", "y0*yinf+1", "x0*yinf+1"];
const POLES15: [usize; 3] = [792, 792, 660];
fn branch15(u: &[u64], v: &[u64], i: usize, n: usize) -> B {
    let p = mul(if i == 2 { u } else { v }, &dilate(v, 121, n), n);
    add(
        &p,
        &if i == 0 {
            vec![]
        } else {
            shift(&one(), if i == 2 { 365 } else { 366 }, n)
        },
    )
}
fn small15(u: &[u64], v: &[u64], i: usize, n: usize) -> B {
    if i == 2 {
        add(&mul(&dilate(u, 121, n), v, n), &shift(&one(), 245, n))
    } else {
        branch15(u, v, i, n)
    }
}
fn middle15(u: &[u64], v: &[u64], i: usize, n: usize) -> B {
    let f = Field::find(10);
    let zeta = (2..1024).map(|a| f.pow(a, 93)).find(|a| *a != 1).unwrap();
    let base = (0..n)
        .map(|k| u32::from(bit(if i == 2 { u } else { v }, k)))
        .collect::<Vec<_>>();
    let mut out = vec![0; n];
    out[0] = 1;
    for b in 1..11 {
        let twist = (0..n)
            .map(|k| {
                if bit(v, k) {
                    f.pow(zeta, (b * ((k + 19) % 11) % 11) as u64)
                } else {
                    0
                }
            })
            .collect::<Vec<_>>();
        let mut w = field_mul(f, &base, &twist, n);
        if i != 0 {
            w[if i == 2 { 5 } else { 6 }] ^= 1;
        }
        out = field_mul(f, &out, &w, n);
    }
    let mut z = vec![];
    for (k, a) in out.into_iter().enumerate() {
        assert!(a <= 1);
        if a == 1 {
            set(&mut z, k);
        }
    }
    assert!(bit(&z, 0));
    z
}
pub fn norms(dir: &Path) {
    REDUCTION_LIMIT.store(1_000_000_000, Ordering::Relaxed);
    let controls = control(N15);
    let (u, v) = xy(N15);
    let (ub, vb) = xy(121 * N15);
    let mut records = vec![];
    for i in 0..3 {
        let small = norm121(&small15(&ub, &vb, i, 121 * N15), N15);
        let unit = mul(
            &mul(&small, &middle15(&u, &v, i, N15), N15),
            &branch15(&u, &v, i, N15),
            N15,
        );
        let mut r = recover(&unit, POLES15[i], &u, &v, N15);
        r["function"] = json!(NAMES15[i]);
        println!(
            "M15:{} recovered,pole{},all1024 residual coefficients zero",
            NAMES15[i], POLES15[i]
        );
        round02::save(dir, &format!("m15-function-{i}.json"), &r);
        records.push(r);
    }
    round02::save(
        dir,
        "m15-new-coupled-Hecke-norms.json",
        &json!({"schema":"koblitz.round15/m15-v1","precision":N15,"input_precision":121*N15,"controls":controls,"Hecke_degree":132,"separate_scalar_cover_degree":5,"records":records,"global_bridge":"written modularity,good reduction,poles and RR;not finite-jet inference","new_relation_rank":null}),
    );
}
pub fn supports(dir: &Path) {
    REDUCTION_LIMIT.store(1_000_000_000, Ordering::Relaxed);
    product_control(dir);
    let norms = crate::round04::read(&dir.join("m15-new-coupled-Hecke-norms.json"));
    let mut records = vec![];
    for i in 0..3 {
        let a = parse(norms["records"][i]["A_bits"].as_str().unwrap());
        let b = parse(norms["records"][i]["B_bits"].as_str().unwrap());
        let (points, factorization) = zero_divisor(&a, &b);
        let mut parts = [0usize; 3];
        let points=points.iter().map(|p|{
            let j=match p.class.as_str(){"cusp"=>0,"supersingular"=>1,_=>2};parts[j]+=p.degree*p.mult;
            json!({"x_minpoly_bits":hex(&p.p),"y_residue_bits":p.y.as_ref().map(|y|hex(y)),"closed_degree":p.degree,"multiplicity":p.mult,"stratum":p.class})
        }).collect::<Vec<_>>();
        assert_eq!(parts.iter().sum::<usize>(), POLES15[i]);
        if i == 0 {
            assert_eq!(parts[..2], [0, 0], "ordinary line-family control");
        }
        let r = json!({"function":NAMES15[i],"points":points,"factorization":factorization,"pushed_cusp_SS_ordinary_degrees":parts,"source_ordinary_degree_under_cover_bridge":5*parts[2],"source_fibers_constructed":false,"individual_images_evaluated":false,"relation_rank":null});
        println!("M16:{} pushed partitions{parts:?}", NAMES15[i]);
        round02::save(dir, &format!("m16-function-{i}.json"), &r);
        records.push(r);
    }
    round02::save(
        dir,
        "m16-new-pushed-supports.json",
        &json!({"schema":"koblitz.round15/m16-v1","records":records,"scope":"complete pushed norms/closed factors;source fibers and projected rows remain separate"}),
    );
}
fn product_control(dir: &Path) {
    let (u, v) = xy(N15);
    let (_, vb) = xy(121 * N15);
    let f = Field::find(10);
    let zeta = (2..1024).map(|a| f.pow(a, 93)).find(|a| *a != 1).unwrap();
    let mut middle = vec![0; N15];
    middle[0] = 1;
    for b in 1..11 {
        let twisted = (0..N15)
            .map(|k| {
                if bit(&v, k) {
                    f.pow(zeta, (b * ((k + 19) % 11) % 11) as u64)
                } else {
                    0
                }
            })
            .collect::<Vec<_>>();
        middle = field_mul(f, &middle, &twisted, N15);
    }
    let mut m = vec![];
    for (i, a) in middle.into_iter().enumerate() {
        assert!(a <= 1);
        if a == 1 {
            set(&mut m, i);
        }
    }
    let unit = mul(
        &mul(&norm121(&vb, N15), &m, N15),
        &dilate(&v, 121, N15),
        N15,
    );
    let control = recover(&unit, 396, &u, &v, N15);
    let cmul = |a: &(B, B), b: &(B, B)| {
        let yy = mul(&a.1, &b.1, N15);
        (
            add(&mul(&a.0, &b.0, N15), &mul(&yy, &[13], N15)),
            add(
                &add(&mul(&a.0, &b.1, N15), &mul(&a.1, &b.0, N15)),
                &mul(&yy, &[2], N15),
            ),
        )
    };
    let mut power = (one(), vec![]);
    let mut y = (vec![], one());
    let mut exponent = 132;
    while exponent > 0 {
        if exponent % 2 == 1 {
            power = cmul(&power, &y);
        }
        y = cmul(&y, &y);
        exponent /= 2;
    }
    let ny = (
        parse(control["A_bits"].as_str().unwrap()),
        parse(control["B_bits"].as_str().unwrap()),
    );
    let product = cmul(&power, &ny);
    let original = crate::round04::read(&dir.join("m15-function-0.json"));
    assert_eq!(product.0, parse(original["A_bits"].as_str().unwrap()));
    assert_eq!(product.1, parse(original["B_bits"].as_str().unwrap()));
    let cusps = crate::round04::read(&dir.join("m13-coupled-cusp-valuations.json"));
    let f8 = Field::find(3);
    let curve = crate::round04::Curve { f: f8, a: 1, b: 1 };
    let yzero = (0..8)
        .filter(|x| curve.valid(crate::round04::Pt::A(*x, 0)))
        .collect::<Vec<_>>();
    assert_eq!(yzero.len(), 3);
    for x in &yzero {
        assert!(cusps["base_cusps"]
            .as_array()
            .unwrap()
            .iter()
            .all(|p| p["point"] != json!([x, 0])));
        assert_ne!(f8.sq(f8.sq(*x)), *x, "not a retained SS base point");
    }
    round02::save(
        dir,
        "m15-target-y-multiplicativity-control.json",
        &json!({"target_y_norm":control,"source_y_power":132,"coordinate_ring_product_matches":true,"F8_y_zero_x_coordinates":yzero,"F8_y_zero_disjoint_from_actual_cusps":true,"F8_y_zero_disjoint_from_SS_base_field":true,"failed_cusp_unit_assumption_corrected":true,"scope":"exact independent norm product plus actual F8 cusp comparison;one-coordinate family,not independent new coupled row"}),
    );
    println!("M15 control:Norm(y0*yinf)=y^132*Norm(yinf) exactly;three y=0 points are noncuspidal ordinary points");
}
