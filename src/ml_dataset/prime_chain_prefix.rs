//! Causal replay of recursive branch decisions, never an intra-F4 trace verifier.

use std::collections::{BTreeMap, BTreeSet};
use std::io::BufRead;

use serde_json::{json, Value};

use super::algebraic_targets;
use super::prefix_algebra::{self, Poly};

const MAX_EVENT_BYTES: u64 = 8 * 1024 * 1024;
const MAX_EVENTS: u64 = 4096;
const MAX_STREAM_BYTES: u64 = 64 * 1024 * 1024;
const MAX_VERIFIER_COEFFICIENT_OPS: u64 = 100_000_000;

#[derive(Default, serde::Serialize)]
struct VerifierWork {
    certificate_coefficient_operations: u64,
    root_coefficient_operations: u64,
    finite_domain_evaluations: u64,
}
impl VerifierWork {
    fn remaining(&self) -> u64 {
        MAX_VERIFIER_COEFFICIENT_OPS.saturating_sub(
            self.certificate_coefficient_operations + self.root_coefficient_operations,
        )
    }
}

fn uint(v: &Value) -> Result<u64, String> {
    v.as_u64().ok_or("prefix integer missing".into())
}
fn boolean(v: &Value) -> Result<bool, String> {
    v.as_bool().ok_or("prefix boolean missing".into())
}
fn array(v: &Value) -> Result<&[Value], String> {
    v.as_array()
        .map(Vec::as_slice)
        .ok_or("prefix array missing".into())
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    ((u128::from(a) * u128::from(b)) % u128::from(p)) as u64
}
fn add(a: u64, b: u64, p: u64) -> u64 {
    ((u128::from(a) + u128::from(b)) % u128::from(p)) as u64
}
fn pow(mut a: u64, mut e: u32, p: u64) -> u64 {
    let mut r = 1;
    while e > 0 {
        if e & 1 == 1 {
            r = mul(r, a, p);
        }
        e >>= 1;
        if e > 0 {
            a = mul(a, a, p);
        }
    }
    r
}
fn inverse(a: u64, p: u64) -> u64 {
    pow(a, (p - 2) as u32, p)
} // Native replay admits p<2^32.

enum PolynomialError {
    SizeCap(&'static str),
    Invalid(String),
}
impl PolynomialError {
    fn message(self) -> String {
        match self {
            Self::SizeCap(message) => message.into(),
            Self::Invalid(message) => message,
        }
    }
}
impl From<String> for PolynomialError {
    fn from(message: String) -> Self {
        Self::Invalid(message)
    }
}
impl From<&str> for PolynomialError {
    fn from(message: &str) -> Self {
        Self::Invalid(message.into())
    }
}

fn polys(value: &Value, n: usize, p: u64, source: bool) -> Result<Vec<Poly>, PolynomialError> {
    let rows = array(value)?;
    if rows.len() > 4096 {
        return Err(PolynomialError::SizeCap("prefix polynomial row cap"));
    }
    let mut total = 0;
    rows.iter()
        .map(|row| {
            let mut out = Vec::new();
            let mut seen = BTreeSet::new();
            for term in array(row)? {
                total += 1;
                if total > 16_384 {
                    return Err(PolynomialError::SizeCap("prefix polynomial term cap"));
                }
                let (ev, c) = if source {
                    (
                        &term["exponents"],
                        term["coefficient"]
                            .as_str()
                            .ok_or("source coefficient")?
                            .parse::<u64>()
                            .map_err(|_| "source coefficient")?,
                    )
                } else {
                    let term = array(term)?;
                    if term.len() != 2 {
                        return Err("prefix native term shape".into());
                    }
                    (&term[0], uint(&term[1])?)
                };
                let e = array(ev)?
                    .iter()
                    .map(|v| u32::try_from(uint(v)?).map_err(|_| "prefix exponent overflow".into()))
                    .collect::<Result<Vec<_>, String>>()?;
                if e.len() != n || c == 0 || c >= p || !seen.insert(e.clone()) {
                    return Err("invalid prefix polynomial".into());
                }
                if e.iter()
                    .try_fold(0u32, |a, b| a.checked_add(*b))
                    .is_none_or(|d| d > 64)
                {
                    return Err(PolynomialError::SizeCap("prefix polynomial degree cap"));
                }
                out.push((e, c));
            }
            Ok(out)
        })
        .collect()
}
fn equivalent_rows(a: &[Poly], b: &[Poly]) -> bool {
    a.len() == b.len()
        && a.iter().zip(b).all(|(a, b)| {
            a.iter().cloned().collect::<BTreeMap<_, _>>() == b.iter().cloned().collect()
        })
}
fn eval(f: &Poly, values: &[u64], p: u64) -> u64 {
    f.iter().fold(0, |sum, (e, c)| {
        add(
            sum,
            e.iter()
                .zip(values)
                .fold(*c, |v, (e, x)| mul(v, pow(*x, *e, p), p)),
            p,
        )
    })
}
fn specialize(basis: &[Poly], var: usize, value: u64, p: u64) -> Vec<Poly> {
    use crate::cryptanalysis::groebner_f4::{cmp_monomial, Ordering};
    basis
        .iter()
        .filter_map(|f| {
            let mut map = BTreeMap::new();
            for (e, c) in f {
                let mut e = e.clone();
                let exponent = e.remove(var);
                let c = mul(*c, pow(value, exponent, p), p);
                let entry = map.entry(e).or_insert(0);
                *entry = add(*entry, c, p);
            }
            let mut out: Poly = map.into_iter().filter(|(_, c)| *c != 0).collect();
            out.sort_by(|(a, _), (b, _)| cmp_monomial(b, a, Ordering::Grevlex));
            if out.is_empty() {
                return None;
            }
            let inv = inverse(out[0].1, p);
            for (_, c) in &mut out {
                *c = mul(*c, inv, p);
            }
            Some(out)
        })
        .collect()
}

#[derive(Clone)]
struct IteratorState {
    local: usize,
    values: Option<Vec<u64>>,
    consumed: u64,
    source: &'static str,
}
impl IteratorState {
    fn length(&self, p: u64) -> u64 {
        self.values.as_ref().map_or(p, |v| v.len() as u64)
    }
    fn next(&self) -> Option<u64> {
        self.values.as_ref().map_or(Some(self.consumed), |v| {
            v.get(self.consumed as usize).copied()
        })
    }
    fn projection(&self, active: &[usize], p: u64) -> Value {
        // This variable/domain is independently derived from the preceding basis,
        // not copied from the current Branch event or any later event.
        match &self.values {
            Some(values) => json!({"original_variable":active[self.local],"kind":"finite",
                "remaining_values":values.iter().skip(self.consumed as usize).map(u64::to_string).collect::<Vec<_>>() }),
            None => json!({"original_variable":active[self.local],"kind":"field_interval",
                "start_inclusive":self.consumed.to_string(),"end_exclusive":p.to_string()}),
        }
    }
}

fn iterator(
    basis: &[Poly],
    active: &[usize],
    domains: &[Option<Vec<u64>>],
    p: u64,
    work: &mut VerifierWork,
) -> Result<Option<IteratorState>, String> {
    let mut best: Option<(usize, Vec<u64>)> = None;
    for f in basis {
        let vars: BTreeSet<_> = f
            .iter()
            .flat_map(|(e, _)| {
                e.iter()
                    .enumerate()
                    .filter(|(_, e)| **e > 0)
                    .map(|(i, _)| i)
            })
            .collect();
        if vars.len() != 1 {
            continue;
        }
        let var = *vars.first().unwrap();
        let roots = if let Some(domain) = &domains[active[var]] {
            work.finite_domain_evaluations += domain.len() as u64;
            domain
                .iter()
                .copied()
                .filter(|x| {
                    f.iter()
                        .fold(0, |v, (e, c)| add(v, mul(*c, pow(*x, e[var], p), p), p))
                        == 0
                })
                .collect()
        } else {
            let degree = f.iter().map(|(e, _)| e[var]).max().unwrap() as usize;
            let mut coefficients = vec![0; degree + 1];
            for (e, c) in f {
                coefficients[e[var] as usize] = *c;
            }
            let mut root_work = prefix_algebra::Work::default();
            let result = prefix_algebra::roots(
                &coefficients,
                p,
                &mut root_work,
                work.remaining()
                    .min(prefix_algebra::Limits::default().operations),
            );
            work.root_coefficient_operations += root_work.coefficient_operations;
            result?
        };
        if best.as_ref().is_none_or(|(_, old)| roots.len() < old.len()) {
            best = Some((var, roots));
        }
        if best.as_ref().is_some_and(|(_, v)| v.is_empty()) {
            return Ok(None);
        }
    }
    let (local, values, source) = if let Some((i, roots)) = best {
        (i, Some(roots), "univariate_roots")
    } else if let Some((i, domain)) = active
        .iter()
        .enumerate()
        .filter_map(|(i, original)| domains[*original].as_ref().map(|d| (i, d)))
        .min_by_key(|(_, d)| d.len())
    {
        (i, Some(domain.clone()), "finite_domain")
    } else {
        (0, None, "full_field")
    };
    Ok(Some(IteratorState {
        local,
        values,
        consumed: 0,
        source,
    }))
}

struct Frame {
    id: u64,
    active: Vec<usize>,
    bindings: BTreeMap<usize, u64>,
    system: Vec<Poly>,
    basis: Option<Vec<Poly>>,
    iterator: Option<IteratorState>,
    terminal: bool,
}
struct Child {
    parent: u64,
    active: Vec<usize>,
    bindings: BTreeMap<usize, u64>,
    system: Vec<Poly>,
}

pub(super) fn graph(polynomials: &[Poly], n: usize) -> Value {
    let monomials: Vec<_> = polynomials
        .iter()
        .flat_map(|f| f.iter().map(|(e, _)| e.clone()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let ids: BTreeMap<_, _> = monomials
        .iter()
        .enumerate()
        .map(|(i, e)| (e.clone(), i))
        .collect();
    let coefficients: Vec<_> = polynomials
        .iter()
        .enumerate()
        .flat_map(|(row, f)| {
            f.iter()
                .map(|(e, c)| json!({"equation":row,"monomial":ids[e],"coefficient":c.to_string()}))
                .collect::<Vec<_>>()
        })
        .collect();
    json!({"schema":"prime-prefix-polynomial-graph/v1","variables":n,"equations":polynomials.len(),
        "monomials":monomials,"coefficient_edges":coefficients})
}

fn replay_summary(
    examples: u64,
    certified_nodes: u64,
    events: u64,
    bytes: u64,
    reason: Option<String>,
    verifier_work: &VerifierWork,
) -> Value {
    json!({"schema":"ml-dataset.prime-chain-prefix-replay/v1","verified_branch_examples":examples,
        "ideal_equality_certified_nodes":certified_nodes,"events_read":events,"bytes_read":bytes,
        "admission_status":if reason.is_some(){"bounded_unknown_suffix"}else{"retained_prefix_checked"},"reason":reason,
        "source_completion_claimed":false,"native_execution_trace_verified":false,
        "native_instruction_count_verified":false,"exact_fall_degree_verified":false,
        "verifier_work":verifier_work,"verifier_work_scope":"certificate/root coefficient operations and finite-domain evaluations; excludes parsing, specialization, graph construction, primality checks and I/O; not native solve costs or CPU instructions",
        "limits":{"events":MAX_EVENTS,"event_bytes":MAX_EVENT_BYTES,"stream_bytes":MAX_STREAM_BYTES,
            "certificate_plus_root_coefficient_operations":MAX_VERIFIER_COEFFICIENT_OPS}})
}

/// Caller has already reconstructed the source against public curve/base inputs.
/// emit receives (public feature, restricted observed label) or a restricted audit.
/// Search caps stop admission from that point; earlier valid prefixes are retained.
pub(super) fn replay(
    input: &Value,
    initial: &Value,
    mut reader: impl BufRead,
    mut emit: impl FnMut(Option<Value>, Value) -> Result<(), String>,
) -> Result<Value, String> {
    let p = input["modulus"]
        .as_str()
        .ok_or("prefix source modulus")?
        .parse::<u64>()
        .map_err(|_| "prefix modulus")?;
    let n = usize::try_from(uint(&input["n_vars"])?).map_err(|_| "prefix n_vars")?;
    let m = usize::try_from(uint(&input["m"])?).map_err(|_| "prefix m")?;
    if !(3..=15).contains(&m)
        || n != 2 * m - 2
        || !(5..1u64 << 32).contains(&p)
        || !crate::cryptanalysis::residual_walk::is_prime_u64(p)
    {
        return Err("unsupported native prefix ring".into());
    }
    let original = match polys(&input["polynomials"], n, p, true) {
        Ok(polynomials) => polynomials,
        // No prefix has been verified when the public source itself exceeds
        // admission bounds. The uninspected suffix remains unknown; a cap does
        // not certify that the rest of an oversized payload is well formed.
        Err(PolynomialError::SizeCap(message)) => {
            return Ok(replay_summary(
                0,
                0,
                0,
                0,
                Some(message.into()),
                &VerifierWork::default(),
            ));
        }
        Err(error) => return Err(error.message()),
    };
    let domains: Vec<Option<Vec<u64>>> = array(&input["domains"])?
        .iter()
        .map(|v| {
            if v.is_null() {
                Ok(None)
            } else {
                array(v)?
                    .iter()
                    .map(uint)
                    .collect::<Result<Vec<_>, _>>()
                    .map(Some)
            }
        })
        .collect::<Result<_, String>>()?;
    if domains.len() != n
        || domains.iter().flatten().any(|d| {
            d.is_empty()
                || d.len() > 32
                || d.iter().any(|x| *x >= p)
                || d.windows(2).any(|w| w[0] >= w[1])
        })
    {
        return Err("invalid prefix source domains".into());
    }
    let mut stack: Vec<Frame> = Vec::new();
    let mut pending: Option<Child> = None;
    let mut awaiting_lift: Option<Vec<u64>> = None;
    let mut next_node = 0;
    let mut events = 0;
    let mut bytes = 0;
    let mut examples = 0;
    let mut certified_nodes = 0;
    let mut verifier_work = VerifierWork::default();
    let mut stopped = false;
    let mut reason = None;
    let mut algebraic = algebraic_targets::Channel::default();
    loop {
        let cut = bytes;
        let mut line = Vec::new();
        let read = std::io::Read::take(&mut reader, MAX_EVENT_BYTES + 1)
            .read_until(b'\n', &mut line)
            .map_err(|e| e.to_string())?;
        if read == 0 {
            break;
        }
        if events >= MAX_EVENTS
            || read as u64 > MAX_EVENT_BYTES
            || bytes + read as u64 > MAX_STREAM_BYTES
        {
            reason = Some("prefix event/byte verification cap".to_owned());
            break;
        }
        if line.last() != Some(&b'\n') {
            return Err("unterminated prime-chain event".into());
        }
        if stopped {
            return Err("event follows stopped traversal".into());
        }
        bytes += read as u64;
        events += 1;
        let row: Value = serde_json::from_slice(&line).map_err(|e| e.to_string())?;
        if row["schema"] != "ml-dataset.prime-f4-chain-event/v1"
            || uint(&row["sequence"])? != events
        {
            return Err("prime-chain event sequence/schema mismatch".into());
        }
        let event = &row["event"];
        let kind = event["kind"].as_str().ok_or("prefix event kind")?;
        if pending.is_some() && kind != "system" {
            return Err("branch must be followed by child system".into());
        }
        if awaiting_lift.is_some() && kind != "group_lift" {
            return Err("verified candidate missing lift event".into());
        }
        while pending.is_none()
            && awaiting_lift.is_none()
            && stack.last().is_some_and(|f| {
                f.terminal
                    || f.iterator
                        .as_ref()
                        .is_some_and(|i| i.consumed == i.length(p))
            })
        {
            stack.pop();
        }
        match kind {
            "system" => {
                let id = uint(&event["node_id"])?;
                if id != next_node {
                    return Err("noncontiguous prefix node IDs".into());
                }
                next_node += 1;
                let active: Vec<usize> = array(&event["active_variables"])?
                    .iter()
                    .map(|v| usize::try_from(uint(v)?).map_err(|_| "active index overflow".into()))
                    .collect::<Result<_, String>>()?;
                if active.len() > n
                    || active.windows(2).any(|w| w[0] >= w[1])
                    || active.iter().any(|i| *i >= n)
                {
                    return Err("invalid active variables".into());
                }
                let mut bindings = BTreeMap::new();
                for pair in array(&event["assignments"])? {
                    let pair = array(pair)?;
                    if pair.len() != 2 {
                        return Err("invalid fixed binding".into());
                    }
                    let var =
                        usize::try_from(uint(&pair[0])?).map_err(|_| "binding index overflow")?;
                    let x = uint(&pair[1])?;
                    if var >= n
                        || x >= p
                        || active.contains(&var)
                        || bindings.insert(var, x).is_some()
                    {
                        return Err("stale or invalid fixed binding".into());
                    }
                }
                let system = match polys(&event["polynomials"], active.len(), p, false) {
                    Ok(polynomials) => polynomials,
                    Err(PolynomialError::SizeCap(message)) => {
                        reason = Some(message.into());
                        break;
                    }
                    Err(error) => return Err(error.message()),
                };
                if id == 0 {
                    if !event["parent_id"].is_null()
                        || !stack.is_empty()
                        || pending.is_some()
                        || active != (0..n).collect::<Vec<_>>()
                        || !bindings.is_empty()
                        || !equivalent_rows(&system, &original)
                    {
                        return Err("root system differs from verified public equations".into());
                    }
                } else {
                    let child = pending.take().ok_or("unannounced child system")?;
                    if uint(&event["parent_id"])? != child.parent
                        || active != child.active
                        || bindings != child.bindings
                        || !equivalent_rows(&system, &child.system)
                    {
                        return Err("invalid parent edge or specialization".into());
                    }
                }
                stack.push(Frame {
                    id,
                    active,
                    bindings,
                    system,
                    basis: None,
                    iterator: None,
                    terminal: false,
                });
            }
            "native_result" => {
                let frame = stack.last_mut().ok_or("native result without system")?;
                if uint(&event["node_id"])? != frame.id
                    || frame.active.is_empty()
                    || frame.basis.is_some()
                {
                    return Err("invalid native result position".into());
                }
                let report = &event["report"];
                let basis = match polys(&report["basis"], frame.active.len(), p, false) {
                    Ok(polynomials) => polynomials,
                    Err(PolynomialError::SizeCap(message)) => {
                        reason = Some(message.into());
                        break;
                    }
                    Err(error) => return Err(error.message()),
                };
                let native_basis_len = basis.len();
                // New traced visitors explicitly distinguish the native
                // returned subset from the actual recursively used generators.
                // Legacy streams retain their old admission and unknown suffix.
                let basis = if let Some(value) = event.get("generating_set") {
                    let preserved = match polys(value, frame.active.len(), p, false) {
                        Ok(polynomials) => polynomials,
                        Err(PolynomialError::SizeCap(message)) => {
                            reason = Some(message.into());
                            break;
                        }
                        Err(error) => return Err(error.message()),
                    };
                    let mut expected = basis;
                    for f in &frame.system {
                        if !f.is_empty() && !expected.contains(f) {
                            expected.push(f.clone());
                        }
                    }
                    if !equivalent_rows(&preserved, &expected) {
                        return Err(
                            "preserved generating set differs from native rows and incoming system"
                                .into(),
                        );
                    }
                    preserved
                } else {
                    basis
                };
                let audit = prefix_algebra::equivalent(
                    &frame.system,
                    &basis,
                    frame.active.len(),
                    p,
                    prefix_algebra::Limits {
                        operations: verifier_work
                            .remaining()
                            .min(prefix_algebra::Limits::default().operations),
                        ..prefix_algebra::Limits::default()
                    },
                )?;
                verifier_work.certificate_coefficient_operations +=
                    audit.work.coefficient_operations;
                let verified = audit.certificate.is_some();
                let audit_reason = audit.reason.clone();
                if let Some(certificate) = &audit.certificate {
                    algebraic.emit_node(
                        algebraic_targets::Node {
                            initial,
                            active: &frame.active,
                            bindings: &frame.bindings,
                            sources: &frame.system,
                            // The appended constraints support traversal, but
                            // are not retained native output training targets.
                            // The contract above guarantees the native prefix.
                            basis: &basis[..native_basis_len],
                            proofs: &certificate.right_from_left[..native_basis_len],
                            p,
                            sequence: events,
                            node_id: frame.id,
                            equality_work: &audit.work,
                        },
                        &mut emit,
                    )?;
                }
                emit(
                    None,
                    json!({"kind":"ideal_equality_audit","sequence":events,"node_id":frame.id,"certificate":audit}),
                )?;
                if !verified {
                    reason = audit_reason;
                    break;
                }
                certified_nodes += 1;
                let inconsistent = basis
                    .iter()
                    .any(|f| f.len() == 1 && f[0].0.iter().all(|e| *e == 0));
                if boolean(&report["inconsistent"])? != inconsistent {
                    return Err("inconsistency flag differs from certified constants".into());
                }
                if boolean(&report["timed_out"])? {
                    stopped = true;
                } else if inconsistent {
                    frame.terminal = true;
                } else {
                    match iterator(&basis, &frame.active, &domains, p, &mut verifier_work) {
                        Ok(value) => {
                            frame.terminal = value.is_none();
                            frame.iterator = value;
                        }
                        Err(error) => {
                            reason = Some(error);
                            break;
                        }
                    }
                }
                frame.basis = Some(basis);
            }
            "branch" => {
                let frame = stack.last_mut().ok_or("branch without live system")?;
                let iterator = frame
                    .iterator
                    .as_mut()
                    .ok_or("branch without verified iterator")?;
                let basis = frame
                    .basis
                    .as_ref()
                    .ok_or("branch without certified polynomial set")?;
                let var = frame.active[iterator.local];
                let value = uint(&event["value"])?;
                if uint(&event["node_id"])? != frame.id
                    || uint(&event["original_variable"])? != var as u64
                    || iterator.next() != Some(value)
                    || event["source"] != iterator.source
                {
                    return Err("branch differs from reconstructed iterator/progress".into());
                }
                let feature = json!({"task":"prime-chain-next-branch-attempt/v1","initial":initial,
                    "active_original_variables":frame.active,
                    "fixed_bindings":frame.bindings.iter().map(|(i,x)|json!({"original_variable":i,"value":x.to_string()})).collect::<Vec<_>>(),
                    "specialized_equations":graph(&frame.system,frame.active.len()),
                    "returned_equations":graph(basis,frame.active.len()),
                    "admissible_action_domain":iterator.projection(&frame.active,p)});
                emit(
                    Some(feature),
                    json!({"kind":"observed_next_branch","cut_byte_offset":cut,"action_sequence":events,
                    "source_node_id":frame.id,"original_variable":var,"value":value.to_string(),"source":iterator.source,
                    "counterfactual_cost":null,"optimal_action_verified":false}),
                )?;
                examples += 1;
                let mut active = frame.active.clone();
                active.remove(iterator.local);
                let mut bindings = frame.bindings.clone();
                bindings.insert(var, value);
                pending = Some(Child {
                    parent: frame.id,
                    active,
                    bindings,
                    system: specialize(basis, iterator.local, value, p),
                });
                iterator.consumed += 1;
            }
            "candidate" => {
                let frame = stack.last_mut().ok_or("candidate without system")?;
                if uint(&event["node_id"])? != frame.id
                    || !frame.active.is_empty()
                    || frame.basis.is_some()
                {
                    return Err("candidate at nonterminal system".into());
                }
                let assignment = array(&event["assignment"])?
                    .iter()
                    .map(uint)
                    .collect::<Result<Vec<_>, _>>()?;
                if assignment.len() != n
                    || assignment
                        .iter()
                        .enumerate()
                        .any(|(i, x)| frame.bindings.get(&i) != Some(x))
                {
                    return Err("candidate differs from fixed ancestor bindings".into());
                }
                let residual = frame.system.iter().all(|f| eval(f, &[], p) == 0);
                let original_ok =
                    residual.then(|| original.iter().all(|f| eval(f, &assignment, p) == 0));
                if boolean(&event["residual_satisfied"])? != residual
                    || event["original_satisfied"] != json!(original_ok)
                {
                    return Err("candidate equation check differs".into());
                }
                frame.terminal = true;
                if original_ok == Some(true) {
                    awaiting_lift = Some(assignment);
                }
            }
            "group_lift" => {
                let assignment = awaiting_lift
                    .take()
                    .ok_or("lift without verified candidate")?;
                if event["leaf_x"] != json!(assignment[..m]) {
                    return Err("lift leaves differ from candidate".into());
                }
                // Group success is independently checked by sealed-cell admission.
                // These booleans only enforce the adapter's stop/control flow.
                stopped = boolean(&event["group_lift_verified"])?
                    || boolean(&event["lift_budget_exhausted"])?;
            }
            _ => return Err("unknown prime-chain event kind".into()),
        }
    }
    let mut summary = replay_summary(
        examples,
        certified_nodes,
        events,
        bytes,
        reason,
        &verifier_work,
    );
    summary["algebraic_consequences"] = algebraic.summary();
    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::{f4_fp, groebner_f4::Ordering};
    use std::cell::RefCell;

    fn fixture(empty: bool) -> (Value, Vec<Value>) {
        let p = 5;
        let equations: Vec<Poly> = if empty {
            vec![]
        } else {
            (0..4)
                .map(|i| {
                    let mut e = vec![0; 4];
                    e[i] = 1;
                    vec![(e, 1), (vec![0; 4], 4)]
                })
                .collect()
        };
        let domains = vec![Some(vec![1, 2]), Some(vec![1, 2]), Some(vec![1, 2]), None];
        let input = json!({"m":3,"n_vars":4,"modulus":"5","domains":domains,
            "polynomials":equations.iter().map(|f|f.iter().map(|(e,c)|json!({"exponents":e,"coefficient":c.to_string()})).collect::<Vec<_>>()).collect::<Vec<_>>()});
        let events = RefCell::new(Vec::new());
        let report=f4_fp::solve_visit_traced(&equations,4,p,&f4_fp::F4Options::new(Ordering::Grevlex,4),&domains,
            |assignment| {events.borrow_mut().push(json!({"kind":"group_lift","leaf_x":assignment[..3],"group_lift_verified":false,"lift_budget_exhausted":false}));false},
            |event| {events.borrow_mut().push(serde_json::to_value(event).unwrap());Ok(())}).unwrap();
        assert!(report.complete);
        let rows=events.into_inner().into_iter().enumerate().map(|(i,event)|json!({"schema":"ml-dataset.prime-f4-chain-event/v1","sequence":i+1,"event":event})).collect();
        (input, rows)
    }
    fn run(input: &Value, rows: &[Value]) -> Result<(Vec<Value>, Value), String> {
        let mut bytes = Vec::new();
        for row in rows {
            serde_json::to_writer(&mut bytes, row).unwrap();
            bytes.push(b'\n');
        }
        let mut features = Vec::new();
        let report = replay(
            input,
            &json!({"verified_public_input":true}),
            std::io::Cursor::new(bytes),
            |feature, record| {
                if record["kind"] == "observed_next_branch" {
                    if let Some(feature) = feature {
                        features.push(feature);
                    }
                }
                Ok(())
            },
        )?;
        Ok((features, report))
    }
    #[test]
    fn native_linear_trace_replays_four_decisions_and_suffix_is_causal() {
        let (input, mut rows) = fixture(false);
        let (features, report) = run(&input, &rows).unwrap();
        assert_eq!(features.len(), 4);
        assert_eq!(report["ideal_equality_certified_nodes"], 4);
        let first_branch = rows
            .iter()
            .position(|r| r["event"]["kind"] == "branch")
            .unwrap();
        let (prefix, _) = run(&input, &rows[..=first_branch]).unwrap();
        assert_eq!(prefix, features[..1]);
        for row in &mut rows {
            row["private_future_metadata"] = json!({"scalar":123,"seed":17,"outcome":"positive"});
        }
        rows.last_mut().unwrap()["event"]["group_lift_verified"] = json!(true);
        assert_eq!(run(&input, &rows).unwrap().0, features);
        for feature in features {
            let rendered = feature.to_string();
            for excluded in [
                "node_id",
                "sequence",
                "candidate",
                "group_lift_verified",
                "scalar",
                "seed",
                "outcome",
            ] {
                assert!(!rendered.contains(excluded));
            }
        }
    }

    #[test]
    fn native_consequences_have_pre_native_context_and_separate_branch_counts() {
        let (input, rows) = fixture(false);
        let collect = |rows: &[Value]| {
            let mut bytes = Vec::new();
            for row in rows {
                serde_json::to_writer(&mut bytes, row).unwrap();
                bytes.push(b'\n');
            }
            let mut algebraic = Vec::new();
            let mut branches = Vec::new();
            let summary = replay(
                &input,
                &json!({"verified_public_input":true}),
                std::io::Cursor::new(bytes),
                |feature, record| {
                    if record["kind"] == "polynomial_consequence" {
                        algebraic.push((feature, record));
                    } else if record["kind"] == "observed_next_branch" {
                        branches.push(feature.unwrap());
                    }
                    Ok(())
                },
            )
            .unwrap();
            (algebraic, branches, summary)
        };
        let (algebraic, branches, summary) = collect(&rows);
        assert_eq!(branches.len(), 4);
        assert_eq!(summary["verified_branch_examples"], 4);
        assert_eq!(algebraic.len(), 10);
        assert_eq!(
            summary["algebraic_consequences"]["counts"]["admitted_nonzero_rows"],
            10
        );
        for (feature, record) in &algebraic {
            let feature = feature.as_ref().unwrap();
            assert_eq!(feature["task"], "prime-polynomial-consequence/v1");
            assert_eq!(record["admission_status"], "verified_nonzero");
            assert_eq!(record["native_operation_witness"], false);
            assert!(record["exact_first_fall_degree"].is_null());
            assert!(record["exact_last_fall_degree"].is_null());
            for forbidden in [
                "certified_target",
                "multiplier_proof",
                "source_node_id",
                "identity_audit",
            ] {
                assert!(feature.get(forbidden).is_none());
            }
        }
        assert!(algebraic[..4].iter().all(|row| row.0 == algebraic[0].0));

        // Scale an output basis row while leaving the pre-native system fixed.
        // Both nodes certify the same ideal, but their output labels differ.
        let mut changed = rows[..2].to_vec();
        changed[1]["event"]
            .as_object_mut()
            .unwrap()
            .remove("generating_set");
        for term in changed[1]["event"]["report"]["basis"][0]
            .as_array_mut()
            .unwrap()
        {
            term[1] = json!((term[1].as_u64().unwrap() * 2) % 5);
        }
        let (changed_targets, _, _) = collect(&changed);
        assert_eq!(changed_targets[0].0, algebraic[0].0);
        assert_ne!(
            changed_targets[0].1["certified_target"],
            algebraic[0].1["certified_target"]
        );
    }
    #[test]
    fn all_field_and_finite_iterators_replay_siblings_without_future_domains() {
        let (input, rows) = fixture(true);
        let (features, report) = run(&input, &rows).unwrap();
        assert!(features
            .iter()
            .any(|f| f["admissible_action_domain"]["kind"] == "field_interval"));
        assert!(features
            .iter()
            .any(|f| f["admissible_action_domain"]["remaining_values"] == json!(["2"])));
        assert_eq!(report["admission_status"], "retained_prefix_checked");
        assert_eq!(report["source_completion_claimed"], false);
    }
    #[test]
    fn forged_edges_bindings_specialization_branches_and_sequence_fail() {
        let (input, rows) = fixture(false);
        let system = rows
            .iter()
            .position(|r| r["event"]["kind"] == "system" && r["event"]["node_id"] == 1)
            .unwrap();
        let branch = rows
            .iter()
            .position(|r| r["event"]["kind"] == "branch")
            .unwrap();
        for (index, key, value) in [
            (system, "parent_id", json!(99)),
            (system, "assignments", json!([[0, 1]])),
            (system, "polynomials", json!([])),
            (branch, "value", json!(2)),
            (branch, "original_variable", json!(99)),
            (branch, "source", json!("full_field")),
        ] {
            let mut bad = rows.clone();
            bad[index]["event"][key] = value;
            assert!(run(&input, &bad).is_err(), "{key}");
        }
        let mut bad = rows.clone();
        bad[1]["sequence"] = json!(9);
        assert!(run(&input, &bad).is_err());
        let mut bad = rows.clone();
        bad[1]["event"]["report"]["inconsistent"] = json!(true);
        assert!(run(&input, &bad).is_err());
        let mut bad = rows.clone();
        bad.last_mut().unwrap()["event"]["leaf_x"] = json!([2, 2, 2]);
        assert!(run(&input, &bad).is_err());
    }
    #[test]
    fn unproved_native_basis_is_restricted_unknown_not_a_false_degree_label() {
        let (input, mut rows) = fixture(false);
        rows[1]["event"]
            .as_object_mut()
            .unwrap()
            .remove("generating_set");
        rows[1]["event"]["report"]["basis"] = json!([[[[0, 0, 0, 0], 1]]]);
        let (features, report) = run(&input, &rows).unwrap();
        assert!(features.is_empty());
        assert_eq!(report["admission_status"], "bounded_unknown_suffix");
        assert_eq!(report["exact_fall_degree_verified"], false);
    }

    #[test]
    fn polynomial_size_caps_preserve_prior_prefixes_for_system_and_native_result() {
        let (input, rows) = fixture(false);
        let (expected, _) = run(&input, &rows).unwrap();
        for (kind, field) in [("system", "polynomials"), ("native_result", "report")] {
            let index = rows
                .iter()
                .position(|row| row["event"]["kind"] == kind && row["event"]["node_id"] == 1)
                .unwrap();
            for term_cap in [false, true] {
                let mut oversized = rows.clone();
                let replacement = if term_cap {
                    // Each row is valid over F5 in the three child variables;
                    // 3277 * 5 terms exceeds the term cap but not the row cap.
                    let row = json!([
                        [[0, 0, 0], 1],
                        [[1, 0, 0], 1],
                        [[2, 0, 0], 1],
                        [[0, 1, 0], 1],
                        [[0, 2, 0], 1]
                    ]);
                    json!(vec![row; 3277])
                } else {
                    json!(vec![json!([[[1, 0, 0], 1]]); 4097])
                };
                if kind == "system" {
                    oversized[index]["event"][field] = replacement;
                } else {
                    oversized[index]["event"][field]["basis"] = replacement;
                }
                let (features, report) = run(&input, &oversized).unwrap();
                assert_eq!(features, expected[..1]);
                assert_eq!(report["verified_branch_examples"], 1);
                assert_eq!(report["ideal_equality_certified_nodes"], 1);
                assert_eq!(report["admission_status"], "bounded_unknown_suffix");
                assert_eq!(
                    report["reason"],
                    if term_cap {
                        "prefix polynomial term cap"
                    } else {
                        "prefix polynomial row cap"
                    }
                );
                assert_eq!(report["source_completion_claimed"], false);
                assert_eq!(report["exact_fall_degree_verified"], false);
            }
        }
    }

    #[test]
    fn initial_polynomial_size_cap_returns_unknown_without_admitting_events() {
        let (mut input, rows) = fixture(false);
        input["polynomials"] = json!(vec![input["polynomials"][0].clone(); 4097]);
        let (features, report) = run(&input, &rows).unwrap();
        assert!(features.is_empty());
        assert_eq!(report["admission_status"], "bounded_unknown_suffix");
        assert_eq!(report["reason"], "prefix polynomial row cap");
        assert_eq!(report["events_read"], 0);
        assert_eq!(report["ideal_equality_certified_nodes"], 0);
        assert_eq!(report["source_completion_claimed"], false);
    }

    #[test]
    fn polynomial_degree_caps_preserve_prior_prefixes_including_degree_sum_overflow() {
        let (input, rows) = fixture(false);
        let (expected, _) = run(&input, &rows).unwrap();
        for kind in ["system", "native_result"] {
            let index = rows
                .iter()
                .position(|row| row["event"]["kind"] == kind && row["event"]["node_id"] == 1)
                .unwrap();
            for exponents in [[65, 0, 0], [u32::MAX, u32::MAX, 0]] {
                let mut oversized = rows.clone();
                let replacement = json!([[[exponents, 1]]]);
                if kind == "system" {
                    oversized[index]["event"]["polynomials"] = replacement;
                } else {
                    oversized[index]["event"]["report"]["basis"] = replacement;
                }
                let (features, report) = run(&input, &oversized).unwrap();
                assert_eq!(features, expected[..1]);
                assert_eq!(report["admission_status"], "bounded_unknown_suffix");
                assert_eq!(report["reason"], "prefix polynomial degree cap");
                assert_eq!(report["ideal_equality_certified_nodes"], 1);
            }
        }
        let mut oversized_input = input.clone();
        oversized_input["polynomials"][0][0]["exponents"] = json!([65, 0, 0, 0]);
        let (features, report) = run(&oversized_input, &rows).unwrap();
        assert!(features.is_empty());
        assert_eq!(report["reason"], "prefix polynomial degree cap");
        assert_eq!(report["events_read"], 0);
    }

    #[test]
    fn malformed_polynomials_within_size_caps_remain_errors() {
        let (input, rows) = fixture(false);
        for kind in ["system", "native_result"] {
            let mut bad = rows.clone();
            let index = rows
                .iter()
                .position(|row| row["event"]["kind"] == kind && row["event"]["node_id"] == 1)
                .unwrap();
            let polynomials = if kind == "system" {
                &mut bad[index]["event"]["polynomials"]
            } else {
                &mut bad[index]["event"]["report"]["basis"]
            };
            polynomials[0][0][1] = json!(0);
            assert_eq!(run(&input, &bad).unwrap_err(), "invalid prefix polynomial");
        }
        for term in [
            json!([[1, 0], 1]),                          // Wrong dimension.
            json!([[u64::from(u32::MAX) + 1, 0, 0], 1]), // Invalid exponent encoding.
        ] {
            let mut bad = rows.clone();
            let index = rows
                .iter()
                .position(|row| {
                    row["event"]["kind"] == "native_result" && row["event"]["node_id"] == 1
                })
                .unwrap();
            bad[index]["event"]["report"]["basis"] = json!([[term]]);
            assert!(run(&input, &bad).is_err());
        }
        let mut duplicate = rows.clone();
        let index = rows
            .iter()
            .position(|row| row["event"]["kind"] == "native_result" && row["event"]["node_id"] == 1)
            .unwrap();
        duplicate[index]["event"]["report"]["basis"] = json!([[[[1, 0, 0], 1], [[1, 0, 0], 1]]]);
        assert!(run(&input, &duplicate).is_err());
        let mut bad_input = input.clone();
        bad_input["polynomials"][0][0]["coefficient"] = json!("0");
        assert_eq!(
            run(&bad_input, &rows).unwrap_err(),
            "invalid prefix polynomial"
        );
    }
    // Retained failures remain failures: these are the original first two
    // events, not regenerated substitutes for the frozen campaign.
    fn retained_generator_fixtures() -> Vec<Value> {
        serde_json::from_str(include_str!(
            "../../research/ml_dataset_cli_generator_preservation_20261101/fixtures/root-systems.json"
        )).unwrap()
    }

    #[test]
    fn preserved_generators_retained_legacy_roots_remain_unknown() {
        let fixtures = retained_generator_fixtures();
        assert_eq!(fixtures.len(), 4);
        for fixture in fixtures {
            let rows = fixture["root_events"].as_array().unwrap();
            assert!(rows[1]["event"].get("generating_set").is_none());
            let (features, summary) = run(&fixture["input"], rows).unwrap();
            assert!(features.is_empty());
            assert_eq!(summary["admission_status"], "bounded_unknown_suffix");
            assert_eq!(summary["ideal_equality_certified_nodes"], 0);
        }
    }

    #[test]
    fn preserved_generators_retained_inputs_certify_and_specialize_same_rows() {
        for fixture in retained_generator_fixtures() {
            let input = &fixture["input"];
            let n = input["n_vars"].as_u64().unwrap() as usize;
            let p = input["modulus"].as_str().unwrap().parse().unwrap();
            let equations = polys(&input["polynomials"], n, p, true)
                .unwrap_or_else(|e| panic!("{}", e.message()));
            let domains: Vec<Option<Vec<u64>>> =
                serde_json::from_value(input["domains"].clone()).unwrap();
            let mut opts = f4_fp::F4Options::new(Ordering::Grevlex, 3)
                .with_budget(std::time::Duration::from_millis(5000));
            opts.max_pairs_per_step = Some(256);
            let mut events = Vec::new();
            let report = f4_fp::solve_visit_traced(
                &equations,
                n,
                p,
                &opts,
                &domains,
                |_| false,
                |event| {
                    let value = serde_json::to_value(event).unwrap();
                    let stop = value["kind"] == "system" && value["node_id"] != 0;
                    events.push(value);
                    if stop {
                        Err("focused test stops at first specialized child".into())
                    } else {
                        Ok(())
                    }
                },
            )
            .unwrap();
            assert!(report.trace_stopped);
            let rows: Vec<Value> = events.into_iter().enumerate().map(|(i,event)|
                json!({"schema":"ml-dataset.prime-f4-chain-event/v1","sequence":i+1,"event":event})
            ).collect();
            assert_eq!(
                rows[1]["event"]["report"]["basis"],
                fixture["root_events"][1]["event"]["report"]["basis"]
            );
            let kept = polys(&rows[1]["event"]["generating_set"], n, p, false)
                .unwrap_or_else(|e| panic!("{}", e.message()));
            assert!(equations.iter().all(|f| kept.contains(f)));
            let (features, summary) = run(input, &rows).unwrap();
            assert_eq!(features.len(), 1);
            assert_eq!(summary["ideal_equality_certified_nodes"], 1);
            assert!(
                summary["algebraic_consequences"]["counts"]["admitted_nonzero_rows"]
                    .as_u64()
                    .unwrap()
                    > 0
            );
            assert_eq!(summary["admission_status"], "retained_prefix_checked");
            // Algebraic provenance indexes only the unchanged raw native
            // report, never appended incoming constraints.
            let bytes = rows
                .iter()
                .map(|row| format!("{row}\n"))
                .collect::<String>();
            let mut native_target_indices = Vec::new();
            replay(
                input,
                &json!({"verified_public_input":true}),
                std::io::Cursor::new(bytes),
                |_, record| {
                    if record["kind"] == "polynomial_consequence" {
                        native_target_indices.push(record["source_basis_row"].as_u64().unwrap());
                    }
                    Ok(())
                },
            )
            .unwrap();
            let raw_len = rows[1]["event"]["report"]["basis"]
                .as_array()
                .unwrap()
                .len();
            assert!(kept.len() > raw_len);
            assert_eq!(
                native_target_indices,
                (0..raw_len as u64).collect::<Vec<_>>()
            );
            // Independently checked child specialization must bind the same
            // enlarged generators, including the deferred quartic constraints.
            let mut bad_child = rows.clone();
            bad_child.last_mut().unwrap()["event"]["polynomials"] = json!([]);
            assert!(run(input, &bad_child).is_err());
            // Neither a missing input nor a changed coefficient can be silently
            // substituted for the explicit native-plus-input contract.
            let mut missing = rows.clone();
            missing[1]["event"]["generating_set"]
                .as_array_mut()
                .unwrap()
                .pop();
            assert!(run(input, &missing).is_err());
            let mut changed = rows.clone();
            let c = changed[1]["event"]["generating_set"][0][0][1]
                .as_u64()
                .unwrap();
            changed[1]["event"]["generating_set"][0][0][1] = json!(if c == 1 { 2 } else { 1 });
            assert!(run(input, &changed).is_err());
            let mut mismatched = rows.clone();
            mismatched[1]["event"]["report"]["basis"] = json!([]);
            assert!(run(input, &mismatched).is_err());
        }
    }
}
