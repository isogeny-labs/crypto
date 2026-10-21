//! The five remaining coupled functions, with independent stratum controls.
use super::*;
const N: usize = 1024;
const NAMES: [&str; 5] = [
    "x0+xinf+1",
    "y0+yinf",
    "y0+yinf+1",
    "x0*yinf+xinf*y0",
    "x0*yinf+xinf*y0+1",
];
const POLES: [usize; 5] = [504, 756, 756, 780, 780];
const PARTS: [[usize; 3]; 5] = [
    [0, 152, 352],
    [72, 308, 376],
    [0, 204, 552],
    [90, 416, 274],
    [0, 344, 436],
];

fn branch(u: &[u64], v: &[u64], i: usize, n: usize) -> B {
    let u121 = dilate(u, 121, n);
    let v121 = dilate(v, 121, n);
    match i {
        0 => add(&add(&u121, &shift(u, 240, n)), &shift(&one(), 242, n)),
        1 | 2 => add(
            &add(&v121, &shift(v, 360, n)),
            &if i == 2 {
                shift(&one(), 363, n)
            } else {
                vec![]
            },
        ),
        3 | 4 => add(
            &add(&mul(u, &v121, n), &shift(&mul(&u121, v, n), 120, n)),
            &if i == 4 {
                shift(&one(), 365, n)
            } else {
                vec![]
            },
        ),
        _ => unreachable!(),
    }
}
fn middle14(u: &[u64], v: &[u64], i: usize, n: usize) -> B {
    let f = Field::find(10);
    let zeta = (2..1024).map(|a| f.pow(a, 93)).find(|a| *a != 1).unwrap();
    let base = |p: &[u64]| (0..n).map(|k| u32::from(bit(p, k))).collect::<Vec<_>>();
    let bu = base(u);
    let bv = base(v);
    let mut out = vec![0; n];
    out[0] = 1;
    for b in 1..11 {
        let twist = |p: &[u64], w: usize| {
            (0..n)
                .map(|k| {
                    if bit(p, k) {
                        f.pow(zeta, (b * ((k + 22 - w) % 11) % 11) as u64)
                    } else {
                        0
                    }
                })
                .collect::<Vec<_>>()
        };
        let mut w = match i {
            0 => bu
                .iter()
                .zip(twist(u, 2))
                .map(|(a, b)| a ^ b)
                .collect::<Vec<_>>(),
            1 | 2 => bv
                .iter()
                .zip(twist(v, 3))
                .map(|(a, b)| a ^ b)
                .collect::<Vec<_>>(),
            3 | 4 => field_mul(f, &bu, &twist(v, 3), n)
                .iter()
                .zip(field_mul(f, &bv, &twist(u, 2), n))
                .map(|(a, b)| a ^ b)
                .collect::<Vec<_>>(),
            _ => unreachable!(),
        };
        if i == 0 {
            w[2] ^= 1;
        }
        if i == 2 {
            w[3] ^= 1;
        }
        if i == 4 {
            w[5] ^= 1;
        }
        out = field_mul(f, &out, &w, n);
    }
    let mut z = vec![];
    for (k, a) in out.into_iter().enumerate() {
        assert!(a <= 1, "middle norm F2 descent");
        if a == 1 {
            set(&mut z, k);
        }
    }
    assert!(bit(&z, 0));
    z
}
pub fn norms(dir: &Path) {
    let controls = control(N);
    println!("O19: norm tower controls passed");
    let (u, v) = xy(N);
    let (ub, vb) = xy(121 * N);
    println!("O19: exact normalized x/y series prepared");
    let mut records = vec![];
    for i in 0..5 {
        let path = dir.join(format!("o19-function-{i}.json"));
        let record = if path.exists() {
            crate::round04::read(&path)
        } else {
            let small = norm121(&branch(&ub, &vb, i, 121 * N), N);
            let unit = mul(
                &mul(&small, &middle14(&u, &v, i, N), N),
                &branch(&u, &v, i, N),
                N,
            );
            let mut r = recover(&unit, POLES[i], &u, &v, N);
            r["function"] = json!(NAMES[i]);
            r["original_function_index"] = json!([1, 2, 3, 6, 7][i]);
            round02::save(dir, &format!("o19-function-{i}.json"), &r);
            r
        };
        assert_eq!(record["function"], NAMES[i]);
        assert_eq!(record["pole_bound"], POLES[i]);
        println!(
            "O19:{} recovered, pole{}, residual zero",
            NAMES[i], POLES[i]
        );
        records.push(record);
    }
    round02::save(
        dir,
        "o19-five-Hecke-norms.json",
        &json!({"schema":"koblitz.round14/o19-v1","precision":N,"input_precision":121*N,"controls":controls,"records":records,"Hecke_degree":132,"source_scalar_cover_degree":5,"global_bridge":"written modularity/good reduction/poles/Riemann-Roch;not finite-jet inference"}),
    );
}
pub fn supports(dir: &Path) {
    let norms = crate::round04::read(&dir.join("o19-five-Hecke-norms.json"));
    let mut records = vec![];
    for i in 0..5 {
        let path = dir.join(format!("o20-function-{i}.json"));
        let r = if path.exists() {
            crate::round04::read(&path)
        } else {
            let a = parse(norms["records"][i]["A_bits"].as_str().unwrap());
            let b = parse(norms["records"][i]["B_bits"].as_str().unwrap());
            let (z, fac) = zero_divisor(&a, &b);
            let mut parts = [0usize; 3];
            let pts=z.iter().map(|p|{let j=match p.class.as_str(){"cusp"=>0,"supersingular"=>1,_=>2};parts[j]+=p.degree*p.mult;
                json!({"x_minpoly_bits":hex(&p.p),"y_residue_bits":p.y.as_ref().map(|y|hex(y)),"y_quadratic_relation":if p.y.is_none(){Some("y^2+x*y+x^3+x^2+1=0")}else{None},"closed_degree":p.degree,"multiplicity":p.mult,"stratum":p.class})}).collect::<Vec<_>>();
            let r = json!({"function":NAMES[i],"points":pts,"stratum_degrees":parts,"expected":PARTS[i],"factorization":fac,"matches_independent_degrees":parts==PARTS[i]});
            round02::save(dir, &format!("o20-function-{i}.json"), &r);
            r
        };
        println!("O20:{} partition{}", NAMES[i], r["stratum_degrees"]);
        assert_eq!(r["matches_independent_degrees"], true);
        records.push(r);
    }
    round02::save(
        dir,
        "o20-five-pushed-divisors.json",
        &json!({"schema":"koblitz.round14/o20-v1","records":records,"reduction_steps":REDUCTIONS.load(Ordering::Relaxed),"source_cover_degree":5,"source_lifts_complete":false,"projected_images_complete":false}),
    );
}
