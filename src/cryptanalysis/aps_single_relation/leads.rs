//! Optional APS extensions. Full-set coverage and informative recovery are
//! separate: a partial traversal never authorizes skipping an old region.

use super::*;

#[derive(Clone)]
pub struct Cell {
    pub system: System,
    /// Coordinates eliminated by explicit guessing, restored before recovery.
    pub fixed_coordinates: Vec<u64>,
}

/// Exact nonhorizontal chart, with a separate horizontal chart unless the
/// existing orbit-and-label certificate authorizes its omission. Ordinate-zero
/// inputs are refused rather than extending the chart's lifting argument.
pub fn line_chart(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    base: &LabeledBase,
    pruning: Option<(u64, u64)>,
) -> Result<Vec<Cell>, String> {
    let values = if let Some((beta, lambda)) = pruning {
        certified_line_values(curve, g, q, base, beta, lambda)?
    } else {
        validate_instance(curve, g, q)?;
        validate_base(curve, g, q, base)?;
        base.points.keys().copied().collect()
    };
    let (p, values, a, b) = checked_model(curve, &[values])?;
    if values
        .iter()
        .any(|&x| (mul(mul(x, x, p), x, p) + mul(a, x, p) + b).is_multiple_of(p))
    {
        return Err("line chart requires nonzero ordinates".into());
    }
    let (u, v) = (Expr::var(0, 2, p), Expr::var(1, 2, p));
    let elementary = [
        u.clone(),
        u.c(a).sub(&u.mul(&v).mul(&u.c(2))),
        u.mul(&v.square()).sub(&u.c(b)),
    ];
    let make = |equations: Vec<Expr>, vars, decode| Cell {
        system: System {
            variables: vars,
            equations: equations
                .into_iter()
                .map(|e| f4_fp::normalise(&e.terms, p, Ordering::Grevlex))
                .collect(),
            modulus: p,
            decode,
            domains: vec![None; vars],
            model: (a, b),
        },
        fixed_coordinates: vec![],
    };
    let mut out = vec![make(
        divisor_constraints(&values, &elementary)?,
        2,
        Decode::LineChart {
            values: values.clone(),
            a,
            b,
        },
    )];
    if pruning.is_none() {
        let c = Expr::var(0, 1, p);
        out.push(make(
            divisor_constraints(&values, &[c.c(0), c.c(a), c.square().sub(&c.c(b))])?,
            1,
            Decode::Horizontal { values, a, b },
        ));
    }
    Ok(out)
}

/// Disjoint new-tuple frontier: the first newly admitted coordinate selects
/// exactly one cell. This identity alone does not certify prior coverage.
pub fn frontier_parts(old: &[Vec<u64>], new: &[Vec<u64>]) -> Result<Vec<Vec<Vec<u64>>>, String> {
    if old.len() != new.len() || !(3..=4).contains(&new.len()) {
        return Err("matching arity-three/four partition profiles required".into());
    }
    let all: Vec<_> = new.iter().flatten().copied().collect();
    if all.len() > 32
        || all.iter().collect::<BTreeSet<_>>().len() != all.len()
        || old.iter().zip(new).any(|(a, b)| {
            a.is_empty()
                || b.is_empty()
                || a.iter().collect::<BTreeSet<_>>().len() != a.len()
                || a.iter().any(|x| !b.contains(x))
        })
    {
        return Err(
            "old partitions must be distinct nonempty subsets of disjoint new partitions".into(),
        );
    }
    Ok((0..new.len())
        .filter_map(|j| {
            let delta: Vec<_> = new[j]
                .iter()
                .filter(|x| !old[j].contains(x))
                .copied()
                .collect();
            (!delta.is_empty()).then(|| {
                (0..new.len())
                    .map(|i| {
                        if i < j {
                            old[i].clone()
                        } else if i == j {
                            delta.clone()
                        } else {
                            new[i].clone()
                        }
                    })
                    .collect()
            })
        })
        .collect())
}

pub fn reusable_coverage(report: &AlgebraReport) -> bool {
    report.complete && !report.stopped_early && !report.timed_out
}

/// Build coordinate/hybrid cells, preserving an arbitrary explicit profile.
/// `old` may be supplied only after a complete no-informative-witness search;
/// the ecbench driver owns that state and the independent reader rechecks it.
pub fn coordinate_plan(
    curve: &FastCurve,
    parts: &[Vec<u64>],
    encoding: Encoding,
    guessed_variable: Option<usize>,
    old: Option<&[Vec<u64>]>,
) -> Result<Vec<Cell>, String> {
    if !matches!(
        encoding,
        Encoding::Partitioned3 | Encoding::Expanded4 | Encoding::Split4
    ) {
        return Err("frontier and hybrid profiles require coordinate encodings".into());
    }
    if guessed_variable.is_some_and(|j| j >= parts.len()) {
        return Err("guessed variable outside partition profile".into());
    }
    let parts_list = if let Some(old) = old {
        frontier_parts(old, parts)?
    } else {
        vec![parts.to_vec()]
    };
    let mut cells = vec![];
    for parts in parts_list {
        let original = build_system(curve, &parts, encoding)?;
        if let Some(j) = guessed_variable {
            for &guess in &parts[j] {
                let equations = original
                    .equations
                    .iter()
                    .map(|f| {
                        let terms: Poly = f
                            .iter()
                            .map(|(e, c)| {
                                let mut k = *c;
                                for _ in 0..e[j] {
                                    k = mul(k, guess, curve.f.p);
                                }
                                let mut powers = e.clone();
                                powers.remove(j);
                                (powers, k)
                            })
                            .collect();
                        f4_fp::normalise(&terms, curve.f.p, Ordering::Grevlex)
                    })
                    .collect();
                let mut domains = original.domains.clone();
                domains.remove(j);
                cells.push(Cell {
                    system: System {
                        variables: original.variables - 1,
                        equations,
                        modulus: curve.f.p,
                        decode: Decode::Coordinates(parts.len() - 1),
                        domains,
                        model: original.model,
                    },
                    fixed_coordinates: vec![guess],
                });
            }
        } else {
            cells.push(Cell {
                system: original,
                fixed_coordinates: vec![],
            });
        }
    }
    Ok(cells)
}

/// Complete/first-informative traversal of all cells under one deadline.
/// Failed replay and fallback are already charged by the native trace pool.
pub fn visit_plan(
    cells: &[Cell],
    p: u64,
    degree: u32,
    budget: Duration,
    mut pool: Option<&mut f4_fp::TracePool>,
    mut visitor: impl FnMut(&[u64]) -> Result<bool, String>,
) -> Result<AlgebraReport, String> {
    if budget.is_zero() || degree == 0 {
        return Err("positive plan budget and degree required".into());
    }
    let start = Instant::now();
    let mut total = AlgebraReport::empty();
    let mut seen = BTreeSet::new();
    for cell in cells {
        let Some(remaining) = budget.checked_sub(start.elapsed()).filter(|d| !d.is_zero()) else {
            total.complete = false;
            total.timed_out = true;
            break;
        };
        let mut emit = |coordinates: &[u64]| {
            let mut full = coordinates.to_vec();
            full.extend(&cell.fixed_coordinates);
            full.sort_unstable();
            if full.windows(2).any(|w| w[0] == w[1]) {
                return Err("hybrid produced repeated coordinates".into());
            }
            if seen.insert(full.clone()) {
                visitor(&full)
            } else {
                Ok(false)
            }
        };
        let r = if let Some(pool) = pool.as_deref_mut() {
            validate_system(&cell.system, p, degree, remaining)?;
            // Validation belongs to the same outer plan deadline. Starting a
            // fresh `remaining` interval here would give it back to F4.
            let Some(remaining) = budget.checked_sub(start.elapsed()).filter(|d| !d.is_zero())
            else {
                total.complete = false;
                total.timed_out = true;
                break;
            };
            let mut error = None;
            let r = f4_fp::solve_visit_reusing(
                &cell.system.equations,
                cell.system.variables,
                p,
                &F4Options::new(Ordering::Grevlex, degree).with_budget(remaining),
                &cell.system.domains,
                pool,
                |root| match decode_root(&cell.system, root).and_then(|x| emit(&x)) {
                    Ok(stop) => stop,
                    Err(e) => {
                        error = Some(e);
                        true
                    }
                },
            )?;
            if let Some(e) = error {
                return Err(e);
            }
            AlgebraReport {
                complete: r.complete,
                stopped_early: r.stopped_early,
                timed_out: r.timed_out,
                solver_calls: r.f4_runs,
                solving_degree: r.solving_degree,
                max_rows: r.max_rows,
                max_columns: r.max_cols,
                f4_field_multiplications: r.field_ops,
                roots: Some(RootDiagnostics {
                    equation_roots_visited: r.solutions_visited,
                    candidates_rejected: r.candidates_rejected,
                    domain_evaluations: r.domain_evaluations,
                    factored_univariates: r.factored_univariates,
                    root_field_multiplications: r.root_field_multiplications,
                    full_field_branches: r.full_field_branches,
                }),
                ..AlgebraReport::default()
            }
        } else {
            visit_system(&cell.system, p, degree, remaining, &mut emit)?
        };
        total.include(&r);
        if !reusable_coverage(&r) {
            break;
        }
    }
    total.coordinate_sets = seen.into_iter().collect();
    Ok(total)
}

pub struct GrowthDraw {
    pub base: LabeledBase,
    pub new_draws: usize,
    pub complete: bool,
}

/// Conditional sampling without replacing old labeled points. A draw cap
/// returns its actual partial base; it never disappears as an empty attempt.
#[allow(clippy::too_many_arguments)]
pub fn sample_growth(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    old: Option<&LabeledBase>,
    sizes: &[usize],
    labels: Labels,
    seed: u64,
    max_draws: usize,
) -> Result<GrowthDraw, String> {
    validate_instance(curve, g, q)?;
    if !(3..=4).contains(&sizes.len())
        || sizes.contains(&0)
        || sizes.iter().any(|s| *s > 4)
        || sizes.iter().sum::<usize>() > 32
        || sizes.iter().sum::<usize>() > (curve.n as usize - 1) / 2
        || old.is_some_and(|b| {
            b.parts.len() != sizes.len() || b.parts.iter().zip(sizes).any(|(p, s)| p.len() > *s)
        })
    {
        return Err("bounded growth profile required".into());
    }
    if let Some(old) = old {
        validate_base(curve, g, q, old)?;
    }
    let weights = if labels == Labels::Guaranteed {
        guaranteed_weights(sizes.len(), curve.n)?
    } else {
        vec![0; sizes.len()]
    };
    if labels == Labels::Guaranteed
        && old.is_some_and(|b| {
            b.parts.iter().enumerate().any(|(i, part)| {
                part.iter().any(|x| {
                    let v = b.points[x].v;
                    v != weights[i] && v != neg(weights[i], curve.n)
                })
            })
        })
    {
        return Err("retained base does not use the requested guaranteed label law".into());
    }
    let shifts: Vec<_> = weights
        .iter()
        .map(|w| counted_mul(curve, Some(q), *w))
        .collect();
    let mut base = old.cloned().unwrap_or(LabeledBase {
        parts: vec![vec![]; sizes.len()],
        points: BTreeMap::new(),
        draws: 0,
    });
    let mut rng = StdRng::seed_from_u64(seed);
    let mut new_draws = 0;
    for (slot, size) in sizes.iter().enumerate() {
        while base.parts[slot].len() < *size {
            if new_draws >= max_draws {
                return Ok(GrowthDraw {
                    base,
                    new_draws,
                    complete: false,
                });
            }
            let u = rng.gen_range(0..curve.n);
            let v = if labels == Labels::Guaranteed {
                weights[slot]
            } else {
                rng.gen_range(0..curve.n)
            };
            let point = if labels == Labels::Guaranteed {
                counted_add(curve, counted_mul(curve, Some(g), u), shifts[slot])
            } else {
                counted_combine(curve, g, q, u, v)
            };
            new_draws += 1;
            base.draws += 1;
            let Some(point) = point else {
                continue;
            };
            let entry = labeled(curve, point, u, v);
            if base.points.contains_key(&entry.x) {
                continue;
            }
            base.parts[slot].push(entry.x);
            base.points.insert(entry.x, entry);
        }
    }
    validate_base(curve, g, q, &base)?;
    Ok(GrowthDraw {
        base,
        new_draws,
        complete: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn curve() -> (FastCurve, Pt, Pt) {
        let c = FastCurve::new("aps-extension-development", 101, 59, 6, 107);
        let g = c.point(78, 4);
        let q = c.mul(Some(g), 37).unwrap();
        (c, g, q)
    }
    fn solve(cells: &[Cell]) -> AlgebraReport {
        visit_plan(cells, 101, 24, Duration::from_secs(10), None, |_| Ok(false)).unwrap()
    }
    #[test]
    fn chart_union_matches_original_line_complete_coordinate_sets() {
        let (c, g, q) = curve();
        let b = sample_base(&c, g, q, &[2, 2, 2], Labels::Random, 20261026, 1000).unwrap();
        let plan = line_chart(&c, g, q, &b, None).unwrap();
        assert_eq!(plan.len(), 2);
        assert_eq!(plan[0].system.variables(), 2);
        assert_eq!(plan[1].system.variables(), 1);
        let r = solve(&plan);
        let original =
            build_system(&c, &[b.points.keys().copied().collect()], Encoding::Line3).unwrap();
        let expected = solve_system(&original, 101, 24, Duration::from_secs(10)).unwrap();
        assert!(r.complete && expected.complete);
        assert_eq!(r.coordinate_sets, expected.coordinate_sets);
    }
    #[test]
    fn two_variable_chart_matches_the_certified_saturated_orbit_system() {
        let c = FastCurve::new("aps-existing-cubic-development", 103, 0, 5, 97);
        let g = c.point(2, 42);
        let q = c.mul(Some(g), 17).unwrap();
        let overlapping =
            sample_base(&c, g, q, &[1, 1, 1], Labels::Guaranteed, 20261026, 1000).unwrap();
        assert!(orbit_base(&c, g, q, &overlapping, 56, 35).is_err());
        // Reuse the established non-overlapping cubic fixture. The different
        // guaranteed-label draw above is a real refusal, not a valid base.
        let seeds = sample_base(&c, g, q, &[1, 1, 1], Labels::Random, 20261026, 1000).unwrap();
        let b = orbit_base(&c, g, q, &seeds, 56, 35).unwrap();
        let plan = line_chart(&c, g, q, &b, Some((56, 35))).unwrap();
        assert_eq!(plan.len(), 1);
        assert_eq!(plan[0].system.variables(), 2);
        let actual =
            visit_plan(&plan, 103, 24, Duration::from_secs(10), None, |_| Ok(false)).unwrap();
        let original = build_pruned_line_system(&c, g, q, &b, 56, 35).unwrap();
        assert_eq!(original.variables(), 3);
        let expected = solve_system(&original, 103, 24, Duration::from_secs(10)).unwrap();
        assert!(actual.complete && expected.complete);
        assert!(!actual.coordinate_sets.is_empty());
        assert_eq!(actual.coordinate_sets, expected.coordinate_sets);
        let recovered = RecoveryContext::new(&c, g, q, &b)
            .unwrap()
            .recover_observed(&actual.coordinate_sets, |_| {})
            .unwrap()
            .unwrap();
        assert_eq!(recovered.logarithm, 17);
    }
    #[test]
    fn unequal_profiles_and_each_guessed_variable_preserve_all_roots() {
        let (c, g, q) = curve();
        let b = sample_base(&c, g, q, &[1, 3, 2], Labels::Random, 31, 1000).unwrap();
        let original = build_system(&c, &b.parts, Encoding::Partitioned3).unwrap();
        let expected = solve_system(&original, 101, 24, Duration::from_secs(10)).unwrap();
        assert!(expected.complete);
        for j in 0..3 {
            let r = solve(
                &coordinate_plan(&c, &b.parts, Encoding::Partitioned3, Some(j), None).unwrap(),
            );
            assert!(r.complete);
            assert_eq!(r.coordinate_sets, expected.coordinate_sets);
        }
    }
    #[test]
    fn complete_old_region_and_frontier_equal_fresh_search() {
        let (c, g, q) = curve();
        let b = sample_base(&c, g, q, &[1, 1, 1], Labels::Random, 17, 1000).unwrap();
        let grown =
            sample_growth(&c, g, q, Some(&b), &[2, 2, 2], Labels::Random, 18, 1000).unwrap();
        assert!(grown.complete);
        let old =
            solve(&coordinate_plan(&c, &b.parts, Encoding::Partitioned3, None, None).unwrap());
        let frontier = solve(
            &coordinate_plan(
                &c,
                &grown.base.parts,
                Encoding::Partitioned3,
                Some(1),
                Some(&b.parts),
            )
            .unwrap(),
        );
        let fresh = solve(
            &coordinate_plan(&c, &grown.base.parts, Encoding::Partitioned3, None, None).unwrap(),
        );
        let mut union: BTreeSet<_> = old.coordinate_sets.into_iter().collect();
        union.extend(frontier.coordinate_sets);
        assert!(old.complete && frontier.complete && fresh.complete);
        assert_eq!(union.into_iter().collect::<Vec<_>>(), fresh.coordinate_sets);
        assert!(!reusable_coverage(&AlgebraReport {
            complete: true,
            stopped_early: true,
            ..AlgebraReport::default()
        }));
    }
    #[test]
    fn capped_growth_retains_old_points_and_actual_partial_draws() {
        let (c, g, q) = curve();
        let b = sample_base(&c, g, q, &[1, 1, 1], Labels::Guaranteed, 17, 1000).unwrap();
        let d = sample_growth(&c, g, q, Some(&b), &[2, 2, 2], Labels::Guaranteed, 18, 1).unwrap();
        assert!(!d.complete);
        assert_eq!(d.new_draws, 1);
        assert_eq!(d.base.draws, b.draws + 1);
        for (&x, p) in &b.points {
            let next = &d.base.points[&x];
            assert_eq!((next.u, next.v, next.y), (p.u, p.v, p.y));
        }
    }

    #[test]
    fn informative_horizontal_control_requires_the_horizontal_cell() {
        // Public planted correctness fixture. The solver receives only Q and
        // verified labels, never the known test logarithm.
        let c = FastCurve::new("aps-horizontal-control", 103, 0, 5, 97);
        let g = c.point(2, 42);
        let q = c.mul(Some(g), 17).unwrap();
        let entries = [(2, 81), (9, 18), (92, 44)].map(|(x, u)| LabeledPoint { x, y: 42, u, v: 1 });
        let base = LabeledBase {
            parts: entries.iter().map(|e| vec![e.x]).collect(),
            points: entries.into_iter().map(|e| (e.x, e)).collect(),
            draws: 3,
        };
        validate_base(&c, g, q, &base).unwrap();
        let plan = line_chart(&c, g, q, &base, None).unwrap();
        assert_eq!(plan.len(), 2);
        let nonhorizontal = visit_plan(&plan[..1], 103, 24, Duration::from_secs(10), None, |_| {
            Ok(false)
        })
        .unwrap();
        assert!(nonhorizontal.complete && nonhorizontal.coordinate_sets.is_empty());
        let all = visit_plan(&plan, 103, 24, Duration::from_secs(10), None, |_| Ok(false)).unwrap();
        assert!(all.complete);
        assert_eq!(all.coordinate_sets, vec![vec![2, 9, 92]]);
        let recovered = recover(&c, g, q, &base, &all.coordinate_sets)
            .unwrap()
            .unwrap();
        assert_eq!(recovered.logarithm, 17);
        assert_ne!(recovered.b, 0);
        assert!(line_chart(&c, g, q, &base, Some((56, 35))).is_err());
        assert!(decode_root(&plan[0].system, &[0, 1]).is_err());
    }
}
