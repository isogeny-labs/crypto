//! Expanded finite boxes, independent Newton masks, and classical explicit lifts.
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
#[path = "../../../../../src/cryptanalysis/twist_ic_field.rs"]
mod toy_field;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs,
    path::{Path, PathBuf},
};
use toy_field::*;
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn read(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn write(p: impl AsRef<Path>, v: &Value) -> Result<()> {
    Ok(fs::write(p, serde_json::to_string_pretty(v)? + "\n")?)
}
fn next(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
fn target(e: Quad) -> P {
    let mut s = 202610210000 + e.p * 65537;
    loop {
        let x = next(&mut s) % e.p;
        if let Some(y) = sqrt(e.rhs(Q(x, 0)).0, e.p) {
            if y != 0 {
                let y = if next(&mut s) & 1 == 0 { y } else { e.p - y };
                return Some((Q(x, 0), Q(y, 0)));
            }
        }
    }
}
fn line(p: u64, values: &[u64]) -> Vec<u64> {
    let mut differences = values.to_vec();
    let mut basis = vec![1];
    let mut factorial = 1;
    let mut result = vec![0; values.len()];
    for j in 0..values.len() {
        if j > 0 {
            factorial = factorial * j as u64 % p;
        }
        let c = differences[0] * inv(factorial, p) % p;
        for (i, b) in basis.iter().enumerate() {
            result[i] = (result[i] + c * b) % p;
        }
        differences = differences
            .windows(2)
            .map(|w| (w[1] + p - w[0]) % p)
            .collect();
        let mut b = vec![0; basis.len() + 1];
        for (i, v) in basis.iter().enumerate() {
            b[i] = (b[i] + v * (p - j as u64) % p) % p;
            b[i + 1] = (b[i + 1] + v) % p;
        }
        basis = b;
    }
    result
}
fn tensor(p: u64, grid: &[Vec<bool>]) -> Vec<Vec<u64>> {
    let h = grid.len();
    let k = grid[0].len();
    let horizontal = grid
        .iter()
        .map(|r| line(p, &r.iter().map(|b| *b as u64).collect::<Vec<_>>()))
        .collect::<Vec<_>>();
    let mut result = vec![vec![0; k]; h];
    for j in 0..k {
        let column = line(p, &horizontal.iter().map(|r| r[j]).collect::<Vec<_>>());
        for i in 0..h {
            result[i][j] = column[i];
        }
    }
    result
}
fn evaluate(p: u64, c: &[Vec<u64>], r: u64, u: u64) -> u64 {
    c.iter().rev().fold(0, |a, row| {
        (a * r + row.iter().rev().fold(0, |b, c| (b * u + c) % p)) % p
    })
}
fn degree(c: &[Vec<u64>]) -> usize {
    c.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, c)| (*c != 0).then_some(i + j))
        })
        .max()
        .unwrap_or(0)
}
fn f3(e: Quad, x: u64, y: u64, z: u64) -> u64 {
    let p = e.p;
    let a = (x + p - y) % p;
    let middle = 2 * ((x + y) % p * ((x * y + e.a) % p) % p + 2 * e.b) % p;
    let tail = ((x * y + p - e.a) % p).pow(2) % p;
    let tail = (tail + p - 4 * e.b % p * ((x + y) % p) % p) % p;
    (a * a % p * z % p * z % p + p - middle * z % p + tail) % p
}
fn lift_y(e: Quad, x: u64, y: u64, z: u64, den_inverse: u64) -> u64 {
    let p = e.p;
    let delta = (x + p - z) % p;
    let middle = 2 * ((x + z) % p * ((x * z + e.a) % p) % p + 2 * e.b) % p;
    ((2 * delta % p * delta % p * y % p + p - middle) % p) * den_inverse % p
}
fn qtext(q: Q) -> String {
    format!("({}+{}*w)", q.0, q.1)
}
fn ptext(p: P) -> String {
    match p {
        None => "[0]".into(),
        Some((x, y)) => format!("[{},{}]", qtext(x), qtext(y)),
    }
}
fn main() -> Result<()> {
    let a = std::env::args().collect::<Vec<_>>();
    if a.len() != 4 {
        return Err("usage: frontier FIXTURES ORIGINAL_INCIDENCE FRESH_OUT".into());
    }
    let fixtures = read(&a[1])?;
    let old = Path::new(&a[2]);
    let out = Path::new(&a[3]);
    if out.exists() {
        return Err("fresh output required".into());
    }
    fs::create_dir_all(out)?;
    let mut rows = vec![];
    let mut gp = String::from("default(parisizemax,4000000000);\ngood=0;mixed=0;\n");
    let mut positive = 0;
    let mut old_checks = 0;
    let mut mixed_count = 0;
    for f in fixtures["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["order"].as_u64().is_some_and(|n| n % 2 == 1 && prime(n)))
    {
        let id = f["id"].as_str().unwrap();
        let p = f["p"].as_u64().unwrap();
        let e = Quad {
            p,
            a: f["a"].as_u64().unwrap(),
            b: f["b"].as_u64().unwrap(),
            d: f["d"].as_u64().unwrap(),
        };
        let t = target(e);
        let (tx, ty) = t.unwrap();
        let deninv = inv(4 * ty.0 % p, p);
        let et = Quad {
            p,
            d: e.d,
            a: e.a * e.d % p * e.d % p,
            b: e.b * e.d % p * e.d % p * e.d % p,
        };
        for h in [2usize, 4, 8, 16] {
            for k in [2usize, 4, 8, 16] {
                let scales = BTreeSet::from([1u64, 2, 3, h as u64]);
                for lambda in scales {
                    let u_values = (0..k as u64)
                        .filter(|u| sqrt(et.rhs(Q(e.d * u % p, 0)).0, p).is_some())
                        .collect::<Vec<_>>();
                    let mut image = BTreeMap::<u64, Vec<(u64, u64)>>::new();
                    let mut all_box = BTreeSet::new();
                    for r in 0..h as u64 {
                        for u in 0..k as u64 {
                            let x = (r + p - lambda * u % p) % p;
                            all_box.insert(x);
                            if u_values.contains(&u) {
                                image.entry(x).or_default().push((r, u));
                            }
                        }
                    }
                    let base = image
                        .keys()
                        .filter(|x| sqrt(e.rhs(Q(**x, 0)).0, p).is_some())
                        .copied()
                        .collect::<BTreeSet<_>>();
                    let grid = (0..h)
                        .map(|r| {
                            (0..k)
                                .map(|u| {
                                    u_values.contains(&(u as u64))
                                        && base
                                            .contains(&((r as u64 + p - lambda * u as u64 % p) % p))
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();
                    let ordinary = (0..h)
                        .map(|r| {
                            (0..k)
                                .map(|u| {
                                    base.contains(&((r as u64 + p - lambda * u as u64 % p) % p))
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();
                    let mask = tensor(p, &grid);
                    let ordinary_mask = tensor(p, &ordinary);
                    for r in 0..h {
                        for u in 0..k {
                            assert_eq!(evaluate(p, &mask, r as u64, u as u64), grid[r][u] as u64);
                            assert_eq!(
                                evaluate(p, &ordinary_mask, r as u64, u as u64),
                                ordinary[r][u] as u64
                            );
                        }
                    }
                    let mut bad = mask.clone();
                    bad[0][0] = (bad[0][0] + 1) % p;
                    assert_ne!(evaluate(p, &bad, 0, 0), evaluate(p, &mask, 0, 0));
                    let name = format!("{id}-l{lambda}-h{h}-k{k}");
                    let original = h <= 8 && k <= 8 && lambda <= 3;
                    let old_hash = if original {
                        let file = old.join(format!("{name}.json"));
                        let o = read(&file)?;
                        assert_eq!(json!(mask), o["mask"]);
                        assert_eq!(json!(ordinary_mask), o["ordinary_mask"]);
                        old_checks += 1;
                        Some(sha(file)?)
                    } else {
                        None
                    };
                    if lambda == h as u64 && h * k <= p as usize {
                        assert_eq!(all_box.len(), h * k);
                    }
                    let mut found = BTreeMap::<Vec<u64>, Vec<u64>>::new();
                    let mut raw_solutions = 0u64;
                    for x in image.keys() {
                        for y in image.keys() {
                            if f3(e, *x, *y, tx.0) == 0 {
                                assert!(
                                    base.contains(x) && base.contains(y),
                                    "curve mask not redundant on F3 task"
                                );
                                let yx = lift_y(e, *x, *y, tx.0, deninv);
                                let yy = lift_y(e, *y, *x, tx.0, deninv);
                                let px = Some((Q(*x, 0), Q(yx, 0)));
                                let py = Some((Q(*y, 0), Q(yy, 0)));
                                assert!(e.on(px) && e.on(py));
                                assert_eq!(e.pa(px, py), t);
                                assert_ne!(e.pa(e.pn(px), py), t);
                                let (xs, ys) = if x <= y {
                                    (vec![*x, *y], vec![yx, yy])
                                } else {
                                    (vec![*y, *x], vec![yy, yx])
                                };
                                found.entry(xs).or_insert(ys);
                                raw_solutions += (image[x].len() * image[y].len()) as u64;
                            }
                        }
                    }
                    let mut independent = BTreeSet::new();
                    let points = base
                        .iter()
                        .map(|x| {
                            (
                                *x,
                                Some((Q(*x, 0), Q(sqrt(e.rhs(Q(*x, 0)).0, p).unwrap(), 0))),
                            )
                        })
                        .collect::<BTreeMap<_, _>>();
                    for x in &base {
                        for y in &base {
                            let a = points[x];
                            let b = points[y];
                            if [e.pa(a, b), e.pa(a, e.pn(b))]
                                .iter()
                                .any(|q| q.is_some_and(|(x, _)| x == tx))
                            {
                                let mut v = vec![*x, *y];
                                v.sort();
                                independent.insert(v);
                            }
                        }
                    }
                    assert_eq!(found.keys().cloned().collect::<BTreeSet<_>>(), independent);
                    gp.push_str(&format!(
                        "E=ellinit([Mod({},{}),Mod({}, {})]);T=[Mod({},{}),Mod({}, {})];\n",
                        e.a, p, e.b, p, tx.0, p, ty.0, p
                    ));
                    for (xs, ys) in &found {
                        gp.push_str(&format!("P=[Mod({},{}),Mod({}, {})];Q=[Mod({},{}),Mod({}, {})];if(!ellisoncurve(E,P)||!ellisoncurve(E,Q)||elladd(E,P,Q)!=T,error(\"explicit F3 lift\"));if(elladd(E,ellneg(E,P),Q)==T,error(\"wrong sign accepted\"));good++;\n",xs[0],p,ys[0],p,xs[1],p,ys[1],p));
                        positive += 1;
                    }
                    let counter = if image.contains_key(&tx.0) {
                        image.keys().find(|x|!base.contains(x)).map(|x|{
    let anti=Some((Q(*x,0),e.sqrt(e.rhs(Q(*x,0))).unwrap()));assert_eq!(e.frob(anti),e.pn(anti));assert_eq!(e.pa(anti,e.pn(anti)),None);let prefix=e.pa(t,anti);assert_ne!(prefix.unwrap().0.1,0);assert_eq!(e.pa(prefix,e.pn(anti)),t);
    gp.push_str(&format!("zz='zz;w=ffgen(Mod(1,{p})*(zz^2-{}),'w);E=ellinit([{},{}],w);T={};A={};S=elladd(E,T,A);if(S!={}||elladd(E,S,ellneg(E,A))!=T,error(\"mixed prefix\"));if(S[1]^{p}==S[1],error(\"mixed prefix incorrectly rational\"));mixed++;\n",e.d,e.a,e.b,ptext(t),ptext(anti),ptext(prefix)));
    mixed_count+=1;json!({"tuple_x":[tx.0,*x,*x],"points":[t,anti,e.pn(anti)],"prefix":prefix,"r_u_witnesses":[image[&tx.0][0],image[x][0],image[x][0]],"prefix_x_not_Fp":true,"twist_sum_zero":true,"new_mixed_DLP_relation":false})
   })
                    } else {
                        None
                    };
                    let direct_degree = base.len().max(4);
                    let full_degree = h.max(k).max(degree(&mask)).max(4);
                    let reduced_degree = h.max(u_values.len()).max(4);
                    let record = json!({"case":name,"fixture":id,"p":p,"lambda":lambda,"h":h,"k":k,"target":t,"U":u_values,"base_x":base,"unfiltered_image":image.keys().collect::<Vec<_>>(),"raw_box_image_size":all_box.len(),"radix_injective_when_HK_le_p_checked":lambda==h as u64&&h*k<=p as usize,"original_case":original,"original_record_sha256":old_hash,"mask":mask,"ordinary_mask":ordinary_mask,"full_mask_degree":degree(&mask),"ordinary_mask_degree":degree(&ordinary_mask),"direct_system_max_degree":direct_degree,"full_mask_system_max_degree":full_degree,"reduced_U_system_max_degree":reduced_degree,"full_mask_lower_than_product":base.len()>=2&&full_degree<direct_degree,"reduced_U_lower_than_product":base.len()>=2&&reduced_degree<direct_degree,"generic_ordinary_reduced_U_has_identical_equations":true,"F3_pair_image_solutions":raw_solutions,"canonical_pairs":found.keys().collect::<Vec<_>>(),"explicit_y_witnesses":found.iter().map(|(x,y)|json!({"x":x,"y":y})).collect::<Vec<_>>(),"independent_source_pair_oracle_equal":true,"mixed_arity_three_counterexample":counter,"solver_cost_measured":false,"novelty_established":false});
                    write(out.join(format!("{name}.json")), &record)?;
                    rows.push(json!({"case":name,"fixture":id,"p":p,"h":h,"k":k,"lambda":lambda,"B":base.len(),"original_case":original,"direct_degree":direct_degree,"full_degree":full_degree,"reduced_U_degree":reduced_degree,"full_mask_lower":base.len()>=2&&full_degree<direct_degree,"reduced_U_lower":base.len()>=2&&reduced_degree<direct_degree,"canonical_pairs":found.len(),"mixed_counterexample":counter.is_some(),"record_sha256":sha(out.join(format!("{name}.json")))?}));
                }
            }
        }
        println!(
            "{}",
            json!({"fixture":id,"cases":rows.len(),"original_coefficient_checks":old_checks})
        );
    }
    assert_eq!(rows.len(), 600);
    assert_eq!(old_checks, 270);
    gp.push_str(&format!("if(good!={positive}||mixed!={mixed_count},error(\"frontier coverage\"));print(\"ALL_FRONTIER_REPLAY_OK \",good,\" \",mixed);quit;\n"));
    fs::write(out.join("verify.gp"), gp)?;
    write(out.join("ROWS.json"), &json!(rows))?;
    let summary = json!({"schema":"twist.incidence-frontier/v1","cases":rows.len(),"original_coefficient_checks":old_checks,"full_mask_degree_reductions":rows.iter().filter(|r|r["full_mask_lower"]==true).count(),"reduced_U_degree_reductions":rows.iter().filter(|r|r["reduced_U_lower"]==true).count(),"original_270_full_mask_degree_reductions":rows.iter().filter(|r|r["original_case"]==true&&r["full_mask_lower"]==true).count(),"original_270_reduced_U_degree_reductions":rows.iter().filter(|r|r["original_case"]==true&&r["reduced_U_lower"]==true).count(),"explicit_signed_pair_checks":positive,"mixed_prefix_counterexamples":mixed_count,"all_pair_projections_equal":true,"newton_matches_old_lagrange":true,"fixture_sha256":sha(&a[1])?,"binary_sha256":sha(std::env::current_exe()?)?,"PARI_replay_executed":false,"solver_cost_measured":false,"speedup_or_novelty_established":false});
    write(out.join("SUMMARY.json"), &summary)?;
    println!("{}", summary);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn newton_interpolation_recovers_a_quadratic() {
        let values = (0..8)
            .map(|x| (3 * x * x + 7 * x + 5) % 59)
            .collect::<Vec<_>>();
        let c = line(59, &values);
        assert_eq!(&c[..3], &[5, 7, 3]);
        assert!(c[3..].iter().all(|c| *c == 0));
    }
}
