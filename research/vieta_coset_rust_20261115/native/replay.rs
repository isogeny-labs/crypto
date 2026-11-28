//! Separate-process replay: wide scalar subtraction and scalar pair-table
//! enumeration, independent of Vieta elimination. Cache only identical inputs.
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use vieta_coset_lab::vieta_coset::*;

type E = BigUint;
type PairTable = HashMap<Point<E>, Vec<Relation<E>>>;
fn integer(s: &Value) -> E {
    E::parse_bytes(s.as_str().expect("hex string").as_bytes(), 16).expect("valid hex")
}
fn point(p: &(E, E)) -> Value {
    json!([p.0.to_str_radix(16), p.1.to_str_radix(16)])
}
fn hash(r: &[Relation<E>]) -> String {
    let mut h = Sha256::new();
    for (p, q) in r {
        for x in [&p.0, &p.1, &q.0, &q.1] {
            let s = x.to_str_radix(16);
            h.update((s.len() as u64).to_le_bytes());
            h.update(s.as_bytes());
        }
    }
    format!("{:x}", h.finalize())
}
struct Replay {
    scope: Value,
    field: WideField,
    curve: Curve<E>,
    base: Vec<(E, E)>,
    pairs: Option<PairTable>,
    answers: HashMap<String, (usize, String)>,
}
impl Replay {
    fn new(scope: &Value) -> Self {
        let field = WideField(integer(&scope["p_hex"]));
        let a = Arithmetic::new(field.clone(), Limits::default()).unwrap();
        let curve = Curve {
            a: integer(&scope["a_hex"]),
            b: integer(&scope["b_hex"]),
        };
        let coset = a
            .coset(
                scope["m"].as_u64().unwrap() as usize,
                integer(&scope["c_hex"]),
            )
            .unwrap();
        let base = a.factor_base(&curve, &coset).unwrap();
        assert_eq!(
            base.len() as u64,
            scope["base_points"].as_u64().unwrap(),
            "base size mismatch"
        );
        assert_eq!(
            hash(
                &base
                    .iter()
                    .map(|p| (p.clone(), p.clone()))
                    .collect::<Vec<_>>()
            ),
            scope["base_sha256"].as_str().unwrap(),
            "base hash mismatch"
        );
        Self {
            scope: scope.clone(),
            field,
            curve,
            base,
            pairs: None,
            answers: HashMap::new(),
        }
    }
    fn answer(&mut self, kind: &str, t: &Value) -> &(usize, String) {
        let key = format!("{kind}:{t}");
        if !self.answers.contains_key(&key) {
            let a = Arithmetic::new(self.field.clone(), Limits::default()).unwrap();
            let target = if t.is_null() {
                None
            } else {
                Some((integer(&t[0]), integer(&t[1])))
            };
            assert!(a.on_curve(&self.curve, &target), "invalid target");
            let result = if kind == "query" {
                let r = a.lookup_scalar(&self.curve, &self.base, &target).unwrap();
                (r.len(), hash(&r))
            } else {
                if self.pairs.is_none() {
                    assert!(self.base.len() <= 512, "three-term replay memory bound");
                    let mut table: PairTable = HashMap::new();
                    for (i, p) in self.base.iter().enumerate() {
                        for q in &self.base[i..] {
                            let sum = a
                                .group_add(&self.curve, &Some(p.clone()), &Some(q.clone()))
                                .unwrap();
                            let bucket = table.entry(sum).or_default();
                            bucket.push((p.clone(), q.clone()));
                            if p != q {
                                bucket.push((q.clone(), p.clone()));
                            }
                        }
                    }
                    self.pairs = Some(table);
                }
                let mut triples = BTreeSet::new();
                for p in &self.base {
                    let r = a
                        .group_add(&self.curve, &target, &a.negate(&Some(p.clone())))
                        .unwrap();
                    for (q, s) in self.pairs.as_ref().unwrap().get(&r).into_iter().flatten() {
                        triples.insert((p.clone(), q.clone(), s.clone()));
                    }
                }
                let rows: Vec<_> = triples
                    .iter()
                    .map(|(p, q, r)| json!([point(p), point(q), point(r)]))
                    .collect();
                (
                    rows.len(),
                    format!("{:x}", Sha256::digest(serde_json::to_vec(&rows).unwrap())),
                )
            };
            self.answers.insert(key.clone(), result);
        }
        &self.answers[&key]
    }
}
fn main() {
    let files: Vec<_> = std::env::args().skip(1).collect();
    assert!(
        !files.is_empty(),
        "usage: vieta-coset-replay records.jsonl [...]"
    );
    let (mut two, mut three, mut failed) = (0, 0, 0);
    for path in files {
        let mut replay: Option<Replay> = None;
        for line in BufReader::new(File::open(&path).expect("open record")).lines() {
            let row: Value = serde_json::from_str(&line.unwrap()).expect("JSON row");
            let kind = row["kind"].as_str().unwrap();
            if !["query", "three-query"].contains(&kind) {
                continue;
            }
            if row["status"] != "ok" {
                failed += 1;
                continue;
            }
            if replay.as_ref().is_none_or(|r| r.scope != row["scope"]) {
                replay = Some(Replay::new(&row["scope"]));
            }
            let (count, digest) = replay.as_mut().unwrap().answer(kind, &row["target"]);
            assert_eq!(
                *count as u64,
                row["relations"].as_u64().unwrap(),
                "relation count mismatch"
            );
            assert_eq!(
                digest,
                row["result_sha256"].as_str().unwrap(),
                "relation hash mismatch"
            );
            if kind == "query" {
                two += 1;
            } else {
                three += 1;
            }
        }
    }
    assert!(two + three > 0, "no successful query rows to replay");
    println!(
        "{}",
        json!({"schema":"vieta-coset-replay/v2","checked":two,"three_checked":three,"preserved_failed_rows":failed,"algorithm":"wide scalar subtraction and scalar pair-table enumeration","standing":"producer check; independent signed PCC verdict required"})
    );
    if failed > 0 {
        std::process::exit(1);
    }
}
