//! Native evidence aggregation and plots. Reads recorded outcomes only.
use serde_json::{json, Value};
use std::{collections::BTreeMap, error::Error, fs, path::Path};
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
    let mut md=format!("# {title}\n\nRun `{run}`; source freeze `{source}`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. These are public synthetic small-field records. No wall-clock or full-cost improvement is admitted.\n\nRaw outputs: [run directory](../../../execution-v1/runs/{run}/). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.\n\n## Recorded observations\n\n| Metric | Count / recorded value |\n|:--|--:|\n");
    for (k, v) in metrics.as_object().unwrap() {
        if !v.is_object() && !v.is_array() {
            md += &format!("| {} | {} |\n", k.replace('_', " "), v);
        }
    }
    md+=&format!("\n![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)\n\n{notes}\n\nThe chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).\n");
    fs::write(out.join("REPORT.md"), md)?;
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let study = Path::new(args.get(1).ok_or("study")?);
    let out = Path::new(args.get(2).ok_or("fresh report root")?);
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
            for y in x["external"].as_array().unwrap() {
                if y["basis_parsed"] == true {
                    *ms.entry(text(y, "engine")).or_insert(0u64) += 1;
                }
            }
        }
        let mut native_equal = 0;
        let mut ms_equal = BTreeMap::new();
        for x in rows(&v) {
            for e in x["independent_ideal_equalities"].as_array().unwrap() {
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
        let m = json!({"solver_cells":rows(&v).len(),"projected_set_equivalence_cells":rows(&eq).len(),"projected_set_mismatches":count(&eq,|x|x["polynomial_and_independent_group_projection_equal"]!=true),"native_complete":count(&v,native),"native_worker_failures":count(&v,|x|x["native_exit_code"]!=0),"native_ideal_equalities_verified":native_equal,"Singular_dimensions_returned":count(&v,dim),"msolve_bases_parsed":ms,"msolve_ideal_equalities_verified":ms_equal,"external_attempt_invalid":invalid});
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
    fs::write(
        out.join("ANALYSIS.json"),
        serde_json::to_vec_pretty(&summary)?,
    )?;
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}
