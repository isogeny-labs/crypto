//! Sound exact known-valuation propagation; unknowns never receive guessed values.
use crate::{exact16::Q, round04::read};
use serde_json::{json, Value};
use std::{collections::BTreeSet, path::Path};
#[derive(Clone, Debug, PartialEq, Eq)]
enum V {
    Inf,
    R(Q),
}
impl V {
    fn j(&self) -> Value {
        match self {
            Self::Inf => json!("infinity"),
            Self::R(q) => json!({"numerator":q.n.to_string(),"denominator":q.d.to_string()}),
        }
    }
    fn less(&self, b: &Self) -> bool {
        match (self, b) {
            (Self::R(a), Self::R(b)) => a.cmp(b).is_lt(),
            (Self::R(_), Self::Inf) => true,
            _ => false,
        }
    }
}
#[derive(Clone)]
struct Known {
    v: V,
    id: usize,
}
struct State {
    known: Vec<Option<Known>>,
    proof: Vec<Value>,
    bad: Option<Value>,
}
impl State {
    fn put(&mut self, node: usize, v: V, why: Value) {
        if self.bad.is_some() {
            return;
        }
        if let Some(old) = &self.known[node] {
            if old.v != v {
                self.bad = Some(
                    json!({"kind":"inconsistent exact valuation","node":node,"old_assignment":old.id,"new_value":v.j(),"new_rule":why}),
                );
            }
            return;
        }
        let id = self.proof.len();
        self.proof
            .push(json!({"id":id,"node":node,"value":v.j(),"rule":why}));
        self.known[node] = Some(Known { v: v.clone(), id });
        if let V::R(q) = v {
            if q.cmp(&Q::z(0)).is_lt() {
                self.bad = Some(
                    json!({"kind":"negative valuation for a regular coordinate","node":node,"assignment":id}),
                );
            }
        }
    }
    fn rule(&mut self, node: usize, v: V, gate: usize, kind: &str, parents: &[usize]) {
        let ids = parents
            .iter()
            .map(|p| self.known[*p].as_ref().unwrap().id)
            .collect::<Vec<_>>();
        self.put(node, v, json!({"gate":gate,"kind":kind,"parents":ids}));
    }
}
fn dependencies(v: &Value, proof: &[Value], ids: &mut BTreeSet<usize>) {
    match v {
        Value::Object(m) => {
            for (k, x) in m {
                if k == "assignment" || k == "old_assignment" {
                    let id = x.as_u64().unwrap() as usize;
                    if ids.insert(id) {
                        dependencies(&proof[id], proof, ids);
                    }
                } else if k == "parents" {
                    for i in x.as_array().unwrap() {
                        let id = i.as_u64().unwrap() as usize;
                        if ids.insert(id) {
                            dependencies(&proof[id], proof, ids);
                        }
                    }
                } else {
                    dependencies(x, proof, ids);
                }
            }
        }
        Value::Array(v) => {
            for x in v {
                dependencies(x, proof, ids)
            }
        }
        _ => {}
    }
}
pub fn run(dir: &Path) {
    let all = read(&dir.join("q17-expanded-product-quadrics.json"));
    let mut records = vec![];
    for g in 0..4 {
        let raw = read(&dir.join(format!("q8-graph-{g}-F2-relations.json")));
        let p = &raw["program"];
        let nodes = p["nodes"].as_array().unwrap();
        let model = all["records"][g]["after_all_Q17"]["model"]
            .as_array()
            .unwrap();
        let mut st = State {
            known: vec![None; nodes.len()],
            proof: vec![],
            bad: None,
        };
        for (i, m) in model.iter().enumerate() {
            if m == 1 {
                st.put(
                    i,
                    V::R(Q::z(0)),
                    json!({"kind":"actual Q17 unit coordinate"}),
                );
            }
        }
        for (i, n) in nodes.iter().enumerate() {
            if n["op"] == "Constant" {
                st.put(
                    i,
                    if n["args"] == "0" {
                        V::Inf
                    } else {
                        V::R(Q::z(1))
                    },
                    json!({"kind":"original constant","coefficient":n["args"]}),
                );
            }
        }
        for r in p["residuals"].as_array().unwrap() {
            st.put(
                r.as_u64().unwrap() as usize,
                V::Inf,
                json!({"kind":"original residual zero"}),
            );
        }
        st.put(
            0,
            V::R(Q::new(2.into(), 3.into())),
            json!({"kind":"Q19 forced normalized x0 valuation","original_gates":[15,16,17,18,19]}),
        );
        st.put(
            1,
            V::R(Q::new(1.into(), 2.into())),
            json!({"kind":"Q19 forced normalized y0 valuation","original_gates":[12]}),
        );
        let mut passes = 0;
        let mut visits = 0;
        for pass in 0..64 {
            passes = pass + 1;
            let before = st.proof.len();
            for (i, n) in nodes.iter().enumerate() {
                visits += 1;
                assert!(visits <= 1000000);
                let args = &n["args"];
                match n["op"].as_str().unwrap() {
                    "Input" | "Constant" => {}
                    "CoefficientFrobenius" => {
                        let a = args.as_u64().unwrap() as usize;
                        for (x, y) in [(a, i), (i, a)] {
                            if let Some(k) = st.known[x].clone() {
                                st.rule(
                                    y,
                                    k.v,
                                    i,
                                    "cyclic shift of uniform conjugate valuations",
                                    &[x],
                                );
                            }
                        }
                    }
                    "Inverse" => {
                        let a = args.as_u64().unwrap() as usize;
                        for (x, y) in [(a, i), (i, a)] {
                            if let Some(k) = st.known[x].clone() {
                                match k.v {
                                    V::Inf => {
                                        st.bad = Some(
                                            json!({"kind":"inverse of identically zero coordinate","gate":i,"assignment":k.id}),
                                        )
                                    }
                                    V::R(q) => st.rule(
                                        y,
                                        V::R(Q::z(2).sub(&q)),
                                        i,
                                        "inverse valuation",
                                        &[x],
                                    ),
                                }
                            }
                        }
                    }
                    "Mul" => {
                        let a = args[0].as_u64().unwrap() as usize;
                        let b = args[1].as_u64().unwrap() as usize;
                        let (ka, kb, kz) = (
                            st.known[a].clone(),
                            st.known[b].clone(),
                            st.known[i].clone(),
                        );
                        if let (Some(x), Some(y)) = (ka.clone(), kb.clone()) {
                            let v = match (x.v, y.v) {
                                (V::R(x), V::R(y)) => V::R(x.add(&y).sub(&Q::z(1))),
                                _ => V::Inf,
                            };
                            st.rule(i, v, i, "product forward", &[a, b]);
                        }
                        for (x, y, kx) in [(a, b, ka), (b, a, kb)] {
                            if let (Some(kx), Some(kz)) = (kx, kz.clone()) {
                                match (kx.v, kz.v) {
                                    (V::R(vx), V::R(vz)) => st.rule(
                                        y,
                                        V::R(Q::z(1).add(&vz).sub(&vx)),
                                        i,
                                        "product reverse",
                                        &[x, i],
                                    ),
                                    (V::R(_), V::Inf) => {
                                        st.rule(y, V::Inf, i, "nonzero times zero", &[x, i])
                                    }
                                    (V::Inf, V::R(_)) => {
                                        st.bad = Some(
                                            json!({"kind":"zero factor with nonzero product","gate":i,"parents":[kx.id,kz.id]}),
                                        )
                                    }
                                    _ => {}
                                }
                            }
                        }
                        if a == b {
                            if let Some(k) = st.known[i].clone() {
                                match k.v {
                                    V::R(q) => st.rule(
                                        a,
                                        V::R(Q::z(1).add(&q).div(&Q::z(2))),
                                        i,
                                        "square root valuation",
                                        &[i],
                                    ),
                                    V::Inf => st.rule(a, V::Inf, i, "square zero", &[i]),
                                }
                            }
                        }
                    }
                    "Add" => {
                        let a = args[0].as_u64().unwrap() as usize;
                        let b = args[1].as_u64().unwrap() as usize;
                        for (x, y, z) in [(a, b, i), (a, i, b), (b, i, a)] {
                            if let (Some(kx), Some(ky)) = (st.known[x].clone(), st.known[y].clone())
                            {
                                if kx.v == V::Inf && ky.v == V::Inf {
                                    st.rule(z, V::Inf, i, "zero sum", &[x, y]);
                                } else if kx.v != ky.v {
                                    let v = if kx.v.less(&ky.v) { kx.v } else { ky.v };
                                    st.rule(z, v, i, "unequal valuation minimum", &[x, y]);
                                }
                            }
                        }
                    }
                    op => panic!("unknown{op}"),
                };
                if st.bad.is_some() {
                    break;
                }
            }
            if st.bad.is_some() || before == st.proof.len() {
                break;
            }
        }
        let mut ids = BTreeSet::new();
        if let Some(bad) = &st.bad {
            dependencies(bad, &st.proof, &mut ids);
        }
        let witness = ids.iter().map(|i| st.proof[*i].clone()).collect::<Vec<_>>();
        println!(
            "Q20:graph{g},passes{passes},{} assignments,contradiction={},{} witness assignments",
            st.proof.len(),
            st.bad.is_some(),
            witness.len()
        );
        records.push(json!({"graph":g,"passes":passes,"visits":visits,"assignments":st.proof,"contradiction":st.bad,"minimal_backward_witness":witness,"original_nodes":nodes.len(),"full_h_saturation":false,"scope":"all original gates including coefficient Frobenius;exact known-valuation rules only;curve-selection interpretation remains written"}));
    }
    crate::round02::save(
        dir,
        "q20-original-gate-valuation-propagation.json",
        &json!({"schema":"koblitz.round16/q20-v1","records":records}),
    );
}
