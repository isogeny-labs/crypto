//! Complete signed-orbit populations. Construction history and diagnostic logs
//! are restricted artifacts; materialized actions do not authorize quotients.
use super::*;
use crate::ml_dataset::types::plan_v2::FactorBasePolicy;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

const SCHEMA: &str = "ml-ecdlp-factor-base/v2";
const RESERVE: u64 = 4096;
const ENTRY_LOGICAL_BYTES: u64 = 128;

#[derive(Clone, Debug)]
pub struct PolicyBase {
    pub factor_base: Value,
    pub restricted_construction: Value,
}

#[derive(Clone, Copy)]
struct Limits {
    candidates: u64,
    points: usize,
    steps: u64,
    bytes: u64,
}
fn limits(policy: &FactorBasePolicy, requested: usize) -> Result<Limits, String> {
    policy.validate_requested_points(requested)?;
    match *policy {
        FactorBasePolicy::SignedFrobeniusUnionV1 {
            max_candidates,
            max_points,
            max_action_steps,
            max_artifact_bytes,
        } => Ok(Limits {
            candidates: max_candidates,
            points: max_points,
            steps: max_action_steps,
            bytes: max_artifact_bytes,
        }),
        _ => Err("legacy policy must use the unchanged curve generator".into()),
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Costs {
    /// Mixed, conservative admission units; never reported as native group ops.
    admission_units: u64,
    lift_calls: u64,
    scalar_calls: u64,
    frobenius_calls: u64,
    negation_calls: u64,
    membership_baby_additions: u64,
    membership_giant_additions: u64,
    membership_queries: u64,
    independent_log_scalar_checks: u64,
}
impl Costs {
    fn fits(&self, amount: u64, cap: u64) -> bool {
        self.admission_units
            .checked_add(amount)
            .is_some_and(|n| n <= cap)
    }
    fn scalar(&mut self, scalar: u64) {
        self.scalar_calls += 1;
        self.admission_units += scalar_bound(scalar);
    }
}
fn scalar_bound(k: u64) -> u64 {
    2 * u64::from(u64::BITS - k.leading_zeros())
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Orbit {
    representative: usize,
    indices: Vec<usize>,
    /// Canonical witnesses in (sign, exponent) lexicographic order.
    witnesses: Vec<(bool, u32)>,
    stabilizer: Vec<(bool, u32)>,
}

#[derive(Default, Clone)]
struct Population {
    points: Vec<WPoint>,
    negation: Vec<usize>,
    frobenius: Vec<usize>,
    orbits: Vec<Orbit>,
}

fn action(public: &Value, c: &Arithmetic) -> Result<(u32, u32), String> {
    let d = number(&public["symmetry"]["frobenius_power"], "Frobenius power")?;
    match c {
        Arithmetic::Prime(_) if d == 1 => Ok((1, 1)),
        Arithmetic::Binary { fast, .. }
            if d > 0 && d <= u64::from(fast.n) && u64::from(fast.n).is_multiple_of(d) =>
        {
            Ok((d as u32, fast.n / d as u32))
        }
        _ => Err("invalid model-preserving action".into()),
    }
}

fn check_representation(public: &Value) -> Result<(), String> {
    let (model, basis) = if public["family"] == "prime" {
        ("short-weierstrass", "canonical-residue")
    } else {
        ("binary-ordinary", "polynomial-lsb-first")
    };
    if public["curve"]["model"] != model || public["field"]["basis"] != basis {
        return Err("model/basis differs from verified arithmetic representation".into());
    }
    Ok(())
}

/// A separate generic field implementation checks the fast Frobenius edges.
fn reference_frobenius(public: &Value, p: WPoint, d: u32) -> Result<WPoint, String> {
    if public["family"] == "prime" {
        return Ok(p);
    }
    let c = binary_curve(public)?;
    Ok(p.map(|(x, y)| {
        let x = F2mElement::from_biguint(&x.into(), c.m).square_k_times(d, &c.irreducible);
        let y = F2mElement::from_biguint(&y.into(), c.m).square_k_times(d, &c.irreducible);
        (
            x.to_biguint().to_u64().expect("word field"),
            y.to_biguint().to_u64().expect("word field"),
        )
    }))
}

fn make_orbit(c: &Arithmetic, p: WPoint, d: u32, t: u32, costs: &mut Costs) -> Population {
    let mut set = BTreeSet::new();
    let mut current = p;
    for _ in 0..t {
        set.insert(current);
        set.insert(c.neg(current));
        current = c.frobenius(current, d);
        costs.negation_calls += 1;
        costs.frobenius_calls += 1;
        costs.admission_units += 2;
    }
    let points: Vec<_> = set.into_iter().collect();
    let representative = points[0];
    let mut witnesses = BTreeMap::new();
    let mut stabilizer = Vec::new();
    for sign in [false, true] {
        let mut current = representative;
        for exponent in 0..t {
            let mapped = if sign {
                costs.negation_calls += 1;
                costs.admission_units += 1;
                c.neg(current)
            } else {
                current
            };
            witnesses.entry(mapped).or_insert((sign, exponent));
            if mapped == representative {
                stabilizer.push((sign, exponent));
            }
            current = c.frobenius(current, d);
            costs.frobenius_calls += 1;
            costs.admission_units += 1;
        }
    }
    let lookup: BTreeMap<_, _> = points
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect();
    let mut negation = Vec::new();
    let mut frobenius = Vec::new();
    for &point in &points {
        negation.push(lookup[&c.neg(point)]);
        frobenius.push(lookup[&c.frobenius(point, d)]);
        costs.negation_calls += 1;
        costs.frobenius_calls += 1;
        costs.admission_units += 2;
    }
    let orbit = Orbit {
        representative: 0,
        indices: (0..points.len()).collect(),
        witnesses: points.iter().map(|p| witnesses[p]).collect(),
        stabilizer,
    };
    Population {
        points,
        negation,
        frobenius,
        orbits: vec![orbit],
    }
}

impl Population {
    fn append(&mut self, mut other: Self) {
        let offset = self.points.len();
        self.points.append(&mut other.points);
        self.negation
            .extend(other.negation.into_iter().map(|i| i + offset));
        self.frobenius
            .extend(other.frobenius.into_iter().map(|i| i + offset));
        for mut orbit in other.orbits {
            orbit.representative += offset;
            for i in &mut orbit.indices {
                *i += offset;
            }
            self.orbits.push(orbit);
        }
    }
    fn definition(
        &self,
        public: &Value,
        policy: &FactorBasePolicy,
        requested: usize,
        d: u32,
        t: u32,
    ) -> Value {
        let points: Vec<_> = self.points.iter().copied().map(point_json).collect();
        let action = json!({"kind":"expanded-signed-model-frobenius/v1","frobenius_power":d,
            "ambient_frobenius_order":t,"abstract_group":"C2-times-Ct","quotient_columns":false});
        let anchor = self.points.iter().copied().min().flatten();
        let (field, model) = match arithmetic(public).expect("verified arithmetic context") {
            Arithmetic::Prime(c) => (
                json!({"characteristic":u(c.f.p),"modulus":u(c.f.p),"degree":1,"basis":"canonical-residue"}),
                "short-weierstrass",
            ),
            Arithmetic::Binary { fast, .. } => (
                json!({"characteristic":"2","degree":fast.n,
                "irreducible_low_terms":(0..fast.n).filter(|i| fast.gf.irr & (1u64 << i) != 0).collect::<Vec<_>>(),
                "basis":"polynomial-lsb-first"}),
                "binary-ordinary",
            ),
        };
        let math = json!({"schema":"ml-dataset.factor-base-domain/v1","field":field,
            "model":model,"a":public["curve"]["a"],"b":public["curve"]["b"],
            "subgroup_order":public["curve"]["subgroup_order"],"subgroup_anchor":point_json(anchor),
            "points":points,"action":action});
        let domain_digest = (!self.points.is_empty())
            .then(|| crate::ml_dataset::digest(&serde_json::to_vec(&math).expect("JSON value")));
        json!({"schema":SCHEMA,"policy":policy,"requested_size":requested,"actual_size":self.points.len(),
            "points":points,"action":action,"negation_action":self.negation,"frobenius_action":self.frobenius,
            "signed_orbits":self.orbits,"negation_closed":!self.points.is_empty(),"frobenius_closed":!self.points.is_empty(),
            "domain_digest":domain_digest,"empty_domain_admitted":false})
    }
}

fn encode_len(value: &Value) -> Result<u64, String> {
    // The storage layer uses pretty JSON and one newline; admit that exact form.
    // Count without allocating a serialization copy of an untrusted input.
    struct Counter(u64, bool);
    impl std::io::Write for Counter {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            let n = self
                .0
                .checked_add(bytes.len() as u64)
                .ok_or_else(|| std::io::Error::other("artifact size overflow"))?;
            if n >= 67_108_864 {
                self.1 = true;
                return Err(std::io::Error::other(
                    "artifact exceeds absolute byte ceiling",
                ));
            }
            self.0 = n;
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut counter = Counter(0, false);
    if let Err(error) = serde_json::to_writer_pretty(&mut counter, value) {
        if counter.1 {
            return Ok(67_108_865);
        }
        return Err(error.to_string());
    }
    Ok(counter.0 + 1)
}

fn restricted(
    rows: &[Value],
    costs: &Costs,
    setup: &Value,
    terminal: &str,
    pending: Value,
    committed_x: u64,
) -> Value {
    json!({"schema":"ml-dataset.factor-base-construction/v1","candidate_rows":rows,
        "committed_x":committed_x,"terminal_status":terminal,"pending_candidate":pending,
        "subgroup_action":setup,"costs":costs,
        "cost_scope":"constructor calls and membership additions; admission_units mix conservative scalar bounds and counted calls; excludes public-context verification, serialization, replay, field internals and instructions",
        "retired_instructions":null,"whole_cost_verified":false,"native_execution_trace_complete":false,
        "replay_scope":"native deterministic reconstruction plus independent reference projection, scalar and field-action checks",
        "mathematical_replay_limits":{"admission_units":10000000,"logical_membership_bytes":67108864,
            "excludes":"public-context verification, serialization and deterministic-construction replay, each separately bounded by its input/policy"}})
}

fn fits_artifacts(base: &Value, history: &Value, cap: u64) -> Result<bool, String> {
    Ok(encode_len(base)?
        .checked_add(encode_len(history)?)
        .and_then(|n| n.checked_add(RESERVE))
        .is_some_and(|n| n <= cap))
}

/// Construct only the signed policy. Context verification is a separate scope.
pub fn construct(
    public: &Value,
    policy: &FactorBasePolicy,
    requested_size: usize,
) -> Result<PolicyBase, String> {
    let cap = limits(policy, requested_size)?;
    verify_public(public)?;
    check_representation(public)?;
    let c = arithmetic(public)?;
    let (d, t) = action(public, &c)?;
    let r = number(&public["curve"]["subgroup_order"], "subgroup order")?;
    let order = number(&public["curve"]["group_order"], "group order")?;
    let g = point_words(&public["generator"])?;
    let mut population = Population::default();
    let mut rows = Vec::new();
    let mut costs = Costs::default();
    let mut setup = json!({"method":"unique-order-r-subgroup","preservation_verified":true,"logical_table_bytes":0});
    let mut terminal = "candidate_cap";
    let mut pending = Value::Null;
    let mut committed_x = 0;
    let mut membership = None;

    if (order / r).is_multiple_of(r) {
        let stride = sqrt_floor(r) + 1;
        let table_bytes = stride
            .checked_mul(ENTRY_LOGICAL_BYTES)
            .ok_or("membership bound overflow")?;
        let query = order.min(r) / stride + 1;
        let admission = stride + query + scalar_bound(r - 1) + 2;
        setup = json!({"method":"bounded-cyclic-membership","preservation_verified":false,
            "logical_table_bytes":table_bytes,"logical_entry_allowance":ENTRY_LOGICAL_BYTES,
            "logical_storage_is_rss":false,"maximum_query_additions":query});
        if table_bytes > cap.bytes || !costs.fits(admission, cap.steps) {
            terminal = "membership_budget_exhausted";
            pending = json!({"stage":"subgroup_action","refused_admission_units":admission,"refused_logical_bytes":table_bytes});
        } else {
            let check = CyclicMembership::new(&c, g, r);
            costs.membership_baby_additions += stride;
            costs.admission_units += stride + 1;
            costs.negation_calls += 1;
            let image = c.frobenius(g, d);
            costs.frobenius_calls += 1;
            costs.admission_units += 1;
            let (log, additions) = check.logarithm_counted(&c, image);
            costs.membership_queries += 1;
            costs.membership_giant_additions += additions;
            costs.admission_units += additions;
            setup["frobenius_generator_log"] = json!(log.map(|x| x.to_string()));
            if let Some(log) = log {
                costs.scalar(log);
                costs.independent_log_scalar_checks += 1;
                if c.reference_mul(g, log) != image {
                    return Err("subgroup action scalar replay failed".into());
                }
                setup["preservation_verified"] = json!(true);
                membership = Some(check);
            } else {
                terminal = "unsupported_subgroup_action";
            }
        }
    }

    if setup["preservation_verified"] == true {
        let last_x = cap.candidates.min(c.q() - 1);
        for x in 1..=last_x {
            // Lift, at most two projections, order checks, membership checks,
            // and two full orbit constructions are admitted before this x.
            let mut clearing = order;
            let mut multiplicity = 0;
            while clearing.is_multiple_of(r) {
                clearing /= r;
                multiplicity += 1;
            }
            let projection_bound = scalar_bound(clearing) + (multiplicity - 1) * scalar_bound(r);
            let query_bound = membership
                .as_ref()
                .map_or(0, |m| r / m.stride + 1 + scalar_bound(r - 1));
            // make_orbit uses at most 9t calls (5t plus 2 per <=2t points).
            let admission =
                1 + 2 * (projection_bound + scalar_bound(r) + query_bound + 9 * u64::from(t));
            if !costs.fits(admission, cap.steps) {
                terminal = "action_cap";
                pending = json!({"x":u(x),"next_lift":0,"stage":"candidate_admission","refused_admission_units":admission});
                break;
            }
            costs.lift_calls += 1;
            costs.admission_units += 1;
            let mut lifts = c.lift(x);
            lifts.sort_unstable();
            if lifts.len() > 2 {
                return Err("more than two rational lifts".into());
            }
            let mut proposed = population.clone();
            let mut lifted_rows = Vec::new();
            for p in lifts {
                // Same prime-primary projection schedule as the legacy primitive,
                // with each executed scalar call admitted and recorded once.
                let mut current = c.mul(p, clearing);
                let mut multiplier = clearing;
                costs.scalar(clearing);
                for _ in 1..multiplicity {
                    if current.is_none() {
                        break;
                    }
                    costs.scalar(r);
                    let next = c.mul(current, r);
                    if next.is_none() {
                        break;
                    }
                    current = next;
                    multiplier *= r;
                }
                let projected = current;
                costs.scalar(r);
                if c.mul(projected, r).is_some() {
                    return Err("projection is not r-torsion".into());
                }
                let mut log = None;
                let member = if projected.is_none() {
                    false
                } else if let Some(check) = &membership {
                    let (found, additions) = check.logarithm_counted(&c, projected);
                    costs.membership_queries += 1;
                    costs.membership_giant_additions += additions;
                    costs.admission_units += additions;
                    log = found;
                    if let Some(k) = found {
                        costs.scalar(k);
                        costs.independent_log_scalar_checks += 1;
                        if c.reference_mul(g, k) != projected {
                            return Err("membership scalar replay failed".into());
                        }
                    }
                    found.is_some()
                } else {
                    true
                };
                let duplicate = member && proposed.points.contains(&projected);
                if member && !duplicate {
                    proposed.append(make_orbit(&c, projected, d, t, &mut costs));
                }
                lifted_rows.push(
                    json!({"lifted":point_json(p),"projected":point_json(projected),
                    "projection_multiplier":u(multiplier),"cyclic_subgroup_member":member,
                    "membership_log":log.map(|v|v.to_string()),"duplicate_orbit":duplicate}),
                );
            }
            if proposed.points.len() > cap.points {
                terminal = "point_cap";
                pending = json!({"x":u(x),"stage":"atomic_candidate_commit","completed_lifts":lifted_rows.len(),"lifts":lifted_rows,"proposed_points":proposed.points.len()});
                break;
            }
            let row = json!({"x":u(x),"lifts":lifted_rows});
            rows.push(row);
            let base = proposed.definition(public, policy, requested_size, d, t);
            let history = restricted(&rows, &costs, &setup, "candidate_pending", Value::Null, x);
            if !fits_artifacts(&base, &history, cap.bytes)? {
                let refused = rows.pop().expect("just appended candidate");
                terminal = "artifact_cap";
                pending = json!({"x":u(x),"stage":"atomic_candidate_commit","lifts":refused["lifts"],"proposed_points":proposed.points.len()});
                break;
            }
            population = proposed;
            committed_x = x;
            if population.points.len() as u64 == r - 1 {
                terminal = "subgroup_exhausted";
                break;
            }
            if population.points.len() >= requested_size {
                terminal = "requested_size_reached";
                break;
            }
            if x == c.q() - 1 {
                terminal = "field_exhausted";
            }
        }
    }
    let base = population.definition(public, policy, requested_size, d, t);
    let history = restricted(&rows, &costs, &setup, terminal, pending, committed_x);
    if encode_len(&base)?
        .checked_add(encode_len(&history)?)
        .is_none_or(|n| n > cap.bytes)
    {
        return Err("policy minimum artifact budget cannot retain terminal metadata".into());
    }
    Ok(PolicyBase {
        factor_base: base,
        restricted_construction: history,
    })
}

/// Mathematical checks only. Worker admission must also call `verify` with the
/// expected plan policy and restricted construction, not trust embedded policy.
pub fn verify_mathematical(public: &Value, base: &Value) -> Result<(), String> {
    if base["schema"] != SCHEMA {
        return Err("invalid v2 factor-base schema".into());
    }
    let serialized_bytes = encode_len(base)?;
    if serialized_bytes > 67_108_864 {
        return Err("oversized policy definition".into());
    }
    let policy: FactorBasePolicy =
        serde_json::from_value(base["policy"].clone()).map_err(|e| e.to_string())?;
    let requested = base["requested_size"]
        .as_u64()
        .and_then(|n| usize::try_from(n).ok())
        .ok_or("invalid requested size")?;
    let cap = limits(&policy, requested)?;
    if serialized_bytes > cap.bytes {
        return Err("oversized policy definition".into());
    }
    verify_public(public)?;
    check_representation(public)?;
    let c = arithmetic(public)?;
    let (d, t) = action(public, &c)?;
    let r = number(&public["curve"]["subgroup_order"], "subgroup order")?;
    let rows = base["points"].as_array().ok_or("missing points")?;
    if rows.len() > cap.points {
        return Err("point cap exceeded".into());
    }
    let points: Vec<_> = rows.iter().map(point_words).collect::<Result<_, _>>()?;
    let replay_limit = 10_000_000u64;
    // Conservative call/scalar bounds for every point even if every supplied
    // point starts a distinct orbit; not a measured field-operation total.
    let replay_base_bound = points.len() as u64 * (scalar_bound(r) + 11 * u64::from(t) + 4);
    if replay_base_bound > replay_limit {
        return Err("replay_budget_exhausted: population checks".into());
    }
    let mut unique = BTreeSet::new();
    for &point in &points {
        if point.is_none()
            || !c.on_curve(point)
            || !unique.insert(point)
            || c.reference_mul(point, r).is_some()
        {
            return Err("invalid exact-order-r population".into());
        }
    }
    let mut reconstructed = Population::default();
    for &point in &points {
        if reconstructed.points.contains(&point) {
            continue;
        }
        let mut diagnostic = Costs::default();
        let orbit = make_orbit(&c, point, d, t, &mut diagnostic);
        if orbit.points.first().copied() != Some(point) {
            return Err("noncanonical orbit representative".into());
        }
        if orbit.points.iter().any(|p| !unique.contains(p))
            || reconstructed.points.len() + orbit.points.len() > points.len()
        {
            return Err("incomplete population orbit".into());
        }
        reconstructed.append(orbit);
    }
    if reconstructed.points != points
        || reconstructed.definition(public, &policy, requested, d, t) != *base
    {
        return Err("factor-base action, partition, metadata or domain digest differs".into());
    }
    for (i, &p) in points.iter().enumerate() {
        let n = reconstructed.negation[i];
        let f = reconstructed.frobenius[i];
        if c.reference_add(p, points[n]).is_some()
            || reference_frobenius(public, p, d)? != points[f]
            || reconstructed.negation[n] != i
            || reconstructed.frobenius[n] != reconstructed.negation[f]
        {
            return Err("independent action check failed".into());
        }
        let mut j = i;
        for _ in 0..t {
            j = reconstructed.frobenius[j];
        }
        if j != i {
            return Err("Frobenius action order failed".into());
        }
    }
    for orbit in &reconstructed.orbits {
        if orbit.indices.len() * orbit.stabilizer.len() != 2 * t as usize {
            return Err("orbit stabilizer equation failed".into());
        }
    }
    // r-torsion alone does not certify membership in the selected H when r²|N.
    // This is a separate fixed replay budget, never a constructor cost label.
    let order = number(&public["curve"]["group_order"], "group order")?;
    if !points.is_empty() && (order / r).is_multiple_of(r) {
        let stride = sqrt_floor(r) + 1;
        let storage = stride
            .checked_mul(ENTRY_LOGICAL_BYTES)
            .ok_or("replay bound overflow")?;
        if storage > 67_108_864 || replay_base_bound + stride + 2 > replay_limit {
            return Err("replay_budget_exhausted: membership setup".into());
        }
        let g = point_words(&public["generator"])?;
        let membership = CyclicMembership::new(&c, g, r);
        let mut used = replay_base_bound + stride + 2;
        let mut queries = vec![reference_frobenius(public, g, d)?];
        queries.extend(
            reconstructed
                .orbits
                .iter()
                .map(|o| points[o.representative]),
        );
        for p in queries {
            let bound = r / stride + 1 + scalar_bound(r - 1);
            if used.checked_add(bound).is_none_or(|n| n > replay_limit) {
                return Err("replay_budget_exhausted: membership query".into());
            }
            let (log, additions) = membership.logarithm_counted(&c, p);
            used += additions;
            let log = log.ok_or("population/action leaves selected cyclic subgroup")?;
            used += scalar_bound(log);
            if c.reference_mul(g, log) != p {
                return Err("independent subgroup membership check failed".into());
            }
        }
    }
    Ok(())
}

/// Full admission: expected policy/request, bounded deterministic history, plus
/// independent mathematical and projection checks. Replay cost is not folded
/// into constructor cost labels.
pub fn verify(
    public: &Value,
    policy: &FactorBasePolicy,
    requested_size: usize,
    base: &Value,
    history: &Value,
) -> Result<(), String> {
    let cap = limits(policy, requested_size)?;
    if encode_len(base)?
        .checked_add(encode_len(history)?)
        .is_none_or(|n| n > cap.bytes)
    {
        return Err("oversized construction artifacts".into());
    }
    if base["policy"] != serde_json::to_value(policy).map_err(|e| e.to_string())?
        || base["requested_size"] != json!(requested_size)
    {
        return Err("factor base differs from expected policy/request".into());
    }
    let expected = construct(public, policy, requested_size)?;
    if expected.factor_base != *base || expected.restricted_construction != *history {
        return Err("deterministic construction/counter/terminal replay differs".into());
    }
    verify_mathematical(public, base)?;
    let c = arithmetic(public)?;
    let order = number(&public["curve"]["group_order"], "group order")?;
    let r = number(&public["curve"]["subgroup_order"], "subgroup order")?;
    let committed = history["candidate_rows"]
        .as_array()
        .ok_or("missing candidate rows")?;
    let pending = history
        .get("pending_candidate")
        .filter(|row| row.get("lifts").is_some());
    for row in committed.iter().chain(pending) {
        for lift in row["lifts"].as_array().ok_or("missing lift rows")? {
            let p = point_words(&lift["lifted"])?;
            let (projected, multiplier) = prime_projection(&c, p, order, r, true);
            if projected != point_words(&lift["projected"])?
                || u(multiplier) != lift["projection_multiplier"]
            {
                return Err("independent projection replay differs".into());
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(family: &str, n: u32, twist: bool) -> CaseConfig {
        CaseConfig {
            family: family.into(),
            n,
            m: 2,
            twist,
            curve_seed: 7,
            generator_index: 1,
            key_index: 1,
            seed: 42,
            factor_base_size: 2,
            degree_bound: 4,
            work_limit: 512,
            solver: "enumerate".into(),
        }
    }
    fn policy() -> FactorBasePolicy {
        FactorBasePolicy::signed_frobenius_union_v1_defaults()
    }
    fn fixture(name: &str, file: &str) -> Vec<u8> {
        std::fs::read(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("research/ml_dataset_factor_base_policy_20261030/legacy-fixtures")
                .join(name)
                .join(file),
        )
        .unwrap()
    }
    fn pretty(value: &Value) -> Vec<u8> {
        let mut result = serde_json::to_vec_pretty(value).unwrap();
        result.push(b'\n');
        result
    }

    #[test]
    fn historical_legacy_curve_and_twist_bytes_are_preserved() {
        // These bytes were retained by producer f4249f62, not synthesized by
        // this refactor. Fixture provenance records its exact historical scope.
        for name in ["62", "63"] {
            let cfg: CaseConfig = serde_json::from_slice(&fixture(name, "config.json")).unwrap();
            for case in [
                generate(&cfg).unwrap(),
                generate_with_policy(&cfg, &FactorBasePolicy::LegacySmallXProjectedV1).unwrap(),
            ] {
                assert_eq!(pretty(&case.public_instance), fixture(name, "public.json"));
                assert_eq!(pretty(&case.factor_base), fixture(name, "factor-base.json"));
                assert_eq!(pretty(&case.private_witness), fixture(name, "witness.json"));
            }
        }
    }

    #[test]
    fn signed_policy_replays_all_families_and_twists() {
        for family in ["koblitz", "binary", "prime"] {
            for twist in [false, true] {
                let case = generate_with_policy(&config(family, 7, twist), &policy()).unwrap();
                verify(
                    &case.public_instance,
                    &policy(),
                    2,
                    &case.factor_base,
                    &case.private_witness["factor_base_policy_construction"],
                )
                .unwrap();
                assert!(!case.factor_base["points"].as_array().unwrap().is_empty());
                assert_eq!(case.factor_base["action"]["quotient_columns"], false);
                if family == "prime" {
                    assert_eq!(case.factor_base["action"]["ambient_frobenius_order"], 1);
                }
            }
        }
    }

    #[test]
    fn model_power_uses_actual_even_degree_twist() {
        let case = generate_with_policy(&config("koblitz", 12, true), &policy()).unwrap();
        assert!(
            case.public_instance["symmetry"]["frobenius_power"]
                .as_u64()
                .unwrap()
                > 1
        );
        assert_eq!(
            case.factor_base["action"]["frobenius_power"],
            case.public_instance["symmetry"]["frobenius_power"]
        );
        let mut forged = case.public_instance.clone();
        forged["symmetry"]["frobenius_power"] = json!(1);
        assert!(construct(&forged, &policy(), 2).is_err());
    }

    #[test]
    fn policy_caps_keep_empty_or_complete_populations_and_exact_terminals() {
        let public = generate(&config("prime", 7, true)).unwrap().public_instance;
        let caps = [
            (
                FactorBasePolicy::SignedFrobeniusUnionV1 {
                    max_candidates: 1,
                    max_points: 4096,
                    max_action_steps: 10_000_000,
                    max_artifact_bytes: 67_108_864,
                },
                64,
                "candidate_cap",
            ),
            (
                FactorBasePolicy::SignedFrobeniusUnionV1 {
                    max_candidates: 100,
                    max_points: 4096,
                    max_action_steps: 1,
                    max_artifact_bytes: 67_108_864,
                },
                2,
                "action_cap",
            ),
            (
                FactorBasePolicy::SignedFrobeniusUnionV1 {
                    max_candidates: 100,
                    max_points: 1,
                    max_action_steps: 10_000_000,
                    max_artifact_bytes: 67_108_864,
                },
                1,
                "point_cap",
            ),
            (
                FactorBasePolicy::SignedFrobeniusUnionV1 {
                    max_candidates: 100,
                    max_points: 4096,
                    max_action_steps: 10_000_000,
                    max_artifact_bytes: 4096,
                },
                2,
                "artifact_cap",
            ),
        ];
        for (p, requested, status) in caps {
            let built = construct(&public, &p, requested).unwrap();
            assert_eq!(built.restricted_construction["terminal_status"], status);
            verify(
                &public,
                &p,
                requested,
                &built.factor_base,
                &built.restricted_construction,
            )
            .unwrap();
            if built.factor_base["actual_size"] == 0 {
                assert!(built.factor_base["domain_digest"].is_null());
                assert_eq!(built.factor_base["frobenius_closed"], false);
            }
            let mut changed = built.restricted_construction.clone();
            changed["terminal_status"] = json!("requested_size_reached");
            assert!(verify(&public, &p, requested, &built.factor_base, &changed).is_err());
        }
    }

    #[test]
    fn field_exhaustion_and_subgroup_exhaustion_are_distinct() {
        // y²=x³+4 over F7 has only O,(0,2),(0,5). The frozen candidate
        // sequence starts at x=1, so this is a genuine empty field exhaustion.
        let public = json!({"schema":"ml-ecdlp-instance/v1","family":"prime","n":3,"m":2,"twist":false,
            "field":{"characteristic":"7","modulus":"7","degree":1,"basis":"canonical-residue"},
            "curve":{"model":"short-weierstrass","a":"0","b":"4","base_a":"0","base_b":"4","twist_parameter":"3",
                "group_order":"3","subgroup_order":"3","cofactor":"1","group_order_factorization":[{"prime":"3","exponent":1}],
                "order_certificate":{"method":"exact-small-field-exhaustive-v1","field_cardinality":"7","x_point_counts":[2,0,0,0,0,0,0]}},
            "generator":{"x":"0","y":"2"},"target":{"x":"0","y":"2"},
            "symmetry":{"coefficient_field_degree":1,"frobenius_power":1}});
        let built = construct(&public, &policy(), 2).unwrap();
        assert_eq!(
            built.restricted_construction["terminal_status"],
            "field_exhausted"
        );
        assert_eq!(built.restricted_construction["committed_x"], 6);
        verify(
            &public,
            &policy(),
            2,
            &built.factor_base,
            &built.restricted_construction,
        )
        .unwrap();
        let other = generate(&config("prime", 7, true)).unwrap().public_instance;
        let built = construct(&other, &policy(), 64).unwrap();
        assert_eq!(
            built.restricted_construction["terminal_status"],
            "subgroup_exhausted"
        );
        assert_eq!(built.factor_base["actual_size"], 36);
    }

    #[test]
    fn mathematical_identity_excludes_generator_target_and_metadata() {
        let public = generate(&config("prime", 7, true)).unwrap().public_instance;
        let original = construct(&public, &policy(), 2).unwrap();
        let c = arithmetic(&public).unwrap();
        let mut changed = public.clone();
        changed["generator"] = point_json(c.mul(point_words(&public["generator"]).unwrap(), 2));
        changed["target"] = changed["generator"].clone();
        changed["m"] = json!(15);
        changed["field"]["restricted_seed_marker"] = json!("must-not-change-domain");
        let new = construct(&changed, &policy(), 2).unwrap();
        assert_eq!(original.factor_base, new.factor_base);
        changed["field"]["basis"] = json!("wrong-coordinate-convention");
        assert!(construct(&changed, &policy(), 2).is_err());
    }

    #[test]
    fn rehashed_semantic_mutations_are_rejected() {
        let public = generate(&config("koblitz", 7, false))
            .unwrap()
            .public_instance;
        let original = construct(&public, &policy(), 2).unwrap();
        let mutations = [
            "negation",
            "frobenius",
            "stabilizer",
            "point",
            "policy",
            "domain",
            "extra",
        ];
        for kind in mutations {
            let mut base = original.factor_base.clone();
            match kind {
                "negation" => base["negation_action"][0] = json!(0),
                "frobenius" => base["frobenius_action"][0] = json!(99999),
                "stabilizer" => base["signed_orbits"][0]["stabilizer"] = json!([]),
                "point" => {
                    base["points"].as_array_mut().unwrap().pop();
                }
                "policy" => base["policy"]["max_candidates"] = json!(1),
                "domain" => base["domain_digest"] = json!("0".repeat(64)),
                _ => base["unexpected"] = json!(true),
            }
            // Call the semantic verifier directly, as after an external
            // artifact-hash recomputation; digest integrity is not the oracle.
            assert!(
                verify(
                    &public,
                    &policy(),
                    2,
                    &base,
                    &original.restricted_construction
                )
                .is_err(),
                "{kind}"
            );
        }
        let mut history = original.restricted_construction.clone();
        history["costs"]["admission_units"] = json!(0);
        assert!(verify(&public, &policy(), 2, &original.factor_base, &history).is_err());
    }

    #[test]
    fn repeated_r_requires_selected_subgroup_preservation_and_pre_admission() {
        let mut public = generate(&config("koblitz", 11, true))
            .unwrap()
            .public_instance;
        let c = arithmetic(&public).unwrap();
        let r = number(&public["curve"]["subgroup_order"], "r").unwrap();
        let order = number(&public["curve"]["group_order"], "N").unwrap();
        assert_eq!(r, 23);
        assert!(order.is_multiple_of(r * r));
        let (mut stable, mut unstable) = (None, None);
        'search: for x in 1..c.q() {
            for p in c.lift(x) {
                let (p, _) = prime_projection(&c, p, order, r, true);
                if p.is_none() {
                    continue;
                }
                let test = CyclicMembership::new(&c, p, r);
                if test.logarithm(&c, c.frobenius(p, 1)).is_some() {
                    stable = Some(p);
                } else {
                    unstable = Some(p);
                }
                if stable.is_some() && unstable.is_some() {
                    break 'search;
                }
            }
        }
        for (g, status) in [
            (stable.unwrap(), "requested_size_reached"),
            (unstable.unwrap(), "unsupported_subgroup_action"),
        ] {
            public["generator"] = point_json(g);
            public["target"] = point_json(g);
            let built = construct(&public, &policy(), 2).unwrap();
            // A stable nonfaithful orbit can exhaust its tiny subgroup at once.
            let actual = built.restricted_construction["terminal_status"]
                .as_str()
                .unwrap();
            assert!(
                actual == status
                    || status == "requested_size_reached" && actual == "subgroup_exhausted"
            );
            verify(
                &public,
                &policy(),
                2,
                &built.factor_base,
                &built.restricted_construction,
            )
            .unwrap();
        }
        let tiny = FactorBasePolicy::SignedFrobeniusUnionV1 {
            max_candidates: 100,
            max_points: 4096,
            max_action_steps: 1,
            max_artifact_bytes: 4096,
        };
        let denied = construct(&public, &tiny, 2).unwrap();
        assert_eq!(
            denied.restricted_construction["terminal_status"],
            "membership_budget_exhausted"
        );
        assert_eq!(
            denied.restricted_construction["costs"]["membership_baby_additions"],
            0
        );
        assert_eq!(
            denied.restricted_construction["costs"]["admission_units"],
            0
        );
    }

    #[test]
    fn bounded_counter_matches_storage_bytes_and_stops_without_copying() {
        let small = json!({"nested":[{"value":"é"}]});
        assert_eq!(encode_len(&small).unwrap(), pretty(&small).len() as u64);
        let oversized = Value::String("a".repeat(67_108_864));
        assert!(encode_len(&oversized).unwrap() > 67_108_864);
    }
}
