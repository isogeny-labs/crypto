//! Matched native full-GB worker and frozen-fixture/CAS exporter.
//! Measurement execution requires the backend decision in SPEED_PROTOCOL.md.
use relation_degree_trace::{
    cheap, counters, reference, reset_counters,
    workload::{generate, Fixture},
    Poly, Row, Term,
};
use serde_json::{json, Value};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    time::Instant,
};
fn terms(f: &Poly) -> Vec<Term> {
    f.iter()
        .map(|(m, c)| Term {
            exponents: m.clone(),
            coefficient: *c as i64,
        })
        .collect()
}
fn expression(f: &Poly, names: &[String]) -> String {
    f.iter()
        .map(|(m, c)| {
            let mut s = c.to_string();
            for (e, name) in m.iter().zip(names) {
                if *e > 0 {
                    s += &format!("*{name}^{e}");
                }
            }
            s
        })
        .collect::<Vec<_>>()
        .join("+")
}
fn singular(
    system: &relation_degree_trace::System,
    inputs: &[Poly],
    algorithm: &str,
    repetitions: usize,
) -> String {
    let p = system.prime;
    let coefficient = if p < 2u64.pow(31) {
        p.to_string()
    } else {
        format!("(integer,{p})")
    };
    let mut code=format!("ring r={coefficient},({}),dp;\noption(redSB);\nsystem(\"--ticks-per-sec\",1000000);\nideal I={};\nideal G;\nint j;\nint start=timer;\nfor(j=0;j<{repetitions};j++){{G={algorithm}(I);}}\nprint(\"CPU_MICROSECONDS:\"+string(timer-start));\nint i;poly f;\n",system.variables.join(","),inputs.iter().map(|f|expression(f,&system.variables)).collect::<Vec<_>>().join(","));
    code+="for(i=1;i<=size(G);i++){print(\"POLY\");f=G[i];while(f!=0){print(\"TERM:\"+string(leadcoef(f))+\":\"+string(leadexp(f)));f=f-lead(f);}print(\"ENDPOLY\");}\nprint(\"DONE\");\nquit;\n";
    code
}
fn export(directory: &Path, projection: bool) -> Result<(), Box<dyn Error>> {
    if directory.exists() {
        return Err("fixture directory already exists".into());
    }
    fs::create_dir_all(directory)?;
    let mut manifest = Vec::new();
    for p in [2147483647, 9007199254740881] {
        for n in [2, 3, 4, 6] {
            for seed in [101, 307, 911] {
                let fixture = generate(p, n, seed)?;
                let f = fixture.system.decode()?;
                let augmented = if projection {
                    relation_degree_trace::projection::augment(&f, n, p).0
                } else {
                    cheap::augment(&f, n, p).0
                };
                let root = directory.join(&fixture.id);
                fs::create_dir(&root)?;
                fs::write(
                    root.join("fixture.json"),
                    serde_json::to_string_pretty(&fixture)? + "\n",
                )?;
                fs::write(
                    root.join("baseline.ms"),
                    format!(
                        "{}\n{p}\n{}\n",
                        fixture.system.variables.join(","),
                        f.iter()
                            .map(|f| expression(f, &fixture.system.variables))
                            .collect::<Vec<_>>()
                            .join(",\n")
                    ),
                )?;
                fs::write(
                    root.join("candidate.ms"),
                    format!(
                        "{}\n{p}\n{}\n",
                        fixture.system.variables.join(","),
                        augmented
                            .iter()
                            .map(|f| expression(f, &fixture.system.variables))
                            .collect::<Vec<_>>()
                            .join(",\n")
                    ),
                )?;
                for (name, polys) in [("baseline", &f), ("candidate", &augmented)] {
                    for algorithm in ["std", "slimgb"] {
                        fs::write(
                            root.join(format!("{name}-{algorithm}.sing")),
                            singular(&fixture.system, polys, algorithm, 100),
                        )?;
                    }
                }
                manifest.push(json!({"id":fixture.id,"prime":p,"variables":n,"seed":seed,"fixture":format!("{}/fixture.json",fixture.id),
            "msolve_supported_by_documented_prime_limit":p<2u64.pow(52),"singular_std_coefficient_ring":if p<2u64.pow(31){"prime_field"}else{"Z/pZ"}}));
            }
        }
    }
    fs::write(
        directory.join("manifest.json"),
        serde_json::to_string_pretty(
            &json!({"schema":"relation-speed-fixtures/v1","cases":manifest,"witness_access":"validation only; no witness is passed to preprocessing or solvers"}),
        )? + "\n",
    )?;
    Ok(())
}
fn execute(path: &Path, arm: &str, repetitions: usize) -> Result<Value, Box<dyn Error>> {
    let fixture: Fixture = serde_json::from_str(&fs::read_to_string(path)?)?;
    let system = &fixture.system;
    let inputs = system.decode()?;
    let n = system.variables.len();
    let p = system.prime;
    if arm != "baseline"
        && arm != "candidate"
        && arm != "projection"
        && arm != "interreduce"
        && arm != "projection-interreduce"
        && arm != "criteria"
        && arm != "projection-criteria"
        && arm != "quadratic-projection-criteria"
        && arm != "quotient"
    {
        return Err("unsupported solver arm".into());
    }
    let (mut preprocessing_ns, mut solving_ns) = (0u128, 0u128);
    let mut charged_ns = 0u128;
    let mut preprocess_products = 0u64;
    let mut solve_products = 0u64;
    let mut basis: Vec<Row> = Vec::new();
    let mut prior: Option<Vec<Poly>> = None;
    let mut pairs = 0;
    let mut projection_info = Vec::new();
    let mut quotient_info = None;
    for _ in 0..repetitions {
        let charged_start = Instant::now();
        reset_counters();
        let start = Instant::now();
        let augmented = if arm == "quadratic-projection-criteria" {
            let (a, info) = relation_degree_trace::projection::augment_quadratic(
                std::hint::black_box(&inputs),
                n,
                p,
            );
            pairs = info.len();
            projection_info = info;
            a
        } else if arm == "projection"
            || arm == "projection-interreduce"
            || arm == "projection-criteria"
        {
            let (a, info) =
                relation_degree_trace::projection::augment(std::hint::black_box(&inputs), n, p);
            pairs = info.len();
            projection_info = info;
            a
        } else if arm == "candidate" {
            let (a, c) = cheap::augment(std::hint::black_box(&inputs), n, p);
            pairs = c.len();
            a
        } else {
            inputs.clone()
        };
        preprocessing_ns += start.elapsed().as_nanos();
        preprocess_products += counters().modular_products;
        reset_counters();
        let start = Instant::now();
        basis = if arm == "quotient" {
            let (result, info) =
                relation_degree_trace::quotient::solve(std::hint::black_box(&augmented), n, p)?;
            quotient_info = Some(info);
            result
        } else if arm == "criteria"
            || arm == "projection-criteria"
            || arm == "quadratic-projection-criteria"
        {
            relation_degree_trace::reference_criteria(std::hint::black_box(&augmented), n, p)?
        } else if arm == "interreduce" || arm == "projection-interreduce" {
            relation_degree_trace::reference_interreduce(std::hint::black_box(&augmented), n, p)?
        } else {
            reference(std::hint::black_box(&augmented), n, p)?
        };
        drop(augmented);
        solving_ns += start.elapsed().as_nanos();
        charged_ns += charged_start.elapsed().as_nanos();
        solve_products += counters().modular_products;
        let result: Vec<Poly> = basis.iter().map(|r| r.poly.clone()).collect();
        if let Some(last) = &prior {
            if *last != result {
                return Err("nondeterministic basis".into());
            }
        }
        prior = Some(result);
    }
    Ok(
        json!({"schema":"relation-speed-worker/v1","id":fixture.id,"arm":arm,"prime":p,"variables":n,"repetitions":repetitions,
        "preprocessing_ns":preprocessing_ns,"solve_and_internal_certificate_checks_ns":solving_ns,"charged_ns":charged_ns,
        "modular_products_preprocess":preprocess_products,"modular_products_solve":solve_products,"counter_build":cfg!(feature="count-field-ops"),
        "counter_scope":"modular products and nontrivial inversions; coefficient updates cover sparse axpy only, excluding dense quotient/linear-algebra writes; not full operations or GAE",
        "preprocessed_components":pairs,"projection_vertices":projection_info,"quotient_info":quotient_info,"quotient_phase_scope":"quotient arm includes projection construction, coordinate construction, change of order and all checks in the solve interval","basis":basis.iter().map(|r|terms(&r.poly)).collect::<Vec<_>>(),
        "completion":"Buchberger criterion, original-input membership and all input normal forms checked by the producer's native reference path",
        "excluded":"fixture construction, input parsing and primality validation, process launch, inter-repetition consistency checks; external replay is separately charged by the driver",
        "claim_status":"unverified producer observation"}),
    )
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let mut prepare = None;
    let mut worker = None;
    let mut arm = "baseline".to_string();
    let mut reps = 1;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--prepare-projection" => {
                arm = "projection".to_string();
                prepare = Some(PathBuf::from(
                    args.next().ok_or("missing fixture directory")?,
                ));
            }
            "--prepare" => {
                prepare = Some(PathBuf::from(
                    args.next().ok_or("missing fixture directory")?,
                ))
            }
            "--worker" => worker = Some(PathBuf::from(args.next().ok_or("missing fixture path")?)),
            "--arm" => arm = args.next().ok_or("missing arm")?,
            "--repetitions" => reps = args.next().ok_or("missing repetition count")?.parse()?,
            "--help" => {
                println!("compare --prepare NEW_DIR | --worker FIXTURE.json --arm baseline|candidate [--repetitions N]");
                return Ok(());
            }
            _ => return Err(format!("unknown option {arg}").into()),
        }
    }
    if reps == 0 || reps > 10000 {
        return Err("repetitions must be 1..10000".into());
    }
    if let Some(path) = prepare {
        export(&path, arm == "projection")?;
    } else if let Some(path) = worker {
        println!("{}", execute(&path, &arm, reps)?);
    } else {
        return Err("choose --prepare or --worker".into());
    }
    Ok(())
}
