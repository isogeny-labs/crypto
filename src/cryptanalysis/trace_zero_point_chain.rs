//! Complete finite-base full-point chains, retaining explicit addition branches.
//!
//! The same quadratic field-product circuit as native S3 is reused. Generic
//! addition, doubling, opposite points and O+P are separate exact branches.
//! Summand domains contain actual admitted (x,y) pairs, not independent x/y
//! lists. A candidate model must replay in the original group.
use crate::binary_ecc::{
    curve::{point_add, point_neg},
    BinaryPoint, F2mElement, TraceZeroVariety,
};
use crate::cryptanalysis::{
    binary_semaev_chain_sat::{
        add_coordinate_domain, finite_domain_clause_count, ChainBuildError, Circuit,
    },
    sat::{SolveResult, Solver},
    trace_zero_relations::{
        TraceZeroRelationDisposition as Disposition, TraceZeroRelationLimits,
        TraceZeroRelationReport,
    },
};
use num_bigint::BigUint;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointAdditionBranch {
    Generic,
    Double,
    Opposite,
    ReturnRight,
}
use PointAdditionBranch::*;
type PointWires = (Vec<u32>, Vec<u32>);

pub fn full_point_chain_max_variables(n: u32, m: usize) -> Option<u64> {
    if !(3..=131).contains(&n) || !(2..=9).contains(&m) {
        return None;
    }
    let (n, m) = (n as u64, m as u64);
    Some(2 * n * (2 * m - 1) + (7 * n + 3 * n * n) * (m - 1))
}

pub struct FullPointChainEncoding {
    pub solver: Solver,
    variety: TraceZeroVariety,
    base: Vec<BinaryPoint>,
    target: BinaryPoint,
    branches: Vec<PointAdditionBranch>,
    blocks: Vec<PointWires>,
    m: usize,
}
impl FullPointChainEncoding {
    pub fn build(
        variety: &TraceZeroVariety,
        base: &[BinaryPoint],
        target: &BinaryPoint,
        branches: &[PointAdditionBranch],
        max_variables: u32,
        max_domain_clauses: usize,
    ) -> Result<Self, ChainBuildError> {
        let m = branches.len() + 1;
        let curve = variety.curve();
        let n = curve.m as usize;
        let required =
            full_point_chain_max_variables(curve.m, m).ok_or(ChainBuildError::Unsupported)?;
        if required > max_variables as u64 || required > i32::MAX as u64 {
            return Err(ChainBuildError::VariableCap {
                required,
                maximum: max_variables,
            });
        }
        if !variety.contains(target)
            || base
                .iter()
                .any(|p| matches!(p, BinaryPoint::Infinity) || !variety.contains(p))
        {
            return Err(ChainBuildError::Unsupported);
        }
        let mut infinity = false;
        for branch in branches {
            if infinity != (*branch == ReturnRight) {
                return Err(ChainBuildError::Unsupported);
            }
            infinity = *branch == Opposite;
        }
        if infinity != matches!(target, BinaryPoint::Infinity) {
            return Err(ChainBuildError::Unsupported);
        }
        let mut codes: Vec<BigUint> = base
            .iter()
            .map(|p| match p {
                BinaryPoint::Affine { x, y } => x.to_biguint() | (y.to_biguint() << n),
                _ => unreachable!(),
            })
            .collect();
        codes.sort_unstable();
        codes.dedup();
        let clauses = finite_domain_clause_count(&codes, 2 * curve.m, m)
            .ok_or(ChainBuildError::Unsupported)?;
        if clauses > max_domain_clauses {
            return Err(ChainBuildError::DomainClauseCap {
                required: clauses,
                maximum: max_domain_clauses,
            });
        }
        let mut c = Circuit::new(&curve.irreducible);
        let blocks: Vec<PointWires> = (0..2 * m - 1).map(|_| (c.block(), c.block())).collect();
        let target_values = match target {
            BinaryPoint::Infinity => (BigUint::default(), BigUint::default()),
            BinaryPoint::Affine { x, y } => (x.to_biguint(), y.to_biguint()),
        };
        for (w, v) in [
            (&blocks[2 * m - 2].0, &target_values.0),
            (&blocks[2 * m - 2].1, &target_values.1),
        ] {
            for (i, &wire) in w.iter().enumerate() {
                c.units.push((wire, v.bit(i as u64)));
            }
        }
        let mut previous = 0;
        for (j, branch) in branches.iter().enumerate() {
            let right = j + 1;
            let output = if j == m - 2 { 2 * m - 2 } else { m + j };
            let (p, q, r) = (&blocks[previous], &blocks[right], &blocks[output]);
            match branch {
                ReturnRight => {
                    c.equal(&q.0, &r.0);
                    c.equal(&q.1, &r.1);
                }
                Opposite => {
                    c.equal(&p.0, &q.0);
                    for bit in 0..n {
                        c.xors.push((vec![p.1[bit], q.1[bit], p.0[bit]], false));
                    }
                    for &wire in r.0.iter().chain(&r.1) {
                        c.units.push((wire, false));
                    }
                }
                Generic | Double => {
                    let lambda = c.block();
                    if *branch == Generic {
                        let denominator = c.block();
                        for bit in 0..n {
                            c.xors
                                .push((vec![denominator[bit], p.0[bit], q.0[bit]], false));
                        }
                        let inverse = c.block();
                        let guard = c.product(&denominator, &inverse);
                        for (bit, &wire) in guard.iter().enumerate() {
                            c.units.push((wire, bit == 0));
                        }
                        let slope = c.product(&denominator, &lambda);
                        for bit in 0..n {
                            c.xors.push((vec![slope[bit], p.1[bit], q.1[bit]], false));
                        }
                    } else {
                        c.equal(&p.0, &q.0);
                        c.equal(&p.1, &q.1);
                        let slope = c.product(&p.0, &lambda);
                        for bit in 0..n {
                            let mut row = vec![slope[bit], p.1[bit]];
                            row.extend(c.square_inputs[bit].iter().map(|&i| p.0[i]));
                            c.xors.push((row, false));
                        }
                    }
                    for bit in 0..n {
                        let mut row = vec![r.0[bit], lambda[bit], p.0[bit], q.0[bit]];
                        row.extend(c.square_inputs[bit].iter().map(|&i| lambda[i]));
                        c.xors.push((row, false));
                    }
                    let delta = c.block();
                    for bit in 0..n {
                        c.xors.push((vec![delta[bit], p.0[bit], r.0[bit]], false));
                    }
                    let ordinate = c.product(&lambda, &delta);
                    for bit in 0..n {
                        c.xors
                            .push((vec![r.1[bit], ordinate[bit], r.0[bit], p.1[bit]], false));
                    }
                }
            }
            previous = output;
        }
        debug_assert!(c.next as u64 - 1 <= required);
        let mut solver = c.finish();
        for block in &blocks[..m] {
            let wires: Vec<_> = block.0.iter().chain(&block.1).copied().collect();
            add_coordinate_domain(&mut solver, &wires, &codes, 2 * n);
        }
        solver.set_branch_priority(&(1..=(2 * n * m) as u32).collect::<Vec<_>>());
        Ok(Self {
            solver,
            variety: variety.clone(),
            base: base.to_vec(),
            target: target.clone(),
            branches: branches.to_vec(),
            blocks,
            m,
        })
    }
    fn point(&self, block: usize, bits: &[bool]) -> BinaryPoint {
        let n = self.variety.curve().m;
        let value = |w: &[u32]| {
            F2mElement::from_bit_positions(
                &w.iter()
                    .enumerate()
                    .filter_map(|(i, &wire)| bits[wire as usize - 1].then_some(i as u32))
                    .collect::<Vec<_>>(),
                n,
            )
        };
        BinaryPoint::Affine {
            x: value(&self.blocks[block].0),
            y: value(&self.blocks[block].1),
        }
    }
    pub fn decode_summands(&self) -> Vec<BinaryPoint> {
        let bits = self.solver.model();
        (0..self.m).map(|i| self.point(i, &bits)).collect()
    }
    pub fn pin_summands(&mut self, points: &[BinaryPoint]) -> Result<(), ChainBuildError> {
        if points.len() != self.m
            || points
                .iter()
                .any(|p| !self.base.contains(p) || !self.variety.contains(p))
        {
            return Err(ChainBuildError::Unsupported);
        }
        for (i, p) in points.iter().enumerate() {
            let BinaryPoint::Affine { x, y } = p else {
                return Err(ChainBuildError::Unsupported);
            };
            for (w, v) in [
                (&self.blocks[i].0, x.to_biguint()),
                (&self.blocks[i].1, y.to_biguint()),
            ] {
                for (bit, &wire) in w.iter().enumerate() {
                    self.solver.add_clause(vec![if v.bit(bit as u64) {
                        wire as i32
                    } else {
                        -(wire as i32)
                    }]);
                }
            }
        }
        Ok(())
    }
    pub fn verify_model(&self) -> bool {
        let bits = self.solver.model();
        if !self.solver.check_original_assignment(&bits) {
            return false;
        }
        let summands = self.decode_summands();
        if summands
            .iter()
            .any(|p| !self.base.contains(p) || !self.variety.contains(p))
        {
            return false;
        }
        let mut previous = summands[0].clone();
        let curve = self.variety.curve();
        for (j, branch) in self.branches.iter().enumerate() {
            let right = &summands[j + 1];
            let matching = match branch {
                ReturnRight => previous == BinaryPoint::Infinity,
                Opposite => previous == point_neg(right),
                Double => previous == *right,
                Generic => {
                    matches!((&previous,right),(BinaryPoint::Affine{x:a,..},BinaryPoint::Affine{x:b,..}) if a!=b)
                }
            };
            if !matching {
                return false;
            }
            let sum = point_add(curve, &previous, right);
            let output = if j == self.m - 2 {
                2 * self.m - 2
            } else {
                self.m + j
            };
            let encoded = if *branch == Opposite {
                BinaryPoint::Infinity
            } else {
                self.point(output, &bits)
            };
            if sum != encoded || !self.variety.contains(&sum) {
                return false;
            }
            previous = sum;
        }
        previous == self.target
    }
}

/// All admissible branch paths up to a cap; the boolean is true iff complete.
pub fn point_chain_branches(
    m: usize,
    target_infinity: bool,
    cap: usize,
) -> (Vec<Vec<PointAdditionBranch>>, bool) {
    fn visit(
        left: usize,
        infinity: bool,
        target: bool,
        cap: usize,
        path: &mut Vec<PointAdditionBranch>,
        out: &mut Vec<Vec<PointAdditionBranch>>,
    ) -> bool {
        if left == 0 {
            if infinity == target {
                if out.len() == cap {
                    return false;
                }
                out.push(path.clone());
            }
            return true;
        }
        let choices: &[PointAdditionBranch] = if infinity {
            &[ReturnRight]
        } else {
            &[Generic, Double, Opposite]
        };
        for &branch in choices {
            path.push(branch);
            if !visit(left - 1, branch == Opposite, target, cap, path, out) {
                path.pop();
                return false;
            }
            path.pop();
        }
        true
    }
    if !(2..=9).contains(&m) {
        return (Vec::new(), false);
    }
    let mut out = Vec::new();
    let complete = visit(
        m - 1,
        false,
        target_infinity,
        cap,
        &mut Vec::new(),
        &mut out,
    );
    (out, complete)
}

pub fn decompose_full_point_chain(
    variety: &TraceZeroVariety,
    base: &[BinaryPoint],
    target: &BinaryPoint,
    m: usize,
    limits: TraceZeroRelationLimits,
) -> TraceZeroRelationReport {
    let mut report = TraceZeroRelationReport::default();
    if !variety.contains(target)
        || base
            .iter()
            .any(|p| matches!(p, BinaryPoint::Infinity) || !variety.contains(p))
    {
        report.reason = Some("invalid full-kernel target/base");
        return report;
    }
    if !(2..=9).contains(&m)
        || limits.conflict_budget == 0
        || limits.max_patterns == 0
        || limits.max_models == 0
    {
        report.reason = Some("arity or zero budget");
        return report;
    }
    if base.is_empty() {
        report.disposition = Disposition::Refuted;
        return report;
    }
    let (branches, complete) = point_chain_branches(
        m,
        matches!(target, BinaryPoint::Infinity),
        limits.max_patterns,
    );
    let deadline = limits.wall_budget.map(|d| std::time::Instant::now() + d);
    for branch in branches {
        if deadline.is_some_and(|d| std::time::Instant::now() >= d)
            || report.conflicts >= limits.conflict_budget
        {
            report.reason = Some("deadline or conflict budget");
            return report;
        }
        let mut enc = match FullPointChainEncoding::build(
            variety,
            base,
            target,
            &branch,
            limits.max_variables,
            limits.max_domain_clauses,
        ) {
            Ok(e) => e,
            Err(_) => {
                report.reason = Some("point-chain admission or allocation cap");
                return report;
            }
        };
        report.patterns_attempted += 1;
        report.peak_variables = report.peak_variables.max(enc.solver.n_vars());
        enc.solver.conflict_budget = limits.conflict_budget - report.conflicts;
        enc.solver.deadline = deadline;
        let result = enc.solver.solve();
        report.conflicts = report.conflicts.saturating_add(enc.solver.conflicts());
        match result {
            SolveResult::Unknown => {
                report.reason = Some("point-chain SAT budget");
                return report;
            }
            SolveResult::Unsat => report.patterns_refuted += 1,
            SolveResult::Sat => {
                report.models += 1;
                if !enc.verify_model() {
                    report.reason = Some("point-chain original group replay failed");
                    return report;
                }
                let indices = enc
                    .decode_summands()
                    .iter()
                    .map(|p| base.iter().position(|q| q == p).expect("verified domain"))
                    .collect();
                report.indices = Some(indices);
                report.disposition = Disposition::Found;
                return report;
            }
        }
    }
    if complete {
        report.disposition = Disposition::Refuted;
    } else {
        report.reason = Some("point-chain branch cap");
    }
    report
}
