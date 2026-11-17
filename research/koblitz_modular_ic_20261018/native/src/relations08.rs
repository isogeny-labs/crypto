//! Complete nonzero-line norms, with field-presentation-independent joins.
use crate::{
    forward07::{ax, ca, line_map, tuple},
    large_field::*,
    reverse06::{decode, dp, poly, A, AP, R},
    round02, round04,
};
use num_bigint::BigInt;
use serde_json::json;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
fn inv11(a: u64) -> u64 {
    (1..11).find(|b| a * b % 11 == 1).unwrap()
}
fn cycles(p: &[usize]) -> Vec<Vec<usize>> {
    let mut seen = vec![false; p.len()];
    let mut out = vec![];
    for i in 0..p.len() {
        if seen[i] {
            continue;
        }
        let mut c = vec![];
        let mut j = i;
        while !seen[j] {
            seen[j] = true;
            c.push(j);
            j = p[j];
        }
        assert_eq!(j, i);
        out.push(c)
    }
    out
}
fn tfrob(f: &F, t: &[P], odd: bool) -> Vec<P> {
    (0..5)
        .map(|i| {
            let p = f.frobp(&t[(i + 4) % 5], 1);
            if odd {
                neg(&p)
            } else {
                p
            }
        })
        .collect()
}
fn coord(p: &AP) -> (A, A) {
    if let AP::Pt(x, y) = p {
        (x.clone(), y.clone())
    } else {
        panic!("finite basis")
    }
}
struct Presentation<'a, 'b> {
    r: &'a R<'b>,
    powers: Vec<A>,
    inverse: Vec<Vec<E>>,
    minimal: Poly,
}
impl<'a, 'b> Presentation<'a, 'b> {
    fn new(r: &'a R<'b>, z: &A) -> Self {
        let d = r.p.len() - 1;
        assert_eq!(d, 11);
        let f = r.f;
        let mut powers = vec![r.o()];
        for _ in 0..d {
            powers.push(r.mul(powers.last().unwrap(), z));
        }
        let mut m = vec![vec![f.z(); 2 * d]; d];
        for i in 0..d {
            for j in 0..d {
                m[i][j] = powers[j].u.get(i).cloned().unwrap_or_else(|| f.z());
            }
            m[i][d + i] = f.o();
        }
        for j in 0..d {
            let p = (j..d)
                .find(|i| !m[*i][j].is_zero())
                .expect("primitive x(P) generator");
            m.swap(p, j);
            let v = f.inv(&m[j][j]);
            for k in 0..2 * d {
                m[j][k] = f.mul(&m[j][k], &v)
            }
            for i in 0..d {
                if i == j {
                    continue;
                }
                let v = m[i][j].clone();
                for k in 0..2 * d {
                    m[i][k] = m[i][k].add(&f.mul(&v, &m[j][k]));
                }
            }
        }
        let inverse: Vec<Vec<E>> = m.iter().map(|r| r[d..].to_vec()).collect();
        let mut out = Self {
            r,
            powers,
            inverse,
            minimal: vec![],
        };
        let mut g = out.convert(&out.powers[d]);
        g.resize(d, f.z());
        g.push(f.o());
        let residual = g.iter().enumerate().fold(r.z(), |v, (i, c)| {
            r.add(&v, &r.mul(&r.c(c), &out.powers[i]))
        });
        assert_eq!(residual, r.z());
        out.minimal = g;
        out
    }
    fn convert(&self, x: &A) -> Poly {
        assert!(x.v.is_empty());
        let r = self.r;
        let f = r.f;
        let d = self.inverse.len();
        let v: Poly = self
            .inverse
            .iter()
            .map(|row| {
                (0..d).fold(f.z(), |z, i| {
                    z.add(&f.mul(&row[i], x.u.get(i).unwrap_or(&f.z())))
                })
            })
            .collect();
        let check = v.iter().enumerate().fold(r.z(), |z, (i, c)| {
            r.add(&z, &r.mul(&r.c(c), &self.powers[i]))
        });
        assert_eq!(check, *x);
        trim(v)
    }
    fn key(&self, base: &P, p: &AP, q: &AP, j: u32) -> String {
        let r = self.r;
        let f = r.f;
        let (px, py) = coord(p);
        let (qx, qy) = coord(q);
        let encode = |poly: &Poly| polyj(&poly.iter().map(|c| f.frob(c, j)).collect::<Vec<_>>());
        let prefix = json!([
            pj(&f.frobp(base, j)),
            ej(&f.frob(&r.a, j)),
            ej(&f.frob(&r.b, j)),
            encode(&self.minimal)
        ]);
        let orient = |yp: &A, yq: &A| {
            json!([
                prefix,
                encode(&self.convert(yp)),
                encode(&self.convert(&qx)),
                encode(&self.convert(yq))
            ])
            .to_string()
        };
        orient(&py, &qy).min(orient(&r.add(&py, &px), &r.add(&qy, &qx)))
    }
}
pub fn run(dir: &Path) {
    run_impl(dir, false);
}
pub fn infinity(dir: &Path) {
    run_impl(dir, true);
}
fn run_impl(dir: &Path, include_infinity: bool) {
    let f = F::new(120, 27);
    let ec = f.curve(f.o(), f.o());
    let pos = round04::read(&dir.join("r3b-complete-split-fiber.json"));
    let negseed = round04::read(&dir.join("r3-reverse-fiber-screen-negative.json"));
    let m8 = round04::read(&dir.join("m8-full-map-principal-row.json"));
    let m9 = round04::read(&dir.join("m9-second-full-map-row.json"));
    let cache = round04::read(&dir.join("n4-independent-seed-targets.json"));
    let old = round04::read(&dir.join("m4-direct-modular-relations.json"));
    assert!(old["closed_points"]
        .as_array()
        .unwrap()
        .iter()
        .all(|c| c["degree"].as_u64().unwrap() % 60 != 0));
    assert_ne!(210 % 60, 0);
    let mut points = vec![];
    for row in pos["neighbors"].as_array().unwrap() {
        for b in row["bases"].as_array().unwrap() {
            points.push((
                dp(&f, &row["source_modular_point"]),
                decode(&f, &row["source_a"]),
                decode(&f, &row["source_b"]),
                dp(&f, &b["P"]),
                dp(&f, &b["Q"]),
            ));
        }
    }
    let pkey = |ix: usize, j: u32, line: u64| {
        let (base, a, b, p, q) = &points[ix];
        let cur = f.curve(f.frob(a, j), f.frob(b, j));
        let p = f.frobp(p, j);
        let q = times(&cur, &f.frobp(q, j), inv11(2u64.pow(j) % 11));
        let (p, q) = if line == 0 {
            (p, q)
        } else if line == 11 {
            (
                neg(&times(&cur, &q, 2u64.pow(j) % 11)),
                times(&cur, &p, inv11(2u64.pow(j) % 11)),
            )
        } else {
            (
                neg(&times(&cur, &q, line)),
                plus(&cur, &times(&cur, &p, inv11(line)), &q),
            )
        };
        crate::reverse06::basiskey(&f.frobp(base, j), &p, &q)
    };
    let mut keys: BTreeMap<String, usize> = BTreeMap::new();
    for (i, c) in m8["columns"].as_array().unwrap().iter().enumerate() {
        if c["seed"] != "positive" {
            continue;
        }
        for ix in c["members_over_F32"].as_array().unwrap() {
            for j in 0..5 {
                assert!(keys
                    .insert(pkey(ix.as_u64().unwrap() as usize, j, 0), 588 + i)
                    .is_none());
            }
        }
    }
    let mut columns = m9["new_columns"].as_array().unwrap().clone();
    let mut rows = vec![];
    let mut summaries = vec![];
    let mut conversions = 0;
    for line in 1..=if include_infinity { 6u64 } else { 5u64 } {
        let infinity = line == 6;
        let period = if infinity { 5u32 } else { 10u32 };
        let mut row: BTreeMap<usize, u64> = BTreeMap::new();
        let mut overlap = 0;
        let mut newcount = 0;
        let mut degrees = 0;
        let mut principal = vec![vec![P::Infinity; 5]; 2];
        let mut control_index = 0;
        for negative in [false, true] {
            let seedname = if negative { "negative" } else { "positive" };
            let si = usize::from(negative);
            let perm: Vec<usize> =
                serde_json::from_value(m8["descent"][si]["successor"].clone()).unwrap();
            let square: Vec<_> = perm.iter().map(|i| perm[*i]).collect();
            let seeds = cache[seedname].as_array().unwrap();
            let mut ckeys: Vec<Vec<String>> = vec![vec![]; seeds.len()];
            if negative {
                for (ix, seed) in seeds.iter().enumerate() {
                    let n = seed["neighbor"].as_u64().unwrap() as usize;
                    let input = round04::read(
                        &dir.join(format!("r3b-negative-fiber-neighbor-{n:02}.json")),
                    );
                    let cp = &input["paired_components"][ix % 5];
                    let base = dp(&f, &negseed["neighbors"][n]["source_modular_point"]);
                    let r = R {
                        f: &f,
                        p: poly(&f, &cp["Q_x_minpoly"]),
                        a: decode(&f, &input["source_a"]),
                        b: decode(&f, &input["source_b"]),
                        fx: vec![],
                    };
                    let p = r.cp(&dp(&f, &cp["P"]));
                    let q = AP::Pt(
                        ca(&r, &vec![f.z(), f.o()]),
                        ca(&r, &poly(&f, &cp["Q_y_polynomial"])),
                    );
                    let pnew = r.neg(&r.times(&q, if infinity { 1 } else { line }));
                    let qnew = if infinity {
                        p
                    } else {
                        r.plus(&r.times(&p, inv11(line)), &q)
                    };
                    assert_eq!(ax(&qnew).u.len() <= 1, infinity);
                    assert!(ax(&pnew).u.len() > 1);
                    let pr = Presentation::new(&r, &ax(&pnew));
                    for j in 0..period {
                        ckeys[ix].push(pr.key(
                            &base,
                            &pnew,
                            &r.times(&qnew, inv11(2u64.pow(j) % 11)),
                            j,
                        ));
                    }
                    conversions += 1;
                }
            }
            let values: Vec<Vec<Vec<P>>> = (0..2)
                .map(|sign| {
                    seeds
                        .iter()
                        .map(|s| {
                            let tt: Vec<_> = (0..12)
                                .map(|i| {
                                    dp(
                                        &f,
                                        &s["target_traces"][if infinity {
                                            line_map(0, 10, 1, 0, i)
                                        } else {
                                            line_map(0, 11 - line, inv11(line), 1, i)
                                        }],
                                    )
                                })
                                .collect();
                            tuple(&ec, &tt, sign == 1)
                        })
                        .collect()
                })
                .collect();
            for cycle in cycles(if infinity { &perm } else { &square }) {
                let degree = period as usize * cycle.len() * if negative { 11 } else { 1 };
                assert_eq!(cycle.len(), if infinity { 12 } else { 6 });
                degrees += degree;
                let mut images = vec![];
                for sign in 0..2 {
                    let mut z: Vec<_> = (0..5)
                        .map(|i| {
                            cycle
                                .iter()
                                .fold(P::Infinity, |z, ix| plus(&ec, &z, &values[sign][*ix][i]))
                        })
                        .collect();
                    let mut sum = vec![P::Infinity; 5];
                    for _ in 0..period {
                        for i in 0..5 {
                            sum[i] = plus(&ec, &sum[i], &z[i]);
                        }
                        z = tfrob(&f, &z, sign == 1);
                    }
                    assert_eq!(tfrob(&f, &sum, sign == 1), sum);
                    assert!(sum.iter().all(|p| times(&ec, p, 11) == P::Infinity));
                    for i in 0..5 {
                        principal[sign][i] = plus(&ec, &principal[sign][i], &sum[i]);
                    }
                    images.push(json!({"a":if sign==0{1}else{0},"nonzero":sum.iter().any(|p|*p!=P::Infinity),"tuple":sum.iter().map(pj).collect::<Vec<_>>()}));
                }
                let mut orbit = BTreeSet::new();
                for ix in &cycle {
                    for j in 0..period {
                        let key = if negative {
                            ckeys[*ix][j as usize].clone()
                        } else {
                            pkey(
                                *ix,
                                j,
                                if infinity {
                                    11
                                } else {
                                    line * 2u64.pow(j) % 11
                                },
                            )
                        };
                        assert!(orbit.insert(key), "repeated coordinate component");
                    }
                }
                assert_eq!(orbit.len(), 60);
                let joins: BTreeSet<_> =
                    orbit.iter().filter_map(|k| keys.get(k).copied()).collect();
                let index = if line == 1 {
                    let i = 648 + control_index;
                    control_index += 1;
                    let c = &columns[i - 648];
                    assert_eq!(c["degree"], degree);
                    assert_eq!(c["images"], json!(images));
                    assert!(joins.is_empty());
                    i
                } else if let Some(i) = joins.first() {
                    assert_eq!(joins.len(), 1);
                    assert!(orbit.iter().all(|k| keys.get(k) == Some(i)));
                    let c = if *i < 648 {
                        &m8["columns"][i - 588]
                    } else {
                        &columns[i - 648]
                    };
                    assert_eq!(c["degree"], degree);
                    assert_eq!(c["images"], json!(images));
                    overlap += 1;
                    *i
                } else {
                    let i = 648 + columns.len();
                    columns.push(json!({"seed":seedname,"degree":degree,"line":line,"members_over_F1024":cycle,"images":images}));
                    newcount += 1;
                    i
                };
                for key in orbit {
                    if let Some(old) = keys.insert(key, index) {
                        assert_eq!(old, index);
                    }
                }
                *row.entry(index).or_default() += 1;
            }
        }
        assert_eq!(degrees, if infinity { 6600 } else { 13200 });
        assert!(principal.iter().flatten().all(|p| *p == P::Infinity));
        if line == 1 {
            assert_eq!(json!(row), m9["row"]);
            assert_eq!(control_index, 120);
        }
        rows.push(json!(row));
        summaries.push(json!({"line":line,"new_columns":newcount,"joined_orbits":overlap,"zero_degree":degrees,"principal_sums":principal.iter().map(|t|t.iter().map(pj).collect::<Vec<_>>()).collect::<Vec<_>>() }));
        println!("M10 line{line}:new{newcount},joined{overlap},degree{degrees}");
    }
    let n = 648 + columns.len();
    let mut matrix: Vec<Vec<BigInt>> = old["principal_relations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            let mut row: Vec<_> = r["row_mod11"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| BigInt::from(v.as_u64().unwrap()))
                .collect();
            row.resize(n, 0.into());
            row
        })
        .collect();
    for range in [544..588, 588..648] {
        let mut r = vec![BigInt::from(0); n];
        for i in range {
            r[i] = 1.into();
        }
        matrix.push(r);
    }
    for row in &rows {
        let row: BTreeMap<usize, u64> = serde_json::from_value(row.clone()).unwrap();
        let mut r = vec![BigInt::from(0); n];
        for (i, v) in row {
            r[i] = v.into();
        }
        matrix.push(r);
    }
    let rank = round02::rank(matrix.clone(), &BigInt::from(11));
    let mut nz = vec![];
    for sign in 0..2 {
        let mut indices = vec![];
        for (i, c) in old["closed_points"].as_array().unwrap().iter().enumerate() {
            if c["images"][sign]["nonzero"] == true {
                indices.push(i)
            }
        }
        for (i, c) in m8["columns"].as_array().unwrap().iter().enumerate() {
            if c["images"][sign]["nonzero"] == true {
                indices.push(588 + i)
            }
        }
        for (i, c) in columns.iter().enumerate() {
            if c["images"][sign]["nonzero"] == true {
                indices.push(648 + i)
            }
        }
        let r = round02::rank(
            matrix
                .iter()
                .map(|row| indices.iter().map(|i| row[*i].clone()).collect())
                .collect(),
            &BigInt::from(11),
        );
        nz.push(json!({"a":if sign==0{1}else{0},"columns":indices.len(),"rank":r}));
    }
    round02::save(
        dir,
        if include_infinity {
            "m11-fixed-infinity-row.json"
        } else {
            "m10-full-line-family.json"
        },
        &json!({"schema":if include_infinity {"koblitz.round08/m11-v1"}else{"koblitz.round08/m10-v1"},"functions":if include_infinity {"five M10 norms plus Norm_F32/F2(x(phi_infinity)+x(P0));line6 labels infinity"}else{"Norm_F1024/F2(x(phi_a)+x(P0)),a=1..5"},"columns_from648":columns,"rows":rows,"family":summaries,"total_columns":n,"evaluated_rank":rank,"nonzero_image_matrices":nz,"primitive_generator_changes_checked":conversions,"scope":"complete principal supports with exact split and degree11 coordinate joins; prior a=1 control; no timing or individual descent"}),
    );
    println!("M10:rank{rank}/{n}, nonzero{}", json!(nz));
}

pub fn verify(dir: &Path, infinity: bool) {
    let file = if infinity {
        "m11-fixed-infinity-row.json"
    } else {
        "m10-full-line-family.json"
    };
    let input = round04::read(&dir.join(file));
    let old = round04::read(&dir.join("m4-direct-modular-relations.json"));
    let m8 = round04::read(&dir.join("m8-full-map-principal-row.json"));
    let base = round04::read(&dir.join("r3b-complete-split-fiber.json"));
    let f = F::new(120, 27);
    let root = decode(&f, &base["F32_embedding_root"]);
    let em: Vec<_> = (0..32).map(|x| f.embed_bits(x, &root)).collect();
    let twist = f.quadratic(&f.o()).unwrap();
    let sf = round04::Field::find(5);
    let cols = input["columns_from648"].as_array().unwrap();
    let mut checks = vec![];
    for sign in 0..2 {
        let ec = round04::Curve {
            f: sf,
            a: if sign == 0 { 1 } else { 0 },
            b: 1,
        };
        let convert = |v: &serde_json::Value| match dp(&f, v) {
            P::Infinity => round04::Pt::O,
            P::Affine { x, y } => {
                let y = if sign == 1 {
                    y.add(&f.mul(&twist, &x))
                } else {
                    y
                };
                round04::Pt::A(
                    em.iter().position(|v| *v == x).unwrap() as u32,
                    em.iter().position(|v| *v == y).unwrap() as u32,
                )
            }
        };
        let mut points = vec![];
        for c in cols {
            let ps: Vec<_> = c["images"][sign]["tuple"]
                .as_array()
                .unwrap()
                .iter()
                .map(convert)
                .collect();
            for p in &ps {
                assert!(ec.valid(*p));
                assert_eq!(ec.times(*p, 11), round04::Pt::O);
            }
            assert_eq!(
                ps[0] != round04::Pt::O,
                c["images"][sign]["nonzero"].as_bool().unwrap()
            );
            points.push(ps);
        }
        for row in input["rows"].as_array().unwrap() {
            let row: BTreeMap<usize, u64> = serde_json::from_value(row.clone()).unwrap();
            for coordinate in 0..5 {
                let mut sum = round04::Pt::O;
                for (i, v) in &row {
                    let p = if *i < 648 {
                        convert(&m8["columns"][i - 588]["images"][sign]["tuple"][coordinate])
                    } else {
                        points[i - 648][coordinate]
                    };
                    sum = ec.add(sum, ec.times(p, *v));
                }
                assert_eq!(sum, round04::Pt::O);
            }
        }
        checks.push(json!({"a":if sign==0{1}else{0},"column_images":points.iter().map(|p|p.iter().map(|p|p.json()).collect::<Vec<_>>()).collect::<Vec<_>>(),"all_principal_coordinate_sums_zero":true}));
    }
    let n = input["total_columns"].as_u64().unwrap() as usize;
    let mut matrix: Vec<Vec<u64>> = old["principal_relations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            let mut v: Vec<_> = r["row_mod11"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_u64().unwrap())
                .collect();
            v.resize(n, 0);
            v
        })
        .collect();
    for range in [544..588, 588..648] {
        let mut r = vec![0; n];
        for i in range {
            r[i] = 1
        }
        matrix.push(r)
    }
    for row in input["rows"].as_array().unwrap() {
        let row: BTreeMap<usize, u64> = serde_json::from_value(row.clone()).unwrap();
        let mut r = vec![0; n];
        for (i, v) in row {
            r[i] = v % 11;
        }
        matrix.push(r);
    }
    let rank = crate::gluing::rref(matrix.clone(), 11).1.len();
    assert_eq!(rank, input["evaluated_rank"].as_u64().unwrap() as usize);
    let mut nz = vec![];
    for s in 0..2 {
        let mut indices = vec![];
        for (i, c) in old["closed_points"].as_array().unwrap().iter().enumerate() {
            if c["images"][s]["nonzero"] == true {
                indices.push(i)
            }
        }
        for (i, c) in m8["columns"].as_array().unwrap().iter().enumerate() {
            if c["images"][s]["nonzero"] == true {
                indices.push(588 + i)
            }
        }
        for (i, c) in cols.iter().enumerate() {
            if c["images"][s]["nonzero"] == true {
                indices.push(648 + i)
            }
        }
        let r = crate::gluing::rref(
            matrix
                .iter()
                .map(|r| indices.iter().map(|i| r[*i]).collect())
                .collect(),
            11,
        )
        .1
        .len();
        assert_eq!(
            r,
            input["nonzero_image_matrices"][s]["rank"].as_u64().unwrap() as usize
        );
        nz.push(r);
    }
    round02::save(
        dir,
        if infinity {
            "m11b-small-field-check.json"
        } else {
            "m10b-small-field-check.json"
        },
        &json!({"schema":"koblitz.round08/relation-check-v1","checks":checks,"u64_rank":rank,"nonzero_ranks":nz,"scope":"parsed generated images in separate u32 F32 arithmetic, every coordinate of every row sum and separate modular ranks"}),
    );
    println!(
        "M10/M11 checker:all{} images both signs, ranks{rank}/{nz:?}",
        cols.len()
    );
}
