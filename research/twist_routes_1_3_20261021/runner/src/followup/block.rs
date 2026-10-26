//! E26 finite projection census and bounded equivalent-encoding pilot.
use crate::{
    digest, f4_fp as f4, solver, toy_field::*, twist_ic_algebra as alg, write_json, Fixture, Result,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    os::unix::process::ExitStatusExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};

#[derive(Clone, Serialize, Deserialize)]
struct Job {
    id: String,
    workload: String,
    arm: String,
    branch: usize,
    expected: u64,
    case: alg::Case,
}

fn norm(terms: f4::Poly, p: u64) -> f4::Poly {
    let mut sums = BTreeMap::new();
    for (e, c) in terms {
        let v = sums.entry(e).or_insert(0u64);
        *v = (*v + c) % p;
    }
    sums.into_iter().filter(|(_, c)| *c != 0).collect()
}
fn add(a: &f4::Poly, b: &f4::Poly, p: u64) -> f4::Poly {
    norm(a.iter().chain(b).cloned().collect(), p)
}
fn scale(a: &f4::Poly, c: u64, p: u64) -> f4::Poly {
    norm(a.iter().map(|(e, v)| (e.clone(), v * c % p)).collect(), p)
}
fn mul(a: &f4::Poly, b: &f4::Poly, p: u64) -> f4::Poly {
    norm(
        a.iter()
            .flat_map(|(e, c)| {
                b.iter()
                    .map(move |(f, d)| (e.iter().zip(f).map(|(u, v)| u + v).collect(), c * d % p))
            })
            .collect(),
        p,
    )
}
fn constant(n: usize, c: u64) -> f4::Poly {
    if c == 0 {
        vec![]
    } else {
        vec![(vec![0; n], c)]
    }
}
fn variable(n: usize, i: usize) -> f4::Poly {
    let mut e = vec![0; n];
    e[i] = 1;
    vec![(e, 1)]
}
fn membership(n: usize, i: usize, xs: &[u64], p: u64) -> f4::Poly {
    xs.iter().fold(constant(n, 1), |a, x| {
        mul(&a, &add(&variable(n, i), &constant(n, (p - x) % p), p), p)
    })
}
fn permute(poly: &f4::Poly, order: &[usize]) -> f4::Poly {
    poly.iter()
        .map(|(e, c)| (order.iter().map(|i| e[*i]).collect(), *c))
        .collect()
}
fn diagonal(c: &alg::Case) -> alg::Case {
    let mut r = c.clone();
    r.encoding = "invariant-diagonal".into();
    r.names.remove(1);
    r.equations = c
        .equations
        .iter()
        .map(|q| {
            norm(
                q.iter()
                    .map(|(e, c)| {
                        let mut e = e.clone();
                        e[0] += e[1];
                        e.remove(1);
                        (e, *c)
                    })
                    .collect(),
                c.p,
            )
        })
        .filter(|q| !q.is_empty())
        .collect();
    r
}
fn frobenius(c: &alg::Case, d: u64) -> alg::Case {
    let n = c.names.len();
    let k = c.arity - 2;
    let width = n + k;
    let order = (n..width)
        .chain(c.arity..n)
        .chain(0..c.arity)
        .collect::<Vec<_>>();
    let mut raw = c.names.clone();
    raw.extend((0..k).map(|i| format!("v{}", i + 1)));
    let mut equations = vec![];
    for q in &c.equations {
        let (mut real, mut imag) = (vec![], vec![]);
        for (powers, coeff) in q {
            let mut term = (constant(width, *coeff), vec![]);
            for (i, power) in powers.iter().enumerate() {
                for _ in 0..*power {
                    let re = variable(width, i);
                    let im = if i >= c.arity {
                        variable(width, n + i - c.arity)
                    } else {
                        vec![]
                    };
                    term = (
                        add(
                            &mul(&term.0, &re, c.p),
                            &scale(&mul(&term.1, &im, c.p), d, c.p),
                            c.p,
                        ),
                        add(&mul(&term.0, &im, c.p), &mul(&term.1, &re, c.p), c.p),
                    );
                }
            }
            real = add(&real, &term.0, c.p);
            imag = add(&imag, &term.1, c.p);
        }
        if !real.is_empty() {
            equations.push(permute(&real, &order));
        }
        if !imag.is_empty() {
            equations.push(permute(&imag, &order));
        }
    }
    for i in 0..k {
        equations.push(permute(&variable(width, n + i), &order));
    }
    let mut result = c.clone();
    result.encoding = "frobenius-variable-blocks".into();
    result.names = order.iter().map(|i| raw[*i].clone()).collect();
    result.equations = equations;
    result
}
fn lift_frobenius(values: &[u64], m: usize) -> Vec<u64> {
    let n = values.len();
    (n..n + m - 2)
        .map(|_| 0)
        .chain(values[m..].iter().copied())
        .chain(values[..m].iter().copied())
        .collect()
}

fn signed(e: Quad, tuple: &[u64], points: &BTreeMap<u64, P>, target: P) -> Option<u64> {
    for mask in 0..(1u64 << tuple.len()) {
        let sum = tuple.iter().enumerate().fold(None, |s, (i, x)| {
            let p = points[x];
            e.pa(s, if mask >> i & 1 == 0 { p } else { e.pn(p) })
        });
        if sum == target {
            return Some(mask);
        }
    }
    None
}
fn canonical(tuple: &[u64]) -> Vec<u64> {
    let mut t = tuple.to_vec();
    t[..2].sort();
    t
}
fn admissible(c: &alg::Case, v: &[u64]) {
    assert_eq!(v.len(), c.names.len());
    assert!(
        c.equations.iter().all(|q| f4::eval(q, v, c.p) == 0),
        "encoding assignment rejected"
    );
}

pub fn build(fixtures: &[Fixture], out: &Path) -> Result<Vec<Value>> {
    fs::create_dir_all(out.join("cases"))?;
    fs::create_dir_all(out.join("witnesses"))?;
    let mut jobs = vec![];
    let mut rows = vec![];
    let mut gp=String::from("default(parisizemax,4000000000);\nblock_good=0;block_bad=0;\nblock_f3(x,y,z,a,b)=(x-y)^2*z^2-2*((x+y)*(x*y+a)+2*b)*z+(x*y-a)^2-4*b*(x+y);\nblock_member(p,B)={my(z='z,F=prod(i=1,#B,z-Mod(B[i],p)));for(i=1,#B,if(#ellordinate(block_E,Mod(B[i],p))==0,error(\"base lift\")));for(i=1,#B,for(j=i+1,#B,if(F%(z^2-Mod(B[i]+B[j],p)*z+Mod(B[i]*B[j],p))!=0,error(\"pair remainder\"))));if(p<=251,for(u=0,p-1,for(v=0,p-1,my(expect=0);for(i=1,#B,for(j=i+1,#B,if((B[i]+B[j])%p==u&&B[i]*B[j]%p==v,expect=1)));if((F%(z^2-Mod(u,p)*z+Mod(v,p))==0)!=expect,error(\"extra pair\")))));};\nblock_witness(p,a,b,T,X,Y,mask)={my(E=ellinit([Mod(a,p),Mod(b,p)]),S=[0],U,V,W,z,q);for(i=1,#X,S=elladd(E,S,[Mod(X[i],p),Mod(if(bittest(mask,i-1),-Y[i],Y[i]),p)]));if(S!=[Mod(T[1],p),Mod(T[2],p)],error(\"signed sum\"));U=[Mod(X[1],p),Mod(Y[1],p)];V=[Mod(X[2],p),Mod(Y[2],p)];W=elladd(E,U,V);if(#W==2,z=W[1];q=block_f3(U[1],V[1],z,Mod(a,p),Mod(b,p));if(q!=0,error(\"F3 law\"));if(q+1==0,error(\"corruption accepted\"));if(4*((U[1]+V[1])*(U[1]*V[1]+a)+2*b)*z!=0,if(block_f3(U[1],V[1],z,Mod(a,p),Mod(b,p))+4*((U[1]+V[1])*(U[1]*V[1]+a)+2*b)*z==0,error(\"wrong sign accepted\"));block_bad++));block_good++;};\n");
    for m in [2, 3, 4] {
        for bsize in [8, 12, 16] {
            for f in fixtures
                .iter()
                .filter(|f| prime(f.order) && f.order % 2 == 1)
            {
                let e = f.curve();
                let target = solver::target(f, 0);
                let rx = target.unwrap().0 .0;
                let xs = (0..f.p)
                    .filter(|x| sqrt(e.rhs(Q(*x, 0)).0, f.p).is_some_and(|y| y != 0))
                    .take(bsize)
                    .collect::<Vec<_>>();
                assert_eq!(xs.len(), bsize);
                let points = xs
                    .iter()
                    .map(|x| {
                        (
                            *x,
                            Some((Q(*x, 0), Q(sqrt(e.rhs(Q(*x, 0)).0, f.p).unwrap(), 0))),
                        )
                    })
                    .collect::<BTreeMap<_, _>>();
                let direct = alg::build(f.p, f.a, f.b, rx, &xs, m, "product");
                let fb = frobenius(&direct, f.d);
                let inv = alg::build(f.p, f.a, f.b, rx, &xs, m, "symmetric");
                let diag = diagonal(&direct);
                let factored = alg::build(f.p, f.a, f.b, rx, &xs, m, "chain");
                let bits = alg::build(f.p, f.a, f.b, rx, &xs, m, "bits");
                let hybrid = (0..8)
                    .map(|h| {
                        let mut c = direct.clone();
                        c.encoding = "hybrid-eight".into();
                        c.equations[0] = membership(
                            c.names.len(),
                            0,
                            &xs.iter()
                                .enumerate()
                                .filter(|(i, _)| i % 8 == h)
                                .map(|(_, x)| *x)
                                .collect::<Vec<_>>(),
                            f.p,
                        );
                        c
                    })
                    .collect::<Vec<_>>();
                let id = format!("{}-b{bsize}-m{m}-s0", f.id);
                let (mut affine, mut off, mut diagonal_n, mut exceptional) =
                    (0u64, 0u64, 0u64, 0u64);
                let mut buckets = [0u64; 8];
                let mut projected = BTreeSet::new();
                let mut independent = BTreeSet::new();
                let mut witnesses = vec![];
                let mut checked = 0u64;
                solver::tuples(&xs, m, |tuple| {
                    let paths = solver::trajectories(e, tuple, rx);
                    let canon = canonical(tuple);
                    if f.p <= 251 && signed(e, tuple, &points, target).is_some() {
                        independent.insert(canon.clone());
                    }
                    if !paths.is_empty() && projected.insert(canon.clone()) {
                        let mask = signed(e, &canon, &points, target).expect("signed source lift");
                        let ys = canon
                            .iter()
                            .map(|x| points[x].unwrap().1 .0)
                            .collect::<Vec<_>>();
                        witnesses.push(json!({"x":canon,"y":ys,"sign_mask":mask}));
                    }
                    for path in paths {
                        if path[1..path.len() - 1].iter().any(Option::is_none) {
                            exceptional += 1;
                            continue;
                        }
                        let values = solver::expand_values(&direct, tuple, &path).unwrap();
                        affine += 1;
                        admissible(&fb, &lift_frobenius(&values, m));
                        checked += 1;
                        let bucket = xs.iter().position(|x| *x == tuple[0]).unwrap() % 8;
                        buckets[bucket] += 1;
                        admissible(&hybrid[bucket], &values);
                        checked += 1;
                        let factored_values =
                            solver::expand_values(&factored, tuple, &path).unwrap();
                        admissible(&factored, &factored_values);
                        checked += 1;
                        admissible(&bits, &solver::expand_values(&bits, tuple, &path).unwrap());
                        checked += 1;
                        if tuple[0] == tuple[1] {
                            diagonal_n += 1;
                            let mut v = values.clone();
                            v.remove(1);
                            admissible(&diag, &v);
                            checked += 1;
                        } else {
                            off += 1;
                            if tuple[0] < tuple[1] {
                                let v = solver::expand_values(&inv, tuple, &path).unwrap();
                                admissible(&inv, &v);
                                checked += 1;
                            }
                        }
                    }
                });
                if f.p <= 251 {
                    assert_eq!(projected, independent, "independent full signed census");
                }
                assert_eq!(off % 2, 0);
                assert_eq!(affine, diagonal_n + off);
                assert_eq!(buckets.iter().sum::<u64>(), affine);
                let mut branches = vec![
                    ("direct-chain", 0, direct.clone(), affine),
                    ("frobenius-block", 0, fb, affine),
                    ("first-pair-invariant", 0, inv, off / 2),
                    ("first-pair-invariant", 1, diag, diagonal_n),
                    ("factored-control", 0, factored, affine),
                    ("bits-control", 0, bits, affine),
                ];
                for (h, c) in hybrid.into_iter().enumerate() {
                    branches.push(("hybrid-eight", h, c, buckets[h]));
                }
                for (arm, branch, case, expected) in branches {
                    let jid = format!("{id}-{arm}-h{branch}");
                    let job = Job {
                        id: jid.clone(),
                        workload: id.clone(),
                        arm: arm.into(),
                        branch,
                        expected,
                        case,
                    };
                    write_json(out.join("cases").join(format!("{jid}.json")), &job)?;
                    jobs.push(job);
                }
                let witness_file = out.join("witnesses").join(format!("{id}.json"));
                write_json(
                    &witness_file,
                    &json!({"fixture":f,"base":xs,"target":target,"witnesses":witnesses,"canonical_projected_tuples":projected,"exceptional_identity_prefix_paths":exceptional}),
                )?;
                gp.push_str(&format!(
                    "block_E=ellinit([Mod({},{}),Mod({}, {})]);block_member({},{});\n",
                    f.a,
                    f.p,
                    f.b,
                    f.p,
                    f.p,
                    gp_vec(&xs)
                ));
                for w in &witnesses {
                    let x: Vec<u64> = serde_json::from_value(w["x"].clone())?;
                    let y: Vec<u64> = serde_json::from_value(w["y"].clone())?;
                    gp.push_str(&format!(
                        "block_witness({},{},{},[{},{}],{},{},{});\n",
                        f.p,
                        f.a,
                        f.b,
                        target.unwrap().0 .0,
                        target.unwrap().1 .0,
                        gp_vec(&x),
                        gp_vec(&y),
                        w["sign_mask"]
                    ));
                }
                rows.push(json!({"workload":id,"fixture":f.id,"p":f.p,"B":bsize,"arity":m,"target_label":0,"base":xs,"target":target,"affine_assignments":affine,"invariant_off_diagonal_assignments":off/2,"invariant_diagonal_assignments":diagonal_n,"exceptional_identity_prefix_paths":exceptional,"canonical_relation_tuples":projected.len(),"canonical_relation_sha256":digest(&serde_json::to_vec(&projected)?),"independent_full_signed_census":f.p<=251,"encoding_assignments_checked":checked,"witness_sha256":digest(&fs::read(&witness_file)?)}));
                println!(
                    "{}",
                    json!({"stage":"block-build","workloads":rows.len(),"id":id})
                );
            }
        }
    }
    gp.push_str("if(block_good==0||block_bad==0,error(\"missing controls\"));print(\"ALL_BLOCK_REPLAY_OK \",block_good,\" \",block_bad);quit;\n");
    fs::write(out.join("verify.gp"), gp)?;
    write_json(out.join("JOBS.json"), &jobs)?;
    write_json(out.join("WORKLOADS.json"), &rows)?;
    Ok(rows)
}
fn gp_vec(v: &[u64]) -> String {
    format!(
        "[{}]",
        v.iter().map(u64::to_string).collect::<Vec<_>>().join(",")
    )
}

/// Exact ordinary control for the Frobenius arm's surviving variable order.
pub fn matched_control(input: &Path, out: &Path) -> Result<Vec<Value>> {
    let mut jobs: Vec<Job> = serde_json::from_slice(&fs::read(input.join("JOBS.json"))?)?;
    let mut proofs = vec![];
    let mut controls = vec![];
    for direct in jobs.iter().filter(|j| j.arm == "direct-chain") {
        let c = &direct.case;
        let order = (c.arity..c.names.len())
            .chain(0..c.arity)
            .collect::<Vec<_>>();
        let mut ordinary = c.clone();
        ordinary.encoding = "ordinary-prefix-order".into();
        ordinary.names = order.iter().map(|i| c.names[*i].clone()).collect();
        ordinary.equations = c.equations.iter().map(|q| permute(q, &order)).collect();
        let frob = jobs
            .iter()
            .find(|j| j.workload == direct.workload && j.arm == "frobenius-block")
            .unwrap();
        let k = c.arity - 2;
        let restricted = frob
            .case
            .equations
            .iter()
            .map(|q| {
                norm(
                    q.iter()
                        .filter(|(e, _)| e[..k].iter().all(|v| *v == 0))
                        .map(|(e, a)| (e[k..].to_vec(), *a))
                        .collect(),
                    c.p,
                )
            })
            .filter(|q| !q.is_empty())
            .map(|q| f4::normalise(&q, c.p, f4::Ordering::Grevlex))
            .collect::<Vec<_>>();
        let expected = ordinary
            .equations
            .iter()
            .map(|q| f4::normalise(q, c.p, f4::Ordering::Grevlex))
            .collect::<Vec<_>>();
        assert_eq!(
            restricted, expected,
            "dropping the zero anti-block must recover the ordinary input exactly"
        );
        proofs.push(json!({"workload":direct.workload,"variable_permutation":order,"dropping_zero_anti_block_recovers_same_polynomial_coefficients":true,"expected_affine_assignments":direct.expected,"ordinary_input_sha256":digest(&serde_json::to_vec(&ordinary)?)}));
        controls.push(Job {
            id: format!("{}-ordinary-prefix-order-h0", direct.workload),
            workload: direct.workload.clone(),
            arm: "ordinary-prefix-order".into(),
            branch: 0,
            expected: direct.expected,
            case: ordinary,
        });
    }
    jobs.extend(controls);
    write_json(out.join("JOBS.json"), &jobs)?;
    fs::copy(input.join("WORKLOADS.json"), out.join("WORKLOADS.json"))?;
    write_json(out.join("MATCHED_CONTROL_PROOFS.json"), &proofs)?;
    Ok(proofs)
}

pub fn worker(input: &Path, out: &Path, budget_ms: u64) -> Result<()> {
    let job: Job = serde_json::from_slice(&fs::read(input)?)?;
    write_json(out, &compute_job(&job, budget_ms))
}
fn compute_job(job: &Job, budget_ms: u64) -> Value {
    let c = &job.case;
    let mut options =
        f4::F4Options::new(f4::Ordering::Grevlex, 32).with_budget(Duration::from_millis(budget_ms));
    options.capture_run_log = true;
    let result = f4::f4(&c.equations, c.names.len(), c.p, &options);
    let log = f4::take_run_log();
    json!({"job":job.id,"basis":result.basis,"timed_out":result.timed_out,"pairs_above_bound":result.pairs_above_bound,"staircase_at_stop":result.staircase_at_stop,"field_multiplications_row_reduction":result.field_ops,"max_macaulay_rows":result.max_rows,"max_macaulay_cols":result.max_cols,"solving_degree":result.solving_degree,"degree_reached":result.degree_reached,"steps":result.steps,"step_log":log.steps.iter().map(|s|json!({"degree":s.degree,"pairs":s.pairs,"rows":s.rows,"all_rows":s.all_rows,"row_reduction_multiplications":s.ops,"new_polynomials":s.new_polys})).collect::<Vec<_>>(),"budget_ms":budget_ms,"wall_time_admitted":false,"complete_cost":false})
}
pub fn batch_worker(input: &Path, out: &Path, budget_ms: u64) -> Result<()> {
    let jobs: Vec<Job> = serde_json::from_slice(&fs::read(input)?)?;
    let start = Instant::now();
    let results = jobs
        .iter()
        .map(|job| {
            compute_job(
                job,
                budget_ms
                    .saturating_sub(start.elapsed().as_millis() as u64)
                    .max(1),
            )
        })
        .collect::<Vec<_>>();
    write_json(out, &results)
}

/// Construct the public finite task, solve it, recover signs and deduplicate.
/// This worker reads no prior basis, expected tuple list or known logarithm.
pub fn decomposition_worker(input: &Path, out: &Path, budget_ms: u64) -> Result<()> {
    let task: Value = serde_json::from_slice(&fs::read(input)?)?;
    let f: Fixture = serde_json::from_value(task["fixture"].clone())?;
    let target: P = serde_json::from_value(task["target"].clone())?;
    let bsize = task["B"].as_u64().ok_or("B")? as usize;
    let m = task["arity"].as_u64().ok_or("arity")? as usize;
    let arm = task["arm"].as_str().ok_or("arm")?;
    if !(2..=3).contains(&m) {
        return Err("decomposition pilot only supports m=2,3".into());
    }
    let started = Instant::now();
    let e = f.curve();
    let tx = target.ok_or("nonidentity target required")?.0 .0;
    assert!(e.on(target));
    let xs = (0..f.p)
        .filter(|x| sqrt(e.rhs(Q(*x, 0)).0, f.p).is_some_and(|y| y != 0))
        .take(bsize)
        .collect::<Vec<_>>();
    assert_eq!(xs.len(), bsize);
    let points = xs
        .iter()
        .map(|x| {
            (
                *x,
                Some((Q(*x, 0), Q(sqrt(e.rhs(Q(*x, 0)).0, f.p).unwrap(), 0))),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let direct = alg::build(f.p, f.a, f.b, tx, &xs, m, "product");
    let cases = match arm {
        "direct-chain" => vec![direct],
        "first-pair-invariant" => vec![
            alg::build(f.p, f.a, f.b, tx, &xs, m, "symmetric"),
            diagonal(&direct),
        ],
        "hybrid-eight" => (0..8)
            .map(|bucket| {
                let mut c = direct.clone();
                c.encoding = "hybrid-eight".into();
                c.equations[0] = membership(
                    c.names.len(),
                    0,
                    &xs.iter()
                        .enumerate()
                        .filter(|(i, _)| i % 8 == bucket)
                        .map(|(_, x)| *x)
                        .collect::<Vec<_>>(),
                    f.p,
                );
                c
            })
            .collect(),
        _ => return Err("unsupported decomposition encoding".into()),
    };
    let mut tuples = BTreeMap::<Vec<u64>, u64>::new();
    let mut branches = vec![];
    let mut complete = true;
    let mut invalid_lifts = 0u64;
    for c in cases {
        let remaining = budget_ms
            .saturating_sub(started.elapsed().as_millis() as u64)
            .max(1);
        let options = f4::F4Options::new(f4::Ordering::Grevlex, 32)
            .with_budget(Duration::from_millis(remaining));
        let r = f4::solve(&c.equations, c.names.len(), c.p, &options);
        let solutions = match &r.verdict {
            f4::Verdict::Solutions(s) => Some(s.clone()),
            f4::Verdict::Inconsistent => Some(vec![]),
            f4::Verdict::Undetermined => None,
        };
        complete &= solutions.is_some() && !r.timed_out;
        if let Some(solutions) = &solutions {
            for s in solutions {
                assert!(c.equations.iter().all(|q| f4::eval(q, s, c.p) == 0));
                let mut t = if c.encoding == "symmetric" {
                    let disc = (s[0] * s[0] + f.p - 4 * s[1] % f.p) % f.p;
                    let root = sqrt(disc, f.p).ok_or("pair discriminant did not split")?;
                    let half = inv(2, f.p);
                    let mut pair = [
                        (s[0] + root) % f.p * half % f.p,
                        (s[0] + f.p - root) % f.p * half % f.p,
                    ];
                    pair.sort();
                    assert_ne!(pair[0], pair[1]);
                    let mut t = pair.to_vec();
                    t.extend_from_slice(&s[2..m]);
                    t
                } else if c.encoding == "invariant-diagonal" {
                    let mut t = vec![s[0], s[0]];
                    t.extend_from_slice(&s[1..m - 1]);
                    t
                } else {
                    s[..m].to_vec()
                };
                t[..2].sort();
                assert!(t.iter().all(|x| xs.contains(x)));
                if let Some(mask) = signed(e, &t, &points, target) {
                    tuples.entry(t).or_insert(mask);
                } else {
                    invalid_lifts += 1;
                }
            }
        }
        branches.push(json!({"encoding":c.encoding,"names":c.names,"solutions":solutions,"timed_out":r.timed_out,"solver_field_multiplications_row_reduction":r.field_ops,"f4_runs":r.f4_runs,"max_rows":r.max_rows,"max_cols":r.max_cols,"candidates_rejected":r.candidates_rejected}));
    }
    // For m=3 the only missing chain chart is P1+P2=O; enumerate and charge it.
    let mut exceptional_candidates = 0;
    if m == 3 && xs.contains(&tx) {
        for x in &xs {
            let t = vec![*x, *x, tx];
            exceptional_candidates += 1;
            if let Some(mask) = signed(e, &t, &points, target) {
                tuples.entry(t).or_insert(mask);
            }
        }
    }
    let witnesses=tuples.iter().map(|(t,mask)|json!({"x":t,"y":t.iter().map(|x|points[x].unwrap().1.0).collect::<Vec<_>>(),"sign_mask":mask})).collect::<Vec<_>>();
    let canonical = tuples.keys().cloned().collect::<Vec<_>>();
    write_json(
        out,
        &json!({"schema":"twist.decomposition-worker/v1","workload":task["workload"],"arm":arm,"base":xs,"target":target,"branches":branches,"canonical_projected_tuples":canonical,"signed_witnesses":witnesses,"exceptional_candidates_enumerated":exceptional_candidates,"invalid_signed_lifts":invalid_lifts,"complete_projection":complete,"construction_roots_signs_dedup_and_source_verification_included":true,"complete_IC_cost":false,"wall_time_admitted":false}),
    )
}
fn suffix(p: &Path, s: &str) -> PathBuf {
    PathBuf::from(format!("{}.{s}", p.display()))
}
fn bytes_below(root: &Path) -> u64 {
    fs::read_dir(root)
        .unwrap()
        .map(|e| {
            let p = e.unwrap().path();
            let m = fs::symlink_metadata(&p).unwrap();
            if m.is_dir() {
                bytes_below(&p)
            } else {
                m.len()
            }
        })
        .sum()
}
fn bounded(
    program: &Path,
    args: &[String],
    stem: &Path,
    budget_ms: u64,
) -> Result<(i32, Option<i32>)> {
    let root = "/workspace/shared/twist-tools-20261023/cas/root/usr";
    let mut cmd = Command::new("/bin/sh");
    cmd.args([
        "-c",
        "ulimit -c 0; ulimit -v 4194304; ulimit -f 65536; exec \"$@\"",
        "e26-bounded",
        "/usr/bin/timeout",
        "--signal=TERM",
        "--kill-after=1s",
        &format!("{:.3}s", budget_ms as f64 / 1000.0),
    ])
    .arg(program)
    .args(args)
    .env("OMP_NUM_THREADS", "1")
    .env("OPENBLAS_NUM_THREADS", "1")
    .env("RAYON_NUM_THREADS", "1")
    .env("F4_FP_GM", "1")
    .env(
        "LD_LIBRARY_PATH",
        format!("{root}/lib/x86_64-linux-gnu:{root}/lib/x86_64-linux-gnu/singular"),
    )
    .env("SINGULARPATH", format!("{root}/share/singular/LIB"))
    .env(
        "SINGULAR_PROCS_DIR",
        format!("{root}/lib/x86_64-linux-gnu/singular/MOD"),
    )
    .stdout(Stdio::from(fs::File::create(suffix(stem, "stdout"))?))
    .stderr(Stdio::from(fs::File::create(suffix(stem, "stderr"))?));
    let status = cmd.status()?;
    Ok((status.code().unwrap_or(-1), status.signal()))
}
fn render(c: &alg::Case, qs: &[f4::Poly]) -> String {
    if qs.is_empty() {
        "0".into()
    } else {
        qs.iter()
            .map(|q| solver::cas_render(q, &c.names, c.p))
            .collect::<Vec<_>>()
            .join(",")
    }
}
fn certificate(job: &Job, basis: &[f4::Poly], stem: &Path, budget_ms: u64) -> Result<Value> {
    let c = &job.case;
    let started = Instant::now();
    let msinput = suffix(stem, "ms");
    let msbasis = suffix(stem, "msolve.gb");
    let msstem = suffix(stem, "msolve");
    fs::write(
        &msinput,
        format!(
            "{}\n{}\n{}\n",
            c.names.join(","),
            c.p,
            c.equations
                .iter()
                .map(|q| solver::cas_render(q, &c.names, c.p))
                .collect::<Vec<_>>()
                .join(",\n")
        ),
    )?;
    let (mscode, mssignal) = bounded(
        Path::new("/workspace/shared/twist-tools-20261023/cas/root/usr/bin/msolve"),
        &[
            "-f".into(),
            msinput.display().to_string(),
            "-o".into(),
            msbasis.display().to_string(),
            "-g".into(),
            "2".into(),
            "-q".into(),
            "0".into(),
            "-t".into(),
            "1".into(),
            "-v".into(),
            "0".into(),
        ],
        &msstem,
        (budget_ms / 2).max(25),
    )?;
    let msparsed = if mscode == 0 {
        fs::read_to_string(&msbasis)
            .ok()
            .and_then(|s| solver::parse_basis(&s, c).ok())
    } else {
        None
    };
    let mut text=format!("ring rr={},({}),dp;\nideal I={};\nideal B={};\nideal G=std(I);\nideal LB=lead(B);ideal LG=lead(G);int good=1;\nif(size(reduce(B,G))!=0){{good=0;}}\nif(size(reduce(I,B))!=0){{good=0;}}\nif(size(reduce(LB,LG))!=0){{good=0;}}\nif(size(reduce(LG,LB))!=0){{good=0;}}\nprint(\"CERT=\"+string(good));print(\"VDIM=\"+string(vdim(G)));print(\"EXPECTED={}\");\n",c.p,c.names.join(","),render(c,&c.equations),render(c,basis),job.expected);
    if let Some(ms) = &msparsed {
        text.push_str(&format!("ideal M={};ideal LM=lead(M);int mgood=1;if(size(reduce(M,G))!=0){{mgood=0;}}if(size(reduce(I,M))!=0){{mgood=0;}}if(size(reduce(LM,LG))!=0){{mgood=0;}}if(size(reduce(LG,LM))!=0){{mgood=0;}}print(\"MSCERT=\"+string(mgood));\n",render(c,ms)));
    }
    text.push_str("print(\"E26_CERTIFICATE_DONE\");quit;\n");
    let script = suffix(stem, "sing");
    fs::write(&script, text)?;
    let (code, signal) = bounded(
        Path::new("/workspace/shared/twist-tools-20261023/cas/root/usr/bin/Singular"),
        &["-q".into(), script.display().to_string()],
        stem,
        budget_ms
            .saturating_sub(started.elapsed().as_millis() as u64)
            .max(1),
    )?;
    let stdout = fs::read_to_string(suffix(stem, "stdout"))?;
    let stderr = fs::read_to_string(suffix(stem, "stderr"))?;
    let dimension = stdout
        .lines()
        .find_map(|s| s.strip_prefix("VDIM="))
        .and_then(|s| s.trim().parse::<u64>().ok());
    let accepted = code == 0
        && stdout.lines().any(|s| s == "CERT=1")
        && stdout.contains("E26_CERTIFICATE_DONE")
        && dimension == Some(job.expected)
        && !stdout.contains("?")
        && stderr.trim().is_empty();
    Ok(
        json!({"accepted":accepted,"exit_code":code,"signal":signal,"expected_vdim":job.expected,"observed_vdim":dimension,"input_sha256":digest(&fs::read(script)?),"stdout_sha256":digest(stdout.as_bytes()),"stderr_sha256":digest(stderr.as_bytes()),"msolve_q0_exit_code":mscode,"msolve_signal":mssignal,"msolve_basis_parsed":msparsed.is_some(),"msolve_basis_independently_certified":accepted&&stdout.lines().any(|s|s=="MSCERT=1"),"msolve_input_sha256":digest(&fs::read(msinput)?),"msolve_output_sha256":if msbasis.exists(){Some(digest(&fs::read(msbasis)?))}else{None}}),
    )
}

pub fn panel(input: &Path, out: &Path) -> Result<Vec<Value>> {
    let mut jobs: Vec<Job> = serde_json::from_slice(&fs::read(input.join("JOBS.json"))?)?;
    if let Some(first) = jobs
        .iter()
        .find(|j| {
            j.arm == "direct-chain" && j.case.p == 59 && j.case.xs.len() == 8 && j.case.arity == 2
        })
        .cloned()
    {
        let mut aa = first;
        aa.arm = "direct-aa".into();
        aa.id.push_str("-aa");
        jobs.push(aa);
    }
    let workload_order: Vec<Value> =
        serde_json::from_slice(&fs::read(input.join("WORKLOADS.json"))?)?;
    let arm_order = [
        "direct-aa",
        "direct-chain",
        "ordinary-prefix-order",
        "frobenius-block",
        "factored-control",
        "bits-control",
        "hybrid-eight",
        "first-pair-invariant",
    ];
    jobs.sort_by_key(|j| {
        (
            workload_order
                .iter()
                .position(|w| w["workload"] == j.workload)
                .unwrap(),
            arm_order.iter().position(|a| *a == j.arm).unwrap(),
            j.branch,
        )
    });
    fs::create_dir_all(out.join("cells"))?;
    let exe = std::env::current_exe()?;
    let mut rows = vec![];
    let start = Instant::now();
    let mut groups = vec![];
    for job in &jobs {
        let key = (job.workload.clone(), job.arm.clone());
        if !groups.contains(&key) {
            groups.push(key);
        }
    }
    for (workload, arm) in &groups {
        let cell = Instant::now();
        let mut statuses = vec![];
        let mut ops = 0u64;
        for job in jobs
            .iter()
            .filter(|j| &j.workload == workload && &j.arm == arm)
        {
            let remaining = 30000u64.saturating_sub(cell.elapsed().as_millis() as u64);
            if start.elapsed().as_secs() >= 1800 || remaining < 50 || bytes_below(out) >= (1 << 30)
            {
                statuses.push(json!({"branch":job.branch,"status":"not-started-budget-exhausted"}));
                continue;
            }
            let stem = out.join("cells").join(&job.id);
            let job_file = suffix(&stem, "input.json");
            write_json(&job_file, job)?;
            let result_file = suffix(&stem, "native.json");
            let args = vec![
                "followup".into(),
                "block-worker".into(),
                job_file.display().to_string(),
                result_file.display().to_string(),
                remaining.saturating_sub(25).to_string(),
            ];
            let (code, signal) = bounded(&exe, &args, &suffix(&stem, "native"), remaining)?;
            let result: Option<Value> = fs::read(&result_file)
                .ok()
                .and_then(|bytes| serde_json::from_slice(&bytes).ok());
            if let Some(r) = &result {
                ops += r["field_multiplications_row_reduction"]
                    .as_u64()
                    .unwrap_or(0);
            }
            let ready = code == 0
                && result.as_ref().is_some_and(|r| {
                    r["timed_out"] == false
                        && r["pairs_above_bound"] == 0
                        && r["staircase_at_stop"].is_null()
                });
            statuses.push(json!({"branch":job.branch,"job":job.id,"exit_code":code,"signal":signal,"native_complete_claim":ready,"status":if ready{"native-finished-awaiting-independent-certificate"}else{"native-incomplete"},"expected_vdim":job.expected,"input_sha256":digest(&fs::read(job_file)?),"result_sha256":if result_file.exists(){Some(digest(&fs::read(result_file)?))}else{None}}));
        }
        let row = json!({"workload":workload,"arm":arm,"branches":statuses,"native_row_reduction_multiplications":ops,"complete_cost":false,"performance_claim":false});
        write_json(
            out.join("cells")
                .join(format!("{workload}-{arm}-CELL.json")),
            &row,
        )?;
        rows.push(row);
        println!(
            "{}",
            json!({"stage":"native","cells":rows.len(),"requested_cells":groups.len(),"global_cap_reached":start.elapsed().as_secs()>=1800})
        );
    }
    let check_start = Instant::now();
    for row in &mut rows {
        let cell = Instant::now();
        let workload = row["workload"].as_str().unwrap().to_string();
        let arm = row["arm"].as_str().unwrap().to_string();
        let mut all = true;
        for branch in row["branches"].as_array_mut().unwrap() {
            if branch["native_complete_claim"] != true {
                all = false;
                continue;
            }
            let remaining = 30000u64.saturating_sub(cell.elapsed().as_millis() as u64);
            if check_start.elapsed().as_secs() >= 1800
                || remaining < 50
                || bytes_below(out) >= (1 << 30)
            {
                branch["certificate"] =
                    json!({"accepted":false,"status":"not-started-budget-exhausted"});
                all = false;
                continue;
            }
            let job = jobs
                .iter()
                .find(|j| j.id == branch["job"].as_str().unwrap())
                .unwrap();
            let stem = out.join("cells").join(&job.id);
            let r: Value = serde_json::from_slice(&fs::read(suffix(&stem, "native.json"))?)?;
            let basis: Vec<f4::Poly> = serde_json::from_value(r["basis"].clone())?;
            let cert = certificate(job, &basis, &suffix(&stem, "certificate"), remaining)?;
            all &= cert["accepted"] == true;
            branch["certificate"] = cert;
        }
        row["all_branches_independently_certified"] = json!(all);
        row["status"] = json!(if all {
            "certified-complete-affine-branches"
        } else {
            "incomplete"
        });
        write_json(
            out.join("cells")
                .join(format!("{workload}-{arm}-CELL.json")),
            row,
        )?;
        println!(
            "{}",
            json!({"stage":"certificate","workload":workload,"arm":arm,"complete":all})
        );
    }
    write_json(out.join("PANEL.json"), &rows)?;
    let workloads: Vec<Value> = serde_json::from_slice(&fs::read(input.join("WORKLOADS.json"))?)?;
    let mut counts = BTreeMap::<String, (u64, u64)>::new();
    for row in &rows {
        let v = counts
            .entry(row["arm"].as_str().unwrap().into())
            .or_default();
        v.0 += 1;
        if row["all_branches_independently_certified"] == true {
            v.1 += 1;
        }
    }
    let mut comparisons = vec![];
    for workload in &workloads {
        let id = workload["workload"].as_str().unwrap();
        let matched = rows
            .iter()
            .filter(|r| r["workload"] == id)
            .collect::<Vec<_>>();
        let baseline = matched
            .iter()
            .filter(|r| {
                [
                    "direct-chain",
                    "factored-control",
                    "bits-control",
                    "ordinary-prefix-order",
                ]
                .contains(&r["arm"].as_str().unwrap())
                    && r["all_branches_independently_certified"] == true
            })
            .min_by_key(|r| r["native_row_reduction_multiplications"].as_u64().unwrap());
        for arm in [
            "direct-chain",
            "frobenius-block",
            "hybrid-eight",
            "first-pair-invariant",
        ] {
            let row = matched.iter().find(|r| r["arm"] == arm).unwrap();
            let paired = baseline.is_some() && row["all_branches_independently_certified"] == true;
            let candidate = row["native_row_reduction_multiplications"]
                .as_u64()
                .unwrap();
            let reference =
                baseline.map(|r| r["native_row_reduction_multiplications"].as_u64().unwrap());
            comparisons.push(json!({"workload":id,"arm":arm,"paired_complete_affine_certificates":paired,"candidate_row_reduction_multiplications":candidate,"baseline_row_reduction_multiplications":reference,"baseline_arm":baseline.map(|r|&r["arm"]),"candidate_to_baseline_ratio":if paired&&reference!=Some(0){reference.map(|r|candidate as f64/r as f64)}else{None},"canonical_relation_tuples":workload["canonical_relation_tuples"],"complete_construction_exception_lifting_and_solver_cost":false,"speedup_established":false}));
        }
    }
    let aa = rows.iter().find(|r| r["arm"] == "direct-aa");
    let aa_equal = aa.and_then(|a| {
        rows.iter()
            .find(|r| r["workload"] == a["workload"] && r["arm"] == "direct-chain")
            .map(|r| {
                a["all_branches_independently_certified"] == true
                    && r["all_branches_independently_certified"] == true
                    && a["native_row_reduction_multiplications"]
                        == r["native_row_reduction_multiplications"]
            })
    });
    write_json(
        out.join("ASSESSMENT.json"),
        &json!({"schema":"twist.block-assessment/v1","workloads_constructed":workloads.len(),"cells":rows.len(),"by_arm":counts.iter().map(|(arm,(requested,certified))|json!({"arm":arm,"requested_cells":requested,"certified_cells":certified,"incomplete_cells":requested-certified})).collect::<Vec<_>>(),"same_input_AA_equal_certified_counts":aa_equal,"comparisons":comparisons,"raw_bytes":bytes_below(out),"no_final_E28_labels_opened":true,"full_E26_complete":false,"E27_complete_cost":false,"novelty_established":false,"speedup_established":false}),
    )?;
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frobenius_expansion_at_zero_and_diagonal_preserve_every_input() {
        let xs = [1, 3, 5, 8];
        let c = alg::build(59, 1, 13, 7, &xs, 3, "product");
        let f = frobenius(&c, 2);
        for x in 0..59 {
            let v = [x, 3, 5, 8];
            let fv = lift_frobenius(&v, 3);
            let re = f
                .equations
                .iter()
                .map(|q| f4::eval(q, &fv, 59))
                .filter(|x| *x != 0)
                .collect::<Vec<_>>();
            let orig = c
                .equations
                .iter()
                .map(|q| f4::eval(q, &v, 59))
                .filter(|x| *x != 0)
                .collect::<Vec<_>>();
            assert_eq!(re, orig);
        }
        let d = diagonal(&c);
        for x in 0..59 {
            let v = [x, x, 5, 8];
            let w = [x, 5, 8];
            for (i, q) in c.equations.iter().enumerate() {
                assert_eq!(f4::eval(q, &v, 59), f4::eval(&d.equations[i], &w, 59));
            }
        }
    }
    #[test]
    fn helper_partition_is_disjoint_and_complete() {
        let xs = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        for x in 0..59 {
            let count = (0..8)
                .filter(|h| {
                    let b = xs
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| i % 8 == *h)
                        .map(|(_, x)| *x)
                        .collect::<Vec<_>>();
                    f4::eval(&membership(1, 0, &b, 59), &[x], 59) == 0
                })
                .count();
            assert_eq!(count, usize::from(xs.contains(&x)));
        }
    }
}
