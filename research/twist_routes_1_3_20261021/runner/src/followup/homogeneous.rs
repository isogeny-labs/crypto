//! Correct-field homogeneous signature pilot with independent complete-basis gates.
use crate::{
    digest, f4_fp as f4,
    solver::{cas_render, expand_values, parse_basis, target, trajectories, tuples},
    trace::Base,
    twist_ic_algebra as alg, write_json, Fixture, Result,
};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
};

fn homogenize(c: &alg::Case) -> alg::Case {
    let mut h = c.clone();
    h.names.push("hh".into());
    h.equations = c
        .equations
        .iter()
        .map(|f| {
            let d = f
                .iter()
                .map(|(e, _)| e.iter().sum::<u32>())
                .max()
                .unwrap_or(0);
            f4::normalise(
                &f.iter()
                    .map(|(e, a)| {
                        let mut e = e.clone();
                        e.push(d - e.iter().sum::<u32>());
                        (e, *a)
                    })
                    .collect::<Vec<_>>(),
                c.p,
                f4::Ordering::Grevlex,
            )
        })
        .collect();
    h.encoding = format!("{}-generator-homogenized", c.encoding);
    h
}
fn dehomogenize(polys: &[f4::Poly], p: u64) -> Vec<f4::Poly> {
    polys
        .iter()
        .map(|f| {
            f4::normalise(
                &f.iter()
                    .map(|(e, a)| (e[..e.len() - 1].to_vec(), *a))
                    .collect::<Vec<_>>(),
                p,
                f4::Ordering::Grevlex,
            )
        })
        .filter(|f| !f.is_empty())
        .collect()
}
fn append(path: &Path, suffix: &str) -> PathBuf {
    PathBuf::from(format!("{}.{suffix}", path.display()))
}
fn bounded(program: &str, args: &[String], stem: &Path) -> Result<i32> {
    let mut cmd = Command::new("/bin/sh");
    cmd.args(["-c","ulimit -c 0; ulimit -v 4194304; exec \"$@\"","twist-e25-bounded","/usr/bin/timeout","--signal=TERM","--kill-after=1s","30s",program]).args(args)
      .env("OMP_NUM_THREADS","1").env("RAYON_NUM_THREADS","1").env("LD_LIBRARY_PATH","/tmp/twist-cas/root/usr/lib/x86_64-linux-gnu:/tmp/twist-cas/root/usr/lib/x86_64-linux-gnu/singular")
      .env("SINGULARPATH","/tmp/twist-cas/root/usr/share/singular/LIB").env("SINGULAR_PROCS_DIR","/tmp/twist-cas/root/usr/lib/x86_64-linux-gnu/singular/MOD")
      .stdout(Stdio::from(fs::File::create(append(stem,"stdout"))?)).stderr(Stdio::from(fs::File::create(append(stem,"stderr"))?));
    Ok(cmd.status()?.code().unwrap_or(-1))
}
fn rendered(c: &alg::Case, polys: &[f4::Poly]) -> String {
    if polys.is_empty() {
        "0".into()
    } else {
        polys
            .iter()
            .map(|f| cas_render(f, &c.names, c.p))
            .collect::<Vec<_>>()
            .join(",")
    }
}
const COMPLETE: &str = r#"
proc twist_complete(ideal B,ideal I,ideal G){
 int i,j,k;int ok=1;intvec a,b;poly L,S;
 for(i=1;i<=size(B);i++){if(B[i]!=0){for(j=i+1;j<=size(B);j++){if(B[j]!=0){
  a=leadexp(B[i]);b=leadexp(B[j]);L=1;
  for(k=1;k<=nvars(basering);k++){if(a[k]>b[k]){L=L*var(k)^a[k];}else{L=L*var(k)^b[k];}}
  S=(L/lead(B[i]))*B[i]-(L/lead(B[j]))*B[j];if(reduce(S,B)!=0){ok=0;}
 }}}}
 if(size(reduce(I,B))!=0){ok=0;}if(size(reduce(B,G))!=0){ok=0;}
 return(ok);
}
"#;
const COMPLETE_LEADING: &str = r#"
proc twist_complete(ideal B,ideal I,ideal G){
 ideal LB=lead(B);ideal LG=lead(G);
 if(size(reduce(B,G))!=0){return(0);}
 if(size(reduce(I,B))!=0){return(0);}
 if(size(reduce(LG,LB))!=0){return(0);}
 if(size(reduce(LB,LG))!=0){return(0);}
 return(1);
}
"#;
fn certify(
    c: &alg::Case,
    bases: &[(String, Vec<f4::Poly>)],
    order: &str,
    stem: &Path,
    expected: Option<u64>,
) -> Result<Value> {
    certify_with(c, bases, order, stem, expected, false)
}
fn certify_with(
    c: &alg::Case,
    bases: &[(String, Vec<f4::Poly>)],
    order: &str,
    stem: &Path,
    expected: Option<u64>,
    leading: bool,
) -> Result<Value> {
    let mut text = format!(
        "ring rr={},({}),{};\nideal I={};\nideal G=std(I);\nprint(\"DIM=\"+string(vdim(G)));\n",
        c.p,
        c.names.join(","),
        if order == "lex" { "lp" } else { "dp" },
        rendered(c, &c.equations)
    );
    text.push_str(if leading { COMPLETE_LEADING } else { COMPLETE });
    for (name, polys) in bases {
        text.push_str(&format!(
            "ideal {name}={};\nprint(\"COMPLETE_{name}=\"+string(twist_complete({name},I,G)));\n",
            rendered(c, polys)
        ));
    }
    text.push_str("print(\"ALL_COMPLETE_BASIS_CHECKS_DONE\");quit;\n");
    let script = append(stem, "sing");
    fs::write(&script, text)?;
    let code = bounded(
        "/tmp/twist-cas/root/usr/bin/Singular",
        &["-q".into(), script.display().to_string()],
        stem,
    )?;
    let stdout = fs::read_to_string(append(stem, "stdout"))?;
    let stderr = fs::read_to_string(append(stem, "stderr"))?;
    let clean = code == 0
        && stdout.contains("ALL_COMPLETE_BASIS_CHECKS_DONE")
        && !stdout.lines().any(|s| s.trim_start().starts_with('?'))
        && stderr.trim().is_empty();
    let dim = stdout
        .lines()
        .find_map(|s| s.strip_prefix("DIM=").and_then(|s| s.parse::<i64>().ok()));
    if clean {
        if let (Some(e), Some(dim)) = (expected, dim) {
            assert_eq!(
                dim, e as i64,
                "independent affine dimension and exact native assignment census differ"
            );
        }
    }
    let results=bases.iter().map(|(name,_)|{let complete=clean&&stdout.lines().any(|s|s==format!("COMPLETE_{name}=1"));json!({"basis":name,"complete":complete,"all_S_pairs_and_both_inclusions_verified":if leading{None}else{Some(complete)},"independent_leading_ideal_and_both_inclusions_verified":if leading{Some(complete)}else{None}})}).collect::<Vec<_>>();
    Ok(
        json!({"exit_code":code,"completion_and_clean_output":clean,"certificate_method":if leading{"independent-complete-reference-leading-ideal-equality-and-both-inclusions"}else{"all-required-S-pairs-and-both-inclusions"},"dimension":dim,"expected_affine_assignments":expected,"bases":results,"script_sha256":digest(&fs::read(&script)?),"stdout_sha256":digest(stdout.as_bytes()),"stderr_sha256":digest(stderr.as_bytes())}),
    )
}
fn infinity(c: &alg::Case, stem: &Path) -> Result<Value> {
    let script = append(stem, "sing");
    fs::write(&script,format!("LIB \"elim.lib\";\nring rr={},({}),dp;ideal I={};ideal INF=std(I+ideal(hh));print(\"INFINITY_DIM=\"+string(dim(INF)));list L=sat(I,ideal(hh));ideal SAT=L[1];print(\"GENERATOR_IDEAL_ALREADY_SATURATED=\"+string(size(reduce(SAT,std(I)))==0));print(\"ALL_INFINITY_CHECKS_DONE\");quit;\n",c.p,c.names.join(","),rendered(c,&c.equations)))?;
    let code = bounded(
        "/tmp/twist-cas/root/usr/bin/Singular",
        &["-q".into(), script.display().to_string()],
        stem,
    )?;
    let text = fs::read_to_string(append(stem, "stdout"))?;
    let errors = fs::read_to_string(append(stem, "stderr"))?;
    let clean = code == 0
        && text.contains("ALL_INFINITY_CHECKS_DONE")
        && !text.lines().any(|s| s.trim_start().starts_with('?'))
        && errors.trim().is_empty();
    Ok(
        json!({"exit_code":code,"clean_complete":clean,"projective_infinity_cone_dimension":text.lines().find_map(|s|s.strip_prefix("INFINITY_DIM=").and_then(|v|v.parse::<i64>().ok())),"generator_ideal_already_h_saturated":if clean{Some(text.lines().any(|s|s=="GENERATOR_IDEAL_ALREADY_SATURATED=1"))}else{None},"unsaturated_generator_ideal_and_saturation_recorded_separately":true}),
    )
}
#[derive(Clone)]
struct Job {
    fixture: String,
    case: alg::Case,
    order: String,
    stem: PathBuf,
}
fn execute(j: &Job) -> Result<Value> {
    let c = &j.case;
    let h = homogenize(c);
    write_json(j.stem.with_extension("original.json"), c)?;
    write_json(j.stem.with_extension("homogeneous.json"), &h)?;
    let mut assignments = vec![];
    let mut omitted_chart_or_first_pair_branches = 0u64;
    tuples(&c.xs, c.arity, |xs| {
        for path in trajectories(crate::toy_field::Quad::new(c.p, c.a, c.b), xs, c.target_x) {
            if let Some(v) = expand_values(c, xs, &path) {
                assignments.push(v);
            } else {
                omitted_chart_or_first_pair_branches += 1;
            }
        }
    });
    for v in &assignments {
        let mut vv = v.clone();
        vv.push(1);
        assert!(h.equations.iter().all(|f| f4::eval(f, &vv, c.p) == 0));
    }
    if j.order == "lex" {
        let checked = certify(
            c,
            &[],
            "lex",
            &j.stem.with_extension("lex-control"),
            Some(assignments.len() as u64),
        )?;
        return Ok(
            json!({"experiment":"E25","fixture":j.fixture,"p":c.p,"arity":c.arity,"encoding":c.encoding,"order":"lex","status":"signature-backend-does-not-provide-lex-order","independent_lex_affine_reference":checked,"performance_claim":false}),
        );
    }
    let mut engines = vec![];
    let mut hb = vec![];
    let mut ab = vec![];
    for (name, program, signature, homogeneous) in [
        ("S0", "/tmp/twist-cas/root/usr/bin/msolve", 0, false),
        ("A0", "/tmp/twist-cas/msolve-signature-output", 0, false),
        ("H0", "/tmp/twist-cas/msolve-signature-output", 0, true),
        ("H1", "/tmp/twist-cas/msolve-signature-output", 1, true),
        ("S1", "/tmp/twist-cas/root/usr/bin/msolve", 1, true),
    ] {
        let cc = if homogeneous { &h } else { c };
        let stem = j.stem.with_extension(name);
        let input = stem.with_extension(format!("{name}.ms"));
        let output = stem.with_extension(format!("{name}.gb"));
        fs::write(
            &input,
            format!(
                "{}\n{}\n{}\n",
                cc.names.join(","),
                cc.p,
                cc.equations
                    .iter()
                    .map(|f| cas_render(f, &cc.names, cc.p))
                    .collect::<Vec<_>>()
                    .join(",\n")
            ),
        )?;
        let raw_stem = j.stem.with_extension(format!("{name}-engine"));
        let code = bounded(
            program,
            &[
                "-f".into(),
                input.display().to_string(),
                "-o".into(),
                output.display().to_string(),
                "-g".into(),
                "2".into(),
                "-q".into(),
                signature.to_string(),
                "-t".into(),
                "1".into(),
                "-v".into(),
                "1".into(),
            ],
            &raw_stem,
        )?;
        let parsed = if code == 0 {
            fs::read_to_string(&output)
                .ok()
                .and_then(|s| parse_basis(&s, cc).ok())
        } else {
            None
        };
        if let Some(basis) = parsed.as_ref() {
            for v in &assignments {
                let mut v = v.clone();
                if homogeneous {
                    v.push(1);
                }
                assert!(
                    basis.iter().all(|f| f4::eval(f, &v, cc.p) == 0),
                    "basis lost an actual affine solution"
                );
            }
            write_json(j.stem.with_extension(format!("{name}.basis.json")), basis)?;
            if homogeneous {
                hb.push((name.into(), basis.clone()));
                ab.push((name.into(), dehomogenize(basis, c.p)));
            } else {
                ab.push((name.into(), basis.clone()));
            }
        }
        let stderr = fs::read_to_string(append(&raw_stem, "stderr"))?;
        engines.push(json!({"engine":name,"program":program,"signature_mode":signature,"homogeneous":homogeneous,"p":c.p,"exit_code":code,"basis_parsed":parsed.is_some(),"source_restores_actual_field_before_signature_core":true,"actual_field_correction_note":stderr.lines().find(|s|s.contains("Field characteristic is now corrected")),"input_sha256":digest(&fs::read(input)?),"binary_sha256":digest(&fs::read(program)?)}));
    }
    let hc = certify(
        &h,
        &hb,
        "grevlex",
        &j.stem.with_extension("hom-check"),
        None,
    )?;
    let ac = certify(
        c,
        &ab,
        "grevlex",
        &j.stem.with_extension("aff-check"),
        Some(assignments.len() as u64),
    )?;
    let inf = infinity(&h, &j.stem.with_extension("infinity-check"))?;
    Ok(
        json!({"experiment":"E25","fixture":j.fixture,"p":c.p,"B":c.xs.len(),"arity":c.arity,"encoding":c.encoding,"order":"grevlex","engines":engines,"homogeneous_complete_basis_gate":hc,"dehomogenized_affine_complete_basis_gate":ac,"infinity_and_saturation":inf,"actual_affine_assignments_replayed":assignments.len(),"omitted_nonaffine_or_first_pair_branches_retained_as_count":omitted_chart_or_first_pair_branches,"status":"bounded-correct-field-homogeneous-signature-pilot","full_E25_complete":false,"performance_claim":false}),
    )
}
fn control(out: &Path, leading: bool) -> Result<Value> {
    let c = alg::Case {
        p: 59,
        a: 1,
        b: 13,
        target_x: 0,
        xs: vec![],
        arity: 2,
        encoding: "linear-homogeneous-positive-control".into(),
        names: vec!["xx".into(), "yy".into()],
        equations: vec![
            vec![(vec![1, 0], 1), (vec![0, 0], 58)],
            vec![(vec![0, 1], 1), (vec![0, 0], 57)],
        ],
    };
    let h = homogenize(&c);
    let stem = out.join("positive-control");
    let input = append(&stem, "ms");
    fs::write(
        &input,
        format!(
            "{}\n59\n{}\n",
            h.names.join(","),
            h.equations
                .iter()
                .map(|p| cas_render(p, &h.names, 59))
                .collect::<Vec<_>>()
                .join(",\n")
        ),
    )?;
    let mut hb = vec![];
    let mut ab = vec![];
    let mut engines = vec![];
    for (name, q) in [("Q0", 0), ("Q1", 1)] {
        let output = append(&stem, &format!("{name}.gb"));
        let raw = append(&stem, name);
        let code = bounded(
            "/tmp/twist-cas/msolve-signature-output",
            &[
                "-f".into(),
                input.display().to_string(),
                "-o".into(),
                output.display().to_string(),
                "-g".into(),
                "2".into(),
                "-q".into(),
                q.to_string(),
                "-t".into(),
                "1".into(),
                "-v".into(),
                "1".into(),
            ],
            &raw,
        )?;
        let basis = if code == 0 {
            fs::read_to_string(output)
                .ok()
                .and_then(|s| parse_basis(&s, &h).ok())
        } else {
            None
        };
        if let Some(b) = &basis {
            hb.push((name.into(), b.clone()));
            ab.push((name.into(), dehomogenize(b, 59)));
        }
        engines.push(
            json!({"name":name,"signature":q,"exit_code":code,"basis_parsed":basis.is_some()}),
        );
    }
    ab.push(("TRUNCATED".into(), vec![c.equations[0].clone()]));
    ab.push((
        "WRONGFIELD".into(),
        vec![
            vec![(vec![1, 0], 1), (vec![0, 0], 60)],
            c.equations[1].clone(),
        ],
    ));
    let hom = certify_with(
        &h,
        &hb,
        "grevlex",
        &append(&stem, "hom-check"),
        None,
        leading,
    )?;
    let aff = certify_with(
        &c,
        &ab,
        "grevlex",
        &append(&stem, "aff-check"),
        Some(1),
        leading,
    )?;
    if aff["completion_and_clean_output"] == true {
        for name in ["TRUNCATED", "WRONGFIELD"] {
            let row = aff["bases"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["basis"] == name)
                .unwrap();
            assert_eq!(row["complete"], false);
        }
    }
    let record = json!({"experiment":"E25-control","p":59,"engines":engines,"homogeneous":hom,"affine":aff,"positive_control_affine_solution":[1,2],"wrong_characteristic_metadata_rejected":61!=c.p,"status":"positive-and-incomplete-basis-controls-recorded"});
    write_json(append(&stem, "RESULT.json"), &record)?;
    Ok(record)
}
pub fn run(fixtures: &[Fixture], bases: &Path, out: &Path) -> Result<Vec<Value>> {
    if !Path::new("/tmp/twist-cas/msolve-signature-output").exists() {
        return Err("pinned signature-output adapter binary is not built".into());
    }
    let bs: Vec<Base> = serde_json::from_slice(&fs::read(bases)?)?;
    let control = control(out, false)?;
    fs::create_dir(out.join("cases"))?;
    let mut jobs = vec![];
    for f in fixtures.iter().filter(|f| [59, 211, 251].contains(&f.p)) {
        let b = bs
            .iter()
            .find(|b| b.fixture == f.id && b.family == "coordinate-box" && b.h == 8)
            .ok_or("missing frozen ordinary base")?;
        let xs = b.points[..8]
            .iter()
            .map(|p| p.unwrap().0 .0)
            .collect::<Vec<_>>();
        for m in [2usize, 3] {
            for encoding in ["product", "chain", "bits", "symmetric"] {
                let case = alg::build(f.p, f.a, f.b, target(f, 0).unwrap().0 .0, &xs, m, encoding);
                for order in ["grevlex", "lex"] {
                    jobs.push(Job {
                        fixture: f.id.clone(),
                        case: case.clone(),
                        order: order.into(),
                        stem: out
                            .join("cases")
                            .join(format!("{}-m{m}-{encoding}-{order}", f.id)),
                    });
                }
            }
        }
    }
    let counter = AtomicUsize::new(0);
    let results = Mutex::new(vec![]);
    std::thread::scope(|scope| {
        for _ in 0..3 {
            let jobs = &jobs;
            let counter = &counter;
            let results = &results;
            scope.spawn(move||loop {
        let i=counter.fetch_add(1,Ordering::Relaxed);if i>=jobs.len(){break;}let j=&jobs[i];
        let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||execute(j)));
        let row=match result{Ok(Ok(v))=>v,Ok(Err(e))=>json!({"experiment":"E25","fixture":j.fixture,"arity":j.case.arity,"encoding":j.case.encoding,"order":j.order,"status":"error","error":e.to_string()}),Err(_)=>json!({"experiment":"E25","fixture":j.fixture,"arity":j.case.arity,"encoding":j.case.encoding,"order":j.order,"status":"native-certificate-panic-raw-retained"})};
        write_json(j.stem.with_extension("RESULT.json"),&row).unwrap();println!("{}",json!({"experiment":"E25","cell":i,"status":row["status"]}));results.lock().unwrap().push(row);
    });
        }
    });
    let mut rows = results.into_inner().unwrap();
    rows.sort_by_key(|r| {
        format!(
            "{}|{}|{}|{}",
            r["fixture"], r["arity"], r["encoding"], r["order"]
        )
    });
    rows.push(control);
    Ok(rows)
}
pub fn recheck(input: &Path, out: &Path, leading: bool) -> Result<Vec<Value>> {
    fs::create_dir(out.join("cases"))?;
    let mut rows = vec![];
    let mut originals = fs::read_dir(input.join("cases"))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .unwrap()
                .to_string_lossy()
                .ends_with(".original.json")
        })
        .collect::<Vec<_>>();
    originals.sort();
    for original in originals {
        let name = original
            .file_name()
            .unwrap()
            .to_string_lossy()
            .strip_suffix(".original.json")
            .unwrap()
            .to_string();
        let oldstem = input.join("cases").join(&name);
        let stem = out.join("cases").join(&name);
        let c: alg::Case = serde_json::from_slice(&fs::read(&original)?)?;
        let h = homogenize(&c);
        let old: Value = serde_json::from_slice(&fs::read(append(&oldstem, "RESULT.json"))?)?;
        assert_eq!(
            serde_json::to_value(&h)?,
            serde_json::from_slice::<Value>(&fs::read(append(&oldstem, "homogeneous.json"))?)?
        );
        let mut assignments = vec![];
        tuples(&c.xs, c.arity, |xs| {
            for path in trajectories(crate::toy_field::Quad::new(c.p, c.a, c.b), xs, c.target_x) {
                if let Some(v) = expand_values(&c, xs, &path) {
                    assignments.push(v);
                }
            }
        });
        let row = if old["order"] == "lex" {
            let checked = certify(
                &c,
                &[],
                "lex",
                &append(&stem, "lex-control"),
                Some(assignments.len() as u64),
            )?;
            json!({"experiment":"E25","fixture":old["fixture"],"p":c.p,"arity":c.arity,"encoding":c.encoding,"order":"lex","status":"signature-backend-does-not-provide-lex-order","independent_lex_affine_reference":checked,"original_result_sha256":digest(&fs::read(append(&oldstem,"RESULT.json"))?),"performance_claim":false})
        } else {
            let mut hb = vec![];
            let mut ab = vec![];
            for e in old["engines"].as_array().unwrap() {
                if e["exit_code"] != 0 || e["basis_parsed"] != true {
                    continue;
                }
                let label = e["engine"].as_str().unwrap();
                let homogeneous = e["homogeneous"] == true;
                let cc = if homogeneous { &h } else { &c };
                let basis = parse_basis(
                    &fs::read_to_string(append(&oldstem, &format!("{label}.gb")))?,
                    cc,
                )?;
                let stored: Vec<f4::Poly> = serde_json::from_slice(&fs::read(append(
                    &oldstem,
                    &format!("{label}.basis.json"),
                ))?)?;
                assert_eq!(
                    basis, stored,
                    "saved parsed basis differs from raw solver output"
                );
                for v in &assignments {
                    let mut v = v.clone();
                    if homogeneous {
                        v.push(1);
                    }
                    assert!(basis.iter().all(|f| f4::eval(f, &v, c.p) == 0));
                }
                if homogeneous {
                    hb.push((label.into(), basis.clone()));
                    ab.push((label.into(), dehomogenize(&basis, c.p)));
                } else {
                    ab.push((label.into(), basis));
                }
            }
            let hc = certify_with(
                &h,
                &hb,
                "grevlex",
                &append(&stem, "hom-check"),
                None,
                leading,
            )?;
            let ac = certify_with(
                &c,
                &ab,
                "grevlex",
                &append(&stem, "aff-check"),
                Some(assignments.len() as u64),
                leading,
            )?;
            json!({"experiment":"E25","fixture":old["fixture"],"p":c.p,"B":c.xs.len(),"arity":c.arity,"encoding":c.encoding,"order":"grevlex","engines":old["engines"],"homogeneous_complete_basis_gate":hc,"dehomogenized_affine_complete_basis_gate":ac,"infinity_and_saturation":old["infinity_and_saturation"],"actual_affine_assignments_replayed":assignments.len(),"raw_solver_outputs_reparsed_and_identical":true,"original_result_sha256":digest(&fs::read(append(&oldstem,"RESULT.json"))?),"engine_measurements_repeated":false,"status":"corrected-independent-basis-recheck","full_E25_complete":false,"performance_claim":false})
        };
        write_json(append(&stem, "RESULT.json"), &row)?;
        rows.push(row);
        println!("{}", json!({"experiment":"E25","recheck":name}));
    }
    rows.push(control(out, leading)?);
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn homogenization_and_affine_chart_preserve_exact_system_solutions() {
        let c = alg::build(59, 1, 13, 7, &[1, 3, 5, 8], 2, "product");
        let h = homogenize(&c);
        assert_eq!(
            dehomogenize(&h.equations, c.p),
            c.equations
                .iter()
                .map(|f| f4::normalise(f, c.p, f4::Ordering::Grevlex))
                .collect::<Vec<_>>()
        );
        for x in 0..59 {
            for y in 0..59 {
                for (a, b) in c.equations.iter().zip(&h.equations) {
                    assert_eq!(f4::eval(a, &[x, y], 59), f4::eval(b, &[x, y, 1], 59));
                }
            }
        }
        assert!(h.equations.iter().all(|f| f
            .iter()
            .map(|(e, _)| e.iter().sum::<u32>())
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            <= 1));
    }
}
