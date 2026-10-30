//! Bounded generic relation oracle for archived factor bases.
//! Derive the final summand; retain the evaluator's ordered-multiset policy.
use crate::{arithmetic::*, recipe::Generated};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Decomposition {
    pub complete: bool,
    pub witnesses: Vec<Option<Vec<usize>>>,
    pub visited_prefixes: usize,
    pub target_lookups: usize,
}
struct Search<'a> {
    g: &'a Generated,
    targets: &'a [Point],
    index: BTreeMap<Point, usize>,
    result: Decomposition,
    cap: usize,
}
impl Search<'_> {
    fn charge(&self) -> bool {
        self.result.visited_prefixes + self.result.target_lookups < self.cap
    }
    fn walk(&mut self, left: usize, start: usize, sum: P, tuple: &mut Vec<usize>) -> bool {
        if !self.charge() {
            return false;
        }
        self.result.visited_prefixes += 1;
        if left == 1 {
            for (query, target) in self.targets.iter().enumerate() {
                if !self.charge() {
                    return false;
                }
                self.result.target_lookups += 1;
                let neg_sum = sum.as_ref().map(|p| self.g.curve.neg(p));
                let Some(last) = self.g.curve.add(&Some(target.clone()), &neg_sum) else {
                    continue;
                };
                let Some(&i) = self.index.get(&last) else {
                    continue;
                };
                if i < start
                    || tuple
                        .iter()
                        .any(|&j| self.g.curve.neg(&self.g.points[j]) == last)
                {
                    continue;
                }
                if self.result.witnesses[query].is_none() {
                    let mut witness = tuple.clone();
                    witness.push(i);
                    self.result.witnesses[query] = Some(witness);
                }
            }
            return true;
        }
        for i in start..self.g.points.len() {
            if tuple
                .iter()
                .any(|&j| self.g.curve.neg(&self.g.points[j]) == self.g.points[i])
            {
                continue;
            }
            tuple.push(i);
            let next = self.g.curve.add(&sum, &Some(self.g.points[i].clone()));
            let complete = self.walk(left - 1, i, next, tuple);
            tuple.pop();
            if !complete {
                return false;
            }
        }
        true
    }
}
/// Exact generic search for finite subgroup targets. Incomplete search is
/// explicitly censored; a missing witness then proves nothing. The work cap
/// counts prefix visits plus target lookups, not bit/group operations or time.
pub fn decompose(
    g: &Generated,
    targets: &[Point],
    arity: usize,
    cap: usize,
) -> Result<Decomposition> {
    ensure!((2..=8).contains(&arity), "arity must be 2..=8");
    ensure!((1..=10_000_000).contains(&cap), "invalid search cap");
    ensure!(
        (1..=1024).contains(&targets.len()),
        "target count must be 1..=1024"
    );
    ensure!(
        targets.iter().all(|p| g.curve.subgroup(p)),
        "target outside subgroup"
    );
    ensure!(
        g.points.len() <= 2_000_000 && !g.points.is_empty(),
        "invalid base size"
    );
    ensure!(
        g.points.iter().all(|p| g.curve.subgroup(p)),
        "base point outside subgroup"
    );
    let index: BTreeMap<_, _> = g
        .points
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect();
    ensure!(index.len() == g.points.len(), "duplicate base points");
    let mut search = Search {
        g,
        targets,
        index,
        cap,
        result: Decomposition {
            complete: false,
            witnesses: vec![None; targets.len()],
            visited_prefixes: 0,
            target_lookups: 0,
        },
    };
    search.result.complete = search.walk(arity, 0, None, &mut Vec::new());
    for (target, witness) in targets.iter().zip(&search.result.witnesses) {
        if let Some(indices) = witness {
            let sum = indices
                .iter()
                .fold(None, |p, &j| g.curve.add(&p, &Some(g.points[j].clone())));
            ensure!(
                sum == Some(target.clone()),
                "derived relation replay failed"
            );
        }
    }
    Ok(search.result)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub targets: Vec<PointSpec>,
    pub arity: usize,
    pub search_cap: usize,
}
/// Bind an opt-in decomposition to its archive and explicit public targets.
pub fn evaluate(root: &std::path::Path, id: &str, request: &Request) -> Result<serde_json::Value> {
    use crate::store::*;
    let (_, g) = verify(root, id)?;
    let targets = request
        .targets
        .iter()
        .map(Point::from_spec)
        .collect::<Result<Vec<_>>>()?;
    let result = decompose(&g, &targets, request.arity, request.search_cap)?;
    let semantic = serde_json::json!({
        "schema":"factor-base-derived-decomposition/v1", "manifest_sha256":id,
        "request":request, "result":result, "indexed_base_points":g.points.len(),
        "relation_policy":"unordered_multisets_repetition_allowed_inverse_pairs_excluded",
        "track":"generic", "useful_relation_rank":null, "cold_total_ecdlp_cost":null,
        "accounting":"prefix visits plus target lookups; index construction, validation and bit/group costs not included"
    });
    Ok(
        serde_json::json!({"semantic_sha256":digest(&canonical(&semantic)?),"semantic":semantic,"source_sha256":source_digest()}),
    )
}
/// Regenerate the witness search, including cap/censorship, from public inputs.
pub fn replay(
    root: &std::path::Path,
    id: &str,
    receipt: &serde_json::Value,
) -> Result<serde_json::Value> {
    let request: Request = serde_json::from_value(receipt["semantic"]["request"].clone())?;
    let actual = evaluate(root, id, &request)?;
    ensure!(
        actual["semantic"] == receipt["semantic"]
            && actual["semantic_sha256"] == receipt["semantic_sha256"],
        "decomposition replay differs"
    );
    Ok(
        serde_json::json!({"status":"replayed", "independence":"same_native_implementation", "semantic_sha256":actual["semantic_sha256"]}),
    )
}
