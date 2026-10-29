//! Native, bounded dataset adapters. No adapter reads the private witness.
//!
//! The finite-domain encodings below preserve the actual factor base, including
//! projected abscissae. A polynomial root is accepted only after a group replay.
//! Native operation counters retain their units; none is an instruction count.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde_json::{json, Value};

use crate::binary_ecc::{curve as binary, BinaryCurve, BinaryPoint, F2mElement};
use crate::cryptanalysis::binary_semaev_chain_sat::ChainedS3Encoding;
use crate::cryptanalysis::f6_wide_geometry::F6WidePairIndex;
use crate::cryptanalysis::koblitz_groebner::{self as boolean, FieldStructure, SymElement};
use crate::cryptanalysis::matrix_f5_f2::matrix_f5_f2;
use crate::cryptanalysis::pq_groebner_f2::{F2BoolMono, F2BoolPoly, Mask};
use crate::cryptanalysis::prime_root_trace::{self, RootEvent, RootLimits, RootReport, RootStop};
use crate::cryptanalysis::sat::{self, SolveResult};
use crate::ecc::{CurveParams, Point};

use super::capture::{AttemptBinding, CaptureEmitter, CaptureLimits};
use super::curves;
use super::types::{ArtifactPayload, CaseConfig, CaseData, SolverData};

#[path = "binary_roots.rs"]
mod binary_roots;

pub(super) const BINARY_ROOT_CERTIFICATE_SCOPE: &str = binary_roots::CERTIFICATE_SCOPE;

const TRACE_EVENT_CAP: u64 = 100_000;
const TRACE_BYTE_CAP: usize = 64 * 1024 * 1024;
pub(super) const PRIME_ROOT_CERTIFICATE_SCOPE: &str =
    "distinct roots per completed prime S3 call; incomplete calls prove no root absence";

const SAT_EVENT_BYTE_CAP: usize = TRACE_BYTE_CAP / 4;
const SAT_EVENT_CAP_REASON: &str = "native SAT event trace budget exhausted";

#[derive(Default)]
struct SatEventLog {
    bytes: Vec<u8>,
    events: u64,
}
impl SatEventLog {
    fn append(
        &mut self,
        pattern: u32,
        event: &sat::SolverEvent,
        byte_cap: usize,
        event_cap: u64,
    ) -> Result<(), String> {
        let row = json!({"schema":"ml-dataset.sat-event/v1","pattern":pattern,"sequence":self.events+1,"native":event});
        let bytes = serde_json::to_vec(&row).map_err(|e| e.to_string())?;
        if self.events >= event_cap || self.bytes.len().saturating_add(bytes.len() + 1) > byte_cap {
            return Err(SAT_EVENT_CAP_REASON.into());
        }
        // Write before advancing the in-memory prefix; interruption preserves
        // every successful event. A sink error is never swallowed by SAT.
        live_event(&row)?;
        self.bytes.extend_from_slice(&bytes);
        self.bytes.push(b'\n');
        self.events += 1;
        Ok(())
    }
}

struct LiveSink {
    directory: PathBuf,
    events: File,
    sequence: u64,
}
enum EventSink {
    Legacy(LiveSink),
    Bound(CaptureEmitter),
}
impl EventSink {
    fn event(&mut self, event: &Value) -> Result<(), String> {
        match self {
            Self::Legacy(s) => s.event(event),
            Self::Bound(s) => s.event(event).map_err(|e| e.to_string()),
        }
    }
    fn payload(&mut self, payload: &ArtifactPayload) -> Result<(), String> {
        match self {
            Self::Legacy(s) => s.payload(payload),
            Self::Bound(s) => s.payload(payload).map_err(|e| e.to_string()),
        }
    }
}
thread_local! { static LIVE_SINK:RefCell<Option<EventSink>>=const { RefCell::new(None) }; }
impl LiveSink {
    fn event(&mut self, event: &Value) -> Result<(), String> {
        self.sequence += 1;
        let bytes = serde_json::to_vec(&json!({"sequence":self.sequence,"event":event}))
            .map_err(|e| e.to_string())?;
        self.events
            .write_all(&bytes)
            .and_then(|_| self.events.write_all(b"\n"))
            .and_then(|_| self.events.sync_data())
            .map_err(|e| format!("persist live solver event: {e}"))
    }
    fn payload(&mut self, payload: &ArtifactPayload) -> Result<(), String> {
        if payload.path.contains('/') || payload.path == "." || payload.path == ".." {
            return Err("invalid live payload basename".into());
        }
        let path = self.directory.join("payloads").join(&payload.path);
        if fs::symlink_metadata(&path).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err("live payload path is a symlink".into());
        }
        if path.exists() {
            return if fs::read(&path).map_err(|e| e.to_string())? == payload.bytes {
                Ok(())
            } else {
                Err("live payload path collision with different bytes".into())
            };
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| format!("create live payload {}: {e}", path.display()))?;
        file.write_all(&payload.bytes)
            .and_then(|_| file.sync_all())
            .map_err(|e| format!("persist live payload {}: {e}", path.display()))?;
        File::open(path.parent().unwrap())
            .and_then(|f| f.sync_all())
            .map_err(|e| format!("sync live payload directory: {e}"))?;
        self.event(&json!({"kind":"artifact_persisted","path":format!("payloads/{}",payload.path),"role":payload.role,"bytes":payload.bytes.len()}))
    }
}
pub(super) fn live_event(event: &Value) -> Result<(), String> {
    LIVE_SINK.with(|slot| match slot.borrow_mut().as_mut() {
        Some(s) => s.event(event),
        None => Ok(()),
    })
}
pub(super) fn live_payload(payload: &ArtifactPayload) -> Result<(), String> {
    LIVE_SINK.with(|slot| match slot.borrow_mut().as_mut() {
        Some(s) => s.payload(payload),
        None => Ok(()),
    })
}
struct LiveSinkGuard;
impl Drop for LiveSinkGuard {
    fn drop(&mut self) {
        LIVE_SINK.with(|slot| {
            slot.borrow_mut().take();
        });
    }
}

/// Persist each exposed event and complete artifact before proceeding. The
/// outer worker retains this directory even when a native call is killed.
/// Native engines' currently unexposed internal states remain explicit gaps.
pub fn solve_with_event_sink(
    config: &CaseConfig,
    case: &CaseData,
    directory: &Path,
) -> Result<SolverData, String> {
    if LIVE_SINK.with(|slot| slot.borrow().is_some()) {
        return Err("nested live solver sink".into());
    }
    fs::create_dir_all(directory.join("payloads")).map_err(|e| e.to_string())?;
    let events = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(directory.join("events.jsonl"))
        .map_err(|e| format!("create exclusive solver event stream: {e}"))?;
    File::open(directory)
        .and_then(|f| f.sync_all())
        .map_err(|e| e.to_string())?;
    if let Some(parent) = directory.parent() {
        File::open(parent)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())?;
    }
    LIVE_SINK.with(|slot| {
        *slot.borrow_mut() = Some(EventSink::Legacy(LiveSink {
            directory: directory.to_owned(),
            events,
            sequence: 0,
        }))
    });
    let _guard = LiveSinkGuard;
    live_event(
        &json!({"kind":"solver_begin","solver":config.solver,"family":config.family,"n":config.n,"m":config.m,"work_limit":config.work_limit,"degree_bound":config.degree_bound,"format":"ml-dataset.live-events/v1","sync_policy":"fsync every emitted event and complete payload"}),
    )?;
    let result = solve(config, case);
    if let Ok(data) = &result {
        for payload in &data.artifacts {
            live_payload(payload)?;
        }
    }
    live_event(
        &json!({"kind":"solver_return","outcome":result.as_ref().map(|r|&r.outcome).ok(),"error":result.as_ref().err()}),
    )?;
    result
}

/// Bound, capped event capture for supervised dataset attempts. The emitter
/// commits its initial watermark before the first native call. Its artifacts
/// can be copied by the supervisor while a native call is still running, but
/// they do not contain a restorable native solver state.
pub fn solve_with_bound_event_sink(
    config: &CaseConfig,
    case: &CaseData,
    directory: &Path,
    binding: AttemptBinding,
    limits: CaptureLimits,
) -> Result<SolverData, String> {
    if LIVE_SINK.with(|slot| slot.borrow().is_some()) {
        return Err("nested live solver sink".into());
    }
    let emitter = CaptureEmitter::create(directory, binding, limits)?;
    LIVE_SINK.with(|slot| *slot.borrow_mut() = Some(EventSink::Bound(emitter)));
    let _guard = LiveSinkGuard;
    live_event(&json!({"kind":"solver_begin","solver":config.solver,
        "family":config.family,"n":config.n,"m":config.m,
        "work_limit":config.work_limit,"degree_bound":config.degree_bound,
        "format":"crypto.ml-dataset.live-event/v2",
        "sync_policy":"event fsync followed by atomic committed watermark",
        "solver_state_restorable":false}))?;
    let result = solve(config, case);
    if let Ok(data) = &result {
        for payload in &data.artifacts {
            live_payload(payload)?;
        }
    }
    live_event(&json!({"kind":"solver_return",
        "outcome":result.as_ref().map(|r| &r.outcome).ok(),
        "error":result.as_ref().err()}))?;
    LIVE_SINK.with(|slot| match slot.borrow_mut().as_mut() {
        Some(EventSink::Bound(s)) => s.finish().map_err(|e| e.to_string()),
        _ => Err("bound capture sink disappeared".into()),
    })?;
    result
}

pub fn capabilities() -> Value {
    json!({
        "schema":"ml-dataset.capabilities/v1",
        "solvers":[
            {"id":"enumerate","families":["koblitz","binary","prime"],"m":[2,15],"kind":"bounded exact signed finite-domain PDP","trace":"all visited group transitions"},
            {"id":"bsgs","families":["koblitz","binary","prime"],"m":[2,15],"kind":"independent ECDLP baseline","trace":"baby table and giant probes"},
            {"id":"sat","families":["koblitz","binary"],"m":[2,15],"kind":"native chained S3 CNF/XOR subject to preflight caps","trace":"bounded native decisions, assignments and copied reasons, conflicts, analysis reasons, learned/deleted clauses, backjumps, restarts, XOR row operations; original formulas and checked models","gaps":["independently checked UNSAT proof","internal watch/heap operations and minimization derivations","full gate semantic map"]},
            {"id":"weil","families":["koblitz","binary"],"m":[2,2],"kind":"exact finite-domain selector Weil restriction","trace":"complete ANF equations and coordinate lift map","solver_completed":false},
            {"id":"f4","families":["koblitz","binary","prime"],"m":[2,2],"prime_modulus_limit_exclusive":"4294967296","kind":"native Boolean matrix F4 plus splitting; native prime F4 for p<2^32 plus bounded finite-domain extraction","trace":"original systems (also retained for larger unsupported prime moduli), reduced rows, splitting nodes or useful-row replay schedule","gaps":["prime-field elimination at p>=2^32","internal pivot operations at every node","complete critical-pair stream"]},
            {"id":"f5","families":["koblitz","binary"],"m":[2,2],"kind":"native Boolean matrix F5 row criterion plus splitting; not general incremental F5","trace":"original ANF, bounded reduced rows, F5 report, splitting nodes","gaps":["per-row signatures and rejected signature witnesses"]},
            {"id":"f6","families":["koblitz","binary"],"m":[4,4],"kind":"native F6WidePairIndex geometric four-sum; not a general Groebner algorithm","trace":"pair sums, domain, verified indices","gaps":["native internal lookup sequence"]},
            {"id":"root","families":["koblitz","binary","prime"],"m":[2,2],"kind":"native S3 univariate root finding with exact domain and group lift","trace":"prime native field operations including inverse exponentiation, modular-power/Euclidean/split states and independently checked distinct-root certificates; binary inverse, trace, half-trace and GF(2) matrix states with independently checked distinct-root certificates","gaps":["higher-m elimination","independent proof of complete execution and operation-count trajectory"]},
            {"id":"pkm","families":["prime"],"m":[2,3],"modulus_limit_exclusive":"2147483648","kind":"native PKM Kummer-tower F4 with finite-grid extraction","trace":"exact alternative Kummer base, quotient rules, prepared and specialized equations, native F4 report, extracted roots","gaps":["private elimination pivots and row operations","m>3 or p>=2^31"]},
            {"id":"double-large-prime","families":["koblitz","binary","prime"],"m":[2,15],"kind":"native verified two-residual elimination with signed Frobenius for admitted Koblitz models or identity/sign action with generic sparse filtering; recovery of determined target columns","trace":"residual discovery, exact domains, partial candidates, source coefficients, native sparse filtering and modular rank/recovery certificates","gaps":["internal pivot-selection events","complete private filter mutation stream"]}
        ],
        "global_gaps":["hardware instructions require permitted worker PMU; interrupted calls may lose counter readback","exact first fall degree certificates","exact last fall degree certificates","private native matrix internals and SAT watch/heap/minimization internals"],
        "checkpoint_mid_call":"exposed events and completed payloads when using solve_with_event_sink; native opaque calls retain their last checkpoint",
        "interrupted_trace_loss_possible":true,
        "checkpoint_reason":"Each emitted event and completed payload is fsynced; interruption inside an uninstrumented native call can lose private intermediate states",
        "bounded_search_event_cap":TRACE_EVENT_CAP,"sat_formula_trace_byte_cap":TRACE_BYTE_CAP,
        "sat_native_event_byte_cap":SAT_EVENT_BYTE_CAP,"sat_native_event_count_cap":TRACE_EVENT_CAP
    })
}

#[derive(Clone)]
enum Group {
    Binary(Box<BinaryCurve>),
    Prime(CurveParams),
}
#[derive(Clone, Debug, PartialEq)]
enum Element {
    Binary(BinaryPoint),
    Prime(Point),
}

impl Element {
    fn json(&self) -> Value {
        match self {
            Self::Binary(BinaryPoint::Infinity) | Self::Prime(Point::Infinity) => Value::Null,
            Self::Binary(BinaryPoint::Affine { x, y }) => {
                json!({"x":x.to_biguint().to_string(),"y":y.to_biguint().to_string()})
            }
            Self::Prime(Point::Affine { x, y }) => {
                json!({"x":x.value.to_string(),"y":y.value.to_string()})
            }
        }
    }
    fn key(&self) -> String {
        self.json().to_string()
    }
    fn x(&self) -> Option<BigUint> {
        match self {
            Self::Binary(BinaryPoint::Affine { x, .. }) => Some(x.to_biguint()),
            Self::Prime(Point::Affine { x, .. }) => Some(x.value.clone()),
            _ => None,
        }
    }
}

impl Group {
    fn read(case: &CaseData) -> Result<Self, String> {
        if case.public_instance["family"] == "prime" {
            Ok(Self::Prime(curves::prime_curve(&case.public_instance)?))
        } else {
            Ok(Self::Binary(Box::new(curves::binary_curve(
                &case.public_instance,
            )?)))
        }
    }
    fn point(&self, value: &Value) -> Result<Element, String> {
        match self {
            Self::Binary(c) => curves::binary_point(value, c.m).map(Element::Binary),
            Self::Prime(c) => curves::prime_point(value, c).map(Element::Prime),
        }
    }
    fn zero(&self) -> Element {
        match self {
            Self::Binary(_) => Element::Binary(BinaryPoint::Infinity),
            Self::Prime(_) => Element::Prime(Point::Infinity),
        }
    }
    fn add(&self, left: &Element, right: &Element) -> Element {
        match (self, left, right) {
            (Self::Binary(c), Element::Binary(p), Element::Binary(q)) => {
                Element::Binary(binary::point_add(c, p, q))
            }
            (Self::Prime(c), Element::Prime(p), Element::Prime(q)) => {
                Element::Prime(p.add_vartime(q, &c.a_fe()))
            }
            _ => unreachable!("validated homogeneous group"),
        }
    }
    fn neg(&self, p: &Element) -> Element {
        match p {
            Element::Binary(p) => Element::Binary(binary::point_neg(p)),
            Element::Prime(p) => Element::Prime(p.neg_vartime()),
        }
    }
    fn mul(&self, p: &Element, k: u64) -> Element {
        match (self, p) {
            (Self::Binary(c), Element::Binary(p)) => {
                Element::Binary(binary::scalar_mul(c, p, &BigUint::from(k)))
            }
            (Self::Prime(c), Element::Prime(p)) => {
                Element::Prime(p.scalar_mul_vartime(&BigUint::from(k), &c.a_fe()))
            }
            _ => unreachable!("validated homogeneous group"),
        }
    }
    fn on_curve(&self, p: &Element) -> bool {
        match (self, p) {
            (Self::Binary(c), Element::Binary(p)) => c.is_on_curve(p),
            (Self::Prime(c), Element::Prime(p)) => c.is_on_curve(p),
            _ => false,
        }
    }
    fn order(&self) -> Option<u64> {
        match self {
            Self::Binary(c) => c.order.to_u64(),
            Self::Prime(c) => c.n.to_u64(),
        }
    }
}

fn domain(group: &Group, case: &CaseData) -> Result<Vec<Element>, String> {
    let rows = case.factor_base["points"]
        .as_array()
        .ok_or("factor_base.points must be an array")?;
    let mut points = Vec::new();
    for row in rows {
        let p = group.point(row)?;
        if p == group.zero() || !group.on_curve(&p) {
            return Err("invalid factor-base point".into());
        }
        for signed in [p.clone(), group.neg(&p)] {
            if !points.contains(&signed) {
                points.push(signed);
            }
        }
    }
    Ok(points)
}

fn outcome(status: &str, reason: &str) -> Value {
    json!({"schema":"ml-dataset.solver-outcome/v1","status":status,"reason":reason,
        "solver_completed":false,"pdp_witness_verified":false,"scalar_recovered_without_witness":false,"scalar_replay_verified":false,
        "first_fall_degree":null,"last_fall_degree":null,"retired_hardware_instructions":null,
        "instruction_count_reason":"Worker-level PMU records, when permitted, are saved separately; this adapter does not invent per-operation instruction attribution",
        "trace_complete":false,"counters":{},"counter_scope":"native adapter operations, not retired instructions"})
}
fn unsupported(reason: &str) -> SolverData {
    SolverData {
        outcome: outcome("unsupported", reason),
        artifacts: vec![],
    }
}
fn save(data: &mut SolverData, path: &str, role: &str, value: &Value) -> Result<(), String> {
    let payload = ArtifactPayload::json(path, role, value)?;
    live_payload(&payload)?;
    data.artifacts.push(payload);
    Ok(())
}
fn set_witness(
    data: &mut SolverData,
    group: &Group,
    points: &[Element],
    target: &Element,
) -> Result<(), String> {
    set_witness_with_definition(data, group, points, target, None)
}
fn set_witness_with_definition(
    data: &mut SolverData,
    group: &Group,
    points: &[Element],
    target: &Element,
    definition: Option<&Value>,
) -> Result<(), String> {
    let mut sum = group.zero();
    for p in points {
        if !group.on_curve(p) {
            return Err("off-curve PDP lift".into());
        }
        sum = group.add(&sum, p);
    }
    if &sum != target {
        return Err("PDP group replay failed".into());
    }
    data.outcome["status"] = json!("verified");
    data.outcome["pdp_witness_verified"] = json!(true);
    data.outcome["solver_completed"] = json!(true);
    data.outcome["reason"] =
        json!("A signed factor-base decomposition was independently replayed in the curve group");
    let mut label = json!({"summands":points.iter().map(Element::json).collect::<Vec<_>>(),"target":target.json(),"sum":sum.json(),"replay_group_add_calls":points.len()});
    if let Some(definition) = definition {
        label["factor_base_definition"] = definition.clone();
    }
    save(
        data,
        "pdp-witness.json",
        "independently-verified-pdp",
        &label,
    )
}

pub fn solve(config: &CaseConfig, case: &CaseData) -> Result<SolverData, String> {
    if !(2..=15).contains(&config.m) {
        return Err("decomposition arity must be 2..15".into());
    }
    if case.factor_base["schema"] == "ml-ecdlp-factor-base/v2"
        && case.factor_base["construction_status"] == "unsupported"
    {
        return Ok(unsupported(
            case.factor_base["unsupported_reason"]
                .as_str()
                .unwrap_or("The requested factor-base policy is unsupported"),
        ));
    }
    let group = Group::read(case)?;
    let target = group.point(&case.public_instance["target"])?;
    if !group.on_curve(&target) {
        return Err("target is not on the declared curve".into());
    }
    let points = domain(&group, case)?;
    let field_tally = crate::binary_ecc::operation_tally::OperationTally::begin();
    let mut result = match config.solver.as_str() {
        "enumerate" | "pdp" => enumerate(config, &group, &points, &target)?,
        "bsgs" => bsgs(config, case, &group, &target)?,
        "sat" => sat_solve(config, &group, &points, &target)?,
        "f4" if matches!(group, Group::Prime(_)) => prime_f4(config, &group, &points, &target)?,
        "weil" | "weil-descent" | "f4" | "f5" => boolean_solve(config, &group, &points, &target)?,
        "f6" => f6_solve(config, &group, &points, &target)?,
        "root" => root_solve(config, case, &group, &points, &target)?,
        "pkm" => pkm_solve(config, case, &group, &points, &target)?,
        "double-large-prime" => super::relations::solve(config, case)?,
        other => return Err(format!("unknown solver {other}")),
    };
    result.outcome["solver"] = json!(config.solver);
    result.outcome["requested_work_limit"] = json!(config.work_limit);
    result.outcome["m"] = json!(config.m);
    if result.outcome.get("ecdlp_pipeline_completed").is_none() {
        result.outcome["ecdlp_pipeline_completed"] = json!(false);
    }
    result.outcome["checkpoint_mid_call"] = json!(LIVE_SINK.with(|slot| slot.borrow().is_some()));
    result.outcome["interrupted_trace_loss_possible"] = json!(true);
    result.outcome["binary_field_operation_tally"] = json!({
        "counts":field_tally.finish(),
        "scope":"calling-thread instrumented binary field APIs during adapter solve and replay; excludes setup before solve and optimized kernels bypassing these APIs",
        "inversion_count_includes_its_mul_and_square_cost":true,
        "complete_machine_instruction_count":false
    });
    save(
        &mut result,
        "solver-domain.json",
        "exact-signed-factor-base",
        &json!({"points":points.iter().map(Element::json).collect::<Vec<_>>(),"negation_closed":true,"repetitions_allowed":true}),
    )?;
    Ok(result)
}

/// Replay security-relevant labels from public data, independently of the
/// generator's known scalar. Call after checking the artifact's digest.
pub fn verify_solution_artifact(
    public: &Value,
    factor_base: &Value,
    role: &str,
    artifact: &Value,
) -> Result<(), String> {
    let case = CaseData {
        public_instance: public.clone(),
        private_witness: Value::Null,
        factor_base: factor_base.clone(),
    };
    let group = Group::read(&case)?;
    let target = group.point(&public["target"])?;
    if artifact["target"] != public["target"] {
        return Err("solution artifact target differs from public target".into());
    }
    match role {
        "independently-verified-pdp" => {
            let domain = if artifact["factor_base_definition"].is_object() {
                verify_kummer_domain(&group, &case, &artifact["factor_base_definition"])?
            } else {
                domain(&group, &case)?
            };
            let points = artifact["summands"]
                .as_array()
                .ok_or("missing PDP summands")?;
            let m = public["m"].as_u64().ok_or("missing public arity")?;
            if points.len() as u64 != m {
                return Err("PDP witness has wrong arity".into());
            }
            let mut sum = group.zero();
            for row in points {
                let p = group.point(row)?;
                if !group.on_curve(&p) || !domain.contains(&p) {
                    return Err("PDP witness has a point outside the signed factor base".into());
                }
                sum = group.add(&sum, &p);
            }
            if sum != target || artifact["sum"] != target.json() {
                return Err("PDP witness group replay failed".into());
            }
        }
        "restricted-independent-scalar-label" => {
            if artifact["generator"] != public["generator"] {
                return Err("scalar artifact generator differs from public generator".into());
            }
            let scalar = artifact["scalar"]
                .as_str()
                .and_then(|s| s.parse::<u64>().ok())
                .ok_or("invalid recovered scalar")?;
            let order = group.order().ok_or("invalid subgroup order")?;
            if scalar >= order {
                return Err("recovered scalar is not canonical".into());
            }
            let generator = group.point(&public["generator"])?;
            if group.mul(&generator, scalar) != target {
                return Err("recovered scalar replay failed".into());
            }
        }
        _ => return Err("not a solution artifact role".into()),
    }
    Ok(())
}

struct Search<'a> {
    group: &'a Group,
    target: &'a Element,
    domain: &'a [Element],
    limit: u64,
    operations: u64,
    trace: Vec<Value>,
    exhausted: bool,
    io_error: Option<String>,
}
impl Search<'_> {
    fn descend(
        &mut self,
        depth: usize,
        m: usize,
        sum: &Element,
        path: &mut Vec<usize>,
    ) -> Option<Vec<usize>> {
        if depth == m {
            return (sum == self.target).then(|| path.clone());
        }
        for i in 0..self.domain.len() {
            if self.operations >= self.limit {
                self.exhausted = true;
                return None;
            }
            let next = self.group.add(sum, &self.domain[i]);
            self.operations += 1;
            let event = json!({"kind":"pdp_transition","event":self.operations,"depth":depth,"domain_index":i,"parent_sum":sum.json(),"sum":next.json()});
            if let Err(error) = live_event(&event) {
                self.io_error = Some(error);
                self.exhausted = true;
                return None;
            }
            self.trace.push(event);
            path.push(i);
            if let Some(found) = self.descend(depth + 1, m, &next, path) {
                return Some(found);
            }
            path.pop();
            if self.exhausted {
                return None;
            }
        }
        None
    }
}
fn enumerate(
    config: &CaseConfig,
    group: &Group,
    points: &[Element],
    target: &Element,
) -> Result<SolverData, String> {
    let mut search = Search {
        group,
        target,
        domain: points,
        limit: config.work_limit.min(TRACE_EVENT_CAP),
        operations: 0,
        trace: vec![],
        exhausted: false,
        io_error: None,
    };
    let found = search.descend(0, config.m, &group.zero(), &mut vec![]);
    if let Some(error) = search.io_error.take() {
        return Err(error);
    }
    let mut data = SolverData {
        outcome: outcome(
            if search.exhausted {
                "budget_exhausted"
            } else {
                "verified"
            },
            if search.exhausted {
                "Finite-domain search stopped at the recorded transition budget"
            } else {
                "Finite-domain search completed"
            },
        ),
        artifacts: vec![],
    };
    data.outcome["solver_completed"] = json!(!search.exhausted);
    data.outcome["exhaustive_no_decomposition"] = json!(found.is_none() && !search.exhausted);
    data.outcome["trace_complete"] = json!(true);
    data.outcome["effective_work_limit"] = json!(search.limit);
    data.outcome["counters"] = json!({"group_add_api_calls":search.operations,"complete_candidate_tests":search.trace.iter().filter(|x|x["depth"]==json!(config.m-1)).count()});
    save(
        &mut data,
        "enumeration-events.json",
        "all-visited-pdp-transitions",
        &json!(search.trace),
    )?;
    if let Some(indices) = found {
        set_witness(
            &mut data,
            group,
            &indices
                .iter()
                .map(|&i| points[i].clone())
                .collect::<Vec<_>>(),
            target,
        )?;
    }
    Ok(data)
}

fn bsgs(
    config: &CaseConfig,
    case: &CaseData,
    group: &Group,
    target: &Element,
) -> Result<SolverData, String> {
    let generator = group.point(&case.public_instance["generator"])?;
    if !group.on_curve(&generator) || generator == group.zero() {
        return Err("invalid BSGS generator".into());
    }
    let Some(order) = group.order().filter(|&r| r > 1) else {
        return Ok(unsupported(
            "BSGS needs a certified subgroup order fitting u64",
        ));
    };
    let mut width = (order as f64).sqrt() as u64;
    while (width as u128) * (width as u128) < order as u128 {
        width += 1;
    }
    while width > 0 && ((width - 1) as u128) * ((width - 1) as u128) >= order as u128 {
        width -= 1;
    }
    let limit = config.work_limit.min(TRACE_EVENT_CAP);
    let mut data = SolverData {
        outcome: outcome(
            "budget_exhausted",
            "Independent BSGS budget insufficient for the complete baby table",
        ),
        artifacts: vec![],
    };
    data.outcome["required_baby_entries"] = json!(width);
    data.outcome["effective_work_limit"] = json!(limit);
    if width > limit {
        return Ok(data);
    }
    let mut baby = HashMap::new();
    let mut rows = Vec::new();
    let mut p = group.zero();
    let mut ops = 0;
    for j in 0..width {
        baby.entry(p.key()).or_insert(j);
        let event = json!({"kind":"bsgs_baby","j":j.to_string(),"point":p.json()});
        live_event(&event)?;
        rows.push(event);
        p = group.add(&p, &generator);
        ops += 1;
    }
    let stride = group.neg(&p);
    let mut giant = target.clone();
    let mut probes = vec![];
    let mut answer = None;
    let mut scalar_checks = 0u64;
    for i in 0..=width {
        if ops >= limit {
            break;
        }
        let hit = baby.get(&giant.key()).copied();
        let event = json!({"kind":"bsgs_giant","i":i.to_string(),"point":giant.json(),"baby_index":hit.map(|j|j.to_string())});
        live_event(&event)?;
        probes.push(event);
        if let Some(j) = hit {
            let k = ((i as u128 * width as u128 + j as u128) % order as u128) as u64;
            scalar_checks += 1;
            if group.mul(&generator, k) == *target {
                answer = Some(k);
                break;
            }
        }
        giant = group.add(&giant, &stride);
        ops += 1;
    }
    save(
        &mut data,
        "bsgs-baby-table.json",
        "independent-scalar-search-table",
        &json!(rows),
    )?;
    save(
        &mut data,
        "bsgs-giant-probes.json",
        "independent-scalar-search-probes",
        &json!(probes),
    )?;
    data.outcome["counters"] = json!({"group_add_api_calls_search":ops,"baby_entries":width,"giant_probes":probes.len(),"scalar_verification_calls":scalar_checks});
    data.outcome["trace_complete"] = json!(true);
    if let Some(k) = answer {
        data.outcome["status"] = json!("verified");
        data.outcome["solver_completed"] = json!(true);
        data.outcome["scalar_recovered_without_witness"] = json!(true);
        data.outcome["scalar_replay_verified"] = json!(true);
        data.outcome["reason"]=json!("Independent bounded BSGS recovered a scalar and replayed Q=[k]G; this is a generic baseline, not index calculus");
        save(
            &mut data,
            "scalar-recovery.json",
            "restricted-independent-scalar-label",
            &json!({"scalar":k.to_string(),"generator":generator.json(),"target":target.json(),"method":"native-bsgs","read_private_witness":false}),
        )?;
    }
    Ok(data)
}

fn lift(
    group: &Group,
    domain: &[Element],
    xs: &[BigUint],
    target: &Element,
    limit: u64,
) -> (Option<Vec<Element>>, u64, bool) {
    let choices: Vec<Vec<Element>> = xs
        .iter()
        .map(|x| {
            domain
                .iter()
                .filter(|p| p.x().as_ref() == Some(x))
                .cloned()
                .collect()
        })
        .collect();
    fn visit(
        group: &Group,
        choices: &[Vec<Element>],
        depth: usize,
        sum: &Element,
        target: &Element,
        path: &mut Vec<Element>,
        used: &mut u64,
        limit: u64,
    ) -> Option<Vec<Element>> {
        if depth == choices.len() {
            return (sum == target).then(|| path.clone());
        }
        for p in &choices[depth] {
            if *used >= limit {
                return None;
            }
            *used += 1;
            let next = group.add(sum, p);
            path.push(p.clone());
            if let Some(found) = visit(group, choices, depth + 1, &next, target, path, used, limit)
            {
                return Some(found);
            }
            path.pop();
        }
        None
    }
    let mut used = 0;
    let result = visit(
        group,
        &choices,
        0,
        &group.zero(),
        target,
        &mut vec![],
        &mut used,
        limit,
    );
    let exhausted = result.is_none() && used >= limit && choices.iter().all(|c| !c.is_empty());
    (result, used, exhausted)
}

fn sat_solve(
    config: &CaseConfig,
    group: &Group,
    domain: &[Element],
    target: &Element,
) -> Result<SolverData, String> {
    let Group::Binary(curve) = group else {
        return Ok(unsupported(
            "Native chained-S3 SAT currently requires a binary curve",
        ));
    };
    if crate::cryptanalysis::binary_semaev_chain_sat::chain_s3_max_variables(curve.m, config.m)
        .is_none()
    {
        return Ok(unsupported(
            "Native chained-S3 circuit rejects this n/m; no substitute solve was performed",
        ));
    }
    let target_x = match target {
        Element::Binary(BinaryPoint::Affine { x, .. }) => Some(x),
        _ => None,
    };
    let mut codes: Vec<BigUint> = domain.iter().filter_map(Element::x).collect();
    codes.sort();
    codes.dedup();
    let patterns = 1u32 << (config.m - 2);
    let limit = config.work_limit.min(TRACE_EVENT_CAP);
    let mut data = SolverData {
        outcome: outcome(
            "budget_exhausted",
            "Native SAT search has not covered all intermediate-identity patterns",
        ),
        artifacts: vec![],
    };
    let mut summaries = vec![];
    let mut lift_calls = 0;
    let mut conflicts = 0;
    let mut attempts = 0;
    let mut complete_patterns = 0;
    let deadline = Instant::now() + Duration::from_secs(10);
    let trace = Arc::new(Mutex::new(SatEventLog::default()));
    'patterns: for pattern in 0..patterns {
        if attempts >= limit || Instant::now() >= deadline {
            break;
        }
        let mut enc = match ChainedS3Encoding::build(
            config.m,
            &curve.irreducible,
            &curve.b,
            target_x,
            pattern,
            100_000,
        ) {
            Ok(x) => x,
            Err(e) => {
                data.outcome["reason"] = json!(format!("Circuit construction cap: {e:?}"));
                break;
            }
        };
        if let Err(e) = enc.constrain_summands(&codes, 1_000_000) {
            data.outcome["reason"] = json!(format!("Finite-domain construction cap: {e:?}"));
            break;
        }
        let formula = sat::to_dimacs_xor(&enc.solver);
        let used_bytes: usize = data.artifacts.iter().map(|a| a.bytes.len()).sum();
        if used_bytes.saturating_add(formula.len()) > TRACE_BYTE_CAP {
            data.outcome["reason"] = json!("Formula trace byte budget exhausted before solve");
            break;
        }
        let payload = ArtifactPayload {
            path: format!("sat-pattern-{pattern}.xor.cnf"),
            role: "complete-original-cnf-xor".into(),
            encoding: "dimacs-xor".into(),
            bytes: formula.into_bytes(),
        };
        live_payload(&payload)?;
        data.artifacts.push(payload);
        enc.solver.conflict_budget = limit.saturating_sub(conflicts);
        enc.solver.deadline = Some(deadline);
        // One model per pattern is a bounded diagnostic. A spurious lift does
        // not refute this pattern; it remains explicitly incomplete.
        attempts += 1;
        live_event(
            &json!({"kind":"sat_call_begin","pattern":pattern,"conflict_budget":enc.solver.conflict_budget}),
        )?;
        let output = Arc::clone(&trace);
        // Initial clauses/units are included by the native attachment snapshot.
        // Installation failure is sticky and solve returns Unknown below.
        let installation = enc.solver.set_event_observer(move |event| {
            output
                .lock()
                .map_err(|_| "SAT trace mutex poisoned".to_string())?
                .append(pattern, event, SAT_EVENT_BYTE_CAP, TRACE_EVENT_CAP)
        });
        let verdict = enc.solver.solve();
        let observer_error = enc.solver.event_observer_error().map(str::to_owned);
        if let Err(error) = installation {
            if observer_error.is_none() {
                return Err(error);
            }
        }
        let stats = &enc.solver.stats;
        conflicts = conflicts.saturating_add(stats.conflicts);
        let mut row = json!({"pattern":pattern,"verdict":format!("{verdict:?}"),"variables":enc.solver.n_vars(),"and_gates":enc.n_and_gates,"s3_nodes":enc.n_s3_nodes,
            "native_counters":{"decisions":stats.decisions,"conflicts":stats.conflicts,"propagations":stats.propagations,"restarts":stats.restarts,"xor_passes":stats.xor_passes,"xor_row_ops":stats.xor_row_ops,"xor_row_scans":stats.xor_row_scans,"learnt_clauses":stats.learnt_clauses,"clause_visits":stats.clause_visits,"clause_lit_visits":stats.clause_lit_visits,"analyze_lit_visits":stats.analyze_lit_visits,"word_ops":enc.solver.word_ops()},"unsat_proof_checked":false});
        if let Some(error) = observer_error {
            if verdict != SolveResult::Unknown {
                return Err("SAT observer failure was not censored".into());
            }
            row["observer_error"] = json!(error);
            live_event(&json!({"kind":"sat_call_result","result":row}))?;
            summaries.push(row);
            if error != SAT_EVENT_CAP_REASON {
                return Err(format!("native SAT event sink failed: {error}"));
            }
            data.outcome["reason"] = json!(error);
            data.outcome["trace_budget_exhausted"] = json!(true);
            break;
        }
        match verdict {
            SolveResult::Sat => {
                let model = enc.solver.model();
                if !enc.verify_external_assignment(&model) {
                    return Err("native SAT model failed original constraint verification".into());
                }
                let xs = enc
                    .decode_summands_assignment(&model)
                    .ok_or("SAT model cannot be decoded")?;
                let xs: Vec<BigUint> = xs.iter().map(F2mElement::to_biguint).collect();
                let (witness, adds, exhausted) =
                    lift(group, domain, &xs, target, limit.saturating_sub(lift_calls));
                lift_calls += adds;
                row["model"] = json!(model);
                row["x_coordinates"] =
                    json!(xs.iter().map(ToString::to_string).collect::<Vec<_>>());
                row["group_lift_verified"] = json!(witness.is_some());
                row["lift_budget_exhausted"] = json!(exhausted);
                live_event(&json!({"kind":"sat_call_result","result":row}))?;
                summaries.push(row);
                if let Some(w) = witness {
                    set_witness(&mut data, group, &w, target)?;
                    break 'patterns;
                }
            }
            SolveResult::Unsat => {
                complete_patterns += 1;
                live_event(&json!({"kind":"sat_call_result","result":row}))?;
                summaries.push(row);
            }
            SolveResult::Unknown => {
                live_event(&json!({"kind":"sat_call_result","result":row}))?;
                summaries.push(row);
                break;
            }
        }
    }
    if complete_patterns == patterns {
        data.outcome["status"] = json!("partial");
        data.outcome["solver_completed"] = json!(true);
        data.outcome["reason"]=json!("All identity patterns were solver-reported UNSAT; no independently checked UNSAT proof is available");
    }
    data.outcome["identity_patterns_required"] = json!(patterns);
    data.outcome["identity_patterns_attempted"] = json!(attempts);
    data.outcome["counters"] =
        json!({"sat_conflicts":conflicts,"group_lift_add_calls":lift_calls,"sat_calls":attempts});
    let mut log = trace
        .lock()
        .map_err(|_| "SAT trace mutex poisoned".to_string())?;
    data.outcome["native_event_trace"] = json!({"events":log.events,"bytes":log.bytes.len(),"event_cap":TRACE_EVENT_CAP,"byte_cap":SAT_EVENT_BYTE_CAP,
        "proof_checked":false,"scope":"native semantic search events; clause arena IDs and XOR matrix row operations","gate_map":"summand blocks are DIMACS 1..=m*n; bit j is coefficient of field basis element x^j; remaining gate provenance not exported"});
    let payload = ArtifactPayload {
        path: "sat-native-events.jsonl".into(),
        role: "native-sat-search-events".into(),
        encoding: "jsonl".into(),
        bytes: std::mem::take(&mut log.bytes),
    };
    live_payload(&payload)?;
    data.artifacts.push(payload);
    data.outcome["trace_gaps"]=json!(["No independently checked UNSAT proof; analysis events do not certify clause minimization","Private watch movements and heap mutations are not exported","Only the first SAT model of each pattern is attempted; rejected lifts leave the pattern inconclusive"]);
    save(
        &mut data,
        "sat-attempts.json",
        "native-sat-models-and-counters",
        &json!(summaries),
    )?;
    Ok(data)
}

fn finite_selector(
    codes: &[BigUint],
    offset: usize,
    bits: usize,
    n: u32,
    n_vars: usize,
) -> SymElement {
    let count = 1usize << bits;
    let coords = (0..n)
        .map(|field_bit| {
            let mut table: Vec<bool> = (0..count)
                .map(|i| codes[i % codes.len()].bit(field_bit as u64))
                .collect();
            for bit in 0..bits {
                for mask in 0..count {
                    if mask & (1 << bit) != 0 {
                        table[mask] ^= table[mask ^ (1 << bit)];
                    }
                }
            }
            F2BoolPoly::from_monos(
                table
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| **v)
                    .map(|(m, _)| F2BoolMono::from_mask((m as Mask) << offset))
                    .collect(),
                n_vars,
            )
        })
        .collect();
    SymElement { coords }
}
fn polys_json(polys: &[F2BoolPoly]) -> Value {
    json!(polys.iter().map(|p|json!({"n_vars":p.n_vars,"monomial_masks_hex":p.terms.iter().map(|t|format!("{:x}",t.mask)).collect::<Vec<_>>()})).collect::<Vec<_>>())
}

fn boolean_solve(
    config: &CaseConfig,
    group: &Group,
    domain: &[Element],
    target: &Element,
) -> Result<SolverData, String> {
    let Group::Binary(curve) = group else {
        return Ok(unsupported(
            "This F5/Weil lane requires characteristic two; use f4 for the native prime-field adapter",
        ));
    };
    if config.m != 2 {
        return Ok(unsupported("Exact finite-domain selector Weil/F4/F5 adapter currently supports m=2; larger arities remain separate planned cells"));
    }
    let Some(xr) = target.x() else {
        return Ok(unsupported("Boolean S3 lane currently requires an affine target; use enumerative PDP for identity targets"));
    };
    let mut codes: Vec<BigUint> = domain.iter().filter_map(Element::x).collect();
    codes.sort();
    codes.dedup();
    if codes.is_empty() {
        return Ok(unsupported("Factor base is empty"));
    }
    let bits = (usize::BITS - (codes.len().saturating_sub(1)).leading_zeros()) as usize;
    if bits > 6 {
        return Ok(unsupported(
            "Finite-domain selector trace cap is 64 distinct x-coordinates",
        ));
    }
    let n_vars = 2 * bits;
    let st = FieldStructure::new(curve.m, &curve.irreducible);
    let left = finite_selector(&codes, 0, bits, curve.m, n_vars);
    let right = finite_selector(&codes, bits, bits, curve.m, n_vars);
    let target_symbol =
        SymElement::constant(&F2mElement::from_biguint(&xr, curve.m), curve.m, n_vars);
    let equations = boolean::sym_semaev_s3(&left, &right, &target_symbol, &curve.b, &st);
    let mut data = SolverData {
        outcome: outcome(
            "partial",
            "Exact finite-domain S3 system was Weil-restricted to the Boolean quotient",
        ),
        artifacts: vec![],
    };
    save(
        &mut data,
        "weil-system.json",
        "complete-original-boolean-polynomials",
        &json!({"coefficient_field":"F2","quotient":"v_i^2+v_i","monomial_order":"degrevlex","n_vars":n_vars,"selector_bits_per_summand":bits,"selector_codes":(0..1usize<<bits).map(|i|codes[i%codes.len()].to_string()).collect::<Vec<_>>(),"coordinate_maps":[polys_json(&left.coords),polys_json(&right.coords)],"equations":polys_json(&equations),"target_x":xr.to_string(),"field_degree":curve.m,"irreducible_low_terms":curve.irreducible.low_terms,"lifting_required":true}),
    )?;
    data.outcome["polynomial_encoding_verified"] = json!(true);
    if config.solver == "weil" || config.solver == "weil-descent" {
        data.outcome["trace_complete"] = json!(true);
        return Ok(data);
    }
    let is_f5 = config.solver == "f5";
    let degree = config.degree_bound.min(6);
    // Save the complete degree-bounded Macaulay input as explicit polynomials.
    // It is small under this selector cap and does not claim to expose every
    // private elimination state at descendant nodes.
    let mut matrix_rows = Vec::new();
    let mut matrix_truncated = false;
    for (equation, p) in equations.iter().enumerate() {
        let d = p.terms.iter().map(|t| t.degree()).max().unwrap_or(0);
        if d > degree {
            continue;
        }
        for mask in 0usize..1usize << n_vars {
            if (mask.count_ones() + d) <= degree {
                if matrix_rows.len() as u64 >= config.work_limit.min(TRACE_EVENT_CAP) {
                    matrix_truncated = true;
                    break;
                }
                let q = p.mul_mono(F2BoolMono::from_mask(mask as Mask));
                matrix_rows.push(json!({"equation":equation,"multiplier_hex":format!("{mask:x}"),"row":polys_json(&[q])}));
            }
        }
        if matrix_truncated {
            break;
        }
    }
    if matrix_truncated {
        data.outcome["status"] = json!("budget_exhausted");
        data.outcome["reason"] =
            json!("Macaulay input trace cap reached before native elimination");
        save(
            &mut data,
            "macaulay-input.json",
            "partial-macaulay-input",
            &json!(matrix_rows),
        )?;
        return Ok(data);
    }
    save(
        &mut data,
        "macaulay-input.json",
        "complete-unpruned-degree-bounded-macaulay-candidates",
        &json!(matrix_rows),
    )?;
    let reduced = if is_f5 {
        matrix_f5_f2(&equations, n_vars, degree).map(|(rows, report)| (rows, json!(report)))
    } else {
        boolean::matrix_f4_f2_counted(&equations, n_vars, degree)
            .map(|(rows, word_ops)| (rows, json!({"elimination_word_xors":word_ops})))
    };
    if let Some((rows, report)) = reduced {
        save(
            &mut data,
            "macaulay-output.json",
            "native-reduced-macaulay-rows",
            &json!({"rows":polys_json(&rows),"report":report}),
        )?;
    } else {
        data.outcome["macaulay_step"] = json!("native_size_cap");
    }
    let before = boolean::f4_word_ops_thread();
    live_event(
        &json!({"kind":"boolean_native_solve_begin","engine":config.solver,"n_vars":n_vars,"node_budget":config.work_limit.min(4096)}),
    )?;
    boolean::set_node_trace(true);
    let opts = boolean::SolveOptions {
        engine: if is_f5 {
            boolean::SolverEngine::MatrixF5 { max_degree: degree }
        } else {
            boolean::SolverEngine::MatrixF4 { max_degree: degree }
        },
        max_solutions: usize::MAX,
        node_budget: config.work_limit.min(4096) as usize,
        split_rule: boolean::SplitRule::LowestFree,
    };
    let mut witnessed = None;
    let mut root_rows = vec![];
    let mut lift_ops = 0;
    let mut lift_censored = false;
    let (roots, stats) = boolean::solve_boolean_system_filtered(
        &equations,
        n_vars,
        &opts,
        |root| {
            let mask = (1usize << bits) - 1;
            let xs = [
                codes[(root as usize & mask) % codes.len()].clone(),
                codes[((root >> bits) as usize & mask) % codes.len()].clone(),
            ];
            let (found, adds, exhausted) = lift(
                group,
                domain,
                &xs,
                target,
                config
                    .work_limit
                    .min(TRACE_EVENT_CAP)
                    .saturating_sub(lift_ops),
            );
            lift_ops += adds;
            lift_censored |= exhausted;
            root_rows.push(json!({"assignment_hex":format!("{root:x}"),"x":xs.iter().map(ToString::to_string).collect::<Vec<_>>(),"group_lift_verified":found.is_some(),"lift_budget_exhausted":exhausted}));
            if found.is_some() {
                witnessed = found;
            }
            witnessed.is_some()
        },
    );
    let events = boolean::take_node_trace();
    boolean::set_node_trace(false);
    save(
        &mut data,
        "boolean-solve.json",
        "native-algebra-result-and-node-trace",
        &json!({"stats":stats,"roots_hex":roots.iter().map(|r|format!("{r:x}")).collect::<Vec<_>>(),"root_lifts":root_rows,"nodes":events.iter().map(|e|json!({"assigned_hex":format!("{:x}",e.assigned),"values_hex":format!("{:x}",e.values),"thread_cumulative_word_ops":e.word_ops})).collect::<Vec<_>>()}),
    )?;
    data.outcome["counters"] = json!({"native_f4_word_ops_search":boolean::f4_word_ops_thread().saturating_sub(before),"group_lift_add_calls":lift_ops,"native_solve_stats":stats});
    data.outcome["observed_max_macaulay_degree"] = json!(stats.max_degree_built);
    data.outcome["algebra_completed"] = json!(!stats.exhausted);
    data.outcome["lift_completed"] = json!(!lift_censored);
    data.outcome["trace_gaps"] = json!([
        "Internal row/pivot and signature operations at recursive solver nodes are not exported",
        "Maximum built degree is an observation, not first or last fall degree"
    ]);
    if let Some(w) = witnessed {
        set_witness(&mut data, group, &w, target)?;
    } else if stats.exhausted {
        data.outcome["status"] = json!(if stats.unsupported {
            "unsupported"
        } else {
            "budget_exhausted"
        });
        data.outcome["reason"] = json!("Native Boolean solve was incomplete");
    } else if lift_censored {
        data.outcome["status"] = json!("budget_exhausted");
        data.outcome["reason"] =
            json!("Native algebra completed but group lifting exhausted its separate budget");
    } else {
        data.outcome["solver_completed"] = json!(true);
        data.outcome["reason"]=json!("Native Boolean root search completed; no accepted group lift was found within the separate lift budget");
    }
    Ok(data)
}

fn verify_kummer_domain(
    group: &Group,
    case: &CaseData,
    definition: &Value,
) -> Result<Vec<Element>, String> {
    let Group::Prime(curve) = group else {
        return Err("Kummer factor-base override requires a prime curve".into());
    };
    if definition["kind"] != "prime-kummer" {
        return Err("unknown factor-base override".into());
    }
    let t = definition["t"].as_u64().ok_or("invalid Kummer height")?;
    let g = definition["g"]
        .as_str()
        .and_then(|s| s.parse::<u64>().ok())
        .ok_or("invalid Kummer coset")?;
    let p = curve.p.to_u64().ok_or("invalid Kummer modulus")?;
    if !(1..=4).contains(&t) || g == 0 || g >= p || !(p - 1).is_multiple_of(1 << t) {
        return Err("invalid Kummer base definition".into());
    }
    let original = domain(group, case)?;
    let expected_g = original
        .iter()
        .filter_map(Element::x)
        .filter_map(|x| x.to_u64())
        .filter(|&x| x != 0)
        .min()
        .ok_or("no original nonzero abscissa")?;
    if g != expected_g {
        return Err("Kummer coset is not bound to the canonical original factor base".into());
    }
    let points = definition["points"]
        .as_array()
        .ok_or("missing Kummer points")?;
    let mut out = Vec::new();
    let inverse = BigUint::from(g).modpow(&BigUint::from(p - 2), &curve.p);
    for row in points {
        let point = group.point(row)?;
        let x = point.x().ok_or("infinite Kummer point")?;
        if !group.on_curve(&point)
            || ((&x * &inverse) % &curve.p).modpow(&BigUint::from(1u64 << t), &curve.p)
                != BigUint::from(1u8)
        {
            return Err("point outside defined Kummer base".into());
        }
        if out.contains(&point) {
            return Err("duplicate Kummer point".into());
        }
        out.push(point);
    }
    Ok(out)
}

/// Reconstruct the small public Kummer base without running PKM elimination.
/// This also rejects a proper subset or forged column/sign map, so an exported
/// feature graph describes the actual domain used by the native oracle.
pub(super) fn verify_pkm_factor_base(
    public: &Value,
    factor_base: &Value,
    artifact: &Value,
) -> Result<(), String> {
    use crate::cryptanalysis::ic_boundary::{GroupOps, PrimeCurve, PrimeInstance, PrimePoint};
    use crate::cryptanalysis::ic_framework::prime_tower::PrimeKummerBase;
    use crate::cryptanalysis::ic_framework::stages::{FactorBaseBuilder, InstanceCtx, Params};
    let case = CaseData {
        public_instance: public.clone(),
        private_witness: Value::Null,
        factor_base: factor_base.clone(),
    };
    let group = Group::read(&case)?;
    let definition = &artifact["definition"];
    let actual_points = verify_kummer_domain(&group, &case, definition)?;
    if definition["subgroup_filtered"] != false || definition["known_logs"] != false {
        return Err("unexpected PKM factor-base provenance flags".into());
    }
    let Group::Prime(curve) = &group else {
        return Err("PKM base requires a prime curve".into());
    };
    let p = curve.p.to_u64().ok_or("invalid PKM modulus")?;
    if p >= 1 << 31 {
        return Err("PKM base exceeds native adapter modulus bound".into());
    }
    let point = |name: &str| -> Result<PrimePoint, String> {
        match group.point(&public[name])? {
            Element::Prime(Point::Affine { x, y }) => Ok(PrimePoint::affine(
                x.value.to_u64().ok_or("PKM coordinate exceeds u64")?,
                y.value.to_u64().ok_or("PKM coordinate exceeds u64")?,
            )),
            _ => Err("PKM public point must be affine".into()),
        }
    };
    let generator = point("generator")?;
    let target = point("target")?;
    let r = group.order().ok_or("missing PKM subgroup order")?;
    let cofactor = public["curve"]["cofactor"]
        .as_str()
        .and_then(|s| s.parse::<u64>().ok())
        .ok_or("invalid PKM cofactor")?;
    let instance = PrimeInstance {
        name: "dataset-prime-kummer-verification".into(),
        curve: PrimeCurve {
            p,
            a: curve.a.to_u64().ok_or("invalid PKM coefficient")?,
            b: curve.b.to_u64().ok_or("invalid PKM coefficient")?,
        },
        group_order: r.checked_mul(cofactor).ok_or("PKM order overflow")?,
        r,
        cofactor,
        generator: (generator.x, generator.y),
    };
    let ctx = InstanceCtx {
        group: &instance.curve,
        generator,
        target,
        r,
        cofactor,
        group_order: instance.group_order,
        name: instance.name.clone(),
        field_degree: None,
    };
    let mut params = Params::default();
    params
        .set("t", definition["t"].as_u64().unwrap().to_string())
        .set("g", definition["g"].as_str().unwrap());
    let base = PrimeKummerBase {
        instance: &instance,
    }
    .build(&ctx, &params, &mut GroupOps::default())?;
    let expected_points: Vec<_> = base
        .points
        .iter()
        .map(|p| json!({"x":p.x.to_string(),"y":p.y.to_string()}))
        .collect();
    if json!(actual_points.iter().map(Element::json).collect::<Vec<_>>()) != json!(expected_points)
        || artifact["columns"] != json!(base.columns)
        || artifact["column_of"] != json!(base.col_of)
        || artifact["coefficient_of"]
            != json!(base
                .coef_of
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>())
        || artifact["abscissae"]
            != json!(base
                .abscissa_list
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>())
    {
        return Err(
            "PKM domain or column/sign map differs from native public reconstruction".into(),
        );
    }
    Ok(())
}

fn pkm_solve(
    config: &CaseConfig,
    case: &CaseData,
    group: &Group,
    domain: &[Element],
    target: &Element,
) -> Result<SolverData, String> {
    use crate::cryptanalysis::ic_boundary::{
        GroupOps, OracleCounters, PrimeCurve, PrimeInstance, PrimePoint,
    };
    use crate::cryptanalysis::ic_framework::pkm_semaev::PkmSemaevOracle;
    use crate::cryptanalysis::ic_framework::prime_tower::PrimeKummerBase;
    use crate::cryptanalysis::ic_framework::stages::{
        DecompositionOracle, FactorBaseBuilder, InstanceCtx, Params,
    };
    let Group::Prime(curve) = group else {
        return Ok(unsupported(
            "PKM Kummer-tower adapter requires a prime curve",
        ));
    };
    let p = curve.p.to_u64().ok_or("PKM modulus exceeds u64")?;
    if p >= 1 << 31 || !(2..=3).contains(&config.m) {
        return Ok(unsupported(
            "Native PKM tower currently supports only m=2 or 3 and p<2^31",
        ));
    }
    let Some(g) = domain
        .iter()
        .filter_map(Element::x)
        .filter_map(|x| x.to_u64())
        .filter(|&x| x != 0)
        .min()
    else {
        return Ok(unsupported("PKM base needs a nonzero source abscissa"));
    };
    let mut t = ((p - 1).trailing_zeros()).min(4);
    while t > 1 && (1u64 << t).pow(config.m as u32) > config.work_limit.min(TRACE_EVENT_CAP) {
        t -= 1;
    }
    if (1u64 << t).pow(config.m as u32) > config.work_limit {
        return Ok(SolverData {
            outcome: outcome("budget_exhausted", "Minimum PKM grid exceeds work limit"),
            artifacts: vec![],
        });
    }
    let word_point = |element: &Element| -> Result<PrimePoint, String> {
        match element {
            Element::Prime(Point::Affine { x, y }) => Ok(PrimePoint::affine(
                x.value.to_u64().ok_or("x too wide")?,
                y.value.to_u64().ok_or("y too wide")?,
            )),
            Element::Prime(Point::Infinity) => Ok(PrimePoint::INFINITY),
            _ => Err("wrong group".into()),
        }
    };
    let generator = word_point(&group.point(&case.public_instance["generator"])?)?;
    let q = word_point(target)?;
    if q.infinity {
        return Ok(unsupported("Native PKM oracle requires an affine target"));
    }
    let r = group.order().ok_or("missing subgroup order")?;
    let cofactor = case.public_instance["curve"]["cofactor"]
        .as_str()
        .and_then(|s| s.parse::<u64>().ok())
        .ok_or("invalid cofactor")?;
    let instance = PrimeInstance {
        name: "dataset-prime-kummer".into(),
        curve: PrimeCurve {
            p,
            a: curve.a.to_u64().unwrap(),
            b: curve.b.to_u64().unwrap(),
        },
        group_order: r.checked_mul(cofactor).ok_or("order overflow")?,
        r,
        cofactor,
        generator: (generator.x, generator.y),
    };
    let ctx = InstanceCtx {
        group: &instance.curve,
        generator,
        target: q,
        r,
        cofactor,
        group_order: instance.group_order,
        name: instance.name.clone(),
        field_degree: None,
    };
    let mut params = Params::default();
    params
        .set("t", t.to_string())
        .set("g", g.to_string())
        .set("degree", config.degree_bound.min(12).to_string())
        .set(
            "max_nnz",
            config
                .work_limit
                .min(TRACE_EVENT_CAP)
                .saturating_mul(64)
                .to_string(),
        );
    let mut ops = GroupOps::default();
    let base = PrimeKummerBase {
        instance: &instance,
    }
    .build(&ctx, &params, &mut ops)?;
    let base_points: Vec<Element> = base
        .points
        .iter()
        .map(|p| group.point(&json!({"x":p.x.to_string(),"y":p.y.to_string()})))
        .collect::<Result<_, _>>()?;
    let definition = json!({"kind":"prime-kummer","t":t,"g":g.to_string(),"points":base_points.iter().map(Element::json).collect::<Vec<_>>(),"subgroup_filtered":false,"variant":"rational Kummer points on the same full curve; separate from small-x-cofactor-projection","known_logs":false});
    verify_kummer_domain(group, case, &definition)?;
    let mut data=SolverData{outcome:outcome("partial","Native PKM Kummer-tower algebra and bounded grid extraction ran on an explicitly separate factor base"),artifacts:vec![]};
    save(
        &mut data,
        "pkm-factor-base.json",
        "explicit-alternative-kummer-factor-base",
        &json!({"definition":definition,"columns":base.columns,"column_of":base.col_of,"coefficient_of":base.coef_of.iter().map(ToString::to_string).collect::<Vec<_>>(),"construction_cost":base.cost,"abscissae":base.abscissa_list.iter().map(ToString::to_string).collect::<Vec<_>>()}),
    )?;
    let mut oracle =
        PkmSemaevOracle::new(config.m as u32, t, g).with_budget(Some(Duration::from_secs(5)));
    oracle.enable_dataset_trace();
    oracle.prepare(&ctx, &base, &params, &mut ops)?;
    if let Some(snapshot) = oracle.prepared_system_artifact() {
        save(
            &mut data,
            "pkm-prepared-system.json",
            "exact-prepared-pkm-system-before-solve",
            &snapshot,
        )?;
    }
    live_event(
        &json!({"kind":"pkm_native_solve_begin","m":config.m,"t":t,"grid_bound":(1u64<<t).pow(config.m as u32)}),
    )?;
    let mut counters = OracleCounters::default();
    let found = oracle.decompose(&ctx, &base, &mut ops, &mut counters, q);
    if let Some(snapshot) = oracle.dataset_artifact() {
        save(
            &mut data,
            "pkm-algebra.json",
            "native-pkm-exact-system-and-solver-artifacts",
            snapshot,
        )?;
    }
    data.outcome["factor_base_variant"] = json!("prime-kummer");
    data.outcome["solver_completed"] = json!(true);
    data.outcome["counters"] = json!({"native_group_ops":ops,"native_setup":oracle.setup_native(),"native_relation_counters":oracle.relation_counters(),"native_solver_totals":oracle.solver_totals(),"lift_failures":counters.lift_failures,"unliftable_systems":counters.unliftable_systems});
    data.outcome["trace_gaps"]=json!(["Native F4 report exports aggregate step traces and final basis, not every private pivot or rejected pair","PKM includes separately counted finite-grid fallback after inconclusive F4","Kummer factor base includes rational points outside the target subgroup; this is PDP-only evidence"]);
    if let Some(indices) = found {
        let witness = indices
            .iter()
            .map(|&i| {
                base_points
                    .get(i)
                    .cloned()
                    .ok_or("PKM returned out-of-range index")
            })
            .collect::<Result<Vec<_>, _>>()?;
        set_witness_with_definition(&mut data, group, &witness, target, Some(&definition))?;
        let payload = data
            .artifacts
            .iter_mut()
            .find(|a| a.role == "independently-verified-pdp")
            .ok_or("missing PKM witness artifact")?;
        let label: Value = serde_json::from_slice(&payload.bytes).map_err(|e| e.to_string())?;
        verify_solution_artifact(
            &case.public_instance,
            &case.factor_base,
            &payload.role,
            &label,
        )?;
    }
    Ok(data)
}

fn prime_f4(
    config: &CaseConfig,
    group: &Group,
    domain: &[Element],
    target: &Element,
) -> Result<SolverData, String> {
    use crate::cryptanalysis::f4_fp::{self, F4Options};
    use crate::cryptanalysis::groebner_f4::Ordering;
    if config.m != 2 {
        return Ok(unsupported(
            "Prime F4 finite-domain S3 adapter currently supports m=2",
        ));
    }
    let Group::Prime(curve) = group else {
        unreachable!()
    };
    let Some(xr) = target.x().and_then(|x| x.to_u64()) else {
        return Ok(unsupported("Prime F4 needs an affine target"));
    };
    let p = curve.p.to_u64().ok_or("modulus exceeds u64")?;
    let a = curve.a.to_u64().ok_or("a exceeds u64")?;
    let b = curve.b.to_u64().ok_or("b exceeds u64")?;
    let mul = |x: u64, y: u64| ((x as u128 * y as u128) % p as u128) as u64;
    let add = |x: u64, y: u64| ((x as u128 + y as u128) % p as u128) as u64;
    let neg = |x: u64| if x == 0 { 0 } else { p - x };
    let r2 = mul(xr, xr);
    let linear = neg(add(mul(2, mul(a, xr)), mul(4, b)));
    let terms = vec![
        (vec![2, 2], 1),
        (vec![2, 1], neg(mul(2, xr))),
        (vec![1, 2], neg(mul(2, xr))),
        (vec![2, 0], r2),
        (vec![0, 2], r2),
        (vec![1, 1], neg(mul(2, add(r2, a)))),
        (vec![1, 0], linear),
        (vec![0, 1], linear),
        (vec![0, 0], add(mul(a, a), neg(mul(4, mul(b, xr))))),
    ];
    let mut codes: Vec<u64> = domain
        .iter()
        .filter_map(Element::x)
        .filter_map(|x| x.to_u64())
        .collect();
    codes.sort();
    codes.dedup();
    if codes.is_empty() || codes.len() > 32 {
        return Ok(unsupported(
            "Prime F4 domain must contain 1..32 distinct abscissae",
        ));
    }
    let mut member = vec![1u64];
    for &x in &codes {
        let mut next = vec![0; member.len() + 1];
        for (i, &c) in member.iter().enumerate() {
            next[i] = add(next[i], neg(mul(c, x)));
            next[i + 1] = add(next[i + 1], c);
        }
        member = next;
    }
    let order = Ordering::Grevlex;
    // f4_fp::Barrett::new asserts this narrower bound, even though some
    // polynomial helpers support larger u64 primes. Preserve the public
    // equations without entering any native elimination for unsupported p.
    let backend_applicable = (2..1u64 << 32).contains(&p);
    let normalize = |terms: &[(Vec<u32>, u64)]| {
        if backend_applicable {
            f4_fp::normalise(terms, p, order)
        } else {
            terms.iter().filter(|(_, c)| *c != 0).cloned().collect()
        }
    };
    let mut input = vec![normalize(&terms)];
    for axis in 0..2 {
        input.push(normalize(
            &member
                .iter()
                .enumerate()
                .map(|(d, &c)| {
                    let mut e = vec![0, 0];
                    e[axis] = d as u32;
                    (e, c)
                })
                .collect::<Vec<_>>(),
        ));
    }
    let mut data = SolverData {
        outcome: outcome(
            "partial",
            "Native prime F4 was run on S3 plus exact finite-domain membership polynomials",
        ),
        artifacts: vec![],
    };
    let encode = |polys: &[f4_fp::Poly]| {
        json!(polys
            .iter()
            .map(|f| f
                .iter()
                .map(|(e, c)| json!({"exponents":e,"coefficient":c.to_string()}))
                .collect::<Vec<_>>())
            .collect::<Vec<_>>())
    };
    save(
        &mut data,
        "prime-f4-input.json",
        "complete-prime-field-polynomial-system",
        &json!({"modulus":p.to_string(),"variables":["x0","x1"],"monomial_order":"degrevlex","native_normalization_applied":backend_applicable,"native_modulus_limit_exclusive":"4294967296","polynomials":encode(&input),"factor_base_x":codes.iter().map(ToString::to_string).collect::<Vec<_>>(),"target_x":xr.to_string()}),
    )?;
    if !backend_applicable {
        data.outcome = outcome(
            "unsupported",
            "Native prime F4 elimination requires 2 <= p < 2^32; the public S3 and exact finite-domain membership system was retained without running F4",
        );
        data.outcome["native_backend_invoked"] = json!(false);
        data.outcome["algebra_completed"] = json!(false);
        data.outcome["native_modulus_limit_exclusive"] = json!("4294967296");
        return Ok(data);
    }
    let mut opts = F4Options::new(order, config.degree_bound.min(12))
        .with_budget(Duration::from_secs(5))
        .capturing_run_log();
    opts.max_pairs_per_step = Some(config.work_limit.clamp(1, 256) as usize);
    live_event(&json!({"kind":"prime_f4_native_begin","degree_bound":opts.max_degree,"n_vars":2}))?;
    let (report, trace) = f4_fp::f4_record(&input, 2, p, &opts);
    data.outcome["native_backend_invoked"] = json!(true);
    let log = f4_fp::take_run_log();
    save(
        &mut data,
        "prime-f4-output.json",
        "native-f4-basis-useful-row-trace-and-counters",
        &json!({"basis":encode(&report.basis),"inconsistent":report.inconsistent,"degree_reached":report.degree_reached,"observed_productive_degree_max":report.solving_degree_max,"pairs_above_bound":report.pairs_above_bound,"timed_out":report.timed_out,"max_rows":report.max_rows,"max_cols":report.max_cols,"elimination_fp_multiplies":report.field_ops,
        "trace":{"input_leading_monomials":trace.input_lms,"stopped":trace.stopped,"steps":trace.steps.iter().map(|s|json!({"degree":s.degree,"contributing_rows_and_multipliers":s.rows,"new_leading_monomials":s.new_lms})).collect::<Vec<_>>()},
        "step_log":log.steps.iter().map(|s|json!({"degree":s.degree,"pairs":s.pairs,"rows":s.rows,"all_rows":s.all_rows,"ops":s.ops,"new_polynomials":s.new_polys,"staircase":s.staircase})).collect::<Vec<_>>()}),
    )?;
    let limit = config.work_limit.min(TRACE_EVENT_CAP);
    let mut tests = 0;
    let mut lift_ops = 0;
    let mut checks = vec![];
    let mut found = None;
    'grid: for &x in &codes {
        for &y in &codes {
            if tests >= limit || lift_ops >= limit {
                break 'grid;
            }
            tests += 1;
            let is_root = input.iter().all(|f| f4_fp::eval(f, &[x, y], p) == 0);
            if !is_root {
                checks
                    .push(json!({"x":[x.to_string(),y.to_string()],"original_system_root":false}));
                continue;
            }
            if report.inconsistent {
                return Err("Native prime F4 refutation contradicted by independently evaluated finite-domain root".into());
            }
            let (w, adds, exhausted) = lift(
                group,
                domain,
                &[BigUint::from(x), BigUint::from(y)],
                target,
                limit.saturating_sub(lift_ops),
            );
            lift_ops += adds;
            checks.push(json!({"x":[x.to_string(),y.to_string()],"original_system_root":true,"group_lift_verified":w.is_some(),"lift_budget_exhausted":exhausted}));
            if w.is_some() {
                found = w;
                break 'grid;
            }
        }
    }
    save(
        &mut data,
        "prime-f4-root-extraction.json",
        "bounded-finite-domain-root-extraction-after-f4",
        &json!(checks),
    )?;
    data.outcome["counters"] = json!({"native_elimination_fp_multiplies":report.field_ops,"finite_domain_root_tests":tests,"group_lift_add_calls":lift_ops});
    data.outcome["algebra_completed"] = json!(!report.timed_out && report.pairs_above_bound == 0);
    data.outcome["trace_gaps"]=json!(["Useful-row replay schedule omits zero rows and private pivots; not a complete event stream","Root extraction is separately counted finite-domain enumeration, not native F4 substitution"]);
    if let Some(w) = found {
        set_witness(&mut data, group, &w, target)?;
    } else if tests < (codes.len() * codes.len()) as u64 || lift_ops >= limit {
        data.outcome["status"] = json!("budget_exhausted");
    } else {
        data.outcome["solver_completed"] = json!(true);
        data.outcome["exhaustive_no_decomposition"] = json!(true);
    }
    Ok(data)
}

fn f6_solve(
    config: &CaseConfig,
    group: &Group,
    domain: &[Element],
    target: &Element,
) -> Result<SolverData, String> {
    let Group::Binary(curve) = group else {
        return Ok(unsupported("F6WidePairIndex requires a binary curve"));
    };
    if config.m != 4 {
        return Ok(unsupported(
            "Native F6 geometric pair index supports exactly four summands",
        ));
    }
    let points: Vec<BinaryPoint> = domain
        .iter()
        .map(|p| match p {
            Element::Binary(p) => p.clone(),
            _ => unreachable!(),
        })
        .collect();
    let target_native = match target {
        Element::Binary(p) => p,
        _ => unreachable!(),
    };
    let cap = config.work_limit.min(TRACE_EVENT_CAP) as usize;
    let Some(index) = F6WidePairIndex::new(curve, &points, cap) else {
        return Ok(SolverData {
            outcome: outcome(
                "budget_exhausted",
                "Native F6 pair construction exceeded its cap or rejected the input",
            ),
            artifacts: vec![],
        });
    };
    let mut pairs = vec![];
    for i in 0..domain.len() {
        for j in i..domain.len() {
            pairs.push(json!({"i":i,"j":j,"sum":group.add(&domain[i],&domain[j]).json()}));
        }
    }
    let found = index.solve4(target_native);
    let mut data = SolverData {
        outcome: outcome(
            "verified",
            "Native four-sum geometric search completed over the signed finite domain",
        ),
        artifacts: vec![],
    };
    data.outcome["solver_completed"] = json!(true);
    data.outcome["exhaustive_no_decomposition"] = json!(found.is_none());
    data.outcome["algorithm"] = json!("F6WidePairIndex exact geometric four-sum; not Groebner F6");
    data.outcome["counters"] = json!({"native_pair_count":index.pair_count(),"artifact_replay_group_add_calls":pairs.len(),"native_query_group_add_calls":null});
    data.outcome["trace_gaps"]=json!(["Private native lookup and batch-arithmetic sequence is not exported; pair sums are independently reconstructed and charged separately"]);
    save(
        &mut data,
        "f6-pair-table.json",
        "independently-reconstructed-native-pair-domain",
        &json!(pairs),
    )?;
    if let Some(indices) = found {
        set_witness(
            &mut data,
            group,
            &indices
                .iter()
                .map(|&i| domain[i].clone())
                .collect::<Vec<_>>(),
            target,
        )?;
    }
    Ok(data)
}

fn root_solve(
    config: &CaseConfig,
    case: &CaseData,
    group: &Group,
    domain: &[Element],
    target: &Element,
) -> Result<SolverData, String> {
    if config.m != 2 {
        return Ok(unsupported(
            "Native S3 univariate root adapter currently supports m=2",
        ));
    }
    if matches!(group, Group::Prime(_)) {
        return prime_root_solve(config, group, domain, target);
    }
    binary_roots::solve(config, case, group, domain, target)
}

pub(super) fn verify_binary_root_artifact(
    public: &Value,
    base: &Value,
    config: &CaseConfig,
    artifact: &Value,
) -> Result<(), String> {
    binary_roots::verify(public, base, config, artifact)
}

// This fixed recipe performs exactly 15 field operations: 3 additions,
// 4 subtractions and 8 multiplications. Integer encoding/validation is separate.
const PRIME_S3_SETUP_WORK: u64 = 15;
fn prime_root_coefficients(p: u64, a: u64, b: u64, u: u64, r: u64) -> Vec<u64> {
    let add = |x: u64, y: u64| ((x as u128 + y as u128) % p as u128) as u64;
    let mul = |x: u64, y: u64| ((x as u128 * y as u128) % p as u128) as u64;
    let sub = |x: u64, y: u64| ((x as u128 + p as u128 - y as u128) % p as u128) as u64;
    let product = mul(u, r);
    let sum = add(u, r);
    let t = sub(product, a);
    let c0 = sub(mul(t, t), mul(4 % p, mul(b, sum)));
    let c1 = sub(0, add(mul(2 % p, mul(sum, add(product, a))), mul(4 % p, b)));
    let d = sub(u, r);
    vec![c0, c1, mul(d, d)]
}

fn prime_root_solve(
    config: &CaseConfig,
    group: &Group,
    domain: &[Element],
    target: &Element,
) -> Result<SolverData, String> {
    let Group::Prime(curve) = group else {
        return Err("prime root group mismatch".into());
    };
    let p = curve.p.to_u64().ok_or("prime root modulus exceeds u64")?;
    let a = curve.a.to_u64().ok_or("prime coefficient exceeds u64")?;
    let b = curve.b.to_u64().ok_or("prime coefficient exceeds u64")?;
    let Some(xr) = target.x().and_then(|v| v.to_u64()) else {
        return Ok(unsupported("S3 root adapter requires affine target"));
    };
    let mut codes: Vec<u64> = domain
        .iter()
        .filter_map(Element::x)
        .map(|x| x.to_u64().ok_or("factor-base x exceeds u64"))
        .collect::<Result<_, _>>()?;
    codes.sort_unstable();
    codes.dedup();
    let limit = config.work_limit.min(TRACE_EVENT_CAP);
    let mut used = 0u64;
    let mut events = 0u64;
    let mut event_bytes = 0usize;
    let mut queries = vec![];
    let mut found = None;
    let mut censored = false;
    let mut data = SolverData {
        outcome: outcome(
            "partial",
            "Bounded native prime roots with independently replayable distinct-root certificates",
        ),
        artifacts: vec![],
    };
    for (call_index, &x) in codes.iter().enumerate() {
        if limit.saturating_sub(used) < PRIME_S3_SETUP_WORK {
            censored = true;
            break;
        }
        let before = used;
        let coefficients = prime_root_coefficients(p, a, b, x, xr);
        used += PRIME_S3_SETUP_WORK;
        let mut io_error = None;
        let mut byte_cap = false;
        let report = prime_root_trace::solve(
            &coefficients,
            p,
            config.seed ^ call_index as u64,
            RootLimits {
                field_work: limit - used,
                split_attempts: 1000,
                events: TRACE_EVENT_CAP.saturating_sub(events),
                degree: 2,
            },
            &mut |event| {
                let row = json!({"schema":"ml-dataset.prime-root-event/v1","call_index":call_index,
                    "fixed_x":x.to_string(),"native":event});
                let size = serde_json::to_vec(&row).map_err(|e| e.to_string())?.len();
                if event_bytes.saturating_add(size) > TRACE_BYTE_CAP / 4 {
                    byte_cap = true;
                    return Err("prime root event byte cap reached".into());
                }
                if let Err(error) = live_event(&row) {
                    io_error = Some(error.clone());
                    return Err(error);
                }
                events += 1;
                event_bytes += size;
                if matches!(event, RootEvent::PowerEnd { .. } | RootEvent::GcdEnd { .. }) {
                    let path = format!("root-state-{call_index:04}-{events:06}.json");
                    if let Err(error) = save(
                        &mut data,
                        &path,
                        "native-prime-root-intermediate-state",
                        &row,
                    ) {
                        io_error = Some(error.clone());
                        return Err(error);
                    }
                }
                Ok(())
            },
        )?;
        used += report.counters.field_work();
        if let Some(error) = io_error {
            return Err(error);
        }
        // Replay is an integrity check, excluded from the solver-work counters.
        prime_root_trace::verify(&report)?;
        let mut checks = vec![];
        let roots = if report.stop == RootStop::AllFieldElements {
            codes.clone()
        } else {
            report.roots.clone()
        };
        if report.complete() {
            for root in roots {
                let evaluation_work = 2 * coefficients.len() as u64;
                if limit.saturating_sub(used) < evaluation_work {
                    censored = true;
                    break;
                }
                used += evaluation_work;
                let value = coefficients.iter().rev().fold(0u64, |acc, &c| {
                    ((acc as u128 * root as u128 + c as u128) % p as u128) as u64
                });
                if value != 0 {
                    return Err("native prime root failed original polynomial evaluation".into());
                }
                let (witness, adds, exhausted) = lift(
                    group,
                    domain,
                    &[BigUint::from(x), BigUint::from(root)],
                    target,
                    limit - used,
                );
                used += adds;
                checks.push(
                    json!({"x":root.to_string(),"evaluation_work":evaluation_work,
                    "group_lift_add_calls":adds,"in_factor_base":codes.contains(&root),
                    "group_lift_verified":witness.is_some(),"lift_budget_exhausted":exhausted}),
                );
                if witness.is_some() {
                    found = witness;
                    break;
                }
                if exhausted {
                    censored = true;
                    break;
                }
            }
        } else {
            censored = true;
        }
        let row = json!({"call_index":call_index,"fixed_x":x.to_string(),
            "coefficients_constant_first":coefficients,"work_before":before,
            "coefficient_setup_work":PRIME_S3_SETUP_WORK,"work_after":used,
            "trace_byte_cap_reached":byte_cap,"report":report,"checks":checks});
        save(
            &mut data,
            &format!("root-call-{call_index:04}.json"),
            "native-prime-root-call",
            &row,
        )?;
        queries.push(row);
        if found.is_some() || censored {
            break;
        }
    }
    let certificate = json!({"schema":"ml-dataset.prime-root-certificates/v1",
        "target":target.json(),"p":p.to_string(),"requested_work_limit":config.work_limit,
        "effective_work_limit":limit,"charged_work":used,"queries":queries,
        "root_queries_completed":!censored && queries.len()==codes.len(),
        "censored":censored,"found_pdp":found.is_some(),
        "distinct_root_certificates_verified":true,"execution_trace_complete_verified":false,
        "total_operation_count_verified":false,
        "counter_scope":"coefficient recipe field operations + native field add/sub/mul/square including inverse exponentiation + original-root evaluation + group lift additions; excludes validation, serialization and replay",
        "all_field_polynomial_policy":"explicit all-field certificate, then bounded actual factor-base x enumeration"});
    save(
        &mut data,
        "prime-root-certificates.json",
        "prime-root-certificates/v1",
        &certificate,
    )?;
    data.outcome["counters"] = json!({"aggregate_root_field_and_group_work":used,
        "root_solver_calls":queries.len(),"acknowledged_native_events":events,
        "acknowledged_native_event_bytes":event_bytes});
    data.outcome["root_certificate_scope"] = json!(PRIME_ROOT_CERTIFICATE_SCOPE);
    data.outcome["root_execution_trace_complete_verified"] = json!(false);
    data.outcome["root_total_operation_count_verified"] = json!(false);
    data.outcome["trace_gaps"] = json!([
        "No higher-m elimination or binary root internals",
        "Event records are retained solver artifacts, not a restorable process checkpoint"
    ]);
    if let Some(witness) = found {
        set_witness(&mut data, group, &witness, target)?;
    } else if censored {
        data.outcome["status"] = json!("budget_exhausted");
        data.outcome["reason"] =
            json!("Aggregate prime-root/lift work, split or trace cap reached");
    } else {
        data.outcome["solver_completed"] = json!(true);
    }
    Ok(data)
}

/// Bind per-call root certificates to the exact public S3 instances and finite
/// domain. No private scalar is read; completed root sets are not ECDLP labels.
pub(super) fn verify_prime_root_artifact(
    public: &Value,
    base: &Value,
    config: &CaseConfig,
    artifact: &Value,
) -> Result<(), String> {
    if base["schema"] == "ml-ecdlp-factor-base/v2" && base["construction_status"] == "unsupported" {
        return Err("root certificate uses an unsupported factor-base policy".into());
    }
    if config.solver != "root"
        || config.family != "prime"
        || config.m != 2
        || public["family"] != "prime"
        || public["m"] != 2
        || public["n"] != config.n
        || public["twist"] != config.twist
        || artifact["schema"] != "ml-dataset.prime-root-certificates/v1"
        || artifact["target"] != public["target"]
        || artifact["requested_work_limit"] != config.work_limit
        || artifact["distinct_root_certificates_verified"] != true
        || artifact["execution_trace_complete_verified"] != false
        || artifact["total_operation_count_verified"] != false
    {
        return Err("prime root artifact public/config binding differs".into());
    }
    let case = CaseData {
        public_instance: public.clone(),
        private_witness: Value::Null,
        factor_base: base.clone(),
    };
    let group = Group::read(&case)?;
    let Group::Prime(curve) = &group else {
        return Err("prime root curve required".into());
    };
    let target = group.point(&public["target"])?;
    if !group.on_curve(&target) {
        return Err("root certificate target is off curve".into());
    }
    let p = curve.p.to_u64().ok_or("prime root modulus exceeds u64")?;
    let a = curve.a.to_u64().ok_or("prime root a exceeds u64")?;
    let b = curve.b.to_u64().ok_or("prime root b exceeds u64")?;
    let xr = target
        .x()
        .and_then(|x| x.to_u64())
        .ok_or("affine target required")?;
    let domain = domain(&group, &case)?;
    let mut codes: Vec<u64> = domain
        .iter()
        .filter_map(Element::x)
        .map(|x| x.to_u64().ok_or("x exceeds u64"))
        .collect::<Result<_, _>>()?;
    codes.sort_unstable();
    codes.dedup();
    let limit = config.work_limit.min(TRACE_EVENT_CAP);
    let modulus_decimal = p.to_string();
    if artifact["p"].as_str() != Some(modulus_decimal.as_str())
        || artifact["effective_work_limit"] != limit
    {
        return Err("prime root modulus or budget binding differs".into());
    }
    let queries = artifact["queries"]
        .as_array()
        .ok_or("missing root queries")?;
    if queries.len() > codes.len() {
        return Err("too many root queries".into());
    }
    let mut used = 0u64;
    let mut events = 0u64;
    let mut found = false;
    let mut censored = false;
    for (index, row) in queries.iter().enumerate() {
        if found || censored {
            return Err("root queries continue past terminal state".into());
        }
        let x = codes[index];
        let fixed_x_decimal = x.to_string();
        let coeff = prime_root_coefficients(p, a, b, x, xr);
        if row["call_index"] != index
            || row["fixed_x"].as_str() != Some(fixed_x_decimal.as_str())
            || row["work_before"] != used
            || row["coefficient_setup_work"] != PRIME_S3_SETUP_WORK
            || row["coefficients_constant_first"] != json!(coeff)
            || limit.saturating_sub(used) < PRIME_S3_SETUP_WORK
        {
            return Err("root query identity or work prefix differs".into());
        }
        used += PRIME_S3_SETUP_WORK;
        let report: RootReport =
            serde_json::from_value(row["report"].clone()).map_err(|e| e.to_string())?;
        let mut canonical_coeff = coeff.clone();
        while canonical_coeff.last() == Some(&0) {
            canonical_coeff.pop();
        }
        if report.p != p
            || report.polynomial != canonical_coeff
            || report.seed != config.seed ^ index as u64
            || report.limits
                != (RootLimits {
                    field_work: limit - used,
                    split_attempts: 1000,
                    events: TRACE_EVENT_CAP - events,
                    degree: 2,
                })
        {
            return Err("root report differs from frozen call".into());
        }
        prime_root_trace::verify(&report)?;
        used += report.counters.field_work();
        events += report.events.len() as u64;
        let checks = row["checks"].as_array().ok_or("missing root lift checks")?;
        let roots = if report.stop == RootStop::AllFieldElements {
            codes.clone()
        } else {
            report.roots.clone()
        };
        if checks.len() > roots.len() || (!report.complete() && !checks.is_empty()) {
            return Err("invalid censored root lifts".into());
        }
        for (j, check) in checks.iter().enumerate() {
            if found || censored {
                return Err("root lifts continue after terminal state".into());
            }
            let root = roots[j];
            let root_decimal = root.to_string();
            let eval = 2 * coeff.len() as u64;
            if check["x"].as_str() != Some(root_decimal.as_str())
                || check["evaluation_work"] != eval
                || limit.saturating_sub(used) < eval
            {
                return Err("root evaluation identity/work differs".into());
            }
            used += eval;
            let (w, adds, exhausted) = lift(
                &group,
                &domain,
                &[BigUint::from(x), BigUint::from(root)],
                &target,
                limit - used,
            );
            if check["group_lift_add_calls"] != adds
                || check["in_factor_base"] != codes.contains(&root)
                || check["group_lift_verified"] != w.is_some()
                || check["lift_budget_exhausted"] != exhausted
            {
                return Err("root group lift replay differs".into());
            }
            used += adds;
            found = w.is_some();
            censored = exhausted && !found;
        }
        if report.complete() && !found && checks.len() < roots.len() {
            if !censored && limit.saturating_sub(used) >= 2 * coeff.len() as u64 {
                return Err("unjustified omitted root lift".into());
            }
            censored = true;
        }
        if !report.complete() {
            censored = true;
        }
        if row["work_after"] != used || used > limit {
            return Err("aggregate root work differs".into());
        }
    }
    if !found && !censored && queries.len() < codes.len() {
        if limit.saturating_sub(used) >= PRIME_S3_SETUP_WORK {
            return Err("unjustified omitted root query".into());
        }
        censored = true;
    }
    if artifact["charged_work"] != used
        || artifact["found_pdp"] != found
        || artifact["censored"] != censored
        || artifact["root_queries_completed"] != (!censored && queries.len() == codes.len())
    {
        return Err("prime root aggregate completion differs".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_sat_trace_caps_preserve_an_exact_accepted_prefix() {
        let mut log = SatEventLog::default();
        let event = sat::SolverEvent::SolveBegin {
            conflicts: 0,
            conflict_budget: 10,
        };
        log.append(3, &event, 4096, 1).unwrap();
        let prefix = log.bytes.clone();
        assert_eq!(
            log.append(3, &event, 4096, 1).unwrap_err(),
            SAT_EVENT_CAP_REASON
        );
        assert_eq!(log.bytes, prefix);
        assert_eq!(log.events, 1);
        let mut tiny = SatEventLog::default();
        assert_eq!(
            tiny.append(0, &event, 1, 100).unwrap_err(),
            SAT_EVENT_CAP_REASON
        );
        assert!(tiny.bytes.is_empty());
        let row: Value = serde_json::from_slice(&prefix).unwrap();
        assert_eq!(row["sequence"], 1);
        assert_eq!(row["pattern"], 3);
        assert_eq!(
            serde_json::from_value::<sat::SolverEvent>(row["native"].clone()).unwrap(),
            event
        );
    }
    #[test]
    fn finite_domain_selector_is_exact_for_non_subspace_codes() {
        let codes = vec![
            BigUint::from(3u32),
            BigUint::from(9u32),
            BigUint::from(22u32),
        ];
        let s = finite_selector(&codes, 0, 2, 5, 4);
        for i in 0..4 {
            assert_eq!(s.eval(i as Mask, 5).to_biguint(), codes[i % 3]);
        }
    }
    #[test]
    fn wrong_target_does_not_pass_group_lift() {
        let c = BinaryCurve::test_curve_f256();
        let group = Group::Binary(Box::new(c.clone()));
        let p = Element::Binary(c.generator.clone());
        let domain = vec![p.clone(), group.neg(&p)];
        let xs = vec![p.x().unwrap(), p.x().unwrap()];
        let target = group.mul(&p, 3);
        let (found, _, _) = lift(&group, &domain, &xs, &target, 100);
        assert!(found.is_none());
    }
    #[test]
    fn exhaustive_pdp_trace_replays_every_transition() {
        let c = BinaryCurve::test_curve_f256();
        let group = Group::Binary(Box::new(c.clone()));
        let p = Element::Binary(c.generator.clone());
        let target = group.mul(&p, 3);
        let mut search = Search {
            group: &group,
            target: &target,
            domain: std::slice::from_ref(&p),
            limit: 3,
            operations: 0,
            trace: vec![],
            exhausted: false,
            io_error: None,
        };
        assert_eq!(
            search.descend(0, 3, &group.zero(), &mut vec![]),
            Some(vec![0, 0, 0])
        );
        assert_eq!(search.trace.len(), 3);
        assert!(!search.exhausted);
    }

    fn config(family: &str, solver: &str) -> CaseConfig {
        CaseConfig {
            family: family.into(),
            n: 7,
            m: 2,
            twist: false,
            curve_seed: 7,
            generator_index: 1,
            key_index: 1,
            seed: 42,
            factor_base_size: 8,
            factor_base_policy: None,
            degree_bound: 4,
            work_limit: 1000,
            solver: solver.into(),
        }
    }

    #[test]
    fn independent_bsgs_never_needs_the_private_scalar_and_rejects_wrong_labels() {
        for family in ["koblitz", "binary", "prime"] {
            let cfg = config(family, "bsgs");
            let mut case = curves::generate(&cfg).unwrap();
            case.private_witness = Value::Null;
            let result = solve(&cfg, &case).unwrap();
            assert_eq!(
                result.outcome["scalar_recovered_without_witness"], true,
                "{family}: {}",
                result.outcome
            );
            let artifact = result
                .artifacts
                .iter()
                .find(|a| a.role == "restricted-independent-scalar-label")
                .unwrap();
            let mut label: Value = serde_json::from_slice(&artifact.bytes).unwrap();
            verify_solution_artifact(
                &case.public_instance,
                &case.factor_base,
                &artifact.role,
                &label,
            )
            .unwrap();
            label["scalar"] = json!("0");
            assert!(verify_solution_artifact(
                &case.public_instance,
                &case.factor_base,
                &artifact.role,
                &label
            )
            .is_err());
        }
    }

    #[test]
    fn native_algebra_and_root_adapters_replay_a_public_planted_pdp() {
        let mut cfg = config("koblitz", "enumerate");
        let mut case = curves::generate(&cfg).unwrap();
        case.private_witness = Value::Null;
        let group = Group::read(&case).unwrap();
        let points = domain(&group, &case).unwrap();
        let target = group.add(&points[0], &points[0]);
        assert_ne!(target, group.zero());
        case.public_instance["target"] = target.json();
        case.public_instance["population"] = json!("unit-test-public-planted-pdp");
        for solver in ["enumerate", "root", "f4", "f5", "sat"] {
            cfg.solver = solver.into();
            let result = solve(&cfg, &case).unwrap();
            if solver == "sat" {
                let trace = result
                    .artifacts
                    .iter()
                    .find(|a| a.role == "native-sat-search-events")
                    .unwrap();
                let events: Vec<Value> = String::from_utf8(trace.bytes.clone())
                    .unwrap()
                    .lines()
                    .map(|line| serde_json::from_str(line).unwrap())
                    .collect();
                assert!(events.iter().any(|e| e["native"]["kind"] == "assignment"));
                assert!(events.iter().any(|e| e["native"]["kind"] == "solve_end"));
                assert_eq!(result.outcome["native_event_trace"]["proof_checked"], false);
            }
            assert_eq!(
                result.outcome["pdp_witness_verified"], true,
                "{solver}: {}",
                result.outcome
            );
            let artifact = result
                .artifacts
                .iter()
                .find(|a| a.role == "independently-verified-pdp")
                .unwrap();
            let mut label: Value = serde_json::from_slice(&artifact.bytes).unwrap();
            verify_solution_artifact(
                &case.public_instance,
                &case.factor_base,
                &artifact.role,
                &label,
            )
            .unwrap();
            label["summands"] = json!([]);
            assert!(verify_solution_artifact(
                &case.public_instance,
                &case.factor_base,
                &artifact.role,
                &label
            )
            .is_err());
        }
    }

    #[test]
    fn prime_root_certificates_bind_public_target_and_domain_on_both_twists() {
        for twist in [false, true] {
            let mut cfg = config("prime", "root");
            cfg.twist = twist;
            cfg.work_limit = TRACE_EVENT_CAP;
            let mut case = curves::generate(&cfg).unwrap();
            case.private_witness = Value::Null;
            let group = Group::read(&case).unwrap();
            let points = domain(&group, &case).unwrap();
            let point = points.iter().min_by_key(|p| p.x()).unwrap();
            let target = group.add(point, point);
            assert_ne!(target, group.zero());
            case.public_instance["target"] = target.json();
            case.public_instance["population"] =
                json!("software-public-planted-prime-root-fixture");
            let result = solve(&cfg, &case).unwrap();
            assert_eq!(
                result.outcome["pdp_witness_verified"], true,
                "{}",
                result.outcome
            );
            let artifact = result
                .artifacts
                .iter()
                .find(|a| a.role == "prime-root-certificates/v1")
                .unwrap();
            let certificate: Value = serde_json::from_slice(&artifact.bytes).unwrap();
            verify_prime_root_artifact(
                &case.public_instance,
                &case.factor_base,
                &cfg,
                &certificate,
            )
            .unwrap();
            assert_eq!(certificate["total_operation_count_verified"], false);
            let mut tampered = certificate.clone();
            tampered["queries"][0]["fixed_x"] = json!("0");
            assert!(verify_prime_root_artifact(
                &case.public_instance,
                &case.factor_base,
                &cfg,
                &tampered
            )
            .is_err());
            let mut tampered = certificate.clone();
            tampered["charged_work"] = json!(0);
            assert!(verify_prime_root_artifact(
                &case.public_instance,
                &case.factor_base,
                &cfg,
                &tampered
            )
            .is_err());
            let mut wrong_public = case.public_instance.clone();
            wrong_public["twist"] = json!(!twist);
            assert!(verify_prime_root_artifact(
                &wrong_public,
                &case.factor_base,
                &cfg,
                &certificate
            )
            .is_err());
            let mut wrong_base = case.factor_base.clone();
            wrong_base["points"] = json!([]);
            assert!(verify_prime_root_artifact(
                &case.public_instance,
                &wrong_base,
                &cfg,
                &certificate
            )
            .is_err());
            let witness = result
                .artifacts
                .iter()
                .find(|a| a.role == "independently-verified-pdp")
                .unwrap();
            verify_solution_artifact(
                &case.public_instance,
                &case.factor_base,
                &witness.role,
                &serde_json::from_slice(&witness.bytes).unwrap(),
            )
            .unwrap();
        }
    }

    #[test]
    fn prime_root_aggregate_caps_preserve_replayable_censoring() {
        let mut cfg = config("prime", "root");
        let mut case = curves::generate(&cfg).unwrap();
        case.private_witness = Value::Null;
        for limit in [
            0,
            PRIME_S3_SETUP_WORK - 1,
            PRIME_S3_SETUP_WORK,
            PRIME_S3_SETUP_WORK + 20,
        ] {
            cfg.work_limit = limit;
            let result = solve(&cfg, &case).unwrap();
            assert_eq!(result.outcome["status"], "budget_exhausted");
            assert_eq!(result.outcome["solver_completed"], false);
            let artifact = result
                .artifacts
                .iter()
                .find(|a| a.role == "prime-root-certificates/v1")
                .unwrap();
            let certificate = serde_json::from_slice(&artifact.bytes).unwrap();
            verify_prime_root_artifact(
                &case.public_instance,
                &case.factor_base,
                &cfg,
                &certificate,
            )
            .unwrap();
        }
    }

    #[test]
    fn unsupported_factor_base_policy_does_not_become_empty_domain_success() {
        let cfg = config("prime", "root");
        let mut case = curves::generate(&cfg).unwrap();
        case.factor_base = json!({"schema":"ml-ecdlp-factor-base/v2","construction_status":"unsupported",
            "unsupported_reason":"binary-only policy","points":[]});
        let result = solve(&cfg, &case).unwrap();
        assert_eq!(result.outcome["status"], "unsupported");
        assert_eq!(result.outcome["solver_completed"], false);
        assert!(result.artifacts.is_empty());
    }

    #[test]
    fn prime61_f4_retains_public_equations_without_entering_small_prime_backend() {
        for twist in [false, true] {
            let mut cfg = config("prime", "f4");
            cfg.n = 61;
            cfg.twist = twist;
            cfg.factor_base_size = 4;
            cfg.work_limit = 64;
            cfg.degree_bound = 2;
            let mut case = curves::generate(&cfg).unwrap();
            case.private_witness = Value::Null;
            let group = Group::read(&case).unwrap();
            let points = domain(&group, &case).unwrap();
            let p = &points[0];
            case.public_instance["target"] = group.add(p, p).json();
            let result = solve(&cfg, &case).unwrap();
            assert_eq!(result.outcome["status"], "unsupported");
            assert_eq!(result.outcome["native_backend_invoked"], false);
            assert_eq!(result.outcome["solver_completed"], false);
            assert_eq!(result.outcome["algebra_completed"], false);
            assert_eq!(result.outcome["pdp_witness_verified"], false);
            assert!(!result
                .artifacts
                .iter()
                .any(|a| a.path == "prime-f4-output.json"));
            let input = result
                .artifacts
                .iter()
                .find(|a| a.path == "prime-f4-input.json")
                .unwrap();
            let input: Value = serde_json::from_slice(&input.bytes).unwrap();
            let modulus = (BigUint::from(1u8) << 61usize) - BigUint::from(1u8);
            assert_eq!(input["modulus"], modulus.to_string());
            assert_eq!(input["native_normalization_applied"], false);
            let x = p.x().unwrap();
            // Independently evaluate S3 and both membership polynomials at the
            // public planted two-point decomposition using arbitrary precision.
            let polys = input["polynomials"].as_array().unwrap();
            assert_eq!(polys.len(), 3);
            for polynomial in polys {
                let value =
                    polynomial
                        .as_array()
                        .unwrap()
                        .iter()
                        .fold(BigUint::from(0u8), |sum, term| {
                            let coefficient: BigUint =
                                term["coefficient"].as_str().unwrap().parse().unwrap();
                            let exponent: u32 = term["exponents"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|e| e.as_u64().unwrap() as u32)
                                .sum();
                            (sum + coefficient * x.modpow(&BigUint::from(exponent), &modulus))
                                % &modulus
                        });
                assert_eq!(value, BigUint::from(0u8));
            }
        }
        let caps = capabilities();
        let f4 = caps["solvers"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["id"] == "f4")
            .unwrap();
        assert_eq!(f4["prime_modulus_limit_exclusive"], "4294967296");
    }

    #[test]
    #[ignore = "360-cell endpoint integration sweep; native SAT has an independent 10-second per-call deadline"]
    fn all_adapter_endpoints_return_structured_outcomes() {
        let methods = [
            "enumerate",
            "bsgs",
            "sat",
            "weil",
            "f4",
            "f5",
            "f6",
            "root",
            "pkm",
            "double-large-prime",
        ];
        let mut visited = 0;
        for n in [23, 61] {
            for family in ["koblitz", "binary", "prime"] {
                for twist in [false, true] {
                    let mut cfg = config(family, "enumerate");
                    cfg.n = n;
                    cfg.twist = twist;
                    cfg.factor_base_size = 4;
                    cfg.work_limit = 64;
                    cfg.degree_bound = 2;
                    let mut case = curves::generate(&cfg).unwrap();
                    case.private_witness = Value::Null;
                    for m in [2, 4, 15] {
                        cfg.m = m;
                        case.public_instance["m"] = json!(m);
                        for method in methods {
                            cfg.solver = method.into();
                            let label =
                                format!("{family} n={n} twist={twist} m={m} solver={method}");
                            let result =
                                solve(&cfg, &case).unwrap_or_else(|e| panic!("{label}: {e}"));
                            assert_eq!(
                                result.outcome["schema"], "ml-dataset.solver-outcome/v1",
                                "{label}"
                            );
                            assert!(
                                matches!(
                                    result.outcome["status"].as_str(),
                                    Some(
                                        "verified" | "partial" | "unsupported" | "budget_exhausted"
                                    )
                                ),
                                "{label}: {}",
                                result.outcome
                            );
                            assert!(
                                result.outcome["reason"]
                                    .as_str()
                                    .is_some_and(|s| !s.is_empty()),
                                "{label}"
                            );
                            assert!(result.outcome["solver_completed"].is_boolean(), "{label}");
                            assert!(result.outcome["first_fall_degree"].is_null(), "{label}");
                            assert!(result.outcome["last_fall_degree"].is_null(), "{label}");
                            if matches!(
                                result.outcome["status"].as_str(),
                                Some("unsupported" | "budget_exhausted")
                            ) {
                                assert_eq!(result.outcome["solver_completed"], false, "{label}");
                            }
                            for artifact in result.artifacts.iter().filter(|a| {
                                matches!(
                                    a.role.as_str(),
                                    "independently-verified-pdp"
                                        | "restricted-independent-scalar-label"
                                )
                            }) {
                                verify_solution_artifact(
                                    &case.public_instance,
                                    &case.factor_base,
                                    &artifact.role,
                                    &serde_json::from_slice(&artifact.bytes).unwrap(),
                                )
                                .unwrap_or_else(|e| panic!("{label}: {e}"));
                            }
                            visited += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(visited, 360);
    }

    #[test]
    fn prime_f4_and_native_pkm_bind_their_actual_domains() {
        let mut cfg = config("prime", "f4");
        let mut case = curves::generate(&cfg).unwrap();
        case.private_witness = Value::Null;
        let group = Group::read(&case).unwrap();
        let points = domain(&group, &case).unwrap();
        let p = points
            .iter()
            .filter(|p| p.x().is_some_and(|x| x != BigUint::from(0u8)))
            .min_by_key(|p| p.x())
            .unwrap();
        case.public_instance["target"] = group.add(p, p).json();
        for solver in ["root", "f4", "pkm"] {
            cfg.solver = solver.into();
            let result = solve(&cfg, &case).unwrap();
            assert_eq!(
                result.outcome["pdp_witness_verified"], true,
                "{solver}: {}",
                result.outcome
            );
            let artifact = result
                .artifacts
                .iter()
                .find(|a| a.role == "independently-verified-pdp")
                .unwrap();
            let mut label: Value = serde_json::from_slice(&artifact.bytes).unwrap();
            verify_solution_artifact(
                &case.public_instance,
                &case.factor_base,
                &artifact.role,
                &label,
            )
            .unwrap();
            if solver == "pkm" {
                let base_artifact = result
                    .artifacts
                    .iter()
                    .find(|a| a.path == "pkm-factor-base.json")
                    .unwrap();
                let base: Value = serde_json::from_slice(&base_artifact.bytes).unwrap();
                verify_pkm_factor_base(&case.public_instance, &case.factor_base, &base).unwrap();
                let mut subset = base.clone();
                subset["definition"]["points"].as_array_mut().unwrap().pop();
                assert!(
                    verify_pkm_factor_base(&case.public_instance, &case.factor_base, &subset)
                        .is_err()
                );
                let mut wrong_map = base.clone();
                wrong_map["coefficient_of"][0] = json!("0");
                assert!(verify_pkm_factor_base(
                    &case.public_instance,
                    &case.factor_base,
                    &wrong_map
                )
                .is_err());
                assert_eq!(label["factor_base_definition"]["kind"], "prime-kummer");
                label["factor_base_definition"]["g"] = json!("0");
                assert!(verify_solution_artifact(
                    &case.public_instance,
                    &case.factor_base,
                    &artifact.role,
                    &label
                )
                .is_err());
                assert!(result
                    .artifacts
                    .iter()
                    .any(|a| a.role == "native-pkm-exact-system-and-solver-artifacts"));
            }
        }
    }

    #[test]
    fn live_sink_persists_exposed_prefix_and_refuses_existing_stream() {
        let mut cfg = config("koblitz", "enumerate");
        cfg.work_limit = 5;
        let mut case = curves::generate(&cfg).unwrap();
        case.private_witness = Value::Null;
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "crypto-dataset-live-{}-{nonce}",
            std::process::id()
        ));
        let result = solve_with_event_sink(&cfg, &case, &dir).unwrap();
        assert_eq!(result.outcome["checkpoint_mid_call"], true);
        let stream = fs::read_to_string(dir.join("events.jsonl")).unwrap();
        let events = stream
            .lines()
            .map(|line| serde_json::from_str::<Value>(line).unwrap())
            .collect::<Vec<_>>();
        assert!(events
            .iter()
            .any(|row| row["event"]["kind"] == "pdp_transition"));
        for (i, row) in events.iter().enumerate() {
            assert_eq!(row["sequence"].as_u64(), Some(i as u64 + 1));
        }
        for payload in result.artifacts {
            assert_eq!(
                fs::read(dir.join("payloads").join(payload.path)).unwrap(),
                payload.bytes
            );
        }
        assert!(solve_with_event_sink(&cfg, &case, &dir).is_err());
        fs::remove_dir_all(dir).unwrap();
    }
}
