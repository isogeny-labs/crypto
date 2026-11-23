//! Independent replay using a flat ordered incidence index, not producer queues.
//! Removes both incidences on activation; the producer keeps per-node stale ids.
use crate::pair_driver::Source;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
type Class = [u64; 6];
type Witness = Vec<(usize, i8)>;
#[derive(Default, Serialize)]
struct Stats {
    sources: usize,
    vertices: usize,
    anchors: usize,
    full_sources: u64,
    cycles: u64,
    deferred: usize,
    activated: usize,
    live_deferred: usize,
    one_class_anchors: u64,
    two_class_anchors: u64,
    vertex_refusals: u64,
    edge_refusals: u64,
    witness_refusals: u64,
    max_stored_witness: usize,
    max_emitted_witness: usize,
}
#[derive(Default)]
pub struct Replay {
    anchors: BTreeMap<Class, Witness>,
    vertices: BTreeSet<Class>,
    waiting: BTreeSet<usize>,
    incident: BTreeSet<(Class, usize)>,
    stats: Stats,
}
fn ensure(ok: bool, why: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn linear(parts: &[(i64, Witness)]) -> Result<Witness, String> {
    let mut out = BTreeMap::<usize, i64>::new();
    for (scale, w) in parts {
        for &(id, c) in w {
            *out.entry(id).or_default() += scale * c as i64;
        }
    }
    out.into_iter()
        .filter(|&(_, c)| c != 0)
        .map(|(id, c)| {
            ensure(
                (-2..=2).contains(&c),
                "independent witness coefficient bound",
            )?;
            Ok((id, c as i8))
        })
        .collect()
}
fn incidence(sources: &[Source], w: &Witness, target: Option<Class>) -> Result<(), String> {
    ensure(
        !w.is_empty() && w.windows(2).all(|p| p[0].0 < p[1].0),
        "canonical integer witness",
    )?;
    let mut sum = BTreeMap::new();
    for &(id, c) in w {
        ensure([-2, -1, 1, 2].contains(&c), "signed integer coefficient")?;
        for r in &sources
            .get(id)
            .ok_or("future witness source")?
            .certificate
            .remainders
        {
            *sum.entry(r.key).or_insert(0i64) += c as i64 * r.coefficient as i64;
        }
    }
    if let Some(k) = target {
        *sum.entry(k).or_default() -= 1;
    }
    ensure(
        sum.values().all(|&v| v == 0),
        "independent exact integer incidence",
    )
}
impl Replay {
    pub fn stats(&self) -> Value {
        serde_json::to_value(&self.stats).expect("checker stats")
    }
    fn process(
        &mut self,
        sources: &[Source],
        id: usize,
        queue: &mut Vec<Class>,
    ) -> Result<Value, String> {
        let large = &sources[id].certificate.remainders;
        ensure(
            large.len() <= 2
                && large.iter().all(|r| [-1, 1].contains(&r.coefficient))
                && (large.len() != 2 || large[0].key != large[1].key),
            "unit distinct class shape",
        )?;
        let unit = vec![(id, 1)];
        let mut target = None;
        let witness = match large.as_slice() {
            [] => unit,
            [x] => match self.anchors.get(&x.key) {
                Some(a) => linear(&[(1, unit), (-(x.coefficient as i64), a.clone())])?,
                None => {
                    target = Some(x.key);
                    linear(&[(x.coefficient as i64, unit)])?
                }
            },
            [x, y] => match (self.anchors.get(&x.key), self.anchors.get(&y.key)) {
                (Some(a), Some(b)) => linear(&[
                    (1, unit),
                    (-(x.coefficient as i64), a.clone()),
                    (-(y.coefficient as i64), b.clone()),
                ])?,
                (Some(a), None) => {
                    target = Some(y.key);
                    linear(&[
                        (y.coefficient as i64, unit),
                        (-(x.coefficient as i64) * y.coefficient as i64, a.clone()),
                    ])?
                }
                (None, Some(b)) => {
                    target = Some(x.key);
                    linear(&[
                        (x.coefficient as i64, unit),
                        (-(x.coefficient as i64) * y.coefficient as i64, b.clone()),
                    ])?
                }
                (None, None) => {
                    let extra = large
                        .iter()
                        .filter(|r| !self.vertices.contains(&r.key))
                        .count();
                    if self.vertices.len() + extra > 262144 {
                        self.stats.vertex_refusals += 1;
                        return Ok(json!({"kind":"refused","source":id,"reason":"vertex cap"}));
                    }
                    if self.stats.deferred >= 262144 {
                        self.stats.edge_refusals += 1;
                        return Ok(
                            json!({"kind":"refused","source":id,"reason":"deferred edge cap"}),
                        );
                    }
                    ensure(self.waiting.insert(id), "repeated deferred source")?;
                    for r in large {
                        self.vertices.insert(r.key);
                        self.incident.insert((r.key, id));
                    }
                    self.stats.vertices = self.vertices.len();
                    self.stats.deferred += 1;
                    self.stats.live_deferred += 1;
                    return Ok(json!({"kind":"deferred","source":id}));
                }
            },
            _ => unreachable!("validated arity"),
        };
        incidence(sources, &witness, target)?;
        if witness.len() > 256 {
            self.stats.witness_refusals += 1;
            return Ok(json!({"kind":"refused","source":id,"reason":"witness cap"}));
        }
        if let Some(class) = target {
            ensure(
                witness.iter().all(|&(_, c)| [-1, 1].contains(&c)),
                "unit anchor path",
            )?;
            if !self.vertices.contains(&class) && self.vertices.len() >= 262144 {
                self.stats.vertex_refusals += 1;
                return Ok(json!({"kind":"refused","source":id,"reason":"vertex cap"}));
            }
            self.vertices.insert(class);
            self.stats.vertices = self.vertices.len();
            ensure(
                self.anchors.insert(class, witness.clone()).is_none(),
                "replaced anchor",
            )?;
            self.stats.anchors = self.anchors.len();
            self.stats.max_stored_witness = self.stats.max_stored_witness.max(witness.len());
            if large.len() == 1 {
                self.stats.one_class_anchors += 1;
            } else {
                self.stats.two_class_anchors += 1;
            }
            queue.push(class);
            Ok(json!({"kind":"anchor","source":id,"class":class,"witness":witness}))
        } else {
            self.stats.max_emitted_witness = self.stats.max_emitted_witness.max(witness.len());
            if large.is_empty() {
                self.stats.full_sources += 1;
            } else {
                self.stats.cycles += 1;
            }
            Ok(json!({"kind":"full","source":id,"witness":witness}))
        }
    }
    /// Returns every exact full witness caused by the newest source, in order.
    pub fn accept(&mut self, sources: &[Source]) -> Result<Vec<Witness>, String> {
        let causal = self.stats.sources;
        ensure(
            causal < 262144 && sources.len() == causal + 1,
            "source cap/order",
        )?;
        self.stats.sources += 1;
        let mut queue = vec![];
        let mut events = vec![self.process(sources, causal, &mut queue)?];
        let mut cursor = 0;
        while cursor < queue.len() {
            let class = queue[cursor];
            cursor += 1;
            let ids: Vec<_> = self
                .incident
                .range((class, 0)..=(class, usize::MAX))
                .map(|&(_, id)| id)
                .collect();
            for id in ids {
                ensure(self.waiting.remove(&id), "activated nonwaiting edge")?;
                for r in &sources[id].certificate.remainders {
                    ensure(
                        self.incident.remove(&(r.key, id)),
                        "missing pending incidence",
                    )?;
                }
                self.stats.activated += 1;
                self.stats.live_deferred -= 1;
                let e = self.process(sources, id, &mut queue)?;
                ensure(e["kind"] != "deferred", "wake without unit anchor")?;
                events.push(e);
            }
        }
        ensure(
            self.stats.deferred == self.stats.activated + self.stats.live_deferred,
            "pending conservation",
        )?;
        ensure(
            sources[causal].graph == json!(events),
            "complete queue decisions/witnesses/order",
        )?;
        events
            .into_iter()
            .filter(|e| e["kind"] == "full")
            .map(|e| serde_json::from_value(e["witness"].clone()).map_err(|e| e.to_string()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn s(large: Vec<(u64, i8)>, graph: Value) -> Source {
        serde_json::from_value(json!({"candidate_index":1,"b_index":0,"a0":[],"a1":[],"b":[],"s":1,
            "certificate":{"terms":[],"remainders":large.iter().map(|&(k,c)|json!({"key":[k,0,0,0,0,0],"coefficient":c})).collect::<Vec<_>>(),"a":[],"c":[],"norm":[]},"graph":graph})).unwrap()
    }
    #[test]
    fn independent_replay_requires_complete_ordered_wake_history() {
        let deferred = json!([{"kind":"deferred","source":0}]);
        let events = json!([{"kind":"anchor","source":1,"class":[2,0,0,0,0,0],"witness":[[1,1]]},
            {"kind":"anchor","source":0,"class":[1,0,0,0,0,0],"witness":[[0,1],[1,-1]]}]);
        let a = s(vec![(1, 1), (2, 1)], deferred);
        let b = s(vec![(2, 1)], events.clone());
        let mut r = Replay::default();
        r.accept(std::slice::from_ref(&a)).unwrap();
        r.accept(&[a, b]).unwrap();
        assert_eq!(r.stats.anchors, 2);
        assert_eq!(r.stats.activated, 1);
        for bad in [
            json!([]),
            json!([events[1].clone(), events[0].clone()]),
            json!([events[0].clone()]),
        ] {
            let mut r = Replay::default();
            let a = s(
                vec![(1, 1), (2, 1)],
                json!([{"kind":"deferred","source":0}]),
            );
            r.accept(std::slice::from_ref(&a)).unwrap();
            assert!(r.accept(&[a, s(vec![(2, 1)], bad)]).is_err());
        }
    }
}
