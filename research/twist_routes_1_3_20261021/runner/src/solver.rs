use crate::{
    digest, f4_fp as f4, toy_field::*, trace::Base, twist_ic_algebra as alg, write_json, Fixture,
    Result,
};
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
};
pub(crate) fn cas_render(poly: &f4::Poly, names: &[String], p: u64) -> String {
    if poly.is_empty() {
        return "0".into();
    }
    poly.iter()
        .map(|(powers, c)| {
            let mut factors = vec![(c % p).to_string()];
            for (i, e) in powers.iter().enumerate() {
                if *e != 0 {
                    factors.push(if *e == 1 {
                        names[i].clone()
                    } else {
                        format!("{}^{e}", names[i])
                    });
                }
            }
            factors.join("*")
        })
        .collect::<Vec<_>>()
        .join("+")
}
fn write_compact(path: &Path, value: &impl serde::Serialize) -> Result<()> {
    fs::write(path, serde_json::to_vec(value)?)?;
    Ok(())
}
fn add_x(e: Quad, a: Option<u64>, b: u64) -> Vec<Option<u64>> {
    let Some(a) = a else { return vec![Some(b)] };
    let p = e.p;
    let lead = (a + p - b) % p * (a + p - b) % p;
    let mid = (2 * ((a + b) % p * ((a * b + e.a) % p) % p + 2 * e.b) % p) % p;
    let tail = (((a * b + p - e.a) % p).pow(2) % p + p - 4 * e.b % p * ((a + b) % p) % p) % p;
    if lead == 0 {
        return vec![None, Some(tail * inv(mid, p) % p)];
    }
    let disc = (mid * mid + p - 4 * lead % p * tail % p) % p;
    let sd = sqrt(disc, p).expect("rational sum discriminant");
    let den = inv(2 * lead % p, p);
    let mut out = vec![
        Some((mid + sd) % p * den % p),
        Some((mid + p - sd) % p * den % p),
    ];
    out.sort();
    out.dedup();
    out
}
pub(crate) fn target(f: &Fixture, seed: u64) -> P {
    let mut s = 202610210000 + seed + f.p * 65537;
    loop {
        let x = crate::next(&mut s) % f.p;
        if let Some(y) = sqrt(f.curve().rhs(Q(x, 0)).0, f.p) {
            if y != 0 {
                let y = if crate::next(&mut s) & 1 == 0 {
                    y
                } else {
                    f.p - y
                };
                return Some((Q(x, 0), Q(y, 0)));
            }
        }
    }
}
pub(crate) fn trajectories(e: Quad, xs: &[u64], r: u64) -> Vec<Vec<Option<u64>>> {
    let mut v = vec![vec![Some(xs[0])]];
    for &i in &xs[1..] {
        let mut next = vec![];
        for path in &v {
            for x in add_x(e, *path.last().unwrap(), i) {
                let mut p = path.clone();
                p.push(x);
                next.push(p);
            }
        }
        v = next;
    }
    v.into_iter()
        .filter(|x| x.last() == Some(&Some(r)))
        .collect()
}
pub(crate) fn expand_values(case: &alg::Case, xs: &[u64], path: &[Option<u64>]) -> Option<Vec<u64>> {
    if path[1..path.len() - 1].iter().any(Option::is_none) {
        return None;
    }
    let mut v = if case.encoding == "symmetric" {
        if xs[0] >= xs[1] {
            return None;
        }
        let mut v = vec![(xs[0] + xs[1]) % case.p, xs[0] * xs[1] % case.p];
        v.extend_from_slice(&xs[2..]);
        v
    } else {
        xs.to_vec()
    };
    for x in &path[1..path.len() - 1] {
        v.push(x.unwrap());
    }
    if case.encoding == "chain" {
        for &x in xs {
            let mut product = 1;
            for &a in &case.xs[..case.xs.len() - 1] {
                product = product * (x + case.p - a) % case.p;
                v.push(product);
            }
        }
    }
    if case.encoding == "bits" {
        let bits = (usize::BITS - (case.xs.len() - 1).leading_zeros()) as usize;
        for &x in xs {
            let index = case.xs.iter().position(|a| *a == x).unwrap();
            v.push(index as u64);
            for j in 0..bits {
                v.push(((index >> j) & 1) as u64);
            }
        }
    }
    assert_eq!(v.len(), case.names.len());
    assert!(case.equations.iter().all(|f| f4::eval(f, &v, case.p) == 0));
    Some(v)
}
pub(crate) fn tuples(xs: &[u64], m: usize, mut f: impl FnMut(&[u64])) {
    fn rec(xs: &[u64], m: usize, v: &mut Vec<u64>, f: &mut impl FnMut(&[u64])) {
        if m == 0 {
            f(v);
            return;
        }
        for &x in xs {
            v.push(x);
            rec(xs, m - 1, v, f);
            v.pop();
        }
    }
    rec(xs, m, &mut vec![], &mut f);
}
fn projection(e: Quad, xs: &[u64], m: usize, rx: u64) -> (BTreeSet<Vec<u64>>, u64, u64) {
    let mut poly = BTreeSet::new();
    let mut group = BTreeSet::new();
    let mut affine = 0;
    let mut exceptional = 0;
    tuples(xs, m, |tuple| {
        let paths = trajectories(e, tuple, rx);
        if !paths.is_empty() {
            let mut canon = tuple.to_vec();
            canon[..2].sort();
            poly.insert(canon);
            affine += paths
                .iter()
                .filter(|p| p[1..p.len() - 1].iter().all(Option::is_some))
                .count() as u64;
            exceptional += paths
                .iter()
                .filter(|p| p[1..p.len() - 1].iter().any(Option::is_none))
                .count() as u64;
        }
        let mut points = BTreeSet::from([None]);
        for &x in tuple {
            let y = sqrt(e.rhs(Q(x, 0)).0, e.p).unwrap();
            let p = Some((Q(x, 0), Q(y, 0)));
            let mut next = BTreeSet::new();
            for &s in &points {
                next.insert(e.pa(s, p));
                next.insert(e.pa(s, e.pn(p)));
            }
            points = next;
        }
        if points.iter().any(|p| p.is_some_and(|(x, _)| x.0 == rx)) {
            let mut canon = tuple.to_vec();
            canon[..2].sort();
            group.insert(canon);
        }
    });
    assert_eq!(poly, group, "full projected relation sets differ");
    (poly, affine, exceptional)
}
fn worker(input: &Path, output: &Path, order: &str) -> Result<()> {
    let c: alg::Case = serde_json::from_slice(&fs::read(input)?)?;
    let ord = if order == "lex" {
        f4::Ordering::Lex
    } else {
        f4::Ordering::Grevlex
    };
    let equations: Vec<_> = c
        .equations
        .iter()
        .map(|p| f4::normalise(p, c.p, ord))
        .collect();
    let opts = f4::F4Options::new(ord, 24)
        .with_budget(std::time::Duration::from_millis(500))
        .capturing_run_log();
    let report = f4::f4(&equations, c.names.len(), c.p, &opts);
    let log = f4::take_run_log();
    let mut evaluated = 0u64;
    let mut invalid = 0u64;
    tuples(&c.xs, c.arity, |xs| {
        for path in trajectories(Quad::new(c.p, c.a, c.b), xs, c.target_x) {
            if let Some(v) = expand_values(&c, xs, &path) {
                evaluated += 1;
                if report.basis.iter().any(|p| f4::eval(p, &v, c.p) != 0) {
                    invalid += 1;
                }
            }
        }
    });
    assert_eq!(invalid, 0, "Groebner basis removed a genuine assignment");
    let complete =
        !report.timed_out && report.pairs_above_bound == 0 && report.staircase_at_stop.is_none();
    write_compact(
        output,
        &json!({"schema":"twist.f4-cell/v1","complete_by_engine":complete,"timed_out":report.timed_out,"pairs_above_bound":report.pairs_above_bound,"field_multiplications_in_row_reduction":report.field_ops,"max_rows":report.max_rows,"max_columns":report.max_cols,"solving_degree_max":report.solving_degree_max,"degree_reached":report.degree_reached,"all_expected_assignments_retained":true,"expected_affine_assignments":evaluated,"basis":report.basis,"steps":log.steps.iter().map(|s|json!({"degree":s.degree,"rows":s.rows,"all_rows":s.all_rows,"field_multiplications":s.ops})).collect::<Vec<_>>(),"wall_time_admitted":false}),
    )?;
    Ok(())
}
fn parse_poly(text: &str, names: &[String], p: u64) -> Result<f4::Poly> {
    let s = text.replace(' ', "").replace('-', "+-");
    let mut terms = vec![];
    for term in s.split('+').filter(|s| !s.is_empty()) {
        let mut coefficient = 1u64;
        let mut powers = vec![0; names.len()];
        for factor in term.split('*') {
            if let Some(i) = names.iter().position(|n| n == factor) {
                powers[i] += 1;
            } else if let Some((name, power)) = factor.split_once('^') {
                let i = names
                    .iter()
                    .position(|n| n == name)
                    .ok_or("unknown msolve variable")?;
                powers[i] += power.parse::<u32>()?;
            } else {
                let c = factor.parse::<i128>()?;
                coefficient = coefficient * (c.rem_euclid(p as i128) as u64) % p;
            }
        }
        terms.push((powers, coefficient));
    }
    Ok(f4::normalise(&terms, p, f4::Ordering::Grevlex))
}
pub(crate) fn parse_basis(text: &str, c: &alg::Case) -> Result<Vec<f4::Poly>> {
    let start = text.find('[').ok_or("missing msolve basis")?;
    let end = text.rfind(']').ok_or("missing closing basis")?;
    text[start + 1..end]
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|p| parse_poly(p.trim(), &c.names, c.p))
        .collect()
}
fn bounded(
    program: &str,
    args: &[String],
    stdout: &Path,
    stderr: &Path,
    seconds: &str,
    envs: &[(&str, &str)],
) -> Result<std::process::ExitStatus> {
    let mut cmd = Command::new("/bin/sh");
    cmd.args([
        "-c",
        "ulimit -c 0; ulimit -v 4194304; exec \"$@\"",
        "twist-bounded",
        "/usr/bin/timeout",
        "--signal=TERM",
        "--kill-after=0.2s",
        seconds,
        program,
    ])
    .args(args)
    .stdout(Stdio::from(fs::File::create(stdout)?))
    .stderr(Stdio::from(fs::File::create(stderr)?));
    for (k, v) in envs {
        cmd.env(k, v);
    }
    Ok(cmd.status()?)
}
#[derive(Clone)]
struct Job {
    case: alg::Case,
    stem: PathBuf,
    order: String,
    fixture: String,
    seed: u64,
}
fn execute(j: &Job, exe: &Path) -> Result<Value> {
    let c = &j.case;
    let input = j.stem.with_extension("json");
    write_json(&input, c)?;
    let native = j.stem.with_extension("native.json");
    let status = bounded(
        exe.to_str().unwrap(),
        &[
            "f4-worker".into(),
            input.display().to_string(),
            native.display().to_string(),
            j.order.clone(),
        ],
        &j.stem.with_extension("native.stdout"),
        &j.stem.with_extension("native.stderr"),
        "1s",
        &[("RAYON_NUM_THREADS", "1"), ("F4_FP_GM", "1")],
    )?;
    let native_result = if status.success() {
        Some(serde_json::from_slice::<Value>(&fs::read(&native)?)?)
    } else {
        None
    };
    let ms_input = j.stem.with_extension("ms");
    let rendered: Vec<_> = c
        .equations
        .iter()
        .map(|p| cas_render(p, &c.names, c.p))
        .collect();
    fs::write(
        &ms_input,
        format!("{}\n{}\n{}\n", c.names.join(","), c.p, rendered.join(",\n")),
    )?;
    let mut references = vec![];
    let mut bases = vec![];
    for signatures in [0, 1] {
        let basis = j.stem.with_extension(format!("msolve-q{signatures}.gb"));
        let code = bounded(
            "/tmp/twist-cas/usr/bin/msolve",
            &[
                "-f".into(),
                ms_input.display().to_string(),
                "-o".into(),
                basis.display().to_string(),
                "-g".into(),
                "2".into(),
                "-q".into(),
                signatures.to_string(),
                "-t".into(),
                "1".into(),
                "-v".into(),
                "0".into(),
            ],
            &j.stem
                .with_extension(format!("msolve-q{signatures}.stdout")),
            &j.stem
                .with_extension(format!("msolve-q{signatures}.stderr")),
            "1s",
            &[("LD_LIBRARY_PATH", "/tmp/twist-cas/usr/lib/x86_64-linux-gnu")],
        )?;
        let parsed = if code.success() {
            fs::read_to_string(&basis)
                .ok()
                .and_then(|s| parse_basis(&s, c).ok())
        } else {
            None
        };
        references.push(json!({"engine":format!("msolve-0.7.5-q{signatures}"),"exit_code":code.code(),"basis_parsed":parsed.is_some()}));
        if let Some(b) = parsed {
            bases.push((format!("M{signatures}"), b));
        }
    }
    if let Some(n) = &native_result {
        bases.push(("N".into(), serde_json::from_value(n["basis"].clone())?));
    }
    let mut program = format!(
        "ring r={},({}),{};\nideal I={};\nideal G=std(I);\nprint(\"DIM=\"+string(vdim(G)));\n",
        c.p,
        c.names.join(","),
        if j.order == "lex" { "lp" } else { "dp" },
        rendered.join(",")
    );
    for (name, basis) in &bases {
        let polys = if basis.is_empty() {
            "0".into()
        } else {
            basis
                .iter()
                .map(|p| cas_render(p, &c.names, c.p))
                .collect::<Vec<_>>()
                .join(",")
        };
        program.push_str(&format!("ideal {name}={polys};\nideal H{name}=std({name});\nprint(\"EQUAL_{name}=\"+string(size(reduce({name},G))==0 && size(reduce(I,H{name}))==0));\n"));
    }
    program.push_str("quit;\n");
    let sing = j.stem.with_extension("sing");
    fs::write(&sing, program)?;
    let sing_out = j.stem.with_extension("singular.stdout");
    let sing_code=bounded("/tmp/twist-cas/usr/bin/Singular",&["-q".into(),sing.display().to_string()],&sing_out,&j.stem.with_extension("singular.stderr"),"1s",&[("LD_LIBRARY_PATH","/tmp/twist-cas/usr/lib/x86_64-linux-gnu:/tmp/twist-cas/usr/lib/x86_64-linux-gnu/singular"),("SINGULARPATH","/tmp/twist-cas/usr/share/singular/LIB"),("SINGULAR_PROCS_DIR","/tmp/twist-cas/usr/lib/x86_64-linux-gnu/singular/MOD")])?;
    let sing_text = fs::read_to_string(&sing_out)?;
    let dimension = sing_text
        .lines()
        .find_map(|l| l.strip_prefix("DIM=").and_then(|s| s.parse::<u64>().ok()));
    let mut expected = 0;
    tuples(&c.xs, c.arity, |xs| {
        for path in trajectories(Quad::new(c.p, c.a, c.b), xs, c.target_x) {
            if expand_values(c, xs, &path).is_some() {
                expected += 1;
            }
        }
    });
    if let Some(dim) = dimension {
        assert_eq!(
            dim, expected,
            "independent CAS dimension differs from exact affine census"
        );
    }
    let equality: Vec<_> = bases
        .iter()
        .map(|(name, _)| {
            (
                name.clone(),
                sing_text.lines().any(|l| l == format!("EQUAL_{name}=1")),
            )
        })
        .collect();
    Ok(
        json!({"fixture":j.fixture,"seed":j.seed,"arity":c.arity,"B":c.xs.len(),"encoding":c.encoding,"variable_order":j.order,"variables":c.names.len(),"equations":c.equations.len(),"native_exit_code":status.code(),"native_result":native_result.map(|mut n|{n.as_object_mut().unwrap().remove("basis");n}),"external":references,"singular_exit_code":sing_code.code(),"singular_dimension":dimension,"expected_affine_assignments":expected,"independent_ideal_equalities":equality,"wall_time_admitted":false,"total_solver_cost_or_speedup":null,"raw_member_stem":j.stem.file_name().unwrap().to_string_lossy()}),
    )
}
fn panel(manifest: &Path, bases: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("solver panel requires fresh output".into());
    }
    fs::create_dir_all(out.join("cases"))?;
    let fixtures: Vec<Fixture> = serde_json::from_value(
        serde_json::from_slice::<Value>(&fs::read(manifest)?)?["fixtures"].clone(),
    )?;
    let bs: Vec<Base> = serde_json::from_slice(&fs::read(bases)?)?;
    let mut equivalences = vec![];
    let mut jobs = vec![];
    for f in fixtures
        .iter()
        .filter(|f| prime(f.order) && f.order % 2 == 1)
    {
        let base = bs
            .iter()
            .find(|b| b.fixture == f.id && b.family == "coordinate-box" && b.h == 8)
            .unwrap();
        assert!(base.points.len() >= 8);
        let xs: Vec<_> = base.points[..8].iter().map(|p| p.unwrap().0 .0).collect();
        for seed in 0..8 {
            let r = target(f, seed);
            let rx = r.unwrap().0 .0;
            for m in [2, 3, 4] {
                let (set, affine, exceptional) = projection(f.curve(), &xs, m, rx);
                equivalences.push(json!({"fixture":f.id,"seed":seed,"target":r,"B":8,"H":8,"arity":m,"canonical_projected_relation_tuples":set.len(),"relation_tuple_sha256":digest(&serde_json::to_vec(&set)?),"ordered_affine_assignments":affine,"exceptional_identity_prefix_paths":exceptional,"polynomial_and_independent_group_projection_equal":true,"diagonal_and_identity_branches_retained":true}));
                for encoding in ["product", "chain", "bits", "symmetric"] {
                    let case = alg::build(f.p, f.a, f.b, rx, &xs, m, encoding);
                    for order in ["grevlex", "lex"] {
                        let id = format!("{}-s{seed}-m{m}-{encoding}-{order}", f.id);
                        jobs.push(Job {
                            case: case.clone(),
                            stem: out.join("cases").join(id),
                            order: order.into(),
                            fixture: f.id.clone(),
                            seed,
                        });
                    }
                }
            }
        }
    }
    write_json(out.join("E07-projected-equivalence.json"), &equivalences)?;
    let aa: Vec<_> = jobs
        .iter()
        .filter(|j| {
            j.seed == 0 && j.case.arity == 2 && j.case.encoding == "product" && j.order == "grevlex"
        })
        .cloned()
        .map(|mut j| {
            let name = format!("{}-aa", j.stem.file_name().unwrap().to_string_lossy());
            j.stem = j.stem.with_file_name(name);
            j
        })
        .collect();
    jobs.extend(aa);
    let next = AtomicUsize::new(0);
    let records = Mutex::new(vec![]);
    let exe = std::env::current_exe()?;
    std::thread::scope(|scope| {
        for _ in 0..3 {
            scope.spawn(||loop{let i=next.fetch_add(1,Ordering::SeqCst);if i>=jobs.len(){break;}let result=execute(&jobs[i],&exe).unwrap_or_else(|e|json!({"fixture":jobs[i].fixture,"seed":jobs[i].seed,"arity":jobs[i].case.arity,"encoding":jobs[i].case.encoding,"variable_order":jobs[i].order,"status":"verification-or-execution-error","error":e.to_string()}));write_json(jobs[i].stem.with_extension("result.json"),&result).unwrap();records.lock().unwrap().push((i,result));if i%24==0{println!("{}",json!({"completed_at_least":i,"total":jobs.len(),"stage":"solver"}));}});
        }
    });
    let mut rows = records.into_inner().unwrap();
    rows.sort_by_key(|x| x.0);
    write_json(
        out.join("E08-solver-panel.json"),
        &rows.into_iter().map(|x| x.1).collect::<Vec<_>>(),
    )?;
    println!(
        "{}",
        json!({"status":"finished-solver-panel","cases":jobs.len(),"output":out})
    );
    Ok(())
}
pub fn main(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("f4-worker") => worker(
            Path::new(args.get(1).ok_or("input")?),
            Path::new(args.get(2).ok_or("output")?),
            args.get(3).ok_or("order")?,
        ),
        Some("solver") => panel(
            Path::new(args.get(1).ok_or("fixtures")?),
            Path::new(args.get(2).ok_or("bases")?),
            Path::new(args.get(3).ok_or("fresh output")?),
        ),
        _ => Err("solver usage".into()),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn external_serialization_is_ascii_and_preserves_all_models() {
        for encoding in ["product", "chain", "bits", "symmetric"] {
            let c = alg::build(59, 1, 13, 7, &[1, 3, 5, 8], 4, encoding);
            for polynomial in &c.equations {
                let text = cas_render(polynomial, &c.names, c.p);
                assert!(text.is_ascii());
                assert_eq!(
                    parse_poly(&text, &c.names, c.p).unwrap(),
                    f4::normalise(polynomial, c.p, f4::Ordering::Grevlex)
                );
            }
        }
    }
    #[test]
    fn x_only_law_matches_independent_full_group() {
        let e = Quad::new(59, 1, 13);
        let ps = e.rational();
        for p in ps.iter().flatten() {
            for q in ps.iter().flatten() {
                let a: BTreeSet<_> = add_x(e, Some(p.0 .0), q.0 .0).into_iter().collect();
                let b: BTreeSet<_> = [e.pa(Some(*p), Some(*q)), e.pa(Some(*p), e.pn(Some(*q)))]
                    .iter()
                    .map(|p| p.map(|p| p.0 .0))
                    .collect();
                assert_eq!(a, b);
            }
        }
    }
}
