//! `verify`: parse msolve's `-P 1` output, extract F_p solutions natively, keep the
//! real decompositions that sum to the target, count useful relations and the rank
//! of their rows over Z/r; every operation is charged.
use crate::cell::Cell;
use crate::ec::*;
use crate::targets::net_vector;
use crate::util::*;
use crypto_lib::cryptanalysis::glv_invariant_base::poly_roots_fp_counted_until;
use crypto_lib::cryptanalysis::ic_boundary::{GroupOps, PrimePoint};
use serde_json::{json, Value};
use std::collections::{BTreeSet, HashMap};
use std::path::Path;
use std::time::{Duration, Instant};

#[derive(Default, Clone)]
pub struct Work {
    pub root_muls: u64,
    pub recon_muls: u64,
    pub recon_invs: u64,
    pub lifts: u64,
    pub lift_muls: u64,
    pub ops: GroupOps,
    pub target_ops: GroupOps,
    pub elim_muls: u64,
    pub elim_invs: u64,
}
impl Work {
    pub fn json(&self) -> Value {
        json!({
            "root_finding_field_muls": self.root_muls,
            "reconstruction_field_muls": self.recon_muls,
            "reconstruction_field_invs": self.recon_invs,
            "lifts": self.lifts, "lift_field_muls": self.lift_muls,
            "point_adds": self.ops.adds, "point_doubles": self.ops.doubles,
            "point_field_ops": {"muls": self.ops.field_ops.muls, "sqrs": self.ops.field_ops.sqrs, "invs": self.ops.field_ops.invs},
            "target_setup": {"adds": self.target_ops.adds, "doubles": self.target_ops.doubles,
                "field_ops": {"muls": self.target_ops.field_ops.muls, "sqrs": self.target_ops.field_ops.sqrs, "invs": self.target_ops.field_ops.invs}},
            "elimination_field_muls": self.elim_muls, "elimination_field_invs": self.elim_invs,
        })
    }
}

pub struct Ctx {
    pub cell: Cell,
    pub m: usize,
    pub name: String,
    pub label: String,
    pub x_r: u64,
    pub ab: Option<(u64, u64)>,
    pub r_point: Option<PrimePoint>,
    pub counts: Value,
    pub work: Work,
}

impl Ctx {
    pub fn load(cell_dir: &str, targets_dir: &str, name: &str) -> Result<Ctx, String> {
        let cell = Cell::load(Path::new(cell_dir))?;
        let tj = read_json(&Path::new(targets_dir).join("targets.json"))?;
        if !check_seal(&tj) {
            return Err("targets.json seal mismatch".into());
        }
        if tj["cell_sha256"].as_str() != Some(&cell.sha256) {
            return Err("targets.json belongs to another cell".into());
        }
        let m = u64_of(&tj, "m")? as usize;
        let tg = tj["targets"]
            .as_array()
            .ok_or("targets")?
            .iter()
            .find(|t| t["name"].as_str() == Some(name))
            .ok_or_else(|| format!("no target {name}"))?;
        let x_r = u64_of(tg, "x_r")?;
        let ab = match (tg["a"].as_u64(), tg["b"].as_u64()) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        };
        let mut work = Work::default();
        let ec = cell.ec();
        let r_point = ab.map(|(a, b)| {
            let pp = ec.pt(cell.px, cell.py);
            let q = ec.pt(tj["Q"]["x"].as_u64().unwrap_or(0), tj["Q"]["y"].as_u64().unwrap_or(0));
            ec.combo(&mut work.target_ops, pp, q, a, b)
        });
        if let Some(rp) = r_point {
            if rp.x != x_r {
                return Err("x_r does not match a*P + b*Q".into());
            }
        }
        Ok(Ctx {
            cell,
            m,
            name: name.into(),
            label: tg["label"].as_str().unwrap_or("").into(),
            x_r,
            ab,
            r_point,
            counts: json!({"real_count": tg["real_count"], "algebraic_count": tg["algebraic_count"], "ordered_algebraic_solutions": tg["ordered_algebraic_solutions"]}),
            work,
        })
    }
}

pub struct Found {
    pub fp_solutions: u64,
    pub multisets_fp: usize,
    pub multisets_real: usize,
    pub decomps: BTreeSet<Vec<u8>>,
    pub nets: BTreeSet<Vec<i64>>,
}

/// Rank of `rows` over `Z/r` (`r` prime), charging multiplications and inversions.
pub fn rank_mod(rows: &[Vec<u64>], r: u64, work: &mut Work) -> usize {
    let mut a: Vec<Vec<u64>> = rows.to_vec();
    let ncols = a.first().map_or(0, Vec::len);
    let mut rank = 0;
    for col in 0..ncols {
        let Some(piv) = (rank..a.len()).find(|&i| a[i][col] != 0) else {
            continue;
        };
        a.swap(rank, piv);
        let inv = invm(a[rank][col], r);
        work.elim_invs += 1;
        for c in col..ncols {
            a[rank][c] = mulm(a[rank][c], inv, r);
            work.elim_muls += 1;
        }
        let prow = a[rank].clone();
        for (i, row) in a.iter_mut().enumerate() {
            if i != rank && row[col] != 0 {
                let f = row[col];
                for c in col..ncols {
                    row[c] = subm(row[c], mulm(f, prow[c], r), r);
                    work.elim_muls += 1;
                }
            }
        }
        rank += 1;
    }
    rank
}

fn signed_row(c: &[i64], b: Option<u64>, r: u64) -> Vec<u64> {
    let mut row: Vec<u64> = c
        .iter()
        .map(|&v| if v >= 0 { v as u64 % r } else { (r - (v.unsigned_abs() % r)) % r })
        .collect();
    // coefficient of log Q is -b (zero for a planted target)
    row.push(b.map_or(0, |b| (r - b % r) % r));
    row
}

/// Analyse F_p solutions given as the values of `y_{0,0} .. y_{m-1,0}`.
pub fn analyze(ctx: &mut Ctx, y0_solutions: &[Vec<u64>]) -> Found {
    let cell_p = ctx.cell.p;
    let (p, a, b, m) = (cell_p, ctx.cell.a, ctx.cell.b, ctx.m);
    let ec = ctx.cell.ec();
    let base_idx: HashMap<u64, usize> = ctx
        .cell
        .real
        .iter()
        .enumerate()
        .map(|(i, e)| (e.x, i))
        .collect();
    let mut lift_cache: HashMap<u64, Option<u64>> = HashMap::new();
    let mut seen: HashMap<Vec<u64>, bool> = HashMap::new();
    let mut found = Found {
        fp_solutions: 0,
        multisets_fp: 0,
        multisets_real: 0,
        decomps: BTreeSet::new(),
        nets: BTreeSet::new(),
    };
    for ys in y0_solutions {
        found.fp_solutions += 1;
        let mut xs: Vec<u64> = ys
            .iter()
            .map(|&y| {
                ctx.work.recon_muls += 1;
                mulm(ctx.cell.g, y, p)
            })
            .collect();
        xs.sort_unstable();
        if seen.contains_key(&xs) {
            continue;
        }
        found.multisets_fp += 1;
        // lift every distinct abscissa (counted square roots)
        let mut all_real = true;
        for &x in &xs {
            let e = lift_cache.entry(x).or_insert_with(|| {
                ctx.work.lifts += 1;
                ctx.work.lift_muls += 4;
                let rr = rhs(p, a, b, x);
                if rr != 0 && jacobi(rr, p) != 1 {
                    return None;
                }
                let y = sqrt_counted(p, rr, &mut ctx.work.lift_muls)?;
                Some(y.min((p - y) % p))
            });
            if e.is_none() {
                all_real = false;
            }
        }
        seen.insert(xs.clone(), all_real);
        if !all_real {
            continue;
        }
        found.multisets_real += 1;
        // sign search: first sign +, depth-first, one counted addition per node
        let pts: Vec<(PrimePoint, PrimePoint, u8)> = xs
            .iter()
            .map(|&x| {
                let y = lift_cache[&x].expect("real");
                let i = base_idx.get(&x).copied().unwrap_or(usize::MAX);
                (ec.pt(x, y), ec.pt(x, (p - y) % p), (2 * i) as u8)
            })
            .collect();
        let mut stack: Vec<(usize, PrimePoint, Vec<u8>)> = vec![(0, PrimePoint::INFINITY, vec![])];
        while let Some((d, acc, syms)) = stack.pop() {
            if d == m {
                let target_x = ctx.x_r;
                let (hit, flip) = match ctx.r_point {
                    Some(rp) => {
                        let nrp = ec.neg(rp);
                        (acc == rp || acc == nrp, acc == nrp)
                    }
                    None => (!acc.infinity && acc.x == target_x, false),
                };
                if hit {
                    let mut s: Vec<u8> = syms.iter().map(|&k| if flip { k ^ 1 } else { k }).collect();
                    s.sort_unstable();
                    found.nets.insert(net_vector(ctx.cell.real.len(), &s));
                    found.decomps.insert(s);
                }
                continue;
            }
            let (pos, neg, s0) = &pts[d];
            for (q, s) in [(*pos, *s0), (*neg, *s0 ^ 1)] {
                if d == 0 && s != *s0 {
                    continue;
                }
                let nacc = ec.add(&mut ctx.work.ops, acc, q);
                let mut ns = syms.clone();
                ns.push(s);
                stack.push((d + 1, nacc, ns));
            }
        }
    }
    found
}

pub fn found_json(ctx: &mut Ctx, found: &Found) -> Value {
    let r = ctx.cell.r;
    let b_real = ctx.cell.real.len();
    let b = ctx.ab.map(|x| x.1);
    let rows: Vec<Vec<u64>> = found.nets.iter().map(|c| signed_row(c, b, r)).collect();
    let rank = rank_mod(&rows, r, &mut ctx.work);
    let crows: Vec<Vec<u64>> = found
        .nets
        .iter()
        .map(|c| {
            let mut v = signed_row(c, None, r);
            v.pop();
            v
        })
        .collect();
    let rank_c = rank_mod(&crows, r, &mut ctx.work);
    json!({
        "fp_rational_solutions": found.fp_solutions,
        "distinct_x_multisets_fp": found.multisets_fp,
        "distinct_x_multisets_real": found.multisets_real,
        "real_signed_decompositions": found.decomps.len(),
        "useful_relations": found.nets.len(),
        "rank_increment": rank,
        "rank_increment_coefficients_only": rank_c,
        "unknowns": b_real + 1,
        "row_layout": "[c_1..c_Breal, -b | a]; rank over Z/r of the left part, computed without hidden logs",
        "decompositions_signed_base": found.decomps.iter().map(|d| crate::targets::signed_json(d)).collect::<Vec<_>>(),
        "public_counts_from_targets_json": ctx.counts.clone(),
    })
}

pub enum Outcome {
    NoSolution,
    PosDim,
    ZeroDim(Value),
}

pub fn parse_msolve(text: &str) -> Result<Outcome, String> {
    let t = text.trim();
    let t = t.strip_suffix(':').unwrap_or(t).trim();
    if t == "[-1]" {
        return Ok(Outcome::NoSolution);
    }
    if t.starts_with("[1,") {
        return Ok(Outcome::PosDim);
    }
    if !t.starts_with("[0,") {
        return Err(format!("unrecognised msolve output starting {:?}", &t[..t.len().min(40)]));
    }
    let v: Value = serde_json::from_str(&t.replace('\'', "\"")).map_err(|e| format!("msolve JSON: {e}"))?;
    Ok(Outcome::ZeroDim(v))
}

pub fn run(args: &[String]) -> Result<(), String> {
    let t0 = Instant::now();
    let cpu0 = cpu_secs();
    let mut ctx = Ctx::load(
        &crate::arg_req(args, "--cell")?,
        &crate::arg_req(args, "--targets")?,
        &crate::arg_req(args, "--target")?,
    )?;
    let ms_path = crate::arg_req(args, "--msolve-out")?;
    let out = crate::arg_req(args, "--out")?;
    let seed: u64 = crate::arg_opt(args, "--seed").map_or(Ok(1), |s| s.parse()).map_err(|_| "seed")?;
    let deadline = crate::arg_opt(args, "--deadline")
        .map(|s| s.parse::<f64>().map(|x| Instant::now() + Duration::from_secs_f64(x)))
        .transpose()
        .map_err(|_| "deadline")?;
    let text = std::fs::read_to_string(&ms_path).map_err(|e| format!("read {ms_path}: {e}"))?;
    let outcome = parse_msolve(&text)?;
    let p = ctx.cell.p;
    let m = ctx.m;
    let mut rec = json!({
        "schema": "pkm.m8-panel.verify/v1",
        "cell_sha256": ctx.cell.sha256, "target": ctx.name, "label": ctx.label, "x_r": ctx.x_r,
        "msolve_output": ms_path, "msolve_output_sha256": sha256_hex(text.as_bytes()),
    });
    let mut y0s: Vec<Vec<u64>> = Vec::new();
    let mut timed_out = false;
    match outcome {
        Outcome::NoSolution => rec["msolve_outcome"] = json!("no_solution_in_closure"),
        Outcome::PosDim => rec["msolve_outcome"] = json!("positive_dimension"),
        Outcome::ZeroDim(v) => {
            rec["msolve_outcome"] = json!("zero_dimensional");
            let hdr = &v[1];
            if u64_of_idx(hdr, 0)? != p {
                return Err("msolve characteristic differs from the cell".into());
            }
            let q = u64_of_idx(hdr, 2)?;
            let names: Vec<String> = hdr[3]
                .as_array()
                .ok_or("names")?
                .iter()
                .map(|n| n.as_str().unwrap_or("").to_string())
                .collect();
            let lin: Vec<i64> = hdr[4]
                .as_array()
                .ok_or("lin")?
                .iter()
                .map(|n| n.as_i64().unwrap_or(0))
                .collect();
            if lin.len() != names.len() {
                return Err("lin/names length mismatch".into());
            }
            let body = &hdr[5][1];
            let f = u64_vec(&body[0][1])?;
            if f.last() != Some(&1) {
                return Err("eliminating polynomial not monic".into());
            }
            let w = u64_vec(&body[1][1])?;
            let params: Vec<Vec<u64>> = body[2]
                .as_array()
                .ok_or("params")?
                .iter()
                .map(|e| u64_vec(&e[0][1]))
                .collect::<Result<_, _>>()?;
            // the separating variable t: the added A, else the unit-vector variable
            let t_var: usize = if names.last().map(String::as_str) == Some("A") {
                names.len() - 1
            } else {
                lin.iter().position(|&c| c == 1).ok_or("no unit separating variable")?
            };
            let others: Vec<usize> = (0..names.len()).filter(|&i| i != t_var).collect();
            if others.len() != params.len() {
                return Err(format!("param count {} != {}", params.len(), others.len()));
            }
            let want: Vec<String> = (0..m).map(|i| format!("y{i}_0")).collect();
            let roots = poly_roots_fp_counted_until(&f, p, seed, &mut ctx.work.root_muls, deadline);
            let roots = match roots {
                Some(r) => r,
                None => {
                    timed_out = true;
                    Vec::new()
                }
            };
            rec["quotient_dimension_with_multiplicity"] = json!(q);
            rec["eliminating_degree"] = json!(f.len() - 1);
            rec["separating_variable"] = json!(names[t_var]);
            rec["fp_roots_of_eliminating_polynomial"] = json!(roots.len());
            for &t0v in &roots {
                let mut ys = Vec::with_capacity(m);
                for nm in &want {
                    let ix = names.iter().position(|n| n == nm).ok_or_else(|| format!("variable {nm} absent"))?;
                    if ix == t_var {
                        ys.push(t0v);
                        continue;
                    }
                    let k = others.iter().position(|&o| o == ix).unwrap_or(0);
                    let g = &params[k];
                    let mut acc = 0u64;
                    for &c in g.iter().rev() {
                        acc = addm(mulm(acc, t0v, p), c, p);
                        ctx.work.recon_muls += 1;
                    }
                    let mut val = (p - acc) % p;
                    if w.len() > 1 {
                        let mut wv = 0u64;
                        for &c in w.iter().rev() {
                            wv = addm(mulm(wv, t0v, p), c, p);
                            ctx.work.recon_muls += 1;
                        }
                        ctx.work.recon_invs += 1;
                        val = mulm(val, invm(wv, p), p);
                        ctx.work.recon_muls += 1;
                    }
                    ys.push(val);
                }
                y0s.push(ys);
            }
        }
    }
    let found = analyze(&mut ctx, &y0s);
    let fj = found_json(&mut ctx, &found);
    rec["extraction"] = fj;
    rec["deadline_hit_in_root_finding"] = json!(timed_out);
    rec["work"] = ctx.work.json();
    rec["wall_secs"] = json!(t0.elapsed().as_secs_f64());
    rec["cpu_secs"] = json!(cpu_secs() - cpu0);
    rec["vm_hwm_kb"] = json!(vm_hwm_kb());
    rec["tool"] = crate::tool_info();
    let rec_rank = rec["extraction"]["rank_increment"].clone();
    write_json(Path::new(&out), &seal(rec))?;
    println!(
        "verify {}: useful_relations={} rank={}",
        ctx.name,
        found.nets.len(),
        rec_rank
    );
    Ok(())
}

fn u64_of_idx(v: &Value, i: usize) -> Result<u64, String> {
    v[i].as_u64().ok_or_else(|| format!("expected integer at {i}"))
}
