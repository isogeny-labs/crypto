//! The frontier: the bounds of a domain nobody has beaten.
//!
//! Built from committed bound records, never written by hand.  Bounds are
//! grouped by domain and compared on the axes a bound carries (`ops`, the
//! ratio to the generic floor; `memory`, table entries per `√r`; and, when
//! asked, `uncharged`, the work the unit counts but does not price).  A
//! bound is *dominated* when another bound of the domain is at least as
//! good on every axis both know and clearly better on one, where clearly
//! better means the intervals do not overlap: two bounds measured on
//! different sessions cannot be paired, so the frontier is conservative
//! and leaves ties standing.  The paired answer is a challenge
//! ([`challenge`](super::challenge)).
//!
//! Unknown is not zero: an axis one side did not report is left out of the
//! comparison and the entry says so.  Inadmissible bounds are listed and
//! never on the frontier.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::cryptanalysis::ecbench::bounds::{
    seal_document, Bound, Domain, SizeRow, Tail, BAND_WEIGHTS, FIELD_AXES, FIT_VERSION,
};

pub const FRONTIER_SCHEMA_V1: &str = "ecbench.frontier/v1";
/// The current frontier procedure (README §4): dominance is read on the
/// sizes two bounds share, only bounds fitted on four or more sizes of a
/// whole session are ranked, a method is ranked on its worst bound, a bound
/// with unpriced work is ranked across the pricing band, and a leader is
/// named only when its interval separates from the runner-up's.
pub const FRONTIER_SCHEMA: &str = "ecbench.frontier/v2";
pub const FRONTIER_PREFIX: &str = "ECFR1h";
/// The axes dominance reads by default.  `uncharged` is reported on every
/// entry and used as an axis only when asked.
pub const DEFAULT_AXES: &[&str] = &["ops", "memory"];
/// The axes every entry reports whether or not they decide.
const REPORTED_AXES: &[&str] = &["ops", "memory", "uncharged"];

/// Every axis name a frontier or a challenge may read: the three every
/// bound carries and the three field-operation axes a bound carries when
/// its runs counted them.
pub fn is_axis(name: &str) -> bool {
    REPORTED_AXES.contains(&name) || FIELD_AXES.contains(&name)
}

/// The axis vocabulary, for error messages.
pub fn axis_names() -> String {
    REPORTED_AXES
        .iter()
        .chain(FIELD_AXES.iter())
        .copied()
        .collect::<Vec<_>>()
        .join(", ")
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Axis {
    pub known: bool,
    pub value: Option<f64>,
    pub ci95: Option<(f64, f64)>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub bound_id: String,
    pub label: String,
    pub method_id: String,
    pub method: String,
    pub params: BTreeMap<String, String>,
    pub level: String,
    pub log2_r: Vec<f64>,
    pub runs: u64,
    pub axes: BTreeMap<String, Axis>,
    pub alpha: Option<f64>,
    pub alpha_ci95: Option<(f64, f64)>,
    pub declared_alpha: Option<f64>,
    pub scaling_claim: bool,
    pub bounded: bool,
    pub is_frontier: bool,
    pub dominated_by: Vec<String>,
    /// Why, per dominating bound; and which axes were left out as unknown.
    pub dominance_notes: Vec<String>,
    pub improves_on: Vec<String>,
    /// Frontier version 2: the fit procedure of the bound, its tail
    /// diagnostics, why it is not ranked (when it is not), and the pricing
    /// checks behind each dominance over or by a bound with unpriced work.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub fit_version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tail: Option<Tail>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unranked_reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub price_checks: Vec<PriceCheck>,
}

fn is_zero(v: &u32) -> bool {
    *v == 0
}

/// What the pricing band (AGENTS.md §5a) says about one dominance claim in
/// which at least one bound has unpriced work.  `dominator` is the bound
/// that wins at the charged cost.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriceCheck {
    pub dominator: String,
    /// The dominance (on `ops` and the other axes) holds at every weight
    /// of the band; `null` when a bound cannot be priced above `w = 0`.
    pub holds_across_band: Option<bool>,
    /// The price per uncharged operation, in GAE, at which the shared-size
    /// mean ops figures cross (the dominance flips): `null` when they never
    /// do at a non-negative price or a bound cannot be priced.
    pub break_even_price: Option<f64>,
    pub band: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainFrontier {
    pub domain: Domain,
    pub domain_id: String,
    pub axes: Vec<String>,
    pub floor_model: String,
    pub entries: Vec<Entry>,
    /// The frontier entry with the least ops value, and the least memory.
    pub ops_leader: Option<String>,
    pub memory_leader: Option<String>,
    /// Frontier version 2: when no leader can be named, the bounds whose
    /// intervals do not separate from the best method's on the sizes they
    /// share (or share none), the best method's included.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ops_tie: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub memory_tie: Vec<String>,
    /// Admissible bounds listed and not ranked, with the reason each.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unranked: Vec<Entry>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rejected {
    pub bound_id: String,
    pub label: String,
    pub reasons: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Frontier {
    pub schema: String,
    pub frontier_id: String,
    pub axes: Vec<String>,
    /// Every bound read, admissible or not, sorted.
    pub built_from: Vec<String>,
    pub domains: Vec<DomainFrontier>,
    pub inadmissible: Vec<Rejected>,
}

impl Frontier {
    pub fn seal(&mut self) -> Result<String, String> {
        self.frontier_id = String::new();
        let (id, text) = seal_document(self, "frontier_id", FRONTIER_PREFIX)?;
        self.frontier_id = id;
        Ok(text)
    }
}

/// Every `*.json` bound under `paths` (files or directories), in path
/// order.  A file that is not a sealed bound is an error, not skipped: a
/// frontier built from a directory with a broken record is wrong, not
/// smaller.
pub fn load_bounds(paths: &[PathBuf]) -> Result<Vec<Bound>, String> {
    let mut files: Vec<PathBuf> = Vec::new();
    for p in paths {
        if p.is_dir() {
            let mut inner: Vec<PathBuf> = std::fs::read_dir(p)
                .map_err(|e| format!("{}: {e}", p.display()))?
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|q| q.extension().and_then(|x| x.to_str()) == Some("json"))
                .collect();
            inner.sort();
            files.extend(inner);
        } else {
            files.push(p.clone());
        }
    }
    let mut out = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for f in files {
        let b = Bound::read(&f)?;
        if !seen.insert(b.bound_id.clone()) {
            return Err(format!(
                "{}: bound {} appears twice",
                f.display(),
                b.bound_id
            ));
        }
        out.push(b);
    }
    Ok(out)
}

fn axis_of(b: &Bound, name: &str) -> Axis {
    match b.dimensions.get(name) {
        Some(d) if d.known => Axis {
            known: true,
            value: d.value,
            ci95: d.ci95,
        },
        _ => Axis {
            known: false,
            value: None,
            ci95: None,
        },
    }
}

fn entry_of(b: &Bound, axes: &[String]) -> Entry {
    let mut ax = BTreeMap::new();
    // A field-operation axis rides on an entry only when its bound
    // carries the dimension — the bound's runs counted it — whether or
    // not dominance was asked to read it; an entry without the key says
    // nothing, and `dominates` leaves the axis out as unknown.  Bounds
    // fitted before the axes existed therefore give the entries, and the
    // page, they always gave.
    for name in axes
        .iter()
        .map(String::as_str)
        .chain(REPORTED_AXES.iter().copied())
        .chain(FIELD_AXES.iter().copied())
    {
        if FIELD_AXES.contains(&name) && !b.dimensions.contains_key(name) {
            continue;
        }
        ax.entry(name.to_string())
            .or_insert_with(|| axis_of(b, name));
    }
    Entry {
        bound_id: b.bound_id.clone(),
        label: b.label.clone(),
        method_id: b.method.method_id.clone(),
        method: b.method.id.clone(),
        params: b.method.params.clone(),
        level: b.level.clone(),
        log2_r: b.sizes.iter().map(|s| s.log2_r).collect(),
        runs: b.provenance.verified,
        axes: ax,
        alpha: b.fit.alpha,
        alpha_ci95: b.fit.alpha_ci95,
        declared_alpha: b.fit.declared_alpha,
        scaling_claim: b.fit.scaling_claim,
        bounded: b.admissibility.bounded,
        is_frontier: false,
        dominated_by: vec![],
        dominance_notes: vec![],
        improves_on: b.improves_on.clone(),
        fit_version: 0,
        tail: None,
        unranked_reasons: vec![],
        price_checks: vec![],
    }
}

/// How `a` compares with `b` on one axis, lower being better.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Order {
    /// `a`'s interval lies wholly below `b`'s.
    ClearlyBetter,
    ClearlyWorse,
    /// The intervals overlap (or only points are known and they differ
    /// within nothing an interval could say).
    Indistinguishable,
    Unknown,
}

/// Compare two axes conservatively: with intervals on both sides, clearly
/// better needs disjoint intervals; with a point on either side, only the
/// points can be read, and a strict inequality is taken as clear.
pub fn order(a: &Axis, b: &Axis) -> Order {
    if !a.known || !b.known {
        return Order::Unknown;
    }
    let (Some(va), Some(vb)) = (a.value, b.value) else {
        return Order::Unknown;
    };
    match (a.ci95, b.ci95) {
        (Some((alo, ahi)), Some((blo, bhi))) => {
            if ahi < blo {
                Order::ClearlyBetter
            } else if alo > bhi {
                Order::ClearlyWorse
            } else {
                Order::Indistinguishable
            }
        }
        _ => {
            if va < vb {
                Order::ClearlyBetter
            } else if va > vb {
                Order::ClearlyWorse
            } else {
                Order::Indistinguishable
            }
        }
    }
}

/// `a` dominates `b` on `axes`: on every axis both know, `a`'s point
/// estimate is no larger and `a` is not clearly worse; on at least one, `a`
/// is clearly better.  The point condition keeps a wide interval from
/// knocking a precise, better entry off the frontier through another
/// axis.  Returns the axes it wins on and the axes left out as unknown.
pub fn dominates(a: &Entry, b: &Entry, axes: &[String]) -> (bool, Vec<String>, Vec<String>) {
    let mut wins = Vec::new();
    let mut unknown = Vec::new();
    for name in axes {
        let (Some(x), Some(y)) = (a.axes.get(name), b.axes.get(name)) else {
            unknown.push(name.clone());
            continue;
        };
        match order(x, y) {
            Order::ClearlyBetter => wins.push(name.clone()),
            Order::ClearlyWorse => return (false, vec![], unknown),
            Order::Indistinguishable => {
                if let (Some(vx), Some(vy)) = (x.value, y.value) {
                    if vx > vy {
                        return (false, vec![], unknown);
                    }
                }
            }
            Order::Unknown => unknown.push(name.clone()),
        }
    }
    (!wins.is_empty(), wins, unknown)
}

/// Build the frontier of every domain in `bounds` under the procedure
/// `version` (1 or 2); a committed frontier re-derives under the version
/// its `schema` names.
pub fn build_version(bounds: &[Bound], axes: &[String], version: u32) -> Result<Frontier, String> {
    match version {
        1 => build_v1(bounds, axes),
        2 => build(bounds, axes),
        v => Err(format!("frontier procedure {v} is not known (1 or 2)")),
    }
}

/// The frontier procedure named by a frontier document's `schema`.
pub fn version_of_schema(schema: &str) -> Option<u32> {
    match schema {
        FRONTIER_SCHEMA_V1 => Some(1),
        FRONTIER_SCHEMA => Some(2),
        _ => None,
    }
}

/// Frontier procedure 1: bounds compared on their headline figures
/// whatever curves they were fitted on.  Kept so a committed v1 frontier
/// re-derives; see [`build`] for the current procedure.
pub fn build_v1(bounds: &[Bound], axes: &[String]) -> Result<Frontier, String> {
    for a in axes {
        if !is_axis(a) {
            return Err(format!("unknown axis `{a}`; axes are {}", axis_names()));
        }
    }
    let mut by_domain: BTreeMap<String, (Domain, Vec<&Bound>)> = BTreeMap::new();
    let mut inadmissible = Vec::new();
    let mut built_from: Vec<String> = bounds.iter().map(|b| b.bound_id.clone()).collect();
    built_from.sort();
    for b in bounds {
        if b.domain.id()? != b.domain_id {
            return Err(format!(
                "bound {} carries domain id {} but its domain hashes to {}",
                b.bound_id,
                b.domain_id,
                b.domain.id()?
            ));
        }
        if !b.admissible() {
            inadmissible.push(Rejected {
                bound_id: b.bound_id.clone(),
                label: b.label.clone(),
                reasons: b.admissibility.reasons.clone(),
            });
            continue;
        }
        by_domain
            .entry(b.domain_id.clone())
            .or_insert_with(|| (b.domain.clone(), Vec::new()))
            .1
            .push(b);
    }
    let mut domains = Vec::new();
    for (domain_id, (domain, bs)) in by_domain {
        let mut entries: Vec<Entry> = bs.iter().map(|b| entry_of(b, axes)).collect();
        let snapshot = entries.clone();
        for e in entries.iter_mut() {
            let mut all_unknown: BTreeSet<String> = BTreeSet::new();
            for other in snapshot.iter().filter(|o| o.bound_id != e.bound_id) {
                let (dom, wins, unknown) = dominates(other, e, axes);
                all_unknown.extend(unknown.iter().cloned());
                if dom {
                    e.dominated_by.push(other.bound_id.clone());
                    e.dominance_notes.push(format!(
                        "{} ({}) is clearly better on {}{}",
                        other.bound_id,
                        other.method,
                        wins.join(", "),
                        if unknown.is_empty() {
                            String::new()
                        } else {
                            format!("; {} unknown on one side and left out", unknown.join(", "))
                        }
                    ));
                }
            }
            e.is_frontier = e.dominated_by.is_empty();
            if e.is_frontier && !all_unknown.is_empty() {
                e.dominance_notes.push(format!(
                    "on the frontier with {} unknown against some entries",
                    all_unknown.into_iter().collect::<Vec<_>>().join(", ")
                ));
            }
        }
        let leader = |axis: &str| {
            entries
                .iter()
                .filter(|e| e.is_frontier)
                .filter_map(|e| {
                    e.axes
                        .get(axis)
                        .and_then(|a| a.value)
                        .map(|v| (v, &e.bound_id))
                })
                .min_by(|a, b| a.0.total_cmp(&b.0))
                .map(|(_, id)| id.clone())
        };
        let (ops_leader, memory_leader) = (leader("ops"), leader("memory"));
        // Frontier first, then by ops.
        entries.sort_by(|a, b| {
            b.is_frontier.cmp(&a.is_frontier).then_with(|| {
                let va = a
                    .axes
                    .get("ops")
                    .and_then(|x| x.value)
                    .unwrap_or(f64::INFINITY);
                let vb = b
                    .axes
                    .get("ops")
                    .and_then(|x| x.value)
                    .unwrap_or(f64::INFINITY);
                va.total_cmp(&vb)
            })
        });
        domains.push(DomainFrontier {
            domain,
            domain_id,
            axes: axes.to_vec(),
            floor_model: "S_floor = sqrt(pi / 2A), A the usable automorphisms; ops = S / S_floor"
                .into(),
            entries,
            ops_leader,
            memory_leader,
            ops_tie: vec![],
            memory_tie: vec![],
            unranked: vec![],
        });
    }
    let mut f = Frontier {
        schema: FRONTIER_SCHEMA_V1.into(),
        frontier_id: String::new(),
        axes: axes.to_vec(),
        built_from,
        domains,
        inadmissible,
    };
    f.schema = FRONTIER_SCHEMA_V1.into();
    f.seal()?;
    Ok(f)
}

// ── Frontier procedure 2 ───────────────────────────────────────────

/// [`order`] for procedure 2: a missing interval is not precision.  Two
/// axes are separated only when both carry an interval and the intervals
/// are disjoint; a point against anything is `Indistinguishable`.
pub fn order_v2(a: &Axis, b: &Axis) -> Order {
    if !a.known || !b.known || a.value.is_none() || b.value.is_none() {
        return Order::Unknown;
    }
    match (a.ci95, b.ci95) {
        (Some((alo, ahi)), Some((blo, bhi))) => {
            if ahi < blo {
                Order::ClearlyBetter
            } else if alo > bhi {
                Order::ClearlyWorse
            } else {
                Order::Indistinguishable
            }
        }
        _ => Order::Indistinguishable,
    }
}

fn unknown_axis() -> Axis {
    Axis {
        known: false,
        value: None,
        ci95: None,
    }
}

/// One axis of one size row.  `ops` is the ratio to the size's floor, from
/// the size's own interval, or the band-priced figure at `price`; the
/// others come from the size's `axes`, and a size that has none is unknown.
fn size_axis(s: &SizeRow, axis: &str, price: Option<f64>) -> Axis {
    let to_ratio = |ci: Option<(f64, f64)>| ci.map(|(l, h)| (l / s.floor_s, h / s.floor_s));
    if axis == "ops" {
        return match price {
            None => match s.ratio_to_floor {
                Some(v) => Axis {
                    known: true,
                    value: Some(v),
                    ci95: to_ratio(s.s_ci95),
                },
                None => unknown_axis(),
            },
            Some(w) => s
                .band
                .iter()
                .find(|p| p.weight == w)
                .and_then(|p| {
                    p.s.map(|v| Axis {
                        known: true,
                        value: Some(v / s.floor_s),
                        ci95: to_ratio(p.ci95),
                    })
                })
                .unwrap_or_else(unknown_axis),
        };
    }
    s.axes
        .get(axis)
        .map(|a| Axis {
            known: true,
            value: Some(a.value),
            ci95: a.ci95,
        })
        .unwrap_or_else(unknown_axis)
}

/// The sizes two bounds both measured, as pairs of their rows.  A size is
/// the same size when slug and subgroup order agree.
pub fn shared_sizes<'a>(a: &'a Bound, b: &'a Bound) -> Vec<(&'a SizeRow, &'a SizeRow)> {
    a.sizes
        .iter()
        .filter(|x| x.verified > 0)
        .filter_map(|x| {
            b.sizes
                .iter()
                .find(|y| y.verified > 0 && y.slug == x.slug && y.r == x.r)
                .map(|y| (x, y))
        })
        .collect()
}

/// How `a` compares with `b` on one axis over the sizes they share.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AxisResult {
    /// Clearly better on at least one shared size, clearly worse on none,
    /// and no worse on the mean of the shared sizes.
    Better,
    /// Clearly worse on some shared size, or worse on the mean.
    Worse,
    Tie,
    Unknown,
}

pub fn axis_on_shared(
    shared: &[(&SizeRow, &SizeRow)],
    axis: &str,
    price: Option<f64>,
) -> AxisResult {
    let (mut better, mut worse, mut known) = (0, 0, 0);
    let (mut sa, mut sb) = (0.0, 0.0);
    for (x, y) in shared {
        let (ax, ay) = (size_axis(x, axis, price), size_axis(y, axis, price));
        match order_v2(&ax, &ay) {
            Order::Unknown => continue,
            Order::ClearlyBetter => better += 1,
            Order::ClearlyWorse => worse += 1,
            Order::Indistinguishable => {}
        }
        known += 1;
        sa += ax.value.unwrap_or(0.0);
        sb += ay.value.unwrap_or(0.0);
    }
    if known == 0 {
        AxisResult::Unknown
    } else if worse > 0 || sa > sb {
        AxisResult::Worse
    } else if better > 0 {
        AxisResult::Better
    } else {
        AxisResult::Tie
    }
}

/// `a` against `b` on the sizes they share.
#[derive(Clone, Debug, PartialEq)]
pub struct SharedDominance {
    pub dominates: bool,
    pub wins: Vec<String>,
    pub unknown: Vec<String>,
    pub shared: usize,
}

/// Procedure 2 dominance of `a` over `b` on `axes`: on the sizes both
/// measured, `a` is clearly better on some axis and not worse on any, with
/// `ops` read at `price` (GAE per unpriced operation) when given.  Two
/// bounds that share no size are not comparable, so neither dominates.
pub fn dominates_shared(
    a: &Bound,
    b: &Bound,
    axes: &[String],
    price: Option<f64>,
) -> SharedDominance {
    let shared = shared_sizes(a, b);
    let mut out = SharedDominance {
        dominates: false,
        wins: vec![],
        unknown: vec![],
        shared: shared.len(),
    };
    if shared.is_empty() {
        return out;
    }
    for name in axes {
        match axis_on_shared(&shared, name, if name == "ops" { price } else { None }) {
            AxisResult::Better => out.wins.push(name.clone()),
            AxisResult::Worse => {
                out.wins.clear();
                return out;
            }
            AxisResult::Tie => {}
            AxisResult::Unknown => out.unknown.push(name.clone()),
        }
    }
    out.dominates = !out.wins.is_empty();
    out
}

/// Mean ops over the shared sizes at weight `w`, when every shared size is
/// priced there.
fn mean_ops_at(shared: &[(&SizeRow, &SizeRow)], w: f64) -> Option<(f64, f64)> {
    let (mut sa, mut sb) = (0.0, 0.0);
    for (x, y) in shared {
        sa += size_axis(x, "ops", Some(w)).value?;
        sb += size_axis(y, "ops", Some(w)).value?;
    }
    let n = shared.len() as f64;
    (n > 0.0).then_some((sa / n, sb / n))
}

/// The pricing check of a dominance of `a` over `b` where either carries
/// unpriced work: the dominance must hold at every weight of the band, and
/// the price at which the shared-size mean ops figures cross is reported.
pub fn price_check(a: &Bound, b: &Bound, axes: &[String]) -> PriceCheck {
    let mut any_false = false;
    let mut any_missing = false;
    for w in BAND_WEIGHTS {
        let shared = shared_sizes(a, b);
        let priced = shared
            .iter()
            .all(|(x, y)| size_axis(x, "ops", Some(w)).known && size_axis(y, "ops", Some(w)).known);
        if !priced {
            any_missing = true;
            continue;
        }
        if !dominates_shared(a, b, axes, Some(w)).dominates {
            any_false = true;
        }
    }
    let holds = if any_false {
        Some(false)
    } else if any_missing {
        None
    } else {
        Some(true)
    };
    let shared = shared_sizes(a, b);
    let break_even = match (mean_ops_at(&shared, 0.0), mean_ops_at(&shared, 1.0)) {
        (Some((a0, b0)), Some((a1, b1))) => {
            // d(w) = ops_b(w) - ops_a(w) is linear in w.
            let (d0, d1) = (b0 - a0, b1 - a1);
            if (d0 - d1).abs() > 1e-12 {
                let w = d0 / (d0 - d1);
                (w >= 0.0).then_some(w)
            } else {
                None
            }
        }
        _ => None,
    };
    PriceCheck {
        dominator: a.bound_id.clone(),
        holds_across_band: holds,
        break_even_price: break_even,
        band: BAND_WEIGHTS.to_vec(),
    }
}

fn entry_v2(b: &Bound, axes: &[String]) -> Entry {
    let mut e = entry_of(b, axes);
    e.fit_version = b.fit_version();
    e.tail = b.tail.clone();
    // `improves_on` of a legacy bound is not a ranking fact: version 1 set
    // it on a `trade` verdict too.
    if b.fit_version() < FIT_VERSION {
        e.improves_on.clear();
    }
    e
}

/// Whether `a` is separated from `b` on `axis`: clearly better on the
/// sizes they share, and they share at least one.
fn separated(a: &Bound, b: &Bound, axis: &str) -> bool {
    let shared = shared_sizes(a, b);
    !shared.is_empty() && axis_on_shared(&shared, axis, None) == AxisResult::Better
}

/// The leader on `axis` among the frontier bounds, and the tie set when
/// none can be named.  Methods are ranked on their worst bound (the
/// luckiest session does not lead); the best method leads only when it is
/// separated from every other method on the sizes they share, each pair of
/// their bounds taken.  A method it shares no size with cannot be separated
/// from, so it joins the tie set.
fn leader(front: &[&Bound], axis: &str) -> (Option<String>, Vec<String>) {
    let value = |b: &Bound| -> Option<f64> {
        if axis == "ops" {
            b.ops().and_then(|d| d.value)
        } else {
            b.dimensions
                .get(axis)
                .filter(|d| d.known)
                .and_then(|d| d.value)
        }
    };
    let mut groups: BTreeMap<&str, Vec<&Bound>> = BTreeMap::new();
    for b in front.iter().filter(|b| value(b).is_some()) {
        groups
            .entry(b.method.method_id.as_str())
            .or_default()
            .push(b);
    }
    let worst = |g: &Vec<&Bound>| {
        g.iter()
            .filter_map(|b| value(b))
            .fold(f64::NEG_INFINITY, f64::max)
    };
    let mut ranked: Vec<(&str, &Vec<&Bound>)> = groups.iter().map(|(k, v)| (*k, v)).collect();
    ranked.sort_by(|a, b| worst(a.1).total_cmp(&worst(b.1)).then(a.0.cmp(b.0)));
    let Some((_, best)) = ranked.first().copied() else {
        return (None, vec![]);
    };
    let mut tie: Vec<String> = Vec::new();
    for (_, other) in ranked.iter().skip(1) {
        let all_separated = best.iter().all(|x| {
            other
                .iter()
                .all(|y| shared_sizes(x, y).is_empty() || separated(x, y, axis))
        }) && best
            .iter()
            .any(|x| other.iter().any(|y| !shared_sizes(x, y).is_empty()));
        if !all_separated {
            tie.extend(other.iter().map(|b| b.bound_id.clone()));
        }
    }
    if tie.is_empty() {
        let lead = best
            .iter()
            .max_by(|a, b| value(a).unwrap_or(0.0).total_cmp(&value(b).unwrap_or(0.0)))
            .map(|b| b.bound_id.clone());
        (lead, vec![])
    } else {
        tie.extend(best.iter().map(|b| b.bound_id.clone()));
        tie.sort();
        tie.dedup();
        (None, tie)
    }
}

/// Build the frontier of every domain in `bounds` under procedure 2.
pub fn build(bounds: &[Bound], axes: &[String]) -> Result<Frontier, String> {
    for a in axes {
        if !is_axis(a) {
            return Err(format!("unknown axis `{a}`; axes are {}", axis_names()));
        }
    }
    let mut by_domain: BTreeMap<String, (Domain, Vec<&Bound>)> = BTreeMap::new();
    let mut inadmissible = Vec::new();
    let mut built_from: Vec<String> = bounds.iter().map(|b| b.bound_id.clone()).collect();
    built_from.sort();
    for b in bounds {
        if b.domain.id()? != b.domain_id {
            return Err(format!(
                "bound {} carries domain id {} but its domain hashes to {}",
                b.bound_id,
                b.domain_id,
                b.domain.id()?
            ));
        }
        if !b.admissible() {
            inadmissible.push(Rejected {
                bound_id: b.bound_id.clone(),
                label: b.label.clone(),
                reasons: b.admissibility.reasons.clone(),
            });
            continue;
        }
        by_domain
            .entry(b.domain_id.clone())
            .or_insert_with(|| (b.domain.clone(), Vec::new()))
            .1
            .push(b);
    }
    let mut domains = Vec::new();
    for (domain_id, (domain, bs)) in by_domain {
        let (ranked, unranked): (Vec<&Bound>, Vec<&Bound>) = bs
            .iter()
            .copied()
            .partition(|b| b.frontier_ineligible().is_empty());
        let mut entries: Vec<Entry> = ranked.iter().map(|b| entry_v2(b, axes)).collect();
        for (e, b) in entries.iter_mut().zip(ranked.iter()) {
            let mut all_unknown: BTreeSet<String> = BTreeSet::new();
            let mut methods: BTreeMap<&str, Vec<&Bound>> = BTreeMap::new();
            for o in ranked.iter().filter(|o| o.bound_id != b.bound_id) {
                methods
                    .entry(o.method.method_id.as_str())
                    .or_default()
                    .push(o);
            }
            let mut incomparable = 0usize;
            for (mid, group) in &methods {
                if *mid == b.method.method_id.as_str() {
                    continue;
                }
                // Rank on the worst: every bound of the method that shares
                // a size with this one must dominate it.
                let comparable: Vec<&&Bound> = group
                    .iter()
                    .filter(|o| !shared_sizes(o, b).is_empty())
                    .collect();
                incomparable += group.len() - comparable.len();
                if comparable.is_empty() {
                    continue;
                }
                let mut verdicts = Vec::new();
                for o in &comparable {
                    let plain = dominates_shared(o, b, axes, None);
                    all_unknown.extend(plain.unknown.iter().cloned());
                    let priced = (plain.dominates
                        && (o.admissibility.bounded || b.admissibility.bounded))
                        .then(|| price_check(o, b, axes));
                    let ok = plain.dominates
                        && priced
                            .as_ref()
                            .is_none_or(|p| p.holds_across_band == Some(true));
                    verdicts.push((o, plain, priced, ok));
                }
                for (o, plain, priced, ok) in &verdicts {
                    if let Some(p) = priced {
                        e.price_checks.push(p.clone());
                        if !ok {
                            e.dominance_notes.push(format!(
                                "{} ({}) is clearly better on {} at the charged cost but not across the pricing band (weights {}): not dominating{}",
                                o.bound_id,
                                o.method.id,
                                plain.wins.join(", "),
                                band_text(),
                                match (p.holds_across_band, p.break_even_price) {
                                    (None, _) => "; unpriced work is uncounted on one side, so it cannot be ranked above w = 0".to_string(),
                                    (_, Some(w)) => format!("; the order flips at about {w:.3} GAE per uncharged operation"),
                                    _ => String::new(),
                                }
                            ));
                        }
                    }
                }
                if verdicts.iter().all(|v| v.3) {
                    for (o, plain, priced, _) in &verdicts {
                        e.dominated_by.push(o.bound_id.clone());
                        e.dominance_notes.push(format!(
                            "{} ({}) is clearly better on {} over {} shared size(s){}{}{}",
                            o.bound_id,
                            o.method.id,
                            plain.wins.join(", "),
                            plain.shared,
                            if plain.unknown.is_empty() {
                                String::new()
                            } else {
                                format!("; {} unknown on one side and left out", plain.unknown.join(", "))
                            },
                            if group.len() > 1 {
                                format!("; all {} comparable bound(s) of the method agree (ranked on the worst)", comparable.len())
                            } else {
                                String::new()
                            },
                            match priced {
                                Some(p) => format!(
                                    "; holds across the pricing band, the order flips {}",
                                    p.break_even_price
                                        .map(|w| format!("{w:.3} GAE per uncharged operation"))
                                        .unwrap_or_else(|| "never at a non-negative price".into())
                                ),
                                None => String::new(),
                            }
                        ));
                    }
                }
            }
            if incomparable > 0 {
                e.dominance_notes.push(format!(
                    "shares no size with {incomparable} other bound(s) of the domain: neither dominates the other"
                ));
            }
            e.dominated_by.sort();
            e.dominated_by.dedup();
            e.is_frontier = e.dominated_by.is_empty();
            if e.is_frontier && !all_unknown.is_empty() {
                e.dominance_notes.push(format!(
                    "on the frontier with {} unknown against some entries",
                    all_unknown.into_iter().collect::<Vec<_>>().join(", ")
                ));
            }
        }
        let front: Vec<&Bound> = entries
            .iter()
            .zip(ranked.iter())
            .filter(|(e, _)| e.is_frontier)
            .map(|(_, b)| *b)
            .collect();
        let (ops_leader, ops_tie) = leader(&front, "ops");
        let (memory_leader, memory_tie) = leader(&front, "memory");
        entries.sort_by(|a, b| {
            b.is_frontier.cmp(&a.is_frontier).then_with(|| {
                let va = a
                    .axes
                    .get("ops")
                    .and_then(|x| x.value)
                    .unwrap_or(f64::INFINITY);
                let vb = b
                    .axes
                    .get("ops")
                    .and_then(|x| x.value)
                    .unwrap_or(f64::INFINITY);
                va.total_cmp(&vb)
            })
        });
        let mut unranked_entries: Vec<Entry> = unranked
            .iter()
            .map(|b| {
                let mut e = entry_v2(b, axes);
                e.unranked_reasons = b.frontier_ineligible();
                e
            })
            .collect();
        unranked_entries.sort_by(|a, b| {
            let va = a
                .axes
                .get("ops")
                .and_then(|x| x.value)
                .unwrap_or(f64::INFINITY);
            let vb = b
                .axes
                .get("ops")
                .and_then(|x| x.value)
                .unwrap_or(f64::INFINITY);
            va.total_cmp(&vb).then(a.bound_id.cmp(&b.bound_id))
        });
        domains.push(DomainFrontier {
            domain,
            domain_id,
            axes: axes.to_vec(),
            floor_model: "S_floor = sqrt(pi / 2A), A the usable automorphisms; ops = S / S_floor"
                .into(),
            entries,
            ops_leader,
            memory_leader,
            ops_tie,
            memory_tie,
            unranked: unranked_entries,
        });
    }
    let mut f = Frontier {
        schema: FRONTIER_SCHEMA.into(),
        frontier_id: String::new(),
        axes: axes.to_vec(),
        built_from,
        domains,
        inadmissible,
    };
    f.seal()?;
    Ok(f)
}

fn band_text() -> String {
    BAND_WEIGHTS
        .iter()
        .map(|w| w.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

fn fmt(v: Option<f64>) -> String {
    v.map(|x| format!("{x:.3}")).unwrap_or_else(|| "–".into())
}

fn fmt_ci(ci: Option<(f64, f64)>) -> String {
    ci.map(|(l, h)| format!("[{l:.3}, {h:.3}]"))
        .unwrap_or_else(|| "–".into())
}

/// The frontier as a page: one table per domain, frontier rows first.
/// The layout follows the procedure the frontier was built under.
pub fn render_markdown(f: &Frontier, records_dir: &str) -> String {
    if f.schema == FRONTIER_SCHEMA_V1 {
        render_markdown_v1(f, records_dir)
    } else {
        render_markdown_v2(f, records_dir)
    }
}

fn fmt_tail(t: &Option<Tail>) -> String {
    match t {
        None => "–".into(),
        Some(t) => format!(
            "{:.1}× / {:.0} %{}",
            t.max_over_median,
            100.0 * t.top_run_share,
            if t.large { " **large**" } else { "" }
        ),
    }
}

fn leader_line(axis: &str, leader: &Option<String>, tie: &[String]) -> String {
    match (leader, tie.is_empty()) {
        (Some(l), _) => format!("{axis} leader: `{l}`"),
        (None, true) => format!("{axis} leader: –"),
        (None, false) => {
            format!(
            "{axis}: no leader named, the intervals do not separate (or share no size); tie set {}",
            tie.iter().map(|t| format!("`{t}`")).collect::<Vec<_>>().join(", ")
        )
        }
    }
}

fn render_markdown_v2(f: &Frontier, records_dir: &str) -> String {
    let mut out = String::new();
    out.push_str("# Measured bounds and the frontier\n\n");
    out.push_str(&format!(
        "Generated by `ecbench frontier build` from the bound records in `{records_dir}`; do not edit. Frontier `{}` (procedure 2), built from {} bound(s), dominance on `{}`. The protocol is [`README.md`](README.md).\n\n",
        f.frontier_id,
        f.built_from.len(),
        f.axes.join("`, `")
    ));
    out.push_str("A row is **on the frontier** when no other method's bounds of its domain are, together, clearly better on some axis and no worse on any, **read on the curves they share** (two bounds that share no curve are not comparable and neither dominates). Clearly better means disjoint 95 % intervals on a shared curve; a missing interval separates nothing; ties stand. A method with several bounds is ranked on its worst: it dominates only if every one of its bounds that shares a curve does. Where either bound has unpriced work, dominance must also hold at 0, 0.1 and 1 GAE per unpriced operation (AGENTS.md §5a), and the price at which the order flips is shown. Only bounds fitted on four or more curves of a whole session are ranked; the rest are listed beneath each table with the reason. A leader is named only when its interval separates from every other method's on shared curves; otherwise the tie set is listed. `ops` is the mean ratio of `S = gae / √r` to the generic floor `√(π / 2A)`; `memory` is table entries per `√r`; `uncharged` is counted-but-unpriced work per `√r`. `ops tail` is the largest run over the median run, and the largest run's share of the total: a mean carried by its tail is marked **large**. `α` is the fitted exponent of `gae = C · r^α`, with an interval only from four sizes. Toy stays toy: nothing here speaks about cryptographic sizes.\n\n");
    let field_cols = |d: &DomainFrontier| -> Vec<&'static str> {
        FIELD_AXES
            .iter()
            .copied()
            .filter(|a| d.entries.iter().any(|e| e.axes.contains_key(*a)))
            .collect()
    };
    if f.domains.iter().any(|d| !field_cols(d).is_empty()) {
        out.push_str("`field muls`, `field sqrs` and `field invs` are the modular multiplications, squarings and inversions behind the group operations, per `√r`. A row showing `unknown` there did not count them; unknown is not zero.\n\n");
    }
    for d in &f.domains {
        out.push_str(&format!(
            "## {} · {} · {} targets · {} · tier {}\n\n",
            d.domain.problem, d.domain.family, d.domain.target_kind, d.domain.unit, d.domain.tier
        ));
        out.push_str(&format!(
            "Domain `{}`. Floor: {}. {}; {}.\n\n",
            d.domain_id,
            d.floor_model,
            leader_line("Ops", &d.ops_leader, &d.ops_tie),
            leader_line("memory", &d.memory_leader, &d.memory_tie)
        ));
        let cols = field_cols(d);
        let col_title = |a: &str| format!(" {} (/√r) |", a.replace('_', " "));
        if d.entries.is_empty() {
            out.push_str("No bound of this domain is ranked.\n\n");
        } else {
            out.push_str("| frontier | method | params | ops (× floor) | 95 % | ops tail (max/median, top run) | memory (/√r) | uncharged (/√r) |");
            for a in &cols {
                out.push_str(&col_title(a));
            }
            out.push_str(
                " α | 95 % | declared α | sizes (log₂ r) | runs | bounded | bound | dominated by |\n",
            );
            out.push_str("|:--|:--|:--|--:|:--|:--|--:|--:|");
            for _ in &cols {
                out.push_str("--:|");
            }
            out.push_str("--:|:--|--:|:--|--:|:--|:--|:--|\n");
            for e in &d.entries {
                out.push_str(&entry_row_v2(e, &cols, true));
            }
            out.push('\n');
        }
        if !d.unranked.is_empty() {
            out.push_str("Listed, not ranked:\n\n");
            out.push_str("| method | params | ops (× floor) | 95 % | ops tail | sizes (log₂ r) | runs | bound | why not ranked |\n");
            out.push_str("|:--|:--|--:|:--|:--|:--|--:|:--|:--|\n");
            for e in &d.unranked {
                out.push_str(&format!(
                    "| `{}` | {} | {} | {} | {} | {} | {} | `{}` | {} |\n",
                    e.method,
                    params_text(e),
                    fmt(e.axes.get("ops").and_then(|a| a.value)),
                    fmt_ci(e.axes.get("ops").and_then(|a| a.ci95)),
                    fmt_tail(&e.tail),
                    sizes_text(e),
                    e.runs,
                    e.bound_id,
                    e.unranked_reasons.join("; ")
                ));
            }
            out.push('\n');
        }
        let notes: Vec<String> = d
            .entries
            .iter()
            .flat_map(|e| {
                e.dominance_notes
                    .iter()
                    .map(move |n| format!("- `{}` ({}): {n}", e.bound_id, e.method))
            })
            .collect();
        if !notes.is_empty() {
            out.push_str(&notes.join("\n"));
            out.push_str("\n\n");
        }
    }
    if !f.inadmissible.is_empty() {
        out.push_str("## Inadmissible bounds (listed, never on a frontier)\n\n");
        for r in &f.inadmissible {
            out.push_str(&format!(
                "- `{}` {}: {}\n",
                r.bound_id,
                r.label,
                r.reasons.join("; ")
            ));
        }
        out.push('\n');
    }
    out
}

fn params_text(e: &Entry) -> String {
    if e.params.is_empty() {
        "–".to_string()
    } else {
        e.params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn sizes_text(e: &Entry) -> String {
    e.log2_r
        .iter()
        .map(|x| format!("{x:.1}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn entry_row_v2(e: &Entry, cols: &[&'static str], _ranked: bool) -> String {
    let ax = |n: &str| e.axes.get(n);
    let field_cells: String = cols
        .iter()
        .map(|a| {
            format!(
                " {} |",
                ax(a)
                    .filter(|x| x.known)
                    .map(|x| fmt(x.value))
                    .unwrap_or_else(|| "unknown".into())
            )
        })
        .collect();
    format!(
        "| {} | `{}` | {} | {} | {} | {} | {} | {} |{} {} | {} | {} | {} | {} | {} | `{}` | {} |\n",
        if e.is_frontier { "**yes**" } else { "no" },
        e.method,
        params_text(e),
        fmt(ax("ops").and_then(|a| a.value)),
        fmt_ci(ax("ops").and_then(|a| a.ci95)),
        fmt_tail(&e.tail),
        ax("memory")
            .filter(|a| a.known)
            .map(|a| fmt(a.value))
            .unwrap_or_else(|| "unknown".into()),
        fmt(ax("uncharged").and_then(|a| a.value)),
        field_cells,
        fmt(e.alpha),
        fmt_ci(e.alpha_ci95),
        fmt(e.declared_alpha),
        sizes_text(e),
        e.runs,
        if e.bounded { "yes" } else { "no" },
        e.bound_id,
        if e.dominated_by.is_empty() {
            "–".to_string()
        } else {
            e.dominated_by
                .iter()
                .map(|x| format!("`{x}`"))
                .collect::<Vec<_>>()
                .join(", ")
        }
    )
}

/// The frontier as a page under procedure 1.
pub fn render_markdown_v1(f: &Frontier, records_dir: &str) -> String {
    let mut out = String::new();
    out.push_str("# Measured bounds and the frontier\n\n");
    out.push_str(&format!(
        "Generated by `ecbench frontier build` from the bound records in `{records_dir}`; do not edit. Frontier `{}`, built from {} bound(s), dominance on `{}`. The protocol is [`README.md`](README.md).\n\n",
        f.frontier_id,
        f.built_from.len(),
        f.axes.join("`, `")
    ));
    out.push_str("A row is **on the frontier** when no other admissible bound of its domain is at least as good on every axis both report and clearly better (disjoint 95 % intervals) on one. Ties stand. `ops` is the mean ratio of `S = gae / √r` to the generic floor `√(π / 2A)`; `memory` is table entries per `√r`; `uncharged` is counted-but-unpriced work per `√r`. `α` is the fitted exponent of `gae = C · r^α` over the sizes listed, a scaling claim only with four or more sizes. Toy stays toy: nothing here speaks about cryptographic sizes.\n\n");
    // The field-operation columns appear in a domain only when some entry
    // there carries the axis, so a page built from bounds that predate
    // them is the page it was.
    let field_cols = |d: &DomainFrontier| -> Vec<&'static str> {
        FIELD_AXES
            .iter()
            .copied()
            .filter(|a| d.entries.iter().any(|e| e.axes.contains_key(*a)))
            .collect()
    };
    if f.domains.iter().any(|d| !field_cols(d).is_empty()) {
        out.push_str("`field muls`, `field sqrs` and `field invs` are the modular multiplications, squarings and inversions behind the group operations, per `√r`: the primitive level, counted today for the prime-field group law the generic walks and tables run on. A row showing `unknown` there did not count them; unknown is not zero.\n\n");
    }
    for d in &f.domains {
        out.push_str(&format!(
            "## {} · {} · {} targets · {} · tier {}\n\n",
            d.domain.problem, d.domain.family, d.domain.target_kind, d.domain.unit, d.domain.tier
        ));
        out.push_str(&format!(
            "Domain `{}`. Floor: {}. Ops leader: `{}`; memory leader: `{}`.\n\n",
            d.domain_id,
            d.floor_model,
            d.ops_leader.as_deref().unwrap_or("–"),
            d.memory_leader.as_deref().unwrap_or("–")
        ));
        let cols = field_cols(d);
        let col_title = |a: &str| format!(" {} (/√r) |", a.replace('_', " "));
        out.push_str("| frontier | method | params | ops (× floor) | 95 % | memory (/√r) | uncharged (/√r) |");
        for a in &cols {
            out.push_str(&col_title(a));
        }
        out.push_str(
            " α | 95 % | declared α | sizes (log₂ r) | runs | bounded | bound | dominated by |\n",
        );
        out.push_str("|:--|:--|:--|--:|:--|--:|--:|");
        for _ in &cols {
            out.push_str("--:|");
        }
        out.push_str("--:|:--|--:|:--|--:|:--|:--|:--|\n");
        for e in &d.entries {
            let ax = |n: &str| e.axes.get(n);
            let params = if e.params.is_empty() {
                "–".to_string()
            } else {
                e.params
                    .iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let field_cells: String = cols
                .iter()
                .map(|a| {
                    format!(
                        " {} |",
                        ax(a)
                            .filter(|x| x.known)
                            .map(|x| fmt(x.value))
                            .unwrap_or_else(|| "unknown".into())
                    )
                })
                .collect();
            out.push_str(&format!(
                "| {} | `{}` | {} | {} | {} | {} | {} |{} {} | {} | {} | {} | {} | {} | `{}` | {} |\n",
                if e.is_frontier { "**yes**" } else { "no" },
                e.method,
                params,
                fmt(ax("ops").and_then(|a| a.value)),
                fmt_ci(ax("ops").and_then(|a| a.ci95)),
                ax("memory")
                    .filter(|a| a.known)
                    .map(|a| fmt(a.value))
                    .unwrap_or_else(|| "unknown".into()),
                fmt(ax("uncharged").and_then(|a| a.value)),
                field_cells,
                fmt(e.alpha),
                fmt_ci(e.alpha_ci95),
                fmt(e.declared_alpha),
                e.log2_r
                    .iter()
                    .map(|x| format!("{x:.1}"))
                    .collect::<Vec<_>>()
                    .join(", "),
                e.runs,
                if e.bounded { "yes" } else { "no" },
                e.bound_id,
                if e.dominated_by.is_empty() {
                    "–".to_string()
                } else {
                    e.dominated_by
                        .iter()
                        .map(|x| format!("`{x}`"))
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            ));
        }
        out.push('\n');
        let notes: Vec<String> = d
            .entries
            .iter()
            .flat_map(|e| {
                e.dominance_notes
                    .iter()
                    .map(move |n| format!("- `{}` ({}): {n}", e.bound_id, e.method))
            })
            .collect();
        if !notes.is_empty() {
            out.push_str(&notes.join("\n"));
            out.push_str("\n\n");
        }
    }
    if !f.inadmissible.is_empty() {
        out.push_str("## Inadmissible bounds (listed, never on a frontier)\n\n");
        for r in &f.inadmissible {
            out.push_str(&format!(
                "- `{}` {}: {}\n",
                r.bound_id,
                r.label,
                r.reasons.join("; ")
            ));
        }
        out.push('\n');
    }
    out
}

/// Whether `path` holds exactly the rendered page.
pub fn check_markdown(f: &Frontier, records_dir: &str, path: &Path) -> Result<(), String> {
    let on_disk = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let fresh = render_markdown(f, records_dir);
    if on_disk != fresh {
        return Err(format!(
            "{} is stale: rebuild it with `ecbench frontier build`",
            path.display()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn axis(v: f64, ci: Option<(f64, f64)>) -> Axis {
        Axis {
            known: true,
            value: Some(v),
            ci95: ci,
        }
    }

    fn entry(id: &str, ops: Axis, memory: Axis) -> Entry {
        let mut axes = BTreeMap::new();
        axes.insert("ops".to_string(), ops);
        axes.insert("memory".to_string(), memory);
        axes.insert(
            "uncharged".to_string(),
            Axis {
                known: true,
                value: Some(0.0),
                ci95: None,
            },
        );
        Entry {
            bound_id: id.into(),
            label: id.into(),
            method_id: id.into(),
            method: id.into(),
            params: BTreeMap::new(),
            level: "constant".into(),
            log2_r: vec![16.0],
            runs: 8,
            axes,
            alpha: None,
            alpha_ci95: None,
            declared_alpha: None,
            scaling_claim: false,
            bounded: false,
            is_frontier: false,
            dominated_by: vec![],
            dominance_notes: vec![],
            improves_on: vec![],
            fit_version: 0,
            tail: None,
            unranked_reasons: vec![],
            price_checks: vec![],
        }
    }

    #[test]
    fn disjoint_intervals_decide_and_overlapping_ones_tie() {
        let a = axis(1.0, Some((0.9, 1.1)));
        let b = axis(1.5, Some((1.3, 1.7)));
        let c = axis(1.15, Some((1.0, 1.3)));
        assert_eq!(order(&a, &b), Order::ClearlyBetter);
        assert_eq!(order(&b, &a), Order::ClearlyWorse);
        assert_eq!(order(&a, &c), Order::Indistinguishable);
        let unknown = Axis {
            known: false,
            value: None,
            ci95: None,
        };
        assert_eq!(order(&a, &unknown), Order::Unknown);
        // Points only: strict inequality.
        assert_eq!(
            order(&axis(1.0, None), &axis(2.0, None)),
            Order::ClearlyBetter
        );
    }

    #[test]
    fn dominance_needs_a_clear_win_and_no_clear_loss() {
        let axes: Vec<String> = DEFAULT_AXES.iter().map(|s| s.to_string()).collect();
        let rho = entry(
            "rho",
            axis(1.0, Some((0.9, 1.1))),
            axis(0.08, Some((0.07, 0.09))),
        );
        let bsgs = entry(
            "bsgs",
            axis(1.7, Some((1.6, 1.8))),
            axis(1.0, Some((0.98, 1.02))),
        );
        let (d, wins, _) = dominates(&rho, &bsgs, &axes);
        assert!(d);
        assert_eq!(wins, vec!["ops".to_string(), "memory".to_string()]);
        assert!(!dominates(&bsgs, &rho, &axes).0);
        // Better on ops, worse on memory: a trade, not dominance.
        let table = entry(
            "table",
            axis(0.5, Some((0.45, 0.55))),
            axis(3.0, Some((2.9, 3.1))),
        );
        assert!(!dominates(&table, &rho, &axes).0);
        assert!(!dominates(&rho, &table, &axes).0);
        // A wide interval with the worse point estimate on ops cannot
        // dominate through memory.
        let wide = entry(
            "wide",
            axis(1.6, Some((0.5, 3.0))),
            axis(0.01, Some((0.0, 0.02))),
        );
        assert!(!dominates(&wide, &rho, &axes).0);
        assert!(!dominates(&rho, &wide, &axes).0);
        // Memory unknown on one side: ops decides, and the axis is reported.
        let mut ic = entry("ic", axis(5.0, Some((4.0, 6.0))), axis(0.0, None));
        ic.axes.get_mut("memory").unwrap().known = false;
        let (d, wins, unknown) = dominates(&rho, &ic, &axes);
        assert!(d);
        assert_eq!(wins, vec!["ops".to_string()]);
        assert_eq!(unknown, vec!["memory".to_string()]);
    }
    // ── Procedure 2 ────────────────────────────────────────────────

    use crate::cryptanalysis::ecbench::bounds::{
        Admissibility, BandPoint, Constant, Dimension, Envelope, Estimate, Fit, FitOptions,
        MethodRef, Provenance, SizeAxis, BOUND_SCHEMA,
    };

    /// One size: `log2 r`, ops ratio with its interval, memory.
    struct Sz(f64, f64, (f64, f64), f64);

    fn size_row(sz: &Sz, uncharged: f64) -> SizeRow {
        let floor = 0.886;
        let r = 2f64.powf(sz.0) as u128;
        let s = |x: f64| x * floor;
        let point = |w: f64| BandPoint {
            weight: w,
            s: Some(s(sz.1) + w * uncharged * floor),
            ci95: Some((
                s(sz.2 .0) + w * uncharged * floor,
                s(sz.2 .1) + w * uncharged * floor,
            )),
        };
        let mut axes = BTreeMap::new();
        axes.insert(
            "memory".to_string(),
            SizeAxis {
                value: sz.3,
                ci95: Some((sz.3 * 0.99, sz.3 * 1.01)),
            },
        );
        SizeRow {
            slug: format!("c{}", sz.0),
            log2_r: sz.0,
            r,
            field_bits: 20,
            automorphisms_available: 2,
            floor_s: floor,
            workloads: 4,
            runs: 8,
            verified: 8,
            mean_gae: None,
            mean_s: Some(s(sz.1)),
            s_ci95: Some((s(sz.2 .0), s(sz.2 .1))),
            ratio_to_floor: Some(sz.1),
            declared_s: None,
            declared_ratio_to_floor: None,
            memory_entries_per_sqrt_r: Some(sz.3),
            uncharged_per_sqrt_r: Some(uncharged),
            levels: BTreeMap::new(),
            axes,
            band: vec![point(0.0), point(0.1), point(1.0)],
        }
    }

    fn fake(id: &str, method: &str, sizes: &[Sz], uncharged: f64, bounded: bool) -> Bound {
        let mean = sizes.iter().map(|s| s.1).sum::<f64>() / sizes.len() as f64;
        let lo = sizes.iter().map(|s| s.2 .0).sum::<f64>() / sizes.len() as f64;
        let hi = sizes.iter().map(|s| s.2 .1).sum::<f64>() / sizes.len() as f64;
        let domain = Domain {
            problem: "ecdlp.single_target".into(),
            family: "prime".into(),
            target_kind: "planted".into(),
            unit: "ecbench.gae".into(),
            tier: "toy".into(),
            envelope: Envelope::default(),
        };
        let mut dims = BTreeMap::new();
        dims.insert(
            "ops".to_string(),
            Dimension {
                statistic: "ops".into(),
                source: "x".into(),
                lower_is_better: true,
                known: true,
                value: Some(mean),
                ci95: Some((lo, hi)),
            },
        );
        let mem = sizes.iter().map(|s| s.3).sum::<f64>() / sizes.len() as f64;
        dims.insert(
            "memory".to_string(),
            Dimension {
                statistic: "m".into(),
                source: "x".into(),
                lower_is_better: true,
                known: true,
                value: Some(mem),
                ci95: Some((mem * 0.99, mem * 1.01)),
            },
        );
        Bound {
            schema: BOUND_SCHEMA.into(),
            bound_id: id.into(),
            label: id.into(),
            domain_id: domain.id().unwrap(),
            domain,
            method: MethodRef {
                id: method.into(),
                params: BTreeMap::new(),
                method_id: method.into(),
                family: "x".into(),
                entry: "x".into(),
            },
            level: "exponent".into(),
            sizes: sizes.iter().map(|z| size_row(z, uncharged)).collect(),
            fit: Fit {
                size_parameter: "r".into(),
                model: "m".into(),
                points: 1,
                sizes: sizes.len() as u64,
                alpha: Some(0.5),
                alpha_ci95: Some((0.4, 0.6)),
                log2_c: None,
                r_squared: None,
                ci_method: "cluster".into(),
                declared_alpha: Some(0.5),
                alpha_agrees_with_declared: Some(true),
                scaling_claim: true,
            },
            constant: Constant {
                s: Estimate {
                    value: 1.0,
                    ci95: None,
                    ci_method: "none".into(),
                },
                ratio_to_floor: Estimate {
                    value: mean,
                    ci95: Some((lo, hi)),
                    ci_method: "cluster".into(),
                },
                declared_ratio_to_floor: None,
            },
            dimensions: dims,
            stages: vec![],
            provenance: Provenance {
                sessions: vec![],
                audits: vec![],
                records: 8,
                verified: 8,
                statuses: BTreeMap::new(),
            },
            admissibility: Admissibility {
                status: "admissible".into(),
                bounded,
                unpriced: vec![],
                deterministic: true,
                reasons: vec![],
            },
            fit_options: FitOptions::default(),
            improves_on: vec![],
            verdict_id: None,
            notes: String::new(),
            tail: None,
        }
    }

    /// Four sizes at 16, 18, 20, 22 bits with one ratio and a ±5 % interval.
    fn flat(ratio: f64, mem: f64, bits: &[f64]) -> Vec<Sz> {
        bits.iter()
            .map(|b| Sz(*b, ratio, (ratio * 0.95, ratio * 1.05), mem))
            .collect()
    }

    const FOUR: [f64; 4] = [16.0, 18.0, 20.0, 22.0];

    fn default_axes() -> Vec<String> {
        DEFAULT_AXES.iter().map(|s| s.to_string()).collect()
    }

    fn entry_of_id<'a>(f: &'a Frontier, id: &str) -> &'a Entry {
        f.domains
            .iter()
            .flat_map(|d| d.entries.iter().chain(d.unranked.iter()))
            .find(|e| e.bound_id == id)
            .unwrap_or_else(|| panic!("{id} not on the page"))
    }

    #[test]
    fn a_missing_interval_separates_nothing_in_procedure_2() {
        let point = axis(1.0, None);
        let wide = axis(2.0, Some((1.5, 2.5)));
        assert_eq!(order(&point, &wide), Order::ClearlyBetter);
        assert_eq!(order_v2(&point, &wide), Order::Indistinguishable);
        assert_eq!(
            order_v2(&axis(1.0, Some((0.9, 1.1))), &wide),
            Order::ClearlyBetter
        );
    }

    #[test]
    fn bounds_that_share_no_size_are_not_comparable_and_neither_dominates() {
        // Procedure 1 let a bound fitted on large curves knock off one
        // fitted on small ones; the ratio depends on the size.
        let strong = fake(
            "strong",
            "rho.negation",
            &flat(1.1, 0.05, &[24.0, 26.0, 28.0, 30.0]),
            0.0,
            false,
        );
        let weak = fake("weak", "bsgs.textbook", &flat(5.0, 1.0, &FOUR), 0.0, false);
        let axes = default_axes();
        let v1 = build_v1(&[strong.clone(), weak.clone()], &axes).unwrap();
        assert!(
            !entry_of_id(&v1, "weak").is_frontier,
            "procedure 1 compared them"
        );
        let f = build(&[strong, weak], &axes).unwrap();
        assert_eq!(f.schema, FRONTIER_SCHEMA);
        assert!(entry_of_id(&f, "weak").is_frontier);
        assert!(entry_of_id(&f, "strong").is_frontier);
        assert!(entry_of_id(&f, "weak")
            .dominance_notes
            .iter()
            .any(|n| n.contains("shares no size")));
    }

    #[test]
    fn dominance_reads_only_the_shared_sizes() {
        // `a` is better on the two sizes it shares with `b`; on its other
        // sizes it is worse, which `b` never measured.
        let mut sa = flat(1.0, 0.05, &[16.0, 18.0]);
        sa.extend(flat(30.0, 0.05, &[24.0, 26.0]));
        let a = fake("a", "rho.signed_frobenius", &sa, 0.0, false);
        let b = fake("b", "bsgs.negation", &flat(5.0, 1.0, &FOUR), 0.0, false);
        let f = build(&[a, b], &default_axes()).unwrap();
        assert!(entry_of_id(&f, "a").is_frontier);
        assert_eq!(entry_of_id(&f, "b").dominated_by, vec!["a".to_string()]);
    }

    #[test]
    fn a_bound_on_fewer_than_four_sizes_or_a_filtered_fit_is_not_ranked() {
        let three = fake(
            "three",
            "rho.negation",
            &flat(1.0, 0.05, &FOUR[..3]),
            0.0,
            false,
        );
        let mut filtered = fake("filtered", "rho.plain", &flat(1.0, 0.05, &FOUR), 0.0, false);
        filtered.fit_options.curves = vec!["c16".into()];
        let mut tiered = fake("tiered", "rho.plain", &flat(1.0, 0.05, &FOUR), 0.0, false);
        tiered.fit_options.tier = Some("toy".into());
        let mut legacy = fake("legacy", "rho.plain", &flat(1.0, 0.05, &FOUR), 0.0, false);
        legacy.fit_options.fit_version = 1;
        legacy.improves_on = vec!["x".into()];
        let whole = fake("whole", "bsgs.textbook", &flat(9.0, 1.0, &FOUR), 0.0, false);
        let f = build(&[three, filtered, tiered, legacy, whole], &default_axes()).unwrap();
        let d = &f.domains[0];
        assert_eq!(d.entries.len(), 1);
        assert_eq!(d.unranked.len(), 4);
        for e in &d.unranked {
            assert!(!e.is_frontier && !e.unranked_reasons.is_empty(), "{e:?}");
        }
        assert!(entry_of_id(&f, "whole").is_frontier);
        // The ranked entry is not dominated by the unranked bounds.
        assert!(entry_of_id(&f, "whole").dominated_by.is_empty());
        assert!(entry_of_id(&f, "legacy").improves_on.is_empty());
    }

    #[test]
    fn a_leader_is_named_only_when_its_interval_separates() {
        // Overlapping intervals: a tie set, no leader (procedure 1 named
        // the smaller point estimate).
        let a = fake(
            "a",
            "rho.signed_frobenius",
            &flat(3.3, 0.5, &FOUR),
            0.0,
            false,
        );
        let b = fake("b", "rho.negation", &flat(3.5, 0.5, &FOUR), 0.0, false);
        let mut wide_a = a.clone();
        wide_a
            .sizes
            .iter_mut()
            .for_each(|s| s.s_ci95 = Some((1.0, 6.0)));
        let mut wide_b = b.clone();
        wide_b
            .sizes
            .iter_mut()
            .for_each(|s| s.s_ci95 = Some((1.0, 6.0)));
        let f = build(&[wide_a, wide_b], &default_axes()).unwrap();
        let d = &f.domains[0];
        assert_eq!(d.ops_leader, None);
        assert_eq!(d.ops_tie, vec!["a".to_string(), "b".to_string()]);
        // Separated (a trade, so both stand): named, on each axis.
        let c = fake("c", "bsgs.negation", &flat(9.0, 0.001, &FOUR), 0.0, false);
        let f = build(&[a, c], &default_axes()).unwrap();
        assert!(f.domains[0].entries.iter().all(|e| e.is_frontier));
        assert_eq!(f.domains[0].ops_leader.as_deref(), Some("a"));
        assert_eq!(f.domains[0].memory_leader.as_deref(), Some("c"));
        assert!(f.domains[0].ops_tie.is_empty());
        // Sharing no size cannot separate.
        let far = fake(
            "far",
            "bsgs.negation",
            &flat(9.0, 0.6, &[30.0, 32.0, 34.0, 36.0]),
            0.0,
            false,
        );
        let near = fake(
            "near",
            "rho.signed_frobenius",
            &flat(3.3, 0.014, &FOUR),
            0.0,
            false,
        );
        let f = build(&[near, far], &default_axes()).unwrap();
        assert_eq!(f.domains[0].ops_leader, None);
        assert_eq!(f.domains[0].ops_tie.len(), 2);
    }

    #[test]
    fn a_method_is_ranked_on_its_worst_bound() {
        // The luckiest session of rho.negation beats bsgs on every size;
        // its other bound does not.  bsgs stays on the frontier.
        let lucky = fake("lucky", "rho.negation", &flat(1.0, 0.05, &FOUR), 0.0, false);
        let typical = fake(
            "typical",
            "rho.negation",
            &flat(4.9, 0.05, &FOUR),
            0.0,
            false,
        );
        let bsgs = fake("bsgs", "bsgs.negation", &flat(5.0, 0.05, &FOUR), 0.0, false);
        let f = build(
            &[lucky.clone(), typical.clone(), bsgs.clone()],
            &default_axes(),
        )
        .unwrap();
        assert!(entry_of_id(&f, "bsgs").dominated_by.is_empty());
        // With a lucky bound alone, as procedure 1 ranked it, bsgs falls.
        let alone = build(&[lucky, bsgs], &default_axes()).unwrap();
        assert!(!entry_of_id(&alone, "bsgs").is_frontier);
        // And when every bound of the method wins, bsgs falls.
        let also = fake("also", "rho.negation", &flat(1.2, 0.05, &FOUR), 0.0, false);
        let lucky2 = fake(
            "lucky2",
            "rho.negation",
            &flat(1.0, 0.05, &FOUR),
            0.0,
            false,
        );
        let bsgs2 = fake(
            "bsgs2",
            "bsgs.negation",
            &flat(5.0, 0.05, &FOUR),
            0.0,
            false,
        );
        let f = build(&[lucky2, also, bsgs2], &default_axes()).unwrap();
        assert_eq!(entry_of_id(&f, "bsgs2").dominated_by.len(), 2);
    }

    #[test]
    fn a_dominance_with_unpriced_work_must_hold_across_the_band() {
        // `cheap` charges 1.0 and leaves 4.0 (per √r) unpriced; `full`
        // charges 2.5 and leaves nothing: cheap wins at w = 0 and loses
        // once the unpriced work costs more than 0.375 GAE per operation.
        let cheap = fake(
            "cheap",
            "bsgs.negation",
            &flat(1.0, 0.5, &FOUR),
            4.0 * 1.0,
            true,
        );
        let full = fake("full", "rho.negation", &flat(2.5, 0.6, &FOUR), 0.0, false);
        let axes = default_axes();
        let pc = price_check(&cheap, &full, &axes);
        // d(w) = 2.5 - (1.0 + 4 w): flips at w = 0.375.
        assert_eq!(pc.holds_across_band, Some(false));
        assert!(
            (pc.break_even_price.unwrap() - 0.375).abs() < 1e-9,
            "{pc:?}"
        );
        let f = build(&[cheap.clone(), full.clone()], &axes).unwrap();
        assert!(
            entry_of_id(&f, "full").is_frontier,
            "dominance fails inside the band"
        );
        let note = entry_of_id(&f, "full").dominance_notes.join(" ");
        assert!(
            note.contains("not across the pricing band") && note.contains("0.375"),
            "{note}"
        );
        assert_eq!(entry_of_id(&f, "full").price_checks.len(), 1);
        // With little unpriced work it holds across the whole band, and the
        // break-even is reported anyway.
        let honest = fake("honest", "bsgs.negation", &flat(1.0, 0.5, &FOUR), 0.2, true);
        let pc = price_check(&honest, &full, &axes);
        assert_eq!(pc.holds_across_band, Some(true));
        assert!((pc.break_even_price.unwrap() - 7.5).abs() < 1e-9, "{pc:?}");
        let f = build(&[honest, full], &axes).unwrap();
        assert_eq!(
            entry_of_id(&f, "full").dominated_by,
            vec!["honest".to_string()]
        );
    }

    #[test]
    fn a_bound_that_cannot_be_priced_above_zero_cannot_dominate_across_the_band() {
        let mut gap = fake("gap", "bsgs.negation", &flat(1.0, 0.5, &FOUR), 0.0, true);
        for s in &mut gap.sizes {
            for p in s.band.iter_mut().filter(|p| p.weight > 0.0) {
                p.s = None;
                p.ci95 = None;
            }
        }
        let full = fake("full", "rho.negation", &flat(2.5, 0.6, &FOUR), 0.0, false);
        let pc = price_check(&gap, &full, &default_axes());
        assert_eq!(pc.holds_across_band, None);
        let f = build(&[gap, full], &default_axes()).unwrap();
        assert!(entry_of_id(&f, "full").is_frontier);
    }

    #[test]
    fn a_committed_v1_frontier_is_the_one_procedure_1_builds() {
        // The schema names the procedure; the dispatcher keeps both.
        assert_eq!(version_of_schema(FRONTIER_SCHEMA_V1), Some(1));
        assert_eq!(version_of_schema(FRONTIER_SCHEMA), Some(2));
        assert_eq!(version_of_schema("x"), None);
        let b = fake("b", "rho.negation", &flat(1.0, 0.05, &FOUR), 0.0, false);
        let one = build_version(&[b.clone()], &default_axes(), 1).unwrap();
        assert_eq!(one.schema, FRONTIER_SCHEMA_V1);
        let text = serde_json::to_string(&one).unwrap();
        assert!(
            !text.contains("fit_version") && !text.contains("ops_tie"),
            "v1 bytes are unchanged"
        );
        let two = build_version(&[b], &default_axes(), 2).unwrap();
        assert_eq!(two.schema, FRONTIER_SCHEMA);
        assert!(build_version(&[], &default_axes(), 3).is_err());
    }
}
