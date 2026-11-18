//! Independent exact-integer reconstruction of each grounded graph decision.
//! Does not call the producer graph; validates stored/emitted source incidence.
use crate::pair_driver::Source;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
type Class = [u64; 6];
type Witness = Vec<(usize, i8)>;
#[derive(Default, Serialize)]
struct Stats {
    sources: usize,
    vertices: usize,
    full_rows: u64,
    anchored_cycles: u64,
    one_class_anchors: u64,
    two_class_extensions: u64,
    discarded_ungrounded: u64,
    vertex_refusals: u64,
    witness_refusals: u64,
    max_stored_witness: usize,
    max_emitted_witness: usize,
}
#[derive(Default)]
pub struct Replay {
    anchors: BTreeMap<Class, Witness>,
    stats: Stats,
}
fn ensure(ok: bool, why: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
/// The checker accumulates in i64 and checks before the final i8 conversion.
fn linear(parts: &[(i64, Witness)]) -> Result<Witness, String> {
    let mut sum = BTreeMap::<usize, i64>::new();
    for (scale, terms) in parts {
        for &(id, c) in terms {
            *sum.entry(id).or_default() += scale * c as i64;
        }
    }
    sum.into_iter()
        .filter(|&(_, c)| c != 0)
        .map(|(id, c)| {
            ensure((-2..=2).contains(&c), "independent graph coefficient bound")?;
            Ok((id, c as i8))
        })
        .collect()
}
fn incidence(sources: &[Source], witness: &Witness, target: Option<Class>) -> Result<(), String> {
    ensure(
        witness.windows(2).all(|w| w[0].0 < w[1].0),
        "witness source order",
    )?;
    let mut columns = BTreeMap::<Class, i64>::new();
    for &(id, c) in witness {
        ensure([-2, -1, 1, 2].contains(&c), "integer witness coefficient")?;
        let row = &sources
            .get(id)
            .ok_or("independent source range")?
            .certificate;
        for q in &row.remainders {
            *columns.entry(q.key).or_default() += c as i64 * q.coefficient as i64;
        }
    }
    if let Some(class) = target {
        *columns.entry(class).or_default() -= 1;
    }
    ensure(
        columns.values().all(|&c| c == 0),
        "independent exact integer incidence",
    )
}
impl Replay {
    pub fn stats(&self) -> Value {
        serde_json::to_value(&self.stats).expect("checker stats")
    }
    pub fn accept(&mut self, sources: &[Source]) -> Result<Option<Witness>, String> {
        let id = self.stats.sources;
        ensure(
            id < 262_144 && sources.len() == id + 1,
            "independent graph source cap/order",
        )?;
        let row = &sources[id].certificate;
        let unit = vec![(id, 1)];
        let large = &row.remainders;
        ensure(
            large.len() <= 2 && (large.len() != 2 || large[0].key != large[1].key),
            "graph class shape",
        )?;
        self.stats.sources += 1;
        let mut target = None;
        let witness = match large.len() {
            0 => unit,
            1 => {
                let q = &large[0];
                if let Some(old) = self.anchors.get(&q.key) {
                    linear(&[(1, unit), (-(q.coefficient as i64), old.clone())])?
                } else {
                    target = Some(q.key);
                    linear(&[(q.coefficient as i64, unit)])?
                }
            }
            _ => {
                let x = &large[0];
                let y = &large[1];
                match (self.anchors.get(&x.key), self.anchors.get(&y.key)) {
                    (None, None) => {
                        self.stats.discarded_ungrounded += 1;
                        ensure(
                            sources[id].graph == json!({"kind":"ungrounded"}),
                            "ungrounded decision",
                        )?;
                        return Ok(None);
                    }
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
                }
            }
        };
        incidence(sources, &witness, target)?;
        if witness.len() > 256 {
            self.stats.witness_refusals += 1;
            ensure(
                sources[id].graph == json!({"kind":"witness_cap"}),
                "witness cap decision",
            )?;
            return Ok(None);
        }
        if let Some(class) = target {
            ensure(
                witness.iter().all(|&(_, c)| c == 1 || c == -1),
                "anchor unit coefficients",
            )?;
            if self.anchors.len() >= 65_536 {
                self.stats.vertex_refusals += 1;
                ensure(
                    sources[id].graph == json!({"kind":"vertex_cap"}),
                    "vertex cap decision",
                )?;
                return Ok(None);
            }
            ensure(
                sources[id].graph == json!({"kind":"anchor","class":class,"witness":witness}),
                "anchor decision/witness",
            )?;
            self.stats.max_stored_witness = self.stats.max_stored_witness.max(witness.len());
            self.anchors.insert(class, witness);
            self.stats.vertices = self.anchors.len();
            if large.len() == 1 {
                self.stats.one_class_anchors += 1;
            } else {
                self.stats.two_class_extensions += 1;
            }
            Ok(None)
        } else {
            ensure(
                sources[id].graph == json!({"kind":"full","witness":witness}),
                "full decision/witness",
            )?;
            self.stats.max_emitted_witness = self.stats.max_emitted_witness.max(witness.len());
            if large.is_empty() {
                self.stats.full_rows += 1;
            } else {
                self.stats.anchored_cycles += 1;
            }
            Ok(Some(witness))
        }
    }
}
