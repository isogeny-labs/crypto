//! Native evidence aggregation and plots. Reads recorded outcomes only.
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs,
    path::Path,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn load(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn rows(v: &Value) -> &[Value] {
    v.as_array().expect("recorded row array")
}
fn num(v: &Value, key: &str) -> u64 {
    v[key].as_u64().unwrap_or(0)
}
fn count(v: &Value, test: impl Fn(&Value) -> bool) -> u64 {
    rows(v).iter().filter(|x| test(x)).count() as u64
}
fn text(v: &Value, key: &str) -> String {
    v[key].as_str().unwrap_or("missing").into()
}
fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
fn status_counts(v: &Value) -> BTreeMap<String, u64> {
    let mut m = BTreeMap::new();
    for x in rows(v) {
        *m.entry(text(x, "status")).or_default() += 1;
    }
    m
}
fn plot(out: &Path, title: &str, labels: &[String], series: &[(&str, Vec<u64>)]) -> Result<()> {
    let max = series
        .iter()
        .flat_map(|(_, v)| v)
        .copied()
        .max()
        .unwrap_or(1)
        .max(1) as f64;
    let colors = ["#176b9c", "#c17717", "#6550a4", "#35815a"];
    let row_height = series.len() * 19 + 13;
    let height = 110 + labels.len() * row_height;
    let mut s=format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\"{height}\" viewBox=\"0 0 1000 {height}\"><rect width=\"100%\" height=\"100%\" fill=\"white\"/><g font-family=\"DejaVu Sans,sans-serif\" fill=\"#172b3a\"><text x=\"20\" y=\"28\" font-size=\"17\">{}</text><text x=\"20\" y=\"50\" font-size=\"12\">Recorded counts; exact censuses or bounded run outcomes. No wall-time comparison.</text>",esc(title));
    let mut data = vec![];
    for (j, (name, _)) in series.iter().enumerate() {
        s+=&format!("<rect x=\"{}\" y=\"65\" width=\"12\" height=\"12\" fill=\"{}\"/><text x=\"{}\" y=\"76\" font-size=\"11\">{}</text>",20+j*240,colors[j%4],38+j*240,esc(name));
    }
    for (i, label) in labels.iter().enumerate() {
        let y = 101 + i * row_height;
        s += &format!(
            "<text x=\"20\" y=\"{}\" font-size=\"11\">{}</text>",
            y + 12,
            esc(label)
        );
        for (j, (name, values)) in series.iter().enumerate() {
            let v = values[i];
            let w = 650.0 * v as f64 / max;
            s+=&format!("<rect x=\"255\" y=\"{}\" width=\"{w:.2}\" height=\"14\" fill=\"{}\"/><text x=\"{:.2}\" y=\"{}\" font-size=\"11\">{v}</text>",y+j*19,colors[j%4],262.0+w,y+j*19+12);
            data.push(json!({"label":label,"series":name,"count":v}));
        }
    }
    s += "</g></svg>";
    fs::write(out.join("METRICS.svg"), s)?;
    fs::write(
        out.join("PLOT_DATA.json"),
        serde_json::to_vec_pretty(&data)?,
    )?;
    Ok(())
}
fn report(
    root: &Path,
    run: &str,
    source: &str,
    title: &str,
    metrics: &Value,
    notes: &str,
    labels: Vec<String>,
    series: Vec<(&str, Vec<u64>)>,
) -> Result<()> {
    let out = root.join(run);
    fs::create_dir(&out)?;
    fs::write(
        out.join("METRICS.json"),
        serde_json::to_vec_pretty(metrics)?,
    )?;
    plot(&out, title, &labels, &series)?;
    let raw = match run {
        "fixture-preparation" => "../../../execution-v1/".to_string(),
        "pairing-preparation" => "../../../execution-v1/pairing-fixture/".to_string(),
        "lean-verification" => "../../../lean/receipts/".to_string(),
        _ => format!("../../../execution-v1/runs/{run}/"),
    };
    let mut md=format!("# {title}\n\nRun `{run}`; source freeze `{source}`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. Workloads use public synthetic fixtures or public-parameter checks. No wall-clock or full-cost improvement is admitted.\n\nRaw outputs: [run directory]({raw}). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.\n\n## Recorded observations\n\n| Metric | Count / recorded value |\n|:--|--:|\n");
    for (k, v) in metrics.as_object().unwrap() {
        if !v.is_object() && !v.is_array() {
            md += &format!("| {} | {} |\n", k.replace('_', " "), v);
        }
    }
    md+=&format!("\n![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)\n\n{notes}\n\nThe chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).\n");
    fs::write(out.join("REPORT.md"), md)?;
    Ok(())
}
#[derive(Default, serde::Serialize)]
struct PipelineTotals {
    attempts: u64,
    verified: u64,
    errors: u64,
    unpriced_native_units: u64,
    missing_counts: BTreeSet<String>,
    unpriced_by_name: BTreeMap<String, u64>,
    sum_s_lower_bound: f64,
    sum_s_at_0_1_gae: f64,
    sum_s_at_1_gae: f64,
}
fn work_count(v: &Value, name: &str) -> Option<u64> {
    let base = name.strip_suffix("_uncharged").unwrap_or(name);
    if let Some(n) = v["counters"]
        .get(name)
        .or_else(|| v["counters"].get(base))
        .and_then(Value::as_u64)
    {
        return Some(n);
    }
    let mut found = false;
    let mut total = 0;
    for phase in v["phases"].as_array().into_iter().flatten() {
        if let Some(n) = phase["native"]
            .get(name)
            .or_else(|| phase["native"].get(base))
            .and_then(Value::as_u64)
        {
            found = true;
            total += n;
        }
    }
    found.then_some(total)
}
fn add_pipeline(study: &Path, out: &Path) -> Result<Value> {
    let run = "ecbench-20261022T0012Z";
    let dir = study.join("execution-v1/runs").join(run);
    let records: Vec<Value> = fs::read_to_string(dir.join("records.jsonl"))?
        .lines()
        .filter(|x| !x.trim().is_empty())
        .map(serde_json::from_str)
        .collect::<std::result::Result<_, _>>()?;
    let mut arms = BTreeMap::<String, PipelineTotals>::new();
    let mut curves = BTreeMap::<String, PipelineTotals>::new();
    let mut errors = BTreeMap::new();
    for v in &records {
        if v["warmup"] == true {
            continue;
        }
        let arm = text(v, "arm");
        let curve = text(&v["workload"]["curve"], "slug");
        for total in [
            arms.entry(arm.clone()).or_default(),
            curves.entry(format!("{curve},{arm}")).or_default(),
        ] {
            total.attempts += 1;
            if v["outcome"]["status"] != "verified" {
                total.errors += 1;
                continue;
            }
            total.verified += 1;
            let mut native = 0;
            for name in v["cost"]["unpriced"].as_array().unwrap() {
                let name = name.as_str().unwrap();
                match work_count(v, name) {
                    Some(n) => {
                        native += n;
                        *total.unpriced_by_name.entry(name.into()).or_default() += n;
                    }
                    None => {
                        total.missing_counts.insert(name.into());
                    }
                }
            }
            total.unpriced_native_units += native;
            let s = v["cost"]["s"].as_f64().unwrap();
            let r = v["workload"]["curve"]["r"].as_u64().unwrap() as f64;
            total.sum_s_lower_bound += s;
            total.sum_s_at_0_1_gae += s + 0.1 * native as f64 / r.sqrt();
            total.sum_s_at_1_gae += s + native as f64 / r.sqrt();
        }
        if v["outcome"]["status"] != "verified" {
            *errors.entry(text(&v["outcome"], "error")).or_insert(0u64) += 1;
        }
    }
    let audit = load(study.join("execution-v1/logs/E15-audit-v1.json"))?;
    assert_eq!(audit["ok"], true);
    let mut ratios = vec![];
    for t in [0, 1] {
        for c in [0, 1] {
            for s in [0, 1] {
                let candidate = format!("t{t}c{c}s{s}");
                let reference = format!("same-base-t{t}c{c}");
                let a = &arms[&reference];
                let b = &arms[&candidate];
                ratios.push(json!({"candidate":candidate,"same_base_reference":reference,"candidate_verified":b.verified,"reference_verified":a.verified,"both_complete_on_80_workloads":a.verified==80&&b.verified==80,"missing_counter_counts":!a.missing_counts.is_empty()||!b.missing_counts.is_empty(),"candidate_over_reference_at_0":b.sum_s_lower_bound/a.sum_s_lower_bound,"candidate_over_reference_at_0_1":b.sum_s_at_0_1_gae/a.sum_s_at_0_1_gae,"candidate_over_reference_at_1":b.sum_s_at_1_gae/a.sum_s_at_1_gae,"full_cost_ranking_admitted":false}));
            }
        }
    }
    let m = json!({"executions":records.len(),"verified":records.iter().filter(|v|v["outcome"]["status"]=="verified").count(),"errors":errors.values().sum::<u64>(),"wrong_answers":records.iter().filter(|v|v["outcome"]["status"]=="wrong_answer").count(),"audit_ok":true,"identical_replays":audit["replays"].as_array().unwrap().len(),"arms":arms,"by_curve_and_arm":curves,"error_reasons":errors,"sensitivity_ratios":ratios,"full_cost_ranking_admitted":false});
    let labels: Vec<_> = [0, 1]
        .iter()
        .flat_map(|t| {
            [0, 1]
                .iter()
                .flat_map(move |c| [0, 1].iter().map(move |s| format!("t{t}c{c}s{s}")))
        })
        .collect();
    report(out,run,"2bd6cd2e95e139fa67b2b41bea59b43ca217165b","Cold factorial IC pipeline: correctness and accounting limits",&m,
        "Eight public one-target workloads on each of ten registered curves, 24 arms, one L0 round and no warmup. All returned logarithms are point-verified. The 216 errors are retained empty trace/cover intersections. All 1,704 verified records replay identically. The field/call, F4, symbolic and extraction work is unpriced. The CSV applies 0/0.1/1 GAE to every recorded unpriced native unit on every arm, including rho; this is a sensitivity model, not calibrated full cost. No ranking or speedup is admitted. Native compare/table wall-admission labels under a requested L0 minimum are explicitly excluded by this protocol; no wall figure is a result. Other sizes, arities, independent-host replay and closed peak-memory/accounting remain open.",labels.clone(),vec![("verified targets",labels.iter().map(|x|arms[x].verified).collect()),("empty-base errors",labels.iter().map(|x|arms[x].errors).collect())])?;
    let mut csv=String::from("curve,arm,attempts,verified,errors,unpriced_native_units,missing_counts,S_sum_at_0,S_sum_at_0.1,S_sum_at_1\n");
    for (key, t) in &curves {
        csv += &format!(
            "{key},{},{},{},{},{},{:.9},{:.9},{:.9}\n",
            t.attempts,
            t.verified,
            t.errors,
            t.unpriced_native_units,
            t.missing_counts.len(),
            t.sum_s_lower_bound,
            t.sum_s_at_0_1_gae,
            t.sum_s_at_1_gae
        );
    }
    fs::write(out.join(run).join("PRICING_BAND.csv"), csv)?;
    Ok(m)
}
fn add_metadata(study: &Path, out: &Path) -> Result<()> {
    let fixture = load(study.join("execution-v1/fixtures.json"))?;
    let v = &fixture["fixtures"];
    let roles = [
        "structural",
        "even-control",
        "odd-composite-control",
        "ladder",
    ];
    let values: Vec<_> = roles
        .iter()
        .map(|r| count(v, |x| text(x, "role").starts_with(*r)))
        .collect();
    let rejected = fixture["rejected_ladder_candidates"]
        .as_array()
        .map_or(0, Vec::len);
    let m = json!({"registered_fixture_models":rows(v).len(),"generated_ladder_fixtures":values[3],"rejected_ladder_candidates_retained":rejected});
    report(out,"fixture-preparation","d86eb011034066532d3645369a783f44be158b1c","Frozen synthetic fixture preparation",&m,
        "The native preparation freezes 13 exact curve/twist/extension fixtures, including eight A=-3 ordinary prime-order ladder models. Public SplitMix seed 20261021 plus declared size/replicate offsets, every rejected candidate, exact whole/subgroup orders, generators and tower-to-flat conversions are retained. This is fixture construction, not a scaling or DLP performance result. Identities are independently reconstructed by the repository's canonical Rust constructor in registry-check-v2.json.",roles.iter().map(|s|s.to_string()).collect(),vec![("fixtures",values)])?;
    let v = load(study.join("execution-v1/pairing-fixture/pairing-fixture.json"))?;
    let m = json!({"source_points":num(&v,"source_order"),"extension_points":num(&v,"extension_order"),"source_subgroup_order":num(&v,"r"),"known_target_controls":v["known_scalar_controls"].as_array().unwrap().len()});
    report(out,"pairing-preparation","0145df8ff32a104d186c31eca2f04c9ee256b21d","Separate pairing-positive fixture preparation",&m,
        "The native preparation exhaustively counts the rational and quadratic-extension groups of the separately registered supersingular F43 model, chooses a nonidentity order-11 source generator, checks the distortion point lies outside that cyclic subgroup, and records ten known-scalar controls. This fixture is separate from the ordinary/NIST study. Independent PARI and source replay occur in the later V2 control.",vec!["Fp group".into(),"Fp² group".into()],vec![("points",vec![num(&v,"source_order"),num(&v,"extension_order")])])?;
    let audit = fs::read_to_string(study.join("lean/receipts/twist-lean-axiom-audit.stdout"))?;
    let source = fs::read_to_string(study.join("lean/TwistRoutes.lean"))?;
    let explicit = source
        .lines()
        .filter(|l| l.trim_start().starts_with("theorem "))
        .count() as u64;
    let ok = audit.lines().filter(|l| l.starts_with("ok ")).count() as u64;
    let failed = audit.lines().filter(|l| l.starts_with("FAIL ")).count() as u64;
    let trusted = audit.lines().filter(|l| l.starts_with("TRUST ")).count() as u64;
    assert_eq!((explicit, ok, failed, trusted), (9, 13, 0, 0));
    let m = json!({"explicit_theorems":explicit,"audited_theorems_including_generated_definitions":ok,"nonstandard_axiom_failures":failed,"compiler_trusted_proofs":trusted});
    report(out,"lean-verification","0145df8ff32a104d186c31eca2f04c9ee256b21d","Kernel-checked polynomial and point-group lemmas",&m,
        "Lean 4.34.1 and the locked Mathlib commit build the five exact cleared polynomial identities and four trace/eigenspace statements on the actual Mathlib Weierstrass point group. The auditor checks nine explicit theorems plus four generated defining equations on standard foundations only. The concrete Frobenius/twist embedding, odd subgroup, field/chart/Jacobian maps and cost/complexity bridges remain open. See the exact proof map in lean/README.md; none of these missing bridges is an axiom or a completed theorem.",vec!["explicit theorems".into(),"audited with definitions".into(),"nonstandard axioms".into()],vec![("declarations",vec![explicit,ok,failed])])?;
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let study = Path::new(args.get(1).ok_or("study")?);
    let out = Path::new(args.get(2).ok_or("fresh report root")?);
    if args.get(3).is_some_and(|s| s == "metadata") {
        return add_metadata(study, out);
    }
    if args.get(3).is_some_and(|s| s == "public3") {
        let run = "public3-20261022T0132Z";
        let raw = fs::read_to_string(
            study
                .join("execution-v1/runs")
                .join(run)
                .join("pari.stdout"),
        )?;
        let values: Vec<Value> = raw
            .lines()
            .map(serde_json::from_str)
            .collect::<std::result::Result<_, _>>()?;
        let toy = load(study.join("execution-v1/runs/geometry-20261021T2256Z/E11-gluing.json"))?;
        assert_eq!(values.len(), 15);
        let mut cross_checks = 0;
        for v in &values {
            if num(v, "trace_mod_3") != 0 {
                assert_eq!(num(v, "det_minus_one_anti_intertwiners"), 0);
            }
            if let Some(old) = rows(&toy).iter().find(|old| old["fixture"] == v["fixture"]) {
                assert_eq!(
                    num(v, "det_minus_one_anti_intertwiners"),
                    num(old, "three_torsion_companion_Frobenius_anti_intertwiners")
                );
                cross_checks += 1;
            }
        }
        assert_eq!(cross_checks, 10);
        let nist: Vec<_> = values
            .iter()
            .filter(|v| text(v, "fixture").starts_with("P-"))
            .collect();
        let positive: Vec<_> = nist
            .iter()
            .filter(|v| num(v, "det_minus_one_anti_intertwiners") > 0)
            .map(|v| text(v, "fixture"))
            .collect();
        assert_eq!(positive, vec!["P-521"]);
        let m = json!({"parameter_cases":15,"toy_cross_checks":cross_checks,
            "toy_cross_check_mismatches":0,"NIST_cases":nist.len(),
            "NIST_positive_module_filters":positive.len(),"NIST_positive_names":positive,
            "geometric_constructions_certified":0,"rows":values});
        report(out,run,"36563882e86dacbbcea0d2e473aca236a116ec97",
            "Public 3-torsion companion-module filter",&m,
            "PARI enumerates det(M)=-1 matrices over F3 with MA=-AM for the Frobenius companion matrix. All ten toy counts equal the earlier E11 counts. Among five public NIST P-curve order pairs, only P-521 passes this necessary module filter, with four matrices; the other four have none. This is a public-parameter calculation, not a NIST DLP. The actual torsion/Weil-pairing bridge, principally polarized quotient, genus-two model and divisor maps are not constructed or certified. A positive count is a candidate for further construction, not a speedup or novelty result.",
            nist.iter().map(|v| text(v,"fixture")).collect(),
            vec![("det −1 anti-intertwiners",nist.iter().map(|v|num(v,"det_minus_one_anti_intertwiners")).collect())])?;
        fs::write(
            out.join("PUBLIC_3_MODULE_ANALYSIS.json"),
            serde_json::to_vec_pretty(&m)?,
        )?;
        return Ok(());
    }
    if args.get(3).is_some_and(|s| s == "isogeny-v2") {
        let run = "isogeny-20261022T0054Z-v2";
        let v = load(
            study
                .join("execution-v1/runs")
                .join(run)
                .join("E14-isogenies.json"),
        )?;
        let m = json!({"degree_fixture_cells":rows(&v).len(),"explicit_maps":count(&v,|x|x["status"]=="verified-explicit-Velu-and-rational-subgroup"),"algebraic_dual_maps":count(&v,|x|!x["algebraic_dual_return_map"].is_null()),"scalar_recovery_controls":rows(&v).iter().map(|x|num(x,"scalar_recovery_controls")).sum::<u64>(),"dual_kernel_searches":rows(&v).iter().map(|x|num(x,"dual_kernel_searches")).sum::<u64>()});
        assert_eq!(num(&m, "algebraic_dual_maps"), 3);
        report(out,run,"1284090fd4db2f55e023afcb275d94f3ce691b10","Repeated-eigenvalue dual-map completion",&m,
            "The repeated-eigenvalue case samples another order-ell torsion point and requires its forward image to be nonzero; an eigenvalue alone is not accepted as an independent line. All three eligible degree-seven maps now have explicit algebraic duals, composition [7] checked on every rational source point, and eight scalar-recovery controls each. The original 30 rows, including its missing dual, remain intact. These exact toy maps supply no IC or NIST speed comparison; full cost and formal map bridges remain open.",vec!["eligible forward maps".into(),"verified algebraic duals".into(),"scalar return controls".into()],vec![("verified counts",vec![3,3,24])])?;
        fs::write(
            out.join("ISOGENY_V2_ANALYSIS.json"),
            serde_json::to_vec_pretty(&m)?,
        )?;
        return Ok(());
    }
    if args.get(3).is_some_and(|s| s == "boundary") {
        let a = load(out.join("ANALYSIS.json"))?;
        let curves = a["ecbench-20261022T0012Z"]["by_curve_and_arm"]
            .as_object()
            .ok_or("curve/arm totals")?;
        let floor = (std::f64::consts::PI / 4.0).sqrt();
        let fields = ["sum_s_lower_bound", "sum_s_at_0_1_gae", "sum_s_at_1_gae"];
        let mut csv="curve,arm,attempts,verified,S_floor,S_at_0,S_at_0.1,S_at_1,S_over_floor_at_0,S_over_floor_at_0.1,S_over_floor_at_1,S_over_matched_rho_at_0,S_over_matched_rho_at_0.1,S_over_matched_rho_at_1,full_cost_ranking_admitted\n".to_string();
        for (key, v) in curves {
            let n = num(v, "verified");
            let (curve, _) = key.split_once(',').ok_or("curve key")?;
            let reference = &curves[&format!("{curve},rho-neg")];
            let nr = num(reference, "verified") as f64;
            let means: Vec<_> = fields
                .iter()
                .map(|f| v[*f].as_f64().unwrap() / n.max(1) as f64)
                .collect();
            let rho: Vec<_> = fields
                .iter()
                .map(|f| reference[*f].as_f64().unwrap() / nr)
                .collect();
            csv += &format!("{key},{},{},{floor:.12}", num(v, "attempts"), n);
            for x in &means {
                csv += &format!(",{x:.12}");
            }
            for x in &means {
                csv += &format!(",{:.12}", x / floor);
            }
            for (x, r) in means.iter().zip(rho) {
                csv += &format!(",{:.12}", x / r);
            }
            csv += ",false\n";
        }
        fs::write(out.join("ecbench-20261022T0012Z/BOUNDARY_PRICING.csv"), csv)?;
        return Ok(());
    }
    if out.exists() {
        return Err("report root must be fresh".into());
    }
    fs::create_dir_all(out)?;
    let runs = study.join("execution-v1/runs");
    let mut summary = BTreeMap::new();
    let run = "trace-20261021T2256Z";
    let base = runs.join(run);
    let fibers = load(base.join("E01-fibers.json"))?;
    let eq = load(base.join("E03-rational-equivalence.json"))?;
    let div = load(base.join("E05-divisors.json"))?;
    let census = load(base.join("E06-census.json"))?;
    let m = json!({"exhaustive_trace_fixtures":rows(&fibers).len(),"quartic_witness_equivalence_cells":rows(&eq).len(),
        "quartic_mismatches":count(&eq,|x|x["extension_witnesses"]!=x["rational_quartic_witnesses"]),
        "divisor_fixtures":rows(&div).len(),"divisor_rows":div,"relation_census_cells":rows(&census).len(),"census_statuses":status_counts(&census)});
    report(out,run,"d86eb011034066532d3645369a783f44be158b1c","Trace fibers, restrictions and relation censuses",&m,
        "E01 is an exhaustive Fp² census for nine fixtures. The rational quartic and extension-coordinate parameterizations give identical witness sets. E02/E04 full larger-field norm-y/power fibers and E05 unsplit degree-three/four divisors remain partial. E06 enumerates finite-base relations, not decomposition time; reused bases and unsupported sizes are retained.",
        rows(&fibers).iter().map(|x|text(x,"fixture")).collect(), vec![("distinct traces",rows(&fibers).iter().map(|x|num(x,"trace_image")).collect()),("lifts per trace",rows(&fibers).iter().map(|x|num(x,"each_trace_fiber")).collect())])?;
    summary.insert(run, m);
    let run = "geometry-20261021T2256Z";
    let base = runs.join(run);
    let covers = load(base.join("E10-cover-search.json"))?;
    let gluing = load(base.join("E11-gluing.json"))?;
    let m = json!({"cover_fixtures":rows(&covers).len(),"c_parameters_tested":rows(&covers).iter().map(|x|num(x,"parameters_tested")).sum::<u64>(),"actual_twist_complements":rows(&covers).iter().map(|x|num(x,"actual_Fp_twist_isomorphisms")).sum::<u64>(),"three_gluing_constructible_module_cases_left_open":count(&gluing,|x|num(x,"three_torsion_companion_Frobenius_anti_intertwiners")>0),"gluing_rows":gluing});
    report(out,run,"d86eb011034066532d3645369a783f44be158b1c","Cover search and gluing applicability",&m,
        "All c in Fp were searched on each of ten fixtures. A matching j-invariant or opposite trace alone was not accepted as a twist isomorphism. E09 translated two-torsion cosets do not become rational points. Positive three-gluing module tests still need a quotient curve, polarization and class transfer; no nonexistence claim is made.",rows(&covers).iter().map(|x|text(x,"fixture")).collect(),vec![("same j",rows(&covers).iter().map(|x|num(x,"same_j_complements")).collect()),("opposite trace",rows(&covers).iter().map(|x|num(x,"opposite_trace_complements")).collect()),("actual twist",rows(&covers).iter().map(|x|num(x,"actual_Fp_twist_isomorphisms")).collect())])?;
    summary.insert(run, m);
    let run = "transfer-20261021T2300Z";
    let v = load(runs.join(run).join("E12-transfers.json"))?;
    let m = json!({"transfer_fixtures":rows(&v).len(),"arbitrary_reduced_classes_checked":rows(&v).iter().map(|x|num(x,"arbitrary_reduced_classes_checked")).sum::<u64>(),"exhaustive_rational_class_fixtures":count(&v,|x|x["all_rational_classes_enumerated"]==true),"norm_additivity_and_associativity_samples":rows(&v).iter().map(|x|num(x,"norm_additivity_and_associativity_samples")).sum::<u64>()});
    report(out,run,"b8acaa16012e3b5009d22d46d3fe50fc87a59989","Executable genus-two pullback and norm checks",&m,
        "The p=59 product covers all 3,551 rational Jacobian classes; the other nine fixtures use 256 fixed sampled classes each. Every source and complement rational-point pullback is checked. The recorded odd branch chart handles the two-infinity model; both norm/pullback compositions are [2]. This is same-host executable evidence. PARI independently checks characteristic polynomials in its separate run, not arbitrary-divisor norm code. Faithful formal field/chart/Jacobian bridges remain open.",rows(&v).iter().map(|x|text(x,"fixture")).collect(),vec![("classes checked",rows(&v).iter().map(|x|num(x,"arbitrary_reduced_classes_checked")).collect())])?;
    summary.insert(run, m);
    let run = "isogeny-20261021T2320Z";
    let v = load(runs.join(run).join("E14-isogenies.json"))?;
    let is_map = |x: &Value| !text(x, "status").starts_with("inapplicable");
    let m = json!({"degree_fixture_cells":rows(&v).len(),"explicit_maps":count(&v,is_map),"algebraic_dual_maps":count(&v,|x|!x["algebraic_dual_return_map"].is_null()),"inapplicable":count(&v,|x|!is_map(x)),"statuses":status_counts(&v)});
    report(out,run,"411a254c0ac518e8462ecd6f07e4f315917a48b6","Explicit low-degree isogeny and return-map controls",&m,
        "Degrees 2, 3 and 7 were checked on ten fixtures. Eligible kernels are constructed over the frozen Fp⁶ tower; the rational functions descend to Fp. Every rational source point, generator addition and available dual composition is checked. The finite lookup return map has linear-in-group-order setup and supplies no scalable DLP gain. New destination representations are registered separately.",vec!["degree 2".into(),"degree 3".into(),"degree 7".into()],vec![("explicit map",[2,3,7].iter().map(|d|count(&v,|x|num(x,"ell")==*d&&is_map(x))).collect()),("inapplicable",[2,3,7].iter().map(|d|count(&v,|x|num(x,"ell")==*d&&!is_map(x))).collect())])?;
    summary.insert(run, m);
    for (run, source, invalid) in [
        (
            "solver-20261021T2320Z",
            "411a254c0ac518e8462ecd6f07e4f315917a48b6",
            true,
        ),
        (
            "solver-20261021T2349Z-v2",
            "5d6cd4dc595020077d189e999fcbbb6ca64e6ded",
            false,
        ),
        (
            "solver-20261022T0004Z-v3",
            "2bd6cd2e95e139fa67b2b41bea59b43ca217165b",
            false,
        ),
    ] {
        let v = load(runs.join(run).join("E08-solver-panel.json"))?;
        let eq = load(runs.join(run).join("E07-projected-equivalence.json"))?;
        let native = |x: &Value| {
            x["native_exit_code"] == 0 && x["native_result"]["complete_by_engine"] == true
        };
        let dim = |x: &Value| !invalid && !x["singular_dimension"].is_null();
        let mut ms = BTreeMap::new();
        for x in rows(&v) {
            for y in x["external"].as_array().into_iter().flatten() {
                if y["basis_parsed"] == true {
                    *ms.entry(text(y, "engine")).or_insert(0u64) += 1;
                }
            }
        }
        let mut native_equal = 0;
        let mut ms_equal = BTreeMap::new();
        for x in rows(&v) {
            for e in x["independent_ideal_equalities"]
                .as_array()
                .into_iter()
                .flatten()
            {
                if e[1] == true && !invalid {
                    let n = e[0].as_str().unwrap();
                    if n == "N" {
                        native_equal += 1;
                    } else {
                        *ms_equal.entry(n).or_insert(0u64) += 1;
                    }
                }
            }
        }
        let m = json!({"solver_cells":rows(&v).len(),"projected_set_equivalence_cells":rows(&eq).len(),"projected_set_mismatches":count(&eq,|x|x["polynomial_and_independent_group_projection_equal"]!=true),"native_complete":count(&v,native),"cell_execution_or_verification_errors":count(&v,|x|x["error"].is_string()),"native_record_missing_or_nonzero":count(&v,|x|!x["native_result"].is_object()||x["native_exit_code"]!=0),"native_ideal_equalities_verified":native_equal,"Singular_dimensions_returned":count(&v,dim),"msolve_bases_parsed":ms,"msolve_ideal_equalities_verified":ms_equal,"external_attempt_invalid":invalid});
        let enc = ["product", "chain", "bits", "symmetric"];
        let labels = enc.iter().map(|s| s.to_string()).collect();
        report(
            out,
            run,
            source,
            "Finite-base solver diagnostic outcomes",
            &m,
            if invalid {
                "The V1 external attempt is INVALID: Unicode operators were sent to the CAS tools. Reject every external comparison from this attempt. Native outputs retain their own scope. Preserve all raw cases in the deterministic split archive; do not count failure as refutation."
            } else if run.ends_with("v2") {
                "V2 uses valid ASCII input but the 768 MiB virtual-address limit crashes msolve even on a tiny valid system. Its intended msolve comparison is unresolved. Native and Singular outcomes retain only their separately verified scope. V3 changes every engine to 4 GiB and keeps both earlier attempts."
            } else {
                "V3 uses valid ASCII input and a 4 GiB virtual-address limit for every engine. Cells retain the one-second hard limit (native 500 ms cooperative), one thread and three concurrent cells. Completed bases, verified ideal equalities, unresolved bases and process failures are separate. Counts of kernel multiplications do not price the full solver. B=12/16 and other factor-base families remain outside this initial panel."
            },
            labels,
            vec![
                (
                    "native complete",
                    enc.iter()
                        .map(|e| count(&v, |x| text(x, "encoding") == *e && native(x)))
                        .collect(),
                ),
                (
                    "Singular dimension",
                    enc.iter()
                        .map(|e| count(&v, |x| text(x, "encoding") == *e && dim(x)))
                        .collect(),
                ),
            ],
        )?;
        summary.insert(run, m);
    }
    let run = "pari-20261021T2349Z";
    let base = runs.join(run);
    let g = load(base.join("E12-pari-independent-geometry.json"))?;
    let v = load(base.join("E17-rational-height-and-rank.json"))?;
    let ranks: Vec<_> = (0..=5)
        .map(|r| count(&v, |x| x["rank_lower"].as_u64() == Some(r)))
        .collect();
    let m = json!({"independent_cover_characteristic_polynomials":count(&g,|x|x["independent_CAS_characteristic_polynomial_matches"]==true),"twist_searches":rows(&v).len(),"searches_with_rational_points":count(&v,|x|num(x,"point_count")>0),"searches_with_three_term_relations":count(&v,|x|x["relations"].as_array().is_some_and(|x|!x.is_empty())),"rank_intervals_unresolved":count(&v,|x|x["rank_lower"]!=x["rank_upper"]),"same_E_reductions_checked":count(&v,|x|x["original_group_relation_replay"]=="verified"),"relations_outside_same_base_Fp_census":count(&v,|x|x["new_relation_outside_same_base_rational_group_census"]==true),"rank_lower_histogram_0_to_5":ranks});
    report(out,run,"5d6cd4dc595020077d189e999fcbbb6ca64e6ded","Independent PARI geometry and bounded rational-twist searches",&m,
        "PARI 2.17.2 independently computes all ten selected-cover characteristic polynomials. The 254 rational searches use d=-64..64 (excluding zero and bad reduction), rank effort zero, and standard-twist x height 16. Rust checks square-d finite-field reductions and every projected three-term relation against the identical-base ordinary census. Rank bounds are not a formal Mordell-Weil proof; standard-model height is not equal original-model height. General rational target lifting and all discovery/height costs remain open.",(0..=5).map(|r|format!("rank lower bound {r}")).collect(),vec![("twist cases",ranks)])?;
    summary.insert(run, m);
    let run = "conditional-20261021T2256Z";
    let v = load(runs.join(run).join("E16-applicability.json"))?;
    let m = json!({"public_parameter_cases":rows(&v).len(),"r_primary_hom_applicable":count(&v,|x|x["nonzero_r_primary_hom_to_rational_twist_possible"]==true),"gcd_one":count(&v,|x|x["gcd"]=="1"),"parameter_rows":v});
    report(out,run,"d86eb011034066532d3645369a783f44be158b1c","Conditional homomorphism and integral-lift ingress",&m,
        "NIST P-curve checks use public parameters only: gcd(r,#twist)=1 and no embedding degree found through the recorded bound 1,024. A bound is not the exact degree. The small integral lift box is supplemented, not replaced, by the separate PARI rational-height run. No NIST DLP, pairing transfer or scalable lift attack was run.",vec!["gcd equals one".into(),"eligible r-primary hom".into()],vec![("parameter cases",vec![num(&m,"gcd_one"),num(&m,"r_primary_hom_applicable")])])?;
    summary.insert(run, m);
    let run = "pari-20261022T0020Z-original-height";
    let v = load(runs.join(run).join("E17-rational-height-and-rank.json"))?;
    let m = json!({"twist_searches":rows(&v).len(),"searches_with_rational_points":count(&v,|x|num(x,"point_count")>0),"searches_with_three_term_relations":count(&v,|x|x["relations"].as_array().is_some_and(|x|!x.is_empty())),"same_E_reductions_checked":count(&v,|x|x["original_group_relation_replay"]=="verified"),"relations_outside_same_base_Fp_census":count(&v,|x|x["new_relation_outside_same_base_rational_group_census"]==true),"rank_intervals_unresolved":count(&v,|x|x["rank_lower"]!=x["rank_upper"])});
    let structural = ["icv1-fp6-tm7-fc20da94", "icv1-fp10-t5-192cb216"];
    report(out,run,"0145df8ff32a104d186c31eca2f04c9ee256b21d","Equal-original-height rational-lift control",&m,
        "The same 254 twist parameters are searched at original x height 16 using enclosing standard-model height 16|d| and an exact original-height filter. This is a distinct height domain from V1. Every square-d reduction and projected triple is replayed in the ordinary group on the identical signed base. No row lies outside that same-base census. General target lifting, a complete Mordell-Weil certificate and a full-cost IC comparison remain open.",structural.iter().map(|x|x.to_string()).collect(),vec![("cases with points",structural.iter().map(|id|count(&v,|x|text(x,"fixture")==*id&&num(x,"point_count")>0)).collect()),("cases with triples",structural.iter().map(|id|count(&v,|x|text(x,"fixture")==*id&&x["relations"].as_array().is_some_and(|x|!x.is_empty()))).collect())])?;
    summary.insert(run, m);
    let run = "pairing-20261022T0020Z";
    let m = json!({"public_target_controls":10,"accepted_independent_logs":0,"GP_exit_code":0,"parsed_result":false});
    report(out,run,"0145df8ff32a104d186c31eca2f04c9ee256b21d","Pairing-control first attempt: no emitted result",&m,
        "The GP function definition consumed its output statements on the same line. GP exited zero with no result. This attempt is unresolved and supplies no independent pairing-validation claim. V2 isolates the function definition and retries the identical inputs in a new directory.",vec!["public controls".into(),"accepted logs".into()],vec![("count",vec![10,0])])?;
    summary.insert(run, m);
    let run = "pairing-20261022T0024Z-v2";
    let v = load(runs.join(run).join("E16-pairing-positive-control.json"))?;
    let m = json!({"embedding_degree":2,"subgroup_order":11,"native_bilinearity_controls":num(&v,"native_Miller_bilinearity_controls"),"independent_PARI_logs":v["independent_PARI_logs"].as_array().unwrap().len(),"source_group_replay":v["source_group_replay"],"known_scalar_positive_control_only":true});
    report(out,run,"e517e80b1e77778d529f51ce5a900e00cb010761","Known small-degree pairing positive control",&m,
        "This separate supersingular control uses the registered F43 model and F43² with w²+1, r=11 and a distortion point outside the source cyclic subgroup. Native Miller arithmetic and independent PARI Tate pairing/finite-field logarithms recover all ten deliberately known scalar controls, checked again on the public source points. The GP script receives only target coordinates. This validates a known positive conditional branch; it is no ordinary/NIST transfer, novelty claim or speed result.",vec!["native bilinearity".into(),"PARI recovered logs".into()],vec![("controls passed",vec![10,10])])?;
    summary.insert(run, m);
    let m = add_pipeline(study, out)?;
    summary.insert("ecbench-20261022T0012Z", m);
    fs::write(
        out.join("ANALYSIS.json"),
        serde_json::to_vec_pretty(&summary)?,
    )?;
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}
