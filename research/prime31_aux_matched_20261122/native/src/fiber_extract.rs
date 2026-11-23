//! Bounded triangular extraction with certified selector-fiber domains.
//!
//! This is orchestration around the unchanged pinned F4 and finite-field root
//! code. It never invokes the visitor's finite-domain or full-field enumeration
//! fallback. A selector can branch only when a nonconstant univariate removes
//! at least one value from its current parent-conditioned domain. Singleton
//! domains propagate without branching. An unresolved coupled system is an
//! unknown outcome, not an empty variety.
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::cryptanalysis::f4_fp::{self, F4Options, Poly, VisitReport};
use crate::oracle::{Presentation, TowerDescription, ValidatedTowerPaths};

pub const DIRECT_DOMAIN_LIMIT: usize = 4;

/// The effective extraction policy is separate from the retained F4 options:
/// `domain_roots` and `sibling_batch` configure its old visitor, which this
/// driver never calls. The schema is repeated in the query and chart records.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct FiberExtractionPolicy {
    pub schema: &'static str,
    pub selector_domain: &'static str,
    pub univariate_roots: &'static str,
    pub direct_domain_limit: usize,
    pub root_filter: &'static str,
    pub substitution: &'static str,
    pub selector_branch: &'static str,
    pub preserves_incoming_equations_and_f4_consequences: bool,
    pub legacy_domain_roots_applied: bool,
    pub legacy_sibling_batch_applied: bool,
    pub static_cartesian_fallback: bool,
    pub full_field_scan_fallback: bool,
}

pub fn effective_policy() -> FiberExtractionPolicy {
    FiberExtractionPolicy {
        schema: "prime31-fiber-extraction-policy/v1",
        selector_domain: "descendants_of_nearest_fixed_selector_ancestor",
        univariate_roots: "direct_domain_horner_at_limit_else_pinned_dense_roots_fp",
        direct_domain_limit: DIRECT_DOMAIN_LIMIT,
        root_filter: "all_available_univariates_and_conditional_domain",
        substitution: "pinned_substitute_var_per_child_and_forced_ancestor",
        selector_branch: "strict_domain_reduction_or_singleton",
        preserves_incoming_equations_and_f4_consequences: true,
        legacy_domain_roots_applied: false,
        legacy_sibling_batch_applied: false,
        static_cartesian_fallback: false,
        full_field_scan_fallback: false,
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct FiberIndexReport {
    pub full_leaf_values: usize,
    pub live_leaf_values: usize,
    pub block_width: usize,
    pub variable_levels: usize,
    pub full_nodes: usize,
    pub live_nodes: usize,
    pub live_edges: usize,
    pub construction_field_multiplications: u64,
    pub construction_edge_insertions: u64,
    pub logical_value_payload_bytes: usize,
    pub reuses_fixed_validation_paths: bool,
    pub reused_block_path_values: usize,
    pub logical_temporary_path_payload_bytes: usize,
    pub elapsed_ns: u64,
    pub accounting: &'static str,
}

/// Exact maps between successive selector levels. Each stored edge is checked
/// by evaluating the original degree-two maps. All live paths start at one of
/// the caller's allowed leaves. No target-curve group law is used here.
pub struct FiberIndex {
    p: u64,
    terminal: u64,
    parents: Vec<BTreeMap<u64, u64>>,
    children: Vec<BTreeMap<u64, Vec<u64>>>,
    report: FiberIndexReport,
}

fn product(a: u64, b: u64, p: u64, count: &mut u64) -> u64 {
    *count += 1;
    (u128::from(a) * u128::from(b) % u128::from(p)) as u64
}

fn power(mut x: u64, mut n: u64, p: u64, count: &mut u64) -> u64 {
    let mut y = 1;
    while n != 0 {
        if n & 1 != 0 {
            y = product(y, x, p, count);
        }
        n >>= 1;
        if n != 0 {
            x = product(x, x, p, count);
        }
    }
    y
}

impl FiberIndex {
    /// Build one index from single-coordinate paths, never leaf tuples. This
    /// independent constructor checks the complete full fiber even if its
    /// caller has already validated it. Its additional arithmetic is reported.
    pub fn prepare(
        p: u64,
        full_values: &[u64],
        allowed_leaves: &[u64],
        tower: &TowerDescription,
        presentation: Presentation,
    ) -> Result<Self, String> {
        let started = Instant::now();
        let depth = tower.depth();
        if p < 5
            || p >= (1u64 << 32)
            || !crate::cryptanalysis::residual_walk::is_prime_u64(p)
            || presentation == Presentation::Dense
            || depth == 0
            || depth >= usize::BITS as usize
            || full_values.len() != 1usize << depth
            || full_values.iter().any(|&x| x >= p)
            || full_values.windows(2).any(|w| w[0] >= w[1])
            || allowed_leaves.windows(2).any(|w| w[0] >= w[1])
            || allowed_leaves
                .iter()
                .any(|x| full_values.binary_search(x).is_err())
        {
            return Err(
                "invalid complete fiber, live leaf subset, field or tower presentation".into(),
            );
        }
        let terminal = match tower {
            TowerDescription::Velu { steps, terminal } => {
                if *terminal >= p
                    || steps
                        .iter()
                        .any(|&(xi, tau)| xi >= p || tau == 0 || tau >= p)
                {
                    return Err("invalid canonical Velu fiber coefficients".into());
                }
                *terminal
            }
            TowerDescription::Dickson {
                shift, terminal, ..
            } => {
                if *shift >= p || *terminal >= p {
                    return Err("invalid canonical Dickson fiber coefficients".into());
                }
                *terminal
            }
        };
        let width = if presentation == Presentation::Tower4 {
            2
        } else {
            1
        };
        let levels = depth.div_ceil(width);
        let mut full_sets = vec![BTreeSet::new(); levels + 1];
        let mut live_sets = vec![BTreeSet::new(); levels + 1];
        let mut parents = vec![BTreeMap::new(); levels];
        let mut children: Vec<BTreeMap<u64, Vec<u64>>> = vec![BTreeMap::new(); levels];
        let mut multiplications = 0;
        let mut edge_insertions = 0;
        for &leaf in full_values {
            let live = allowed_leaves.binary_search(&leaf).is_ok();
            let mut x = leaf;
            let mut prior = leaf;
            let mut level = 0;
            full_sets[0].insert(leaf);
            if live {
                live_sets[0].insert(leaf);
            }
            for step in 0..depth {
                x = match tower {
                    TowerDescription::Velu { steps, .. } => {
                        let (xi, tau) = steps[step];
                        let denominator = (x + p - xi) % p;
                        if denominator == 0 {
                            return Err("full fiber contains an original-map pole".into());
                        }
                        let inverse = power(denominator, p - 2, p, &mut multiplications);
                        (x + product(tau, inverse, p, &mut multiplications)) % p
                    }
                    TowerDescription::Dickson { shift, .. } => {
                        if step == 0 {
                            x = (x + p - shift) % p;
                        }
                        (product(x, x, p, &mut multiplications) + p - 2) % p
                    }
                };
                if (step + 1) % width == 0 || step + 1 == depth {
                    full_sets[level + 1].insert(x);
                    if live {
                        live_sets[level + 1].insert(x);
                        if let Some(old) = parents[level].insert(prior, x) {
                            if old != x {
                                return Err("selector child has inconsistent parents".into());
                            }
                        } else {
                            children[level].entry(x).or_default().push(prior);
                            edge_insertions += 1;
                        }
                    }
                    prior = x;
                    level += 1;
                }
            }
            if x != terminal || level != levels {
                return Err("full fiber misses the retained terminal".into());
            }
        }
        for level in 0..levels {
            let step_count = width.min(depth - level * width);
            if full_sets[level].len() != full_sets[level + 1].len() * (1usize << step_count) {
                return Err("full fiber is not split with the required local degrees".into());
            }
            for children in children[level].values_mut() {
                children.sort_unstable();
                children.dedup();
                if children.len() > 1usize << step_count {
                    return Err("local inverse fiber exceeds the map degree".into());
                }
            }
        }
        let full_nodes = full_sets.iter().map(BTreeSet::len).sum();
        let live_nodes: usize = live_sets.iter().map(BTreeSet::len).sum();
        let live_edges: usize = parents.iter().map(BTreeMap::len).sum();
        let inverse_keys: usize = children.iter().map(BTreeMap::len).sum();
        let report = FiberIndexReport {
            full_leaf_values: full_values.len(),
            live_leaf_values: allowed_leaves.len(),
            block_width: width,
            variable_levels: levels,
            full_nodes,
            live_nodes,
            live_edges,
            construction_field_multiplications: multiplications,
            construction_edge_insertions: edge_insertions,
            logical_value_payload_bytes: (2 + 3 * live_edges + inverse_keys) * std::mem::size_of::<u64>(),
            reuses_fixed_validation_paths: false,
            reused_block_path_values: 0,
            logical_temporary_path_payload_bytes: 0,
            elapsed_ns: started.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64,
            accounting: "Standalone charged setup: one independent traversal of each full-V single-coordinate path, never a Cartesian leaf-tuple enumeration. Multiplications include original-map inversion powers; edge insertions and payload slots are logical counts. BTreeMap/BTreeSet comparisons, allocator overhead and temporary full sets are not a complete native-work or peak-memory total. This standalone constructor does not reuse any PreparedOracle validation.",
        };
        Ok(Self {
            p,
            terminal,
            parents,
            children,
            report,
        })
    }

    /// Consume an opaque successful validation result. The field, original
    /// depth, presentation width and terminal all come from that same token;
    /// callers cannot pair checked paths with replacement raw tower metadata.
    /// This constructor performs no original-map arithmetic. All structural
    /// and local-degree checks from the standalone construction remain here.
    pub(crate) fn from_validated_paths(
        validated: ValidatedTowerPaths,
        allowed_leaves: &[u64],
    ) -> Result<Self, String> {
        let started = Instant::now();
        let (p, depth, width, terminal, paths) = validated.into_parts();
        if p < 5
            || p >= (1u64 << 32)
            || depth == 0
            || depth >= usize::BITS as usize
            || !matches!(width, 1 | 2)
            || terminal >= p
            || paths.len() != 1usize << depth
        {
            return Err("invalid validated-path metadata boundary".into());
        }
        let levels = depth.div_ceil(width);
        if paths.iter().any(|path| {
            path.len() != levels + 1
                || path.iter().any(|&x| x >= p)
                || path.last().copied() != Some(terminal)
        }) || paths.windows(2).any(|pair| pair[0][0] >= pair[1][0])
            || allowed_leaves.windows(2).any(|pair| pair[0] >= pair[1])
            || allowed_leaves
                .iter()
                .any(|leaf| paths.binary_search_by_key(leaf, |path| path[0]).is_err())
        {
            return Err("invalid validated full paths or live leaf subset".into());
        }
        let path_values = paths
            .len()
            .checked_mul(levels + 1)
            .ok_or("validated-path logical value count overflow")?;
        let path_payload_bytes = path_values
            .checked_mul(std::mem::size_of::<u64>())
            .ok_or("validated-path logical payload overflow")?;
        let mut full_sets = vec![BTreeSet::new(); levels + 1];
        let mut live_sets = vec![BTreeSet::new(); levels + 1];
        let mut parents = vec![BTreeMap::new(); levels];
        let mut children: Vec<BTreeMap<u64, Vec<u64>>> = vec![BTreeMap::new(); levels];
        let mut edge_insertions = 0;
        for path in &paths {
            let live = allowed_leaves.binary_search(&path[0]).is_ok();
            for (level, &value) in path.iter().enumerate() {
                full_sets[level].insert(value);
                if live {
                    live_sets[level].insert(value);
                }
            }
            if live {
                for level in 0..levels {
                    let child = path[level];
                    let parent = path[level + 1];
                    if let Some(old) = parents[level].insert(child, parent) {
                        if old != parent {
                            return Err("selector child has inconsistent parents".into());
                        }
                    } else {
                        children[level].entry(parent).or_default().push(child);
                        edge_insertions += 1;
                    }
                }
            }
        }
        // Keep the standalone constructor's full-fiber split and local
        // inverse-degree checks, including the final unpaired quadratic.
        for level in 0..levels {
            let step_count = width.min(depth - level * width);
            if full_sets[level].len() != full_sets[level + 1].len() * (1usize << step_count) {
                return Err("full fiber is not split with the required local degrees".into());
            }
            for children in children[level].values_mut() {
                children.sort_unstable();
                children.dedup();
                if children.len() > 1usize << step_count {
                    return Err("local inverse fiber exceeds the map degree".into());
                }
            }
        }
        let full_nodes = full_sets.iter().map(BTreeSet::len).sum();
        let live_nodes = live_sets.iter().map(BTreeSet::len).sum();
        let live_edges: usize = parents.iter().map(BTreeMap::len).sum();
        let inverse_keys: usize = children.iter().map(BTreeMap::len).sum();
        let report = FiberIndexReport {
            full_leaf_values: paths.len(),
            live_leaf_values: allowed_leaves.len(),
            block_width: width,
            variable_levels: levels,
            full_nodes,
            live_nodes,
            live_edges,
            construction_field_multiplications: 0,
            construction_edge_insertions: edge_insertions,
            logical_value_payload_bytes: (2 + 3 * live_edges + inverse_keys) * std::mem::size_of::<u64>(),
            reuses_fixed_validation_paths: true,
            reused_block_path_values: path_values,
            logical_temporary_path_payload_bytes: path_payload_bytes,
            elapsed_ns: started.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64,
            accounting: "Charged index setup reuses block endpoints captured during PreparedOracle's one full-V original-map validation. All original-map arithmetic, including inversions, is charged once in validation_field_multiplications; no second map traversal is performed here. Zero construction_field_multiplications does not mean zero index cost: structural checks, BTreeMap/BTreeSet work, edge insertion and allocation remain in cold setup. Captured-path logical payload is temporary and excludes Vec headers, spare capacity and allocator overhead; persistent payload likewise excludes container overhead. Neither payload is a peak-memory total.",
        };
        Ok(Self {
            p,
            terminal,
            parents,
            children,
            report,
        })
    }

    pub fn setup_report(&self) -> &FiberIndexReport {
        &self.report
    }

    fn variable_levels(&self) -> usize {
        self.parents.len()
    }

    fn domain(
        &self,
        original: usize,
        summands: usize,
        assignment: &[Option<u64>],
        stats: &mut FiberStats,
        deadline: Option<Instant>,
    ) -> Result<Option<Vec<u64>>, End> {
        if original >= summands * self.variable_levels() {
            return Ok(None); // Target-group prefix coordinate, not a selector.
        }
        stats.domain_queries += 1;
        let level = original / summands;
        let chain = original % summands;
        let mut anchor = self.variable_levels();
        let mut value = self.terminal;
        for next in level + 1..self.variable_levels() {
            if let Some(fixed) = assignment[next * summands + chain] {
                anchor = next;
                value = fixed;
                break;
            }
        }
        let mut current = vec![value];
        for j in (level..anchor).rev() {
            let mut next = Vec::new();
            for parent in current {
                if deadline.is_some_and(|d| Instant::now() >= d) {
                    return Err(End::TimedOut);
                }
                stats.domain_index_lookups += 1;
                if let Some(children) = self.children[j].get(&parent) {
                    stats.domain_edges_read += children.len() as u64;
                    next.extend_from_slice(children);
                }
            }
            current = next;
        }
        current.sort_unstable();
        current.dedup();
        if anchor == level + 1 {
            stats.immediate_parent_domains += 1;
        }
        Ok(Some(current))
    }

    fn assign_ancestors(
        &self,
        original: usize,
        value: u64,
        summands: usize,
        assignment: &mut [Option<u64>],
        stats: &mut FiberStats,
    ) -> bool {
        if assignment[original].is_some_and(|old| old != value) {
            return false;
        }
        assignment[original] = Some(value);
        if original >= summands * self.variable_levels() {
            return true;
        }
        let chain = original % summands;
        let mut current = value;
        for level in original / summands..self.variable_levels() {
            stats.domain_index_lookups += 1;
            let Some(&parent) = self.parents[level].get(&current) else {
                return false;
            };
            if level + 1 == self.variable_levels() {
                return parent == self.terminal;
            }
            let index = (level + 1) * summands + chain;
            if assignment[index].is_some_and(|old| old != parent) {
                return false;
            }
            if assignment[index].is_none() {
                stats.forced_ancestor_assignments += 1;
                assignment[index] = Some(parent);
            }
            current = parent;
        }
        true
    }
}

/// Explicit bounds on the additional driver. They are independent of the
/// original F4 degree/pair/deadline options and are emitted in every report.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct FiberLimits {
    pub max_nodes: usize,
    pub max_f4_runs: usize,
    pub max_root_degree: usize,
    pub max_retained_terms: usize,
}

impl Default for FiberLimits {
    fn default() -> Self {
        Self {
            max_nodes: 4096,
            max_f4_runs: 64,
            max_root_degree: 1024,
            max_retained_terms: 100_000,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct FiberStats {
    pub nodes: usize,
    pub branches: usize,
    pub domain_queries: u64,
    pub domain_index_lookups: u64,
    pub domain_edges_read: u64,
    pub immediate_parent_domains: u64,
    pub domain_values_tested: u64,
    pub selector_univariates_without_pruning: u64,
    pub singleton_assignments: u64,
    pub forced_ancestor_assignments: u64,
    pub root_core_calls: u64,
    pub original_equation_checks: u64,
    pub original_equation_multiplications: u64,
    pub filter_multiplications: u64,
    pub substitution_multiplications: u64,
    pub retained_incoming_terms: u64,
    pub retained_consequence_terms: u64,
    pub unresolved_nodes: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct FiberVisitReport {
    pub driver: &'static str,
    pub effective_policy: FiberExtractionPolicy,
    pub common: VisitReport,
    pub limits: FiberLimits,
    pub stats: FiberStats,
    pub reason: Option<String>,
    pub accounting: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum End {
    Closed,
    Stopped,
    TimedOut,
    Limited,
    Unresolved,
}

enum Choice {
    Empty,
    Branch(usize, Vec<u64>),
    NoProgress,
}

struct Driver<'a, F> {
    original: &'a [Poly],
    p: u64,
    summands: usize,
    fiber: &'a FiberIndex,
    opts: &'a F4Options,
    limits: FiberLimits,
    report: FiberVisitReport,
    visitor: F,
}

fn horner(coefficients: &[u64], x: u64, p: u64, count: &mut u64) -> u64 {
    let Some((&last, rest)) = coefficients.split_last() else {
        return 0;
    };
    rest.iter().rev().fold(last, |value, &coefficient| {
        (product(value, x, p, count) + coefficient) % p
    })
}

fn exact_eval(poly: &Poly, point: &[u64], p: u64, count: &mut u64) -> u64 {
    poly.iter().fold(0, |sum, (exponents, coefficient)| {
        let mut term = *coefficient;
        for (&x, &exponent) in point.iter().zip(exponents) {
            if exponent != 0 {
                let raised = power(x, u64::from(exponent), p, count);
                term = product(term, raised, p, count);
            }
        }
        (sum + term) % p
    })
}

impl<F: FnMut(&[u64]) -> bool> Driver<'_, F> {
    fn expired(&self) -> bool {
        self.opts.deadline.is_some_and(|d| Instant::now() >= d)
    }

    fn mark(&mut self, end: End, reason: &'static str) -> End {
        if self.report.reason.is_none() {
            self.report.reason = Some(reason.into());
        }
        end
    }

    fn select(
        &mut self,
        polys: &[Poly],
        active: &[usize],
        assignment: &[Option<u64>],
    ) -> Result<Choice, End> {
        let mut univariates: Vec<Vec<Vec<u64>>> = vec![Vec::new(); active.len()];
        for g in polys {
            if self.expired() {
                return Err(End::TimedOut);
            }
            let mut support = None;
            let mut single = true;
            for (exponents, _) in g {
                for (j, &e) in exponents.iter().enumerate() {
                    if e != 0 {
                        if support.is_some_and(|old| old != j) {
                            single = false;
                        }
                        support = Some(j);
                    }
                }
            }
            if let (true, Some(j)) = (single, support) {
                let degree = g.iter().map(|(e, _)| e[j] as usize).max().unwrap_or(0);
                if degree > self.limits.max_root_degree {
                    return Err(self.mark(End::Limited, "fiber_root_degree_limit"));
                }
                let mut dense = vec![0; degree + 1];
                for (e, c) in g {
                    dense[e[j] as usize] = (dense[e[j] as usize] + c) % self.p;
                }
                while dense.last() == Some(&0) {
                    dense.pop();
                }
                if !dense.is_empty() {
                    univariates[j].push(dense);
                }
            }
        }
        let mut best: Option<(usize, Vec<u64>)> = None;
        for (j, equations) in univariates.iter_mut().enumerate() {
            if equations.is_empty() {
                continue;
            }
            equations.sort_by_key(Vec::len);
            let domain = self.fiber.domain(
                active[j],
                self.summands,
                assignment,
                &mut self.report.stats,
                self.opts.deadline,
            )?;
            let mut roots = if domain
                .as_ref()
                .is_some_and(|d| d.len() <= DIRECT_DOMAIN_LIMIT)
            {
                domain.clone().unwrap()
            } else {
                self.report.stats.root_core_calls += 1;
                self.report.common.factored_univariates += 1;
                let Some(roots) = f4_fp::dense_roots_fp(
                    &equations[0],
                    self.p,
                    &mut self.report.common.root_field_multiplications,
                    self.opts.deadline,
                ) else {
                    return Err(End::TimedOut);
                };
                roots
            };
            if self.expired() {
                return Err(End::TimedOut);
            }
            if let Some(domain) = &domain {
                roots.retain(|root| domain.binary_search(root).is_ok());
            }
            let mut accepted = Vec::new();
            for root in roots {
                if self.expired() {
                    return Err(End::TimedOut);
                }
                self.report.stats.domain_values_tested += 1;
                self.report.common.domain_evaluations += 1;
                if equations.iter().all(|h| {
                    horner(
                        h,
                        root,
                        self.p,
                        &mut self.report.stats.filter_multiplications,
                    ) == 0
                }) {
                    accepted.push(root);
                }
            }
            accepted.sort_unstable();
            accepted.dedup();
            if accepted.is_empty() {
                return Ok(Choice::Empty);
            }
            if domain
                .as_ref()
                .is_some_and(|d| accepted.len() == d.len() && d.len() > 1)
            {
                self.report.stats.selector_univariates_without_pruning += 1;
                continue;
            }
            if domain.is_none() && accepted.len() as u64 == self.p {
                continue;
            }
            if best
                .as_ref()
                .is_none_or(|(_, old)| accepted.len() < old.len())
            {
                best = Some((j, accepted));
            }
        }
        Ok(best.map_or(Choice::NoProgress, |(j, roots)| Choice::Branch(j, roots)))
    }

    fn walk(
        &mut self,
        mut polys: Vec<Poly>,
        mut active: Vec<usize>,
        mut assignment: Vec<Option<u64>>,
    ) -> End {
        if self.expired() {
            return End::TimedOut;
        }
        if self.report.stats.nodes >= self.limits.max_nodes {
            return self.mark(End::Limited, "fiber_node_limit");
        }
        self.report.stats.nodes += 1;
        loop {
            let mut singleton = None;
            for (j, &original) in active.iter().enumerate() {
                if assignment[original].is_some() {
                    continue;
                }
                let domain = match self.fiber.domain(
                    original,
                    self.summands,
                    &assignment,
                    &mut self.report.stats,
                    self.opts.deadline,
                ) {
                    Ok(domain) => domain,
                    Err(end) => return end,
                };
                if let Some(domain) = domain {
                    if domain.is_empty() {
                        return End::Closed;
                    }
                    if domain.len() == 1 {
                        singleton = Some((j, domain[0]));
                        break;
                    }
                }
            }
            if let Some((j, value)) = singleton {
                self.report.stats.singleton_assignments += 1;
                if !self.fiber.assign_ancestors(
                    active[j],
                    value,
                    self.summands,
                    &mut assignment,
                    &mut self.report.stats,
                ) {
                    return End::Closed;
                }
            }
            // Specialize every forced ancestor at once, in decreasing local
            // position so remaining exponent-vector positions stay valid.
            for j in (0..active.len()).rev() {
                if let Some(value) = assignment[active[j]] {
                    polys = f4_fp::substitute_var(
                        &polys,
                        j,
                        value,
                        self.p,
                        &mut self.report.stats.substitution_multiplications,
                    );
                    active.remove(j);
                }
            }
            if singleton.is_none() {
                break;
            }
            if self.expired() {
                return End::TimedOut;
            }
        }
        let mut f4_here = false;
        loop {
            if self.expired() {
                return End::TimedOut;
            }
            if polys.iter().map(Vec::len).sum::<usize>() > self.limits.max_retained_terms {
                return self.mark(End::Limited, "fiber_retained_term_limit");
            }
            let mut live = Vec::new();
            for poly in polys {
                if poly.iter().all(|(e, _)| e.iter().all(|&v| v == 0)) {
                    if poly.iter().fold(0u64, |sum, (_, c)| (sum + c) % self.p) != 0 {
                        return End::Closed;
                    }
                } else {
                    live.push(poly);
                }
            }
            polys = live;
            if active.is_empty() {
                let Some(point) = assignment.iter().copied().collect::<Option<Vec<_>>>() else {
                    return self.mark(End::Unresolved, "fiber_internal_unassigned_terminal");
                };
                for original in self.original {
                    self.report.stats.original_equation_checks += 1;
                    if exact_eval(
                        original,
                        &point,
                        self.p,
                        &mut self.report.stats.original_equation_multiplications,
                    ) != 0
                    {
                        self.report.common.candidates_rejected += 1;
                        return End::Closed;
                    }
                }
                self.report.common.solutions_visited += 1;
                return if (self.visitor)(&point) {
                    End::Stopped
                } else {
                    End::Closed
                };
            }
            let choice = match self.select(&polys, &active, &assignment) {
                Ok(choice) => choice,
                Err(end) => return end,
            };
            match choice {
                Choice::Empty => return End::Closed,
                Choice::Branch(j, values) => {
                    let mut unresolved = false;
                    for value in values {
                        if self.expired() {
                            return End::TimedOut;
                        }
                        self.report.stats.branches += 1;
                        let mut child_assignment = assignment.clone();
                        if !self.fiber.assign_ancestors(
                            active[j],
                            value,
                            self.summands,
                            &mut child_assignment,
                            &mut self.report.stats,
                        ) {
                            continue;
                        }
                        match self.walk(polys.clone(), active.clone(), child_assignment) {
                            End::Closed => (),
                            End::Unresolved => unresolved = true,
                            end => return end,
                        }
                    }
                    return if unresolved {
                        End::Unresolved
                    } else {
                        End::Closed
                    };
                }
                Choice::NoProgress => (),
            }
            if f4_here {
                self.report.stats.unresolved_nodes += 1;
                return self.mark(End::Unresolved, "no_informative_univariate_after_f4");
            }
            if self.report.common.f4_runs >= self.limits.max_f4_runs {
                return self.mark(End::Limited, "fiber_f4_run_limit");
            }
            // Normalize for the original core, then retain every incoming
            // polynomial as well as every returned consequence. A truncated
            // alive-only F4 basis is never substituted for the full inputs.
            let incoming: Vec<Poly> = polys
                .iter()
                .map(|g| f4_fp::normalise(g, self.p, self.opts.order))
                .filter(|g| !g.is_empty())
                .collect();
            let result = f4_fp::f4(&incoming, active.len(), self.p, self.opts);
            self.report.common.f4_runs += 1;
            self.report.common.field_ops += result.field_ops;
            self.report.common.solving_degree =
                self.report.common.solving_degree.max(result.solving_degree);
            self.report.common.max_rows = self.report.common.max_rows.max(result.max_rows);
            self.report.common.max_cols = self.report.common.max_cols.max(result.max_cols);
            if result.timed_out || self.expired() {
                return End::TimedOut;
            }
            if result.inconsistent {
                return End::Closed;
            }
            self.report.stats.retained_incoming_terms +=
                incoming.iter().map(Vec::len).sum::<usize>() as u64;
            polys = incoming;
            for consequence in result.basis {
                if !polys.contains(&consequence) {
                    self.report.stats.retained_consequence_terms += consequence.len() as u64;
                    polys.push(consequence);
                }
            }
            f4_here = true;
        }
    }
}

/// Solve only the closure reached by exact informative univariates and one
/// bounded native F4 call at each unresolved node. There is no unrestricted
/// field scan or Cartesian-domain fallback. Complete is true only when every
/// branch of that exact cover closes; any unresolved child keeps it false.
pub fn solve_visit<F: FnMut(&[u64]) -> bool>(
    input: &[Poly],
    n_vars: usize,
    p: u64,
    opts: &F4Options,
    fiber: &FiberIndex,
    summands: usize,
    limits: FiberLimits,
    visitor: F,
) -> Result<FiberVisitReport, String> {
    if p != fiber.p
        || !(1..=4).contains(&summands)
        || n_vars < summands.saturating_mul(fiber.variable_levels())
        || input
            .iter()
            .flatten()
            .any(|(e, c)| e.len() != n_vars || *c >= p)
    {
        return Err("fiber extraction input dimensions or field do not match the index".into());
    }
    let mut driver = Driver {
        original: input,
        p,
        summands,
        fiber,
        opts,
        limits,
        report: FiberVisitReport {
            driver: "bounded_parent_conditioned_fiber_triangular",
            effective_policy: effective_policy(),
            common: VisitReport::default(),
            limits,
            stats: FiberStats::default(),
            reason: None,
            accounting: "Unchanged native F4 row-reduction and root-core multiplication counters plus separately counted domain filtering, substitution and original-equation replay. No Cartesian-domain fallback is used. Domain/index traversals and forced assignments are explicit logical counters; allocation, ordering, normalization and root-core inversion internals are not a complete native-work total. Unknown closure is retained when F4 supplies no informative univariate. Full cold setup and process-envelope costs remain external and mandatory.",
        },
        visitor,
    };
    let end = driver.walk(input.to_vec(), (0..n_vars).collect(), vec![None; n_vars]);
    driver.report.common.complete = end == End::Closed;
    driver.report.common.stopped_early = end == End::Stopped;
    driver.report.common.timed_out = end == End::TimedOut;
    if end == End::TimedOut {
        driver.report.reason = Some("fiber_deadline".into());
    }
    Ok(driver.report)
}
