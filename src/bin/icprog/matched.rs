//! Rule v8's read-out: the matched rho against its same-binary control.
//!
//! `icprog rule matched --comparison v8` reads two checked run trees, rule
//! v8's (rho with the batched canonicalisation) and rule v8c's (the same
//! binary, `KIC_RHO_CANON=scalar`, which is rho as at rule v7), pairs their
//! rows by target and prints
//!
//! - per size: the walks' agreement (steps and logarithms, row by row), the
//!   cold ratio of each run with its bootstrap interval, the paired ratio of
//!   the two rho online times and of the two cold ratios, and the index
//!   calculus's own v8c-over-v8 drift (the identical code in both runs, so
//!   the drift is the measurement's noise and the host's);
//! - across sizes: the slope of ln(cold ratio) on ln r for each run, from the
//!   rule's six-point fit, from a bootstrap over targets, by leaving one size
//!   out, and from a fit on ln r and n together (six points: descriptive);
//! - the fit's extrapolation to 50 and 60 bits as a band through the
//!   centroid, a prediction and not a measurement.
//!
//! Pure arithmetic on checked rows; nothing here runs a solver.

use super::*;

/// One size's rows from both runs, paired by target.
struct SizeData {
    a: u8,
    n: u32,
    r: f64,
    /// The runs' reusable set-up medians, in the runs' own units.
    setup: [f64; 2],
    rho_setup: [f64; 2],
    /// Per paired target, `[matched, control]`.
    ic: Vec<[f64; 2]>,
    rho: Vec<[f64; 2]>,
    units_per_step: [f64; 2],
    walks_checked: usize,
    steps_equal: usize,
    logs_equal: usize,
    differing: Vec<String>,
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.total_cmp(b));
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    }
}

fn mean_of(v: &[[f64; 2]], idx: &[usize], arm: usize) -> f64 {
    idx.iter().map(|&i| v[i][arm]).sum::<f64>() / idx.len() as f64
}

impl SizeData {
    /// Cold ratio of one arm (`0` matched, `1` control) on a set of targets.
    fn cold(&self, idx: &[usize], arm: usize) -> f64 {
        (self.setup[arm] + mean_of(&self.ic, idx, arm))
            / (self.rho_setup[arm] + mean_of(&self.rho, idx, arm))
    }
}

/// The 2.5% and 97.5% points, as the rule's `boot_ci` takes them.
fn percentile_ci(mut vals: Vec<f64>) -> Result<J, String> {
    vals.retain(|v| v.is_finite());
    if vals.is_empty() {
        return Err("no finite resample".into());
    }
    vals.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    let lo = (0.025 * vals.len() as f64) as usize;
    let hi = (0.975 * vals.len() as f64) as usize - 1;
    Ok(J::Arr(vec![J::Float(vals[lo]), J::Float(vals[hi])]))
}

fn resample(rng: &mut Random, len: usize, out: &mut Vec<usize>) {
    out.clear();
    for _ in 0..len {
        out.push(rng.randbelow(len as u32) as usize);
    }
}

/// Ordinary least squares of `y` on `x`: `(slope, intercept)`.
fn ols(points: &[(f64, f64)]) -> (f64, f64) {
    let k = points.len() as f64;
    let (mx, my) = (
        points.iter().map(|p| p.0).sum::<f64>() / k,
        points.iter().map(|p| p.1).sum::<f64>() / k,
    );
    let sxx: f64 = points.iter().map(|p| (p.0 - mx).powi(2)).sum();
    let slope = points.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum::<f64>() / sxx;
    (slope, my - slope * mx)
}

/// `y ~ c0 + c1 x1 + c2 x2` by the normal equations; `None` when singular.
fn ols2(rows: &[(f64, f64, f64)]) -> Option<[f64; 3]> {
    let mut m = [[0.0f64; 4]; 3];
    for &(x1, x2, y) in rows {
        let v = [1.0, x1, x2];
        for i in 0..3 {
            for j in 0..3 {
                m[i][j] += v[i] * v[j];
            }
            m[i][3] += v[i] * y;
        }
    }
    for c in 0..3 {
        let p = (c..3).max_by(|&a, &b| m[a][c].abs().total_cmp(&m[b][c].abs()))?;
        if m[p][c].abs() < 1e-12 {
            return None;
        }
        m.swap(c, p);
        for r in 0..3 {
            if r != c {
                let f = m[r][c] / m[c][c];
                for k in c..4 {
                    m[r][k] -= f * m[c][k];
                }
            }
        }
    }
    Some([m[0][3] / m[0][0], m[1][3] / m[1][1], m[2][3] / m[2][2]])
}

impl Comparison {
    /// Rule v8's read-out against `control` (rule v8c).
    pub fn matched(&self, control: &Comparison) -> Result<J, String> {
        let mut data: Vec<SizeData> = Vec::new();
        for (a, n) in SIZES {
            let (r1m, _, badm) = self.load_rows(a, n)?;
            let (r1c, _, badc) = control.load_rows(a, n)?;
            if r1m.is_empty() || r1c.is_empty() {
                continue;
            }
            if !badm.is_empty() || !badc.is_empty() {
                return Err(format!(
                    "k{a}n{n}: {} + {} rows did not pass their check; the read-out needs checked rows",
                    badm.len(),
                    badc.len()
                ));
            }
            let replay_of = |cmp: &Comparison, claim: &J| -> Result<J, String> {
                let p = claim
                    .at("independent_replay_pointer")?
                    .as_str()
                    .ok_or("replay pointer")?;
                read(&cmp.root.join(p))
            };
            let r = num(self.report_of(&r1m[0].claim)?.at("r")?, "r")?;
            let med = |rows: &[Checked], key: &str| {
                let mut v: Vec<f64> = rows.iter().map(|c| c.unit(key)).collect();
                median(&mut v)
            };
            let mut d = SizeData {
                a,
                n,
                r,
                setup: [med(&r1m, "setup_units"), med(&r1c, "setup_units")],
                rho_setup: [med(&r1m, "rho_setup_units"), med(&r1c, "rho_setup_units")],
                ic: Vec::new(),
                rho: Vec::new(),
                units_per_step: [
                    med(&r1m, "rho_units_per_step"),
                    med(&r1c, "rho_units_per_step"),
                ],
                walks_checked: 0,
                steps_equal: 0,
                logs_equal: 0,
                differing: Vec::new(),
            };
            for m in &r1m {
                let Some(c) = r1c.iter().find(|c| c.target == m.target) else {
                    continue;
                };
                d.ic.push([m.unit("ic_online_units"), c.unit("ic_online_units")]);
                d.rho
                    .push([m.unit("rho_online_units"), c.unit("rho_online_units")]);
                d.walks_checked += 1;
                let steps = |row: &Checked| -> Result<i128, String> {
                    row.claim
                        .at("operation_accounting")?
                        .at("rho_online_operations")?
                        .as_i128()
                        .ok_or("rho steps".to_string())
                };
                let same_steps = steps(m)? == steps(c)?;
                let (rm, rc) = (replay_of(self, &m.claim)?, replay_of(control, &c.claim)?);
                let mut same_logs =
                    m.claim.at("ic_target_hash")? == c.claim.at("ic_target_hash")?;
                for arm in ["ic", "rho"] {
                    same_logs &= rm.at(arm)?.at("scalar")? == rc.at(arm)?.at("scalar")?;
                }
                d.steps_equal += usize::from(same_steps);
                d.logs_equal += usize::from(same_logs);
                if !(same_steps && same_logs) {
                    d.differing.push(format!("T{:02}", m.target));
                }
            }
            data.push(d);
        }
        if data.is_empty() {
            return Err("no size has rows in both runs".into());
        }

        let mut seed = BOOT_SEED + 700;
        let mut sizes = Vec::new();
        let mut rng = Random::new(seed);
        for d in &data {
            let len = d.ic.len();
            let all: Vec<usize> = (0..len).collect();
            let stats = |idx: &[usize]| {
                let cm = d.cold(idx, 0);
                let cc = d.cold(idx, 1);
                let rho_speed = mean_of(&d.rho, idx, 1) / mean_of(&d.rho, idx, 0);
                let ic_drift = mean_of(&d.ic, idx, 0) / mean_of(&d.ic, idx, 1);
                [cm, cc, cm / cc, rho_speed, ic_drift]
            };
            let point = stats(&all);
            let mut cols: [Vec<f64>; 5] = Default::default();
            let mut idx = Vec::with_capacity(len);
            for _ in 0..BOOT {
                resample(&mut rng, len, &mut idx);
                let s = stats(&idx);
                for (c, v) in cols.iter_mut().zip(s) {
                    c.push(v);
                }
            }
            let [c0, c1, c2, c3, c4] = cols;
            let fig = |value: f64, vals: Vec<f64>| -> Result<J, String> {
                Ok(J::Obj(vec![
                    ("value".into(), J::Float(value)),
                    ("ci95".into(), percentile_ci(vals)?),
                ]))
            };
            let log2_r = d.r.log2();
            sizes.push(J::Obj(vec![
                ("a".into(), J::Int(d.a.into())),
                ("n".into(), J::Int(d.n.into())),
                ("r".into(), J::Float(d.r)),
                ("log2_r".into(), J::Float(round2(log2_r))),
                ("targets_paired".into(), J::Int(len as i128)),
                (
                    "walks_checked".into(),
                    J::Obj(vec![
                        ("rows".into(), J::Int(d.walks_checked as i128)),
                        ("same_rho_steps".into(), J::Int(d.steps_equal as i128)),
                        (
                            "same_logarithms_and_target_hash".into(),
                            J::Int(d.logs_equal as i128),
                        ),
                        (
                            "differing".into(),
                            J::Arr(d.differing.iter().cloned().map(J::Str).collect()),
                        ),
                    ]),
                ),
                ("cold_ratio_matched".into(), fig(point[0], c0)?),
                ("cold_ratio_control".into(), fig(point[1], c1)?),
                ("cold_matched_over_control".into(), fig(point[2], c2)?),
                ("rho_online_control_over_matched".into(), fig(point[3], c3)?),
                ("ic_online_matched_over_control".into(), fig(point[4], c4)?),
                (
                    "rho_units_per_step_median".into(),
                    J::Obj(vec![
                        ("matched".into(), J::Float(d.units_per_step[0])),
                        ("control".into(), J::Float(d.units_per_step[1])),
                    ]),
                ),
                (
                    "setup_units_median".into(),
                    J::Obj(vec![
                        ("ic_matched".into(), J::Float(d.setup[0])),
                        ("ic_control".into(), J::Float(d.setup[1])),
                        ("rho_matched".into(), J::Float(d.rho_setup[0])),
                        ("rho_control".into(), J::Float(d.rho_setup[1])),
                    ]),
                ),
            ]));
        }

        // Across sizes.
        let k = data.len();
        let xs: Vec<f64> = data.iter().map(|d| d.r.ln()).collect();
        let ns: Vec<f64> = data.iter().map(|d| f64::from(d.n)).collect();
        let point_ln = |arm: usize| -> Vec<f64> {
            data.iter()
                .map(|d| d.cold(&(0..d.ic.len()).collect::<Vec<_>>(), arm).ln())
                .collect()
        };
        let mut slopes = Vec::new();
        for (arm, name) in [(0usize, "matched"), (1, "control")] {
            let ys = point_ln(arm);
            let points: Vec<(f64, f64)> = xs.iter().copied().zip(ys.iter().copied()).collect();
            let (slope, icept) = ols(&points);
            let mut boot = Vec::with_capacity(BOOT);
            let mut idxs: Vec<Vec<usize>> = vec![Vec::new(); k];
            for _ in 0..BOOT {
                let mut pts = Vec::with_capacity(k);
                for (i, d) in data.iter().enumerate() {
                    resample(&mut rng, d.ic.len(), &mut idxs[i]);
                    pts.push((xs[i], d.cold(&idxs[i], arm).ln()));
                }
                boot.push(ols(&pts).0);
            }
            let mut leave_out = Vec::new();
            for skip in 0..k {
                let pts: Vec<(f64, f64)> = points
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != skip)
                    .map(|(_, p)| *p)
                    .collect();
                leave_out.push(J::Obj(vec![
                    (
                        "left_out_log2_r".into(),
                        J::Float(round2(data[skip].r.log2())),
                    ),
                    ("slope".into(), J::Float(ols(&pts).0)),
                ]));
            }
            let two = ols2(&(0..k).map(|i| (xs[i], ns[i], ys[i])).collect::<Vec<_>>());
            let mx = xs.iter().sum::<f64>() / k as f64;
            let my = ys.iter().sum::<f64>() / k as f64;
            let t_ci = fit(&points);
            let band = |log2_r: f64| -> J {
                let x = log2_r * std::f64::consts::LN_2;
                let at = |s: f64| (my + s * (x - mx)).exp();
                let (lo, hi) = match t_ci.get("ci95").and_then(J::as_arr) {
                    Some([lo, hi]) => (
                        lo.as_f64().unwrap_or(f64::NAN),
                        hi.as_f64().unwrap_or(f64::NAN),
                    ),
                    _ => (f64::NAN, f64::NAN),
                };
                J::Obj(vec![
                    ("log2_r".into(), J::Float(log2_r)),
                    ("point".into(), J::Float(at(slope))),
                    (
                        "through_centroid_slope_ci".into(),
                        J::Arr(vec![J::Float(at(lo)), J::Float(at(hi))]),
                    ),
                ])
            };
            slopes.push((
                name.to_string(),
                J::Obj(vec![
                    ("points".into(), J::Int(k as i128)),
                    ("slope".into(), J::Float(slope)),
                    ("intercept".into(), J::Float(icept)),
                    ("t_interval_over_sizes".into(), t_ci.clone()),
                    ("bootstrap_over_targets_ci95".into(), percentile_ci(boot)?),
                    ("leave_one_size_out".into(), J::Arr(leave_out)),
                    (
                        "ln_r_and_n_together".into(),
                        two.map_or(J::Null, |c| {
                            J::Obj(vec![
                                ("intercept".into(), J::Float(c[0])),
                                ("coefficient_on_ln_r".into(), J::Float(c[1])),
                                ("coefficient_on_n".into(), J::Float(c[2])),
                                (
                                    "note".into(),
                                    J::Str(
                                        "six points, three parameters: descriptive, no interval; \
                                         n and log2 r are not separated by these curves"
                                            .into(),
                                    ),
                                ),
                            ])
                        }),
                    ),
                    ("extrapolation".into(), J::Arr(vec![band(50.0), band(60.0)])),
                ]),
            ));
        }
        seed += 1;
        Ok(J::Obj(vec![
            (
                "what_this_is".into(),
                J::Str(
                    "Rule v8: the matched rho (batched canonicalisation) against its same-binary \
                     control (KIC_RHO_CANON=scalar, rho as at rule v7), rows paired by target"
                        .into(),
                ),
            ),
            (
                "bootstrap".into(),
                J::Obj(vec![
                    ("resamples".into(), J::Int(BOOT as i128)),
                    ("seed".into(), J::Int(seed as i128 - 1)),
                ]),
            ),
            ("sizes".into(), J::Arr(sizes)),
            ("slope_of_ln_cold_ratio_on_ln_r".into(), J::Obj(slopes)),
        ]))
    }
}
