//! Native public-fixture generator and bounded Singular driver.
//! No private scalars or end-to-end ECDLP performance is measured.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

const N: usize = 83;
const MASK: u128 = (1u128 << N) - 1;
const MODLOW: u128 = (1u128 << 45) | 7;
const SEED: u64 = 20261027;
type Point = Option<(u128, u128)>;
type Poly = BTreeMap<Vec<usize>, u128>;

fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn save(p: &Path, v: &Value) {
    fs::write(p, format!("{}\n", serde_json::to_string_pretty(v).unwrap())).unwrap();
}
fn next(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
fn draw(s: &mut u64) -> u128 {
    ((next(s) as u128) << 64 | next(s) as u128) & MASK
}
fn bm(mut a: u128, mut b: u128) -> u128 {
    let mut r = 0;
    while b != 0 {
        if b & 1 != 0 {
            r ^= a;
        }
        b >>= 1;
        let carry = a >> 82;
        a = (a << 1) & MASK;
        if carry != 0 {
            a ^= MODLOW;
        }
    }
    r
}
fn bp(mut a: u128, mut e: u128) -> u128 {
    let mut r = 1;
    while e != 0 {
        if e & 1 != 0 {
            r = bm(r, a);
        }
        a = bm(a, a);
        e >>= 1;
    }
    r
}
fn bi(a: u128) -> u128 {
    assert_ne!(a, 0);
    bp(a, MASK - 1)
}
fn pm(mut a: u128, mut b: u128, p: u128) -> u128 {
    let mut r = 0;
    while b != 0 {
        if b & 1 != 0 {
            r = (r + a) % p;
        }
        a = (a + a) % p;
        b >>= 1;
    }
    r
}
fn pp(mut a: u128, mut e: u128, p: u128) -> u128 {
    let mut r = 1;
    while e != 0 {
        if e & 1 != 0 {
            r = pm(r, a, p);
        }
        a = pm(a, a, p);
        e >>= 1;
    }
    r
}
fn sqrt(a: u128, p: u128) -> Option<u128> {
    if a == 0 {
        return Some(0);
    }
    if pp(a, (p - 1) / 2, p) != 1 {
        return None;
    }
    let s = (p - 1).trailing_zeros();
    let q = (p - 1) >> s;
    let mut z = 2;
    while pp(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let (mut c, mut x, mut t, mut m) = (pp(z, q, p), pp(a, (q + 1) / 2, p), pp(a, q, p), s);
    while t != 1 {
        let (mut i, mut u) = (1, pm(t, t, p));
        while u != 1 {
            u = pm(u, u, p);
            i += 1;
            assert!(i < m);
        }
        let b = pp(c, 1u128 << (m - i - 1), p);
        x = pm(x, b, p);
        c = pm(b, b, p);
        t = pm(t, c, p);
        m = i;
    }
    Some(x)
}
fn prime() -> (u128, u128) {
    // Proth certificate: odd c < 2^42, p=c*2^42+1, a^((p-1)/2)=-1.
    for c in ((1u128 << 40) + 1..(1u128 << 40) + 10000).step_by(2) {
        let p = (c << 42) + 1;
        if [3, 5, 7, 11, 13, 17, 19, 23, 29, 31]
            .iter()
            .any(|q| p % q == 0)
        {
            continue;
        }
        for a in 2..20 {
            if pp(a, (p - 1) / 2, p) == p - 1 {
                return (p, a);
            }
        }
    }
    panic!("no certified prime in frozen search range")
}
#[derive(Clone, Copy)]
struct Curve {
    binary: bool,
    a: u128,
    b: u128,
    p: u128,
}
impl Curve {
    fn on(self, q: Point) -> bool {
        match q {
            None => true,
            Some((x, y)) => {
                if self.binary {
                    bm(y, y) ^ bm(x, y) == bm(bm(x, x), x) ^ bm(self.a, bm(x, x)) ^ self.b
                } else {
                    pm(y, y, self.p)
                        == (pm(pm(x, x, self.p), x, self.p) + pm(self.a, x, self.p) + self.b)
                            % self.p
                }
            }
        }
    }
    fn lift(self, x: u128) -> Point {
        if self.binary {
            if x == 0 {
                return Some((0, bp(self.b, 1u128 << 82)));
            }
            let c = x ^ self.a ^ bm(self.b, bm(bi(x), bi(x)));
            let (mut z, mut t) = (c, c);
            for _ in 1..42 {
                t = bm(bm(t, t), bm(t, t));
                z ^= t;
            }
            if bm(z, z) ^ z != c {
                return None;
            }
            Some((x, bm(x, z)))
        } else {
            sqrt(
                (pm(pm(x, x, self.p), x, self.p) + pm(self.a, x, self.p) + self.b) % self.p,
                self.p,
            )
            .map(|y| (x, y))
        }
    }
    fn add(self, q: Point, r: Point) -> Point {
        let (Some((x, y)), Some((u, v))) = (q, r) else {
            return q.or(r);
        };
        let out = if self.binary {
            if x == u {
                if y != v || x == 0 {
                    return None;
                }
                let l = x ^ bm(y, bi(x));
                let xx = bm(l, l) ^ l ^ self.a;
                Some((xx, bm(x, x) ^ bm(l ^ 1, xx)))
            } else {
                let l = bm(y ^ v, bi(x ^ u));
                let xx = bm(l, l) ^ l ^ x ^ u ^ self.a;
                Some((xx, bm(l, x ^ xx) ^ xx ^ y))
            }
        } else {
            let p = self.p;
            if x == u && (y != v || y == 0) {
                return None;
            }
            let l = if x == u {
                pm((3 * pm(x, x, p) + self.a) % p, pp((2 * y) % p, p - 2, p), p)
            } else {
                pm((v + p - y) % p, pp((u + p - x) % p, p - 2, p), p)
            };
            let xx = (pm(l, l, p) + 2 * p - x - u) % p;
            Some((xx, (pm(l, (x + p - xx) % p, p) + p - y) % p))
        };
        assert!(self.on(out));
        out
    }
}
fn padd(a: &mut Poly, b: Poly) {
    for (m, c) in b {
        let e = a.entry(m).or_default();
        *e ^= c;
    }
    a.retain(|_, c| *c != 0);
}
fn psq(a: &Poly) -> Poly {
    a.iter().map(|(m, c)| (m.clone(), bm(*c, *c))).collect()
}
fn pmul(a: &Poly, b: &Poly) -> Poly {
    let mut r = Poly::new();
    for (m, c) in a {
        for (n, d) in b {
            let mut k = m.clone();
            k.extend(n);
            k.sort_unstable();
            k.dedup();
            *r.entry(k).or_default() ^= bm(*c, *d);
        }
    }
    r.retain(|_, c| *c != 0);
    r
}
fn bs3(x: &Poly, y: &Poly, z: &Poly, b: u128) -> Poly {
    let mut r = pmul(&psq(x), &psq(y));
    padd(&mut r, pmul(&psq(x), &psq(z)));
    padd(&mut r, pmul(&psq(y), &psq(z)));
    padd(&mut r, pmul(&pmul(x, y), z));
    padd(&mut r, [(vec![], b)].into());
    r
}
fn linear(start: usize, width: usize) -> Poly {
    (0..width).map(|i| (vec![start + i], 1u128 << i)).collect()
}
fn peval(p: &Poly, bits: &[u128]) -> u128 {
    p.iter()
        .filter(|(m, _)| m.iter().all(|i| bits[*i] == 1))
        .fold(0, |a, (_, c)| a ^ c)
}
fn s3text(x: &str, y: &str, z: &str, a: u128, b: u128) -> String {
    format!("(({x})-({y}))^2*({z})^2-2*((({x})+({y}))*(({x})*({y})+{a})+2*{b})*({z})+(({x})*({y})-{a})^2-4*{b}*(({x})+({y}))")
}
fn basis_bit(j: usize, coordinate_bit: Option<usize>) -> usize {
    if j == 26 {
        coordinate_bit.unwrap_or(26)
    } else {
        j
    }
}
fn base_linear(start: usize, coordinate_bit: Option<usize>) -> Poly {
    (0..27)
        .map(|j| (vec![start + j], 1u128 << basis_bit(j, coordinate_bit)))
        .collect()
}
fn selector(x: u128, coordinate_bit: Option<usize>) -> usize {
    let i = if let Some(bit) = coordinate_bit {
        (x & ((1 << 26) - 1)) | ((x >> bit) << 26)
    } else {
        x
    };
    assert!(i < (1 << 27));
    let decoded = if let Some(bit) = coordinate_bit {
        (i & ((1 << 26) - 1)) | ((i >> 26) << bit)
    } else {
        i
    };
    assert_eq!(x, decoded, "x outside base domain");
    i as usize
}
fn base_record(raw: &[u8], pt: (u128, u128), coordinate_bit: Option<usize>) -> bool {
    assert!(raw.len() >= 32);
    let spike = &raw[..8] == b"N83FB02\n";
    assert!(spike || &raw[..8] == b"N83FB01\n");
    if spike {
        assert_eq!(
            raw[10] as usize,
            coordinate_bit.expect("candidate shard coordinate bit")
        );
        assert!(raw[11..16].iter().all(|b| *b == 0));
    } else {
        assert!(
            coordinate_bit.is_none(),
            "candidate bit cannot use baseline shard"
        );
    }
    assert_eq!((raw.len() - 32) % 20, 0);
    let read = |i: usize| {
        let offset = 32 + i * 20;
        (
            {
                let i = u32::from_le_bytes(raw[offset..offset + 4].try_into().unwrap()) as u128;
                if spike {
                    let bit = coordinate_bit.unwrap();
                    (i & ((1 << 26) - 1)) | ((i >> 26) << bit)
                } else {
                    i
                }
            },
            u128::from_le_bytes(raw[offset + 4..offset + 20].try_into().unwrap()),
        )
    };
    let (mut lo, mut hi) = (0, (raw.len() - 32) / 20);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if read(mid) < pt {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo < (raw.len() - 32) / 20 && read(lo) == pt
}
fn generate(root: &Path, bases: &Path, matched: &Path, arm: &str) -> Vec<Value> {
    let coordinate_bit = if arm == "baseline" {
        None
    } else {
        Some(
            arm.strip_prefix("spike")
                .and_then(|s| s.parse::<usize>().ok())
                .filter(|bit| (27..=40).contains(bit))
                .expect("candidate arm must be spike27 through spike40"),
        )
    };
    let spike = coordinate_bit.is_some();
    assert!(!root.exists(), "fresh output required");
    let matched_bytes = fs::read(matched.join("inputs.json")).unwrap();
    assert_eq!(
        hash(&matched_bytes),
        "f7325211302b4ca21f98ea9a7022d2dcc023b29ea2fad9329867e90b75a01c82"
    );
    let prior: Vec<Value> = serde_json::from_slice(&matched_bytes).unwrap();
    fs::create_dir_all(root).unwrap();
    let index_bytes = fs::read(bases.join("index.json")).unwrap();
    let index: Value = serde_json::from_slice(&index_bytes).unwrap();
    assert_eq!(index["complete"], true);
    assert_eq!(index["ell"], 27);
    assert_eq!(index["candidate_x_count_per_curve"], 1u64 << 27);
    if let Some(bit) = coordinate_bit {
        assert_eq!(index["candidate_id"], arm);
        assert_eq!(index["coordinate_bit_positions"][26], bit);
    }
    let mut manifests = BTreeMap::new();
    for (family, baseline_expected) in [
        (
            "koblitz",
            "31343c5145d44695e2388020501eb009a34bf3d7ebdafe0f6566d637ce30f67b",
        ),
        (
            "binary",
            "1d974ee32f633c4b2c220e28f550cc93afecb56aa836a36cc8d8e5962d7f2c6d",
        ),
        (
            "prime",
            "199d1b214c78650b2e2e2b539f458eae2688296b9bc51985830951b1cac5aaa5",
        ),
    ] {
        let entry = index["families"]
            .as_array()
            .unwrap()
            .iter()
            .find(|e| e["family"] == family)
            .unwrap();
        let expected = match coordinate_bit {
            Some(40) => match family {
                "koblitz" => "95057db18ee3365b1927e84f43a62f5b7cba4dd4859654bc50e606caa62b4ca6",
                "binary" => "a9b7d2b4de575095b3a512d53da60c8e4a3e3d6e46c0799346d53aa9158f5bbe",
                _ => "78f21fdb94cd84163c8054258f9a4de914df8f82c24897555cb4c15ffe5b4324",
            },
            None => baseline_expected,
            Some(_) => entry["manifest_sha256"].as_str().unwrap(),
        };
        let bytes = fs::read(bases.join(family).join("manifest.json")).unwrap();
        assert_eq!(hash(&bytes), expected);
        assert_eq!(entry["manifest_sha256"], expected);
        manifests.insert(
            family,
            (serde_json::from_slice::<Value>(&bytes).unwrap(), expected),
        );
    }
    fs::write(root.join("factor-base-index.json"), index_bytes).unwrap();
    let (p, cert) = prime();
    save(
        &root.join("prime-certificate.json"),
        &json!({"p":p.to_string(),"c":((p-1)>>42).to_string(),"k":42,"witness":cert.to_string(),"check":pp(cert,(p-1)/2,p).to_string(),"theorem":"Proth"}),
    );
    let curves = [
        (
            "koblitz",
            Curve {
                binary: true,
                a: 0,
                b: 1,
                p: 0,
            },
        ),
        (
            "binary",
            Curve {
                binary: true,
                a: 1,
                b: (1u128 << 82) | 0x51b,
                p: 0,
            },
        ),
        (
            "prime",
            Curve {
                binary: false,
                a: 2,
                b: 3,
                p,
            },
        ),
    ];
    let mut jobs = Vec::new();
    for m in 2..=9 {
        let ell = 27;
        for (family, curve) in curves {
            let mut rng = SEED
                ^ (m as u64) << 32
                ^ match family {
                    "koblitz" => 1,
                    "binary" => 2,
                    _ => 3,
                };
            let (mut pts, mut sums, mut q) = (Vec::new(), Vec::new(), None);
            while pts.len() < m {
                let x = draw(&mut rng) & ((1u128 << 26) - 1);
                if let Some(pt) = curve.lift(x) {
                    let r = curve.add(q, Some(pt));
                    if r.is_none() {
                        continue;
                    }
                    pts.push(pt);
                    q = r;
                    sums.push(r.unwrap());
                }
            }
            let plant = q.unwrap();
            let (manifest, manifest_hash) = &manifests[family];
            let mut membership = Vec::new();
            for &pt in &pts {
                let part = selector(pt.0, coordinate_bit) >> 20;
                let cert = &manifest["shards"][part];
                let bytes =
                    fs::read(bases.join(family).join(cert["file"].as_str().unwrap())).unwrap();
                assert_eq!(hash(&bytes), cert["sha256"].as_str().unwrap());
                let raw = zstd::stream::decode_all(bytes.as_slice()).unwrap();
                assert_eq!(
                    raw[8],
                    match family {
                        "koblitz" => 0,
                        "binary" => 1,
                        _ => 2,
                    }
                );
                assert_eq!(raw[9], 27);
                assert_eq!(&raw[..8], if spike { b"N83FB02\n" } else { b"N83FB01\n" });
                assert!(
                    base_record(&raw, pt, coordinate_bit),
                    "planted point absent from saved factor base"
                );
                membership.push(json!({"x":pt.0.to_string(),"y":pt.1.to_string(),"shard":cert["file"],"sha256":cert["sha256"],"present":true}));
            }
            let prior_natural = prior
                .iter()
                .find(|r| r["family"] == family && r["m"] == m && r["target_kind"] == "natural")
                .unwrap();
            let natural = (
                prior_natural["target_x"]
                    .as_str()
                    .unwrap()
                    .parse::<u128>()
                    .unwrap(),
                prior_natural["target_y"]
                    .as_str()
                    .unwrap()
                    .parse::<u128>()
                    .unwrap(),
            );
            for (kind, target) in [("planted", plant), ("natural", natural)] {
                assert!(curve.on(Some(target)));
                let nv = m * ell + (m - 2) * if curve.binary { N } else { 1 };
                let id = format!("{family}-m{m}-{kind}");
                let dir = root.join(&id);
                fs::create_dir(&dir).unwrap();
                let mut witness = Vec::new();
                for (x, _) in &pts {
                    for j in 0..ell {
                        witness.push((x >> basis_bit(j, coordinate_bit)) & 1)
                    }
                }
                for s in sums.iter().take(m - 1).skip(1) {
                    if curve.binary {
                        for j in 0..N {
                            witness.push((s.0 >> j) & 1)
                        }
                    } else {
                        witness.push(s.0)
                    }
                }
                assert_eq!(witness.len(), nv);
                let mut f = BufWriter::new(File::create(dir.join("system.sing")).unwrap());
                writeln!(
                    f,
                    "ring r={},(v(1..{})),dp;",
                    if curve.binary {
                        "2".into()
                    } else {
                        format!("(integer,{p})")
                    },
                    nv
                )
                .unwrap();
                writeln!(f, "ideal I;").unwrap();
                let mut eqs = 0;
                let mut terms = 0;
                if curve.binary {
                    let x: Vec<Poly> = (0..m)
                        .map(|i| base_linear(i * ell, coordinate_bit))
                        .collect();
                    let u: Vec<Poly> = (0..m - 2).map(|i| linear(m * ell + i * N, N)).collect();
                    let t: Poly = [(vec![], target.0)].into();
                    for k in 0..m - 1 {
                        let (lhs, rhs, out) = if m == 2 {
                            (&x[0], &x[1], &t)
                        } else if k == 0 {
                            (&x[0], &x[1], &u[0])
                        } else if k == m - 2 {
                            (&u[k - 1], &x[k + 1], &t)
                        } else {
                            (&u[k - 1], &x[k + 1], &u[k])
                        };
                        let poly = bs3(lhs, rhs, out, curve.b);
                        if kind == "planted" {
                            assert_eq!(peval(&poly, &witness), 0, "planted S3");
                        }
                        for bit in 0..N {
                            let row: Vec<_> =
                                poly.iter().filter(|(_, c)| (*c >> bit) & 1 != 0).collect();
                            if row.is_empty() {
                                continue;
                            }
                            eqs += 1;
                            terms += row.len();
                            write!(f, "I[{}]=", eqs).unwrap();
                            for (i, (mo, _)) in row.iter().enumerate() {
                                if i > 0 {
                                    write!(f, "+").unwrap()
                                }
                                if mo.is_empty() {
                                    write!(f, "1").unwrap()
                                } else {
                                    for (j, v) in mo.iter().enumerate() {
                                        if j > 0 {
                                            write!(f, "*").unwrap()
                                        }
                                        write!(f, "v({})", v + 1).unwrap()
                                    }
                                }
                            }
                            writeln!(f, ";").unwrap();
                        }
                    }
                } else {
                    for i in 0..m {
                        let expr = (0..ell)
                            .map(|j| {
                                format!(
                                    "{}*v({})",
                                    1u128 << basis_bit(j, coordinate_bit),
                                    i * ell + j + 1
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("+");
                        writeln!(f, "poly x{i}={expr};").unwrap();
                    }
                    for k in 0..m - 1 {
                        let (lhs, rhs, out) = if m == 2 {
                            ("x0".into(), "x1".into(), target.0.to_string())
                        } else if k == 0 {
                            ("x0".into(), "x1".into(), format!("v({})", m * ell + 1))
                        } else if k == m - 2 {
                            (
                                format!("v({})", m * ell + k),
                                format!("x{}", k + 1),
                                target.0.to_string(),
                            )
                        } else {
                            (
                                format!("v({})", m * ell + k),
                                format!("x{}", k + 1),
                                format!("v({})", m * ell + k + 1),
                            )
                        };
                        eqs += 1;
                        writeln!(
                            f,
                            "I[{eqs}]={};",
                            s3text(&lhs, &rhs, &out, curve.a, curve.b)
                        )
                        .unwrap();
                    }
                }
                let nbits = if curve.binary { nv } else { m * ell };
                for i in 1..=nbits {
                    eqs += 1;
                    writeln!(f, "I[{eqs}]=v({i})^2-v({i});").unwrap();
                }
                if kind == "planted" {
                    writeln!(
                        f,
                        "map witness=r,{};",
                        witness
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(",")
                    )
                    .unwrap();
                    writeln!(f,"if(size(witness(I))!=0){{print(\"WITNESS_FAILED\");exit;}} print(\"WITNESS_OK\");").unwrap();
                }
                f.flush().unwrap();
                drop(f);
                let raw = fs::read(dir.join("system.sing")).unwrap();
                if !spike && kind == "natural" {
                    assert_eq!(
                        hash(&raw),
                        prior_natural["source_sha256"].as_str().unwrap(),
                        "baseline natural system changed"
                    );
                }
                let row = json!({"id":id,"family":family,"n":N,"m":m,"ell":ell,"candidate_x_count":(1u128<<ell).to_string(),"target_kind":kind,"target_x":target.0.to_string(),"target_y":target.1.to_string(),"n_vars":nv,"input_equations":eqs,"binary_terms":if curve.binary{Some(terms)}else{None},"field_modulus":if curve.binary{format!("z^83+z^45+z^2+z+1")}else{p.to_string()},"curve_a":curve.a.to_string(),"curve_b":curve.b.to_string(),"source_sha256":hash(&raw),"points":pts.iter().map(|(x,y)|json!([x.to_string(),y.to_string()])).collect::<Vec<_>>(),"witness":if kind=="planted"{Some(witness.iter().map(ToString::to_string).collect::<Vec<_>>())}else{None},"subgroup_order":if family=="koblitz"{Some("2417851639230796216685689")}else{None},"membership":"candidate abscissas; roots require curve lift and sign verification"});
                let mut row = row;
                row["arm"] = json!(arm);
                row["candidate_id"] = json!(arm);
                row["coordinate_bit_positions"] = json!((0..27)
                    .map(|j| basis_bit(j, coordinate_bit))
                    .collect::<Vec<_>>());
                row["target_distribution"] = json!(if kind == "planted" {
                    "common-support-planted: same low26 tuple and target in both arms; not uniform over full domain"
                } else {
                    "same saved original natural target in both arms"
                });
                row["matched_original_inputs_sha256"] = json!(hash(&matched_bytes));
                row["factor_base_manifest_sha256"] = json!(manifest_hash);
                row["factor_base_membership"] = json!(membership);
                save(&dir.join("input.json"), &row);
                jobs.push(row);
            }
        }
    }
    save(&root.join("inputs.json"), &json!(jobs));
    jobs
}
fn solver_script(mode: &str, planted: bool) -> String {
    if mode == "top" {
        return r#"
ideal T; int j; poly f;
for(j=1;j<=size(I);j++){ f=I[j]; if(f!=0){ T[j]=jet(f,deg(f))-jet(f,deg(f)-1); }}
print("SOLVER_BEGIN"); option(prot); ideal G=std(T); option(noprot); print("SOLVER_END");
int td=dim(G); print("TOP_DIM="+string(td));
if(td==-1){print("DREG_TOP=0");}
if(td==0){
  ideal LM; for(j=1;j<=size(G);j++){LM[j]=leadmonom(G[j]);}
  string lmtext=string(LM); int nv=nvars(basering);
  ring hr=0,(v(1..nv)),dp; execute("ideal H="+lmtext+";"); attrib(H,"isSB",1);
  intvec hv=hilb(H,2); int d=size(hv); while(d>0 && hv[d]==0){d=d-1;}
  print("DREG_TOP="+string(d)); print("HILBERT="+string(hv));
}
print("COMPLETE"); quit;
"#
        .into();
    }
    format!(
        r#"
print("SOLVER_BEGIN"); option(prot); ideal G=std(I); option(noprot); print("SOLVER_END");
print("BASIS_MAX_DEGREE="+string(deg(G))); print("BASIS_SIZE="+string(size(G)));
print("INPUT_REDUCES="+string(size(reduce(I,G))==0));
{}
write(":w basis.txt",G); print("COMPLETE"); quit;
"#,
        if planted {
            "print(\"BASIS_WITNESS_OK=\"+string(size(witness(G))==0));"
        } else {
            ""
        }
    )
}
fn marker(log: &str, prefix: &str) -> Option<i64> {
    log.lines()
        .find_map(|l| l.strip_prefix(prefix).and_then(|s| s.trim().parse().ok()))
}
fn execute(root: &Path, input: &Value, mode: &str, seconds: u64) -> Value {
    let id = input["id"].as_str().unwrap();
    let dir = root.join(id);
    let work = dir.join(mode);
    fs::create_dir(&work).unwrap();
    let mut script = fs::read_to_string(dir.join("system.sing")).unwrap();
    script.push_str(&solver_script(mode, input["target_kind"] == "planted"));
    fs::write(work.join("run.sing"), script).unwrap();
    let out = File::create(work.join("stdout.log")).unwrap();
    let err = File::create(work.join("stderr.log")).unwrap();
    let start = Instant::now();
    let status = Command::new("timeout")
        .args([
            "--signal=TERM",
            "--kill-after=5",
            &seconds.to_string(),
            "Singular",
            "-q",
            "run.sing",
        ])
        .current_dir(&work)
        .stdout(Stdio::from(out))
        .stderr(Stdio::from(err))
        .status()
        .unwrap();
    let log = fs::read_to_string(work.join("stdout.log")).unwrap();
    let error =
        log.lines().any(|l| l.trim_start().starts_with('?')) || log.contains("WITNESS_FAILED");
    let verified = mode == "top"
        || (marker(&log, "INPUT_REDUCES=") == Some(1)
            && (input["target_kind"] != "planted" || marker(&log, "BASIS_WITNESS_OK=") == Some(1)));
    let complete = status.success() && log.lines().any(|l| l == "COMPLETE") && !error && verified;
    let row = json!({"input":input,"mode":mode,"status":if error{"solver_error"}else if complete{"complete"}else if status.code()==Some(124)||status.code()==Some(137){"resource_limit"}else{"incomplete"},"exit_code":status.code(),"diagnostic_wall_seconds":start.elapsed().as_secs_f64(),"wall_valid":false,"degree_of_regularity_top":if complete{marker(&log,"DREG_TOP=")}else{None},"top_dimension":if complete{marker(&log,"TOP_DIM=")}else{None},"basis_max_degree":if complete{marker(&log,"BASIS_MAX_DEGREE=")}else{None},"basis_size":if complete{marker(&log,"BASIS_SIZE=")}else{None},"solving_degree":Value::Null,"log_sha256":hash(log.as_bytes()),"interpretation":"Top-form regularity, final basis degree and raw solver trace are distinct. No final dreg or solving degree is inferred from timeout."});
    save(&work.join("result.json"), &row);
    row
}
fn selfcheck(root: &Path) {
    fs::create_dir_all(root).unwrap();
    let (p, _) = prime();
    for (name, coeff) in [
        ("binary", "2".to_string()),
        ("prime", format!("(integer,{p})")),
    ] {
        for (kind, ideal, expected) in [
            ("positive", "v(1)^2-v(1),v(2)^2-v(2)", 3),
            ("negative", "v(1),v(1)-1,v(2)^2-v(2)", 2),
        ] {
            let id = format!("control-{name}-{kind}");
            let dir = root.join(&id);
            fs::create_dir(&dir).unwrap();
            fs::write(
                dir.join("system.sing"),
                format!("ring r={coeff},(v(1..2)),dp; ideal I={ideal};\n"),
            )
            .unwrap();
            let input = json!({"id":id,"target_kind":"control"});
            for mode in ["top", "affine"] {
                let row = execute(root, &input, mode, 20);
                assert_eq!(row["status"], "complete", "{row}");
                if mode == "top" {
                    assert_eq!(row["degree_of_regularity_top"], expected, "{row}");
                } else if kind == "negative" {
                    assert_eq!(row["basis_max_degree"], 0, "{row}");
                }
                println!("{}", row);
            }
        }
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("selfcheck") => selfcheck(Path::new(&args[2])),
        Some("generate") => {
            generate(Path::new(&args[2]), Path::new(&args[3]), Path::new(&args[4]), &args[5]);
        }
        Some("run") => {
            let root = PathBuf::from(&args[2]);
            let inputs: Vec<Value> =
                serde_json::from_slice(&fs::read(root.join("inputs.json")).unwrap()).unwrap();
            let seconds = args[3].parse().unwrap();
            let shard: usize = args[4].parse().unwrap();
            let shards: usize = args[5].parse().unwrap();
            for (i, input) in inputs.iter().enumerate() {
                if i % shards != shard {
                    continue;
                }
                for mode in ["top", "affine"] {
                    let row = execute(&root, input, mode, seconds);
                    println!("{}", row);
                    std::io::stdout().flush().unwrap();
                }
                if let Ok(uri) = std::env::var("ISOLAB_RESULTS_URI") {
                    let id = input["id"].as_str().unwrap();
                    let uploaded = Command::new("aws")
                        .args(["s3", "sync"])
                        .arg(root.join(id))
                        .arg(format!("{}/inputs/{id}", uri.trim_end_matches('/')))
                        .arg("--only-show-errors")
                        .status()
                        .unwrap();
                    assert!(
                        uploaded.success(),
                        "failed to preserve completed target {id}"
                    );
                }
            }
        }
        Some("summary") => {
            let root = Path::new(&args[2]);
            let inputs: Vec<Value> =
                serde_json::from_slice(&fs::read(root.join("inputs.json")).unwrap()).unwrap();
            let mut rows = Vec::new();
            for input in inputs {
                for mode in ["top", "affine"] {
                    let p = root
                        .join(input["id"].as_str().unwrap())
                        .join(mode)
                        .join("result.json");
                    if p.exists() {
                        rows.push(serde_json::from_slice::<Value>(&fs::read(p).unwrap()).unwrap());
                    }
                }
            }
            save(&root.join("results.json"), &json!(rows));
        }
        _ => panic!("generate OUT BASES MATCHED_INPUTS baseline|spike27..spike40 | run OUT SECONDS SHARD SHARDS | summary OUT"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn saved_base_lookup_checks_both_coordinates() {
        let mut raw = vec![0; 32];
        raw[..8].copy_from_slice(b"N83FB01\n");
        for (x, y) in [(0u32, 1u128), (3, 6), (3, 99), (9, 17)] {
            raw.extend(x.to_le_bytes());
            raw.extend(y.to_le_bytes());
        }
        for pt in [(0, 1), (3, 6), (3, 99), (9, 17)] {
            assert!(base_record(&raw, pt, None));
        }
        for pt in [(0, 0), (1, 1), (3, 7), (9, 18), (10, 1)] {
            assert!(!base_record(&raw, pt, None));
        }
    }
    #[test]
    fn binary_arithmetic() {
        for x in [1, 2, 3, 17, MASK, 1 << 82] {
            assert_eq!(bm(x, bi(x)), 1);
            assert_eq!(bp(x, 1u128 << 83), x);
        }
    }
    #[test]
    fn proth_and_prime_arithmetic() {
        let (p, a) = prime();
        assert_eq!(128 - p.leading_zeros(), 83);
        assert_eq!(pp(a, (p - 1) / 2, p), p - 1);
        assert!(((p - 1) >> 42) < 1 << 42);
        for x in [1, 2, 17, p - 2] {
            assert_eq!(pm(x, pp(x, p - 2, p), p), 1);
            let y = sqrt(pm(x, x, p), p).unwrap();
            assert_eq!(pm(y, y, p), pm(x, x, p));
        }
    }
    #[test]
    fn curves_and_semaev() {
        let (p, _) = prime();
        for c in [
            Curve {
                binary: true,
                a: 0,
                b: 1,
                p: 0,
            },
            Curve {
                binary: true,
                a: 1,
                b: (1 << 82) | 0x51b,
                p: 0,
            },
            Curve {
                binary: false,
                a: 2,
                b: 3,
                p,
            },
        ] {
            let pts: Vec<_> = (1..100).filter_map(|x| c.lift(x)).take(4).collect();
            assert_eq!(pts.len(), 4);
            for q in &pts {
                assert!(c.on(Some(*q)));
            }
            let r = c.add(Some(pts[0]), Some(pts[1])).unwrap();
            if c.binary {
                let cp = |x| [(vec![], x)].into();
                assert!(bs3(&cp(pts[0].0), &cp(pts[1].0), &cp(r.0), c.b).is_empty());
            }
        }
    }
    #[test]
    fn symbolic_evaluation() {
        let p = bs3(&linear(0, 3), &linear(3, 3), &[(vec![], 17)].into(), 1);
        for mask in 0..64 {
            let bits: Vec<_> = (0..6).map(|i| (mask >> i) & 1).collect();
            let x = mask & 7;
            let y = mask >> 3;
            let expected = bm(bm(x, x), bm(y, y))
                ^ bm(bm(x, x), bm(17, 17))
                ^ bm(bm(y, y), bm(17, 17))
                ^ bm(bm(x, y), 17)
                ^ 1;
            assert_eq!(peval(&p, &bits), expected);
        }
    }
}

#[cfg(test)]
mod candidate_tests {
    use super::*;
    #[test]
    fn coordinate_map_and_lift_lookup() {
        for bit in 27..=40 {
            for i in [0u128, 7, (1 << 26) - 1, 1 << 26, (1 << 27) - 1] {
                let x = (i & ((1 << 26) - 1)) | ((i >> 26) << bit);
                assert_eq!(selector(x, Some(bit)) as u128, i);
                let mut raw = vec![0; 32];
                raw[..8].copy_from_slice(b"N83FB02\n");
                raw[10] = bit as u8;
                raw.extend((i as u32).to_le_bytes());
                raw.extend(19u128.to_le_bytes());
                assert!(base_record(&raw, (x, 19), Some(bit)));
                assert!(!base_record(&raw, (x, 18), Some(bit)));
                let bits: Vec<_> = (0..27).map(|j| (i >> j) & 1).collect();
                assert_eq!(peval(&base_linear(0, Some(bit)), &bits), x);
            }
        }
        assert!(std::panic::catch_unwind(|| selector(1 << 26, Some(40))).is_err());
    }
}
