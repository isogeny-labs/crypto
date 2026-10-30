use super::*;
fn read(dir: &Path, name: &str) -> Value {
    serde_json::from_str(&fs::read_to_string(dir.join(name)).unwrap()).unwrap()
}
fn range(rows: &[&Value], key: &str) -> [f64; 2] {
    [
        rows.iter()
            .map(|r| r[key][0].as_f64().unwrap())
            .fold(f64::INFINITY, f64::min),
        rows.iter()
            .map(|r| r[key][1].as_f64().unwrap())
            .fold(0., f64::max),
    ]
}
fn interval_text(a: [f64; 2]) -> String {
    format!("{:.3}–{:.3}", a[0], a[1])
}
pub fn generate(study: &str) {
    let study = Path::new(study);
    let results = study.join("results");
    let costs = read(&results, "cost-summary.json");
    let costs = costs.as_array().unwrap();
    let coverage = read(&results, "coverage-summary.json");
    let coverage = coverage.as_array().unwrap();
    let cycles = read(&results, "genus-cycle-events.json");
    let controls = read(&results, "genus-controls.json");
    let sessions = read(&results, "session-summaries.json");
    let hardware = read(&results, "retired-instruction-summary.json");
    let aa = read(&results, "instruction-AA.json");
    let trace = read(&results, "trace-filter.json");
    let mut audited = 0u64;
    let mut replayed = 0u64;
    let mut records = 0u64;
    for path in fs::read_dir(study.join("replay-receipts")).unwrap() {
        let receipt: Value =
            serde_json::from_str(&fs::read_to_string(path.unwrap().path()).unwrap()).unwrap();
        assert_eq!(receipt["ok"], true);
        for replay in receipt["replays"].as_array().unwrap() {
            assert_eq!(replay["reproduced"], true);
        }
        audited += 1;
        replayed += receipt["replays"].as_array().unwrap().len() as u64;
        records += receipt["records"].as_u64().unwrap();
    }
    assert_eq!((audited, replayed, records), (945, 42862, 42996));
    let exhausted = read(&study.join("exhaustion-replays"), "summary.json");
    assert_eq!(exhausted["all_identical"], true);
    assert_eq!(exhausted["exhausted_records"], 134);
    let aa_replay = read(&study.join("correctness"), "instruction-AA-replay.json");
    assert_eq!(aa_replay["ok"], true);
    assert_eq!(aa_replay["replays"].as_array().unwrap().len(), 10);
    assert!(aa_replay["replays"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["reproduced"] == true));
    write(
        results.join("replay-summary.json"),
        &json!({"study_sessions":audited,"study_records":records,"successful_measured_records_replayed_identically":replayed,"exhausted_diagnostic_records_reexecuted_identically":134,"AA_measured_records_replayed_identically":10,"AA_warmup_records":2,"all_audits_ok":true}),
    );
    let mut fixtures_csv = String::from(
        "cohort,nominal_bits,p,actual_field_bits,r,actual_subgroup_bits,cofactor,order\n",
    );
    for (cohort, path) in [
        ("field", "genus-fixtures-v1"),
        ("subgroup", "genus-order-fixtures-v1"),
    ] {
        let models = read(&study.join(path), "fixtures.json");
        for (i, model) in models.as_array().unwrap().iter().enumerate() {
            let p = model["curve"]["f"]["p"].as_u64().unwrap();
            let r = model["r"].as_u64().unwrap();
            fixtures_csv.push_str(&format!(
                "{cohort},{},{p},{},{r},{},{},{}\n",
                16 + 4 * (i / 3),
                64 - (p * p).leading_zeros(),
                64 - r.leading_zeros(),
                model["h"],
                model["order"]
            ));
        }
    }
    fs::write(results.join("genus-fixture-sizes.csv"), fixtures_csv).unwrap();
    let mut data = json!({"primary_decomposition_targets":3520,"primary_genus_targets_per_cohort":1260,"primary_genus_cohorts":2,"primary_solves":23952,"control_solves":84,"oracle_checks":{"bases":854,"targets":9378482,"disagreements":0},"genus_tables":[],"decomposition":[],"dashboard_rows":[],"progress_points":[]});
    let mut md=String::from("# Fresh end-to-end checks of three ECDLP leads\n\nThis is a new execution, not a reconstruction of the unlocated historical runs. The complete claim-by-claim verdicts are in [CLAIMS.md](CLAIMS.md). Exact frozen observations are in [results/](results/), with source models, subgroup witnesses, input specs, sealed sessions and replay receipts.\n\nThe study covers 21 binary curves, 24 odd-characteristic coverage cases, and 18 PKM cells with 23 eligible primes each. Decomposition uses 3,520 target workloads. Genus-2 transfer uses two separate 1,260-target cohorts: one indexed by field size, and one by actual prime-subgroup size through 40 bits. Each genus target has four solver arms. Primary execution therefore comprises 23,952 solves, with 84 independent/unfiltered control solves and 168 supplementary trace-filter solves. Every completed answer is checked on its original elliptic curve; rejected fixtures and unsuccessful development attempts are preserved separately.\n\n## What the costs mean\n\nAll figures below are cold one-target operation-count observations. Algorithms receive P, Q and r, never the planted scalar. Charge destination construction, both transfers, base construction/projection, all initialization, roots and rational lifts, relation collection, elimination, failed work, retries and source verification. Common fixture generation/order certification is separately recorded.\n\nOdd quadratic arithmetic expands into F_p M/S/I operations. Binary arithmetic uses the specified ambient-field M/S/I unit. Twelve arithmetic prices use S/M={0,0.5,1}, I/M={1,10,100,1000}. The retained ecbench GAE conversion uses the separately declared S/M=0.75, I/M=10. Native charges {0,0.1,1} source-addition equivalents apply symmetrically to both arms. These logical native counters do not measure allocations, physical hash buckets or CPU instructions. L0 walls are descriptive; no hardware-speed or fastest-tool claim follows.\n\nThe matched references are memoized r-adding rho with and without negation. Their collision-table memory is reported alongside the IC graph. `rho_neg` is the predeclared main reference; `rho_plain` remains visible throughout.\n\n## Genus-2 transfer\n\nThe implementation uses the published Scholten family and an explicit one-infinity quintic, `12Y²=12+100t+299t²+380t³+187t⁴+30t⁵`. Pullback plus conjugation defines tau. The dual norm checks `norm(tau(P))=2P`; addition, exceptional points, descent and odd-prime subgroup preservation are checked. See [TRANSFER.md](TRANSFER.md), the exact [polynomial certificate](certificates/cover-identity-v1.json) and [small zeta/order checks](certificates/small-independent-zeta-orders.json). The certificate is an integer polynomial witness plus the repository's identity checker, not a Lean proof of the geometric theorem.\n\nThe primary collector discards repeated split divisor classes up to sign. It restarts on repeated walk divisors and cannot recover a scalar from those collisions. A signed negative cycle only anchors a large-prime component; extraction waits for a useful target dependence. Initial scalar work, restarts, all root work and graph/scalar operations remain charged.\n\n| Cohort / size label | Targets | IC/rho-neg, 12 arithmetic prices | Including native charge band | Declared default ratio | rho wins / 12 |\n|:--|--:|--:|--:|--:|--:|\n");
    for (prefix, cohort) in [("genus-", "field"), ("order-genus-", "subgroup")] {
        for bits in [16, 20, 24, 28, 32, 36, 40] {
            let group = if cohort == "field" {
                format!("{prefix}{bits}-v2")
            } else {
                format!("{prefix}{bits}")
            };
            let row = costs
                .iter()
                .find(|r| {
                    r["group"] == group
                        && r["candidate"] == "genus_ic"
                        && r["reference"] == "rho_neg"
                })
                .unwrap();
            assert_eq!(row["matched_targets"], 180);
            let a = [
                row["arithmetic_ratio_range"][0].as_f64().unwrap(),
                row["arithmetic_ratio_range"][1].as_f64().unwrap(),
            ];
            let n = [
                row["native_band_ratio_range"][0].as_f64().unwrap(),
                row["native_band_ratio_range"][1].as_f64().unwrap(),
            ];
            md.push_str(&format!(
                "| {cohort} / {bits} | 180 | {} | {} | {:.3} | {} |\n",
                interval_text(a),
                interval_text(n),
                row["declared_default_ratio"].as_f64().unwrap(),
                row["rho_wins_of_12"]
            ));
            data["genus_tables"]
                .as_array_mut()
                .unwrap()
                .push(row.clone());
        }
    }
    md.push_str("\nThe field cohort labels are approximate field-size levels; actual field and subgroup bits/cofactors are retained in the manifests. The order cohort requires exact subgroup bit lengths and cofactor at most 64. A 40-bit field is not a 40-bit prime-order subgroup. Tables report ratios of sums over matched targets, not a claim that every individual target has the same cost. Per-curve rows and all price cells remain in the machine-readable tables.\n\nA generic full-degree Jacobian addition in the largest field fixture costs **70M+3S+I** in this backend; the matched source addition costs **9M+4S+I**, in base-field units. The supplied 41M+I Jacobian formula does not describe this backend; the source formula is reconciled below. Independent-trial and unfiltered controls, including all scalar-generation cost, are in [genus-controls.json](results/genus-controls.json). First graph cycles versus useful solve positions are in [genus-cycle-events.json](results/genus-cycle-events.json).\n\n## Subfield and PKM decomposition\n\nThe binary linearized solver and odd S3 solver verify every rational lift by point addition. Factor-base logarithms are unknown variables; incremental elimination recovers the target. Cofactor offsets let relation collection visit the full relevant group, followed by projection into the prime subgroup. The corrected sampler tests initial/restarted targets and uses seeded counted jump choices; it does not assume a fixed point-partition map reaches a tiny sumset.\n\n| Family | Curves / targets | Per-curve IC/rho-neg across all 12 arithmetic prices | With native charge band | Whole-oracle M / decomposition call |\n|:--|--:|--:|--:|--:|\n");
    for (prefix, label, curves, targets) in [
        ("binary-", "Binary quadratic", 21, 168),
        ("odd-", "Odd quadratic", 5, 40),
        ("pkm_", "Kummer/Dickson", 414, 3312),
    ] {
        let rows: Vec<_> = costs
            .iter()
            .filter(|r| {
                let g = r["group"].as_str().unwrap();
                g.starts_with(prefix)
                    && g.contains("-v2::")
                    && r["candidate"] == "structured_ic"
                    && r["reference"] == "rho_neg"
            })
            .collect();
        assert_eq!(rows.len(), curves);
        let a = range(&rows, "arithmetic_ratio_range");
        let n = range(&rows, "native_band_ratio_range");
        let means: Vec<_> = rows
            .iter()
            .filter_map(|r| {
                let calls = r["mean_decomposition_calls"].as_f64().unwrap();
                (calls > 0.).then(|| r["mean_oracle_M"].as_f64().unwrap() / calls)
            })
            .collect();
        let root_range = [
            means.iter().copied().fold(f64::INFINITY, f64::min),
            means.iter().copied().fold(0., f64::max),
        ];
        md.push_str(&format!(
            "| {label} | {curves} / {targets} | {} | {} | {} |\n",
            interval_text(a),
            interval_text(n),
            interval_text(root_range)
        ));
        data["decomposition"].as_array_mut().unwrap().push(json!({"family":label,"curves":curves,"targets":targets,"arithmetic_ratio_range":a,"native_band_ratio_range":n,"oracle_M_per_call_range":root_range}));
    }
    let pkm: Vec<_> = coverage
        .iter()
        .filter(|r| r["cell"].as_str().unwrap().starts_with("pkm_"))
        .collect();
    assert_eq!(pkm.len(), 18);
    let maxz = pkm
        .iter()
        .filter_map(|r| r["z"].as_f64())
        .map(f64::abs)
        .fold(0., f64::max);
    let maxdelta = pkm
        .iter()
        .map(|r| r["delta"].as_f64().unwrap().abs())
        .fold(0., f64::max);
    let binary = coverage
        .iter()
        .find(|r| r["cell"] == "binary-q512")
        .unwrap();
    md.push_str(&format!("\nBinary q=512 mean full-group coverage is **{:.6}**, compatible with approach to 3/8 over this range. Every x in each tested subfield has absolute trace zero; all 21 doubled images equal the trace kernel. This proves the finite coset obstruction, not a universal 3/8 limit. Prime-subgroup coverage is separately retained; a trace filter rejects no target in an odd-order subgroup. The 4.7% number compares two whole-group hit probabilities, not all relation multiplicities or the subgroup attack yield.\n\nThe 18 PKM cells have fresh maximum |z| **{maxz:.3}** under the predeclared paired-prime statistic, with maximum absolute structured-minus-control coverage **{maxdelta:.8}**. Thus the supplied |z|≤1.84 bound is not reproduced. Each curve has 32 sign-orbit/torsion-matched random controls. All effects and bootstrap intervals remain visible; failure to reject zero is not a proof of random equivalence.\n\nThe complete oracle census checks **9,378,482 targets across 854 curve/base combinations**, with zero disagreements against direct point-pair sumsets. Cost comparisons include complete solving on fixed tower fibers and matched random bases. This measures the implemented two-summand solving route; it does not exhaust every possible multivariate or auxiliary-isogeny PKM solver.\n",binary["structured"].as_f64().unwrap()));
    md.push_str("\n## Reproduction and retained failures\n\nBuild the common engine with `cargo build --release --offline --bin ecbench`, and the standalone driver with `cargo build --release --offline --manifest-path research/three_leads_e2e_20261018/native/Cargo.toml`. The [protocol](PROTOCOL.md) and [order-cohort protocol](GENUS_ORDER_PROTOCOL.md) fix eligibility, seeds, caps and prices. `run-batch.sh` records each session; `verify --replay-all` re-executes every deterministic record. Archived sessions must be extracted to their original study-relative paths before replay.\n\nThe earlier v1 batches are diagnostic evidence, not silently discarded targets: unsplit-only genus walks and fixed-partition decomposition walks could exhaust. Their original implementations remain registered for exact replay. Corrected v2 cohorts use the same original models/targets/caps; the additional exact-order cohort is separately identified. The general-cubic coverage diagnostic and failed build/check attempts are retained as development evidence.\n\nThe report graph data are native Rust outputs. Python only renders the retained observations. All individual session reports and plots are indexed in the report bundle; the main plots and PDF are mirrored under `benchmarks/20261018-three-leads-e2e`. Canonical dashboard entries remain operation-model diagnostics outside fully priced speed rankings. No claim of fastest implementation or deployed-curve security is made.\n");
    md.push_str("\n## Supplemental checks and implementation cost\n\nThe 168 additional binary trace-filter solves all verify and stop at exactly the same relation/trial positions as their unfiltered counterparts. The filter skips 3,044 of 6,046 candidate relations (50.35%). It tests relation-walk points in the full group; it rejects no odd-prime-subgroup DLP target. Its mask costs n(n−1) squarings once, followed by four logical bit operations per finite candidate. At q=512 it reduces the arithmetic-model cost to 0.971–0.987 of the unfiltered route; the native sensitivity band reaches 1.004. This is inexpensive filtering, not free setup. See [trace-filter.json](results/trace-filter.json).\n\nThe source formula 9M+4S+I becomes exactly 12M+I at S/M=0.75. Thus the supplied source-addition formula is compatible with that convention; 41M+I does not describe this Jacobian backend.\n\nIn the PKM decomposition pipeline, 562 of 6,624 structured/random-base returns occurred before any decomposition, including 64 empty-base returns. Those are generic identity hits rather than relation-based IC successes. All costs include that behavior; the result does not certify a pure IC mechanism on every tiny target. See [decomposition-mechanisms.json](results/decomposition-mechanisms.json).\n\nThe supplementary uniform-random-x null (32 controls per curve, added after the conditional analysis) gives maximum absolute PKM z=2.867, compared with 4.255 for controls conditioned on actual point count. Neither reproduces 1.84. These nominal cell standardizations are not a multiplicity-adjusted universal significance claim.\n\nAt actual 40-bit subgroup size, complete instrumented solves retire 7.4868 times as many user-space instructions as negation rho (paired curve/target bootstrap 95% interval 6.6571–8.3716), or 5.7823 times plain rho (5.2151–6.4286). All 180 pairs use the same binary and valid unmultiplexed counters. These counts include native work and instrumentation overhead. A five-pair identical-rho A/A control has ratio range 0.9999703–1.0000121 and identical primitive/native counters. See [retired-instruction-summary.json](results/retired-instruction-summary.json) and [instruction-AA.json](results/instruction-AA.json). Both attempted L2 timing preflights rejected a busy host; no qualified wall-time result is claimed.\n\nAll 945 sealed study sessions pass their audits. Every one of 42,862 successful measured records replays identically. The 134 exhausted development records were separately rerun and reproduce their exhaustion and exact counters; the standard replay command skips unsuccessful records. The A/A session adds ten measured records and two warmups separately.\n");
    md.push_str("\nThe complete [evidence and reproduction guide](EVIDENCE.md) documents the deterministic raw archive, exact replay commands, source snapshots and validation limits. Actual field/subgroup bit lengths and cofactors for all 42 genus models are in [genus-fixture-sizes.csv](results/genus-fixture-sizes.csv).\n");
    md.push_str("\nRetired instructions favor the tiny subfield pipeline in some cases despite its arithmetic-model loss: binary q=8 gives 0.4491× rho-neg, q=16 gives 0.4942×; odd p=5 gives 0.4363× and p=7 gives 0.4854×. All 18 structured PKM cell instruction ratios exceed one (1.928–8.190). These complete instrumented-backend observations are a separate unit; they establish no qualified wall-speed or fastest-implementation claim.\n");
    data["trace_filter"] = trace;
    data["hardware_observations"] = hardware.clone();
    data["instruction_AA"] = aa.clone();
    write(results.join("report-data.json"), &data);
    fs::write(study.join("README.md"), md).unwrap();
    let mut dashboard = vec![];
    for (prefix, label) in [
        ("binary-", "Binary subfield"),
        ("odd-", "Odd subfield"),
        ("pkm_", "PKM fixed fibers"),
        ("genus-", "Genus transfer, field cohort"),
        ("order-genus-", "Genus transfer, subgroup cohort"),
    ] {
        let best = costs
            .iter()
            .filter(|r| {
                let g = r["group"].as_str().unwrap();
                g.starts_with(prefix)
                    && g.contains("::")
                    && (g.contains("-v2::")
                        || g.starts_with("order-genus-")
                        || g.contains("filtered::"))
                    && r["reference"] == "rho_neg"
                    && (r["candidate"] == "structured_ic"
                        || r["candidate"] == "genus_ic"
                        || r["candidate"] == "filtered_ic")
            })
            .min_by(|a, b| {
                a["declared_default_ratio"]
                    .as_f64()
                    .unwrap()
                    .total_cmp(&b["declared_default_ratio"].as_f64().unwrap())
            })
            .unwrap();
        dashboard.push(json!({"label":label,"group":best["group"],"candidate":best["candidate"],"ratio":best["declared_default_ratio"],"ratio_text":format!("{:.4}",best["declared_default_ratio"].as_f64().unwrap()),"arithmetic_band_text":interval_text([best["arithmetic_ratio_range"][0].as_f64().unwrap(),best["arithmetic_ratio_range"][1].as_f64().unwrap()]),"matched_targets":best["matched_targets"],"source":"research/three_leads_e2e_20261018/results/cost-summary.json"}));
    }
    write(
        results.join("dashboard.json"),
        &json!({"schema":"three-leads.dashboard/v1","date":"2026-10-19","primary_solves":23952,"controls":84,"trace_filter_controls":168,"genus_targets_per_cohort":1260,"genus_cohorts":2,"unit":"Cold arithmetic model; S/M=.75, I/M=10; native unpriced; L0","scope":"Matched synthetic one-target solves; rows select the best tested curve/recipe after observation; no qualified hardware speed claim","source":"research/three_leads_e2e_20261018/README.md","rows":dashboard,"genus_cycles":cycles,"hardware_40":hardware.as_array().unwrap().iter().filter(|r|r["group"]=="order-genus-40" && r["candidate"]=="genus_ic").collect::<Vec<_>>(),"instruction_AA":aa,"control_summary":controls.as_array().unwrap().iter().map(|r|{let mut v=r.clone();v.as_object_mut().unwrap().remove("pairs");v}).collect::<Vec<_>>(),"sessions":sessions.as_array().unwrap().len()}),
    );
}

/// Publish only frozen native observations; no new measurements or statistics.
pub fn publish(study: &str) {
    let data = read(&Path::new(study).join("results"), "dashboard.json");
    let mut timeline: Value =
        serde_json::from_str(&fs::read_to_string("docs/ic/progress-timeline.json").unwrap())
            .unwrap();
    timeline["updated"] = json!("2026-10-19");
    let series = timeline["series"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|s| s["id"] == "ec-bounded-counts")
        .unwrap();
    assert_eq!(series["barrier_test"], false);
    let points = series["points"].as_array_mut().unwrap();
    points.retain(|p| {
        !p["anchor"]
            .as_str()
            .unwrap_or("")
            .starts_with("#three-leads-e2e-")
    });
    let source =
        "https://github.com/aburan28/crypto/blob/main/research/three_leads_e2e_20261018/README.md";
    let mut panel = String::from("<section class=\"dash-section\" id=\"three-leads-e2e\" aria-labelledby=\"three-leads-title\"><h2 id=\"three-leads-title\">Three leads: fresh complete solves</h2><p class=\"panel-summary\">23,952 primary solves verify, including two separate 1,260-target genus-2 cohorts: field size and actual prime-subgroup size through 40 bits. Another 84 independent/unfiltered and 168 trace-filter controls verify. The complete oracle census checks 9,378,482 targets with zero disagreements.</p><p>The table selects the best tested curve and recipe per regime after observation, at the declared default S/M=0.75, I/M=10. It is an arithmetic-model diagnostic with native work unpriced, outside fully priced speed rankings. Rho is the matched negation reference. The intervals are price sensitivity, not statistical confidence intervals.</p><table><thead><tr><th>Regime</th><th>Selected curve/recipe</th><th>Matched targets</th><th>Default IC/rho</th><th>12 arithmetic prices</th></tr></thead><tbody>");
    let mut table = String::new();
    for (i, row) in data["rows"].as_array().unwrap().iter().enumerate() {
        let anchor = format!("#three-leads-e2e-{i}");
        assert!(
            !points.iter().any(|p| p["anchor"] == anchor),
            "Already published"
        );
        let label = row["label"].as_str().unwrap();
        let group = row["group"].as_str().unwrap();
        let ratio = row["ratio_text"].as_str().unwrap();
        let band = row["arithmetic_band_text"].as_str().unwrap();
        let detail = format!("Best tested curve/recipe after observation: {group}, {} matched targets; cold default arithmetic IC/rho-neg {ratio}; 12-price range {band}. Native work unpriced; L0 walls unqualified. Complete frozen costs and all curves retained.",row["matched_targets"]);
        points.push(json!({"date":"2026-10-19","value":row["ratio"],"reference":"matched","unit":"cold arithmetic-model diagnostic; native unpriced","label":format!("Three-leads fresh {label}"),"detail":detail,"class":"accounting","anchor":anchor,"source":source}));
        panel.push_str(&format!("<tr id=\"three-leads-e2e-{i}\"><td>{label}</td><td><code>{group}</code> / {}</td><td>{}</td><td>{ratio}</td><td>{band}</td></tr>\n",row["candidate"].as_str().unwrap(),row["matched_targets"]));
        table.push_str(&format!("<tr><td>2026-10-19</td><td>Elliptic curve, cold counted-work diagnostics with unpriced native work</td><td>Three-leads fresh {label}</td><td class=\"n\">{ratio}&times;</td><td>matched</td><td>accounting</td><td><a href=\"{anchor}\">panel</a> &middot; <a href=\"{source}\">source</a></td></tr>\n"));
    }
    panel.push_str("</tbody></table><p>At actual 40-bit subgroup size, the 180-target genus IC/rho-neg arithmetic ratio is 1.421–18.362 (default 11.091); rho wins all 12 price cells. Complete instrumented retired-instruction ratio is 7.487 (paired bootstrap 95% 6.657–8.372); five-pair identical-rho control range 0.9999703–1.0000121. These user-space counters include native and ledger/report overhead, and are separate from the arithmetic prices. L2 wall-time preflights rejected host contention.</p><p>Instruction counts favor some tiny subfield cases: binary q=8 gives 0.4491, q=16 gives 0.4942; odd p=5 gives 0.4363 and p=7 gives 0.4854 versus rho-neg. All 18 structured PKM cell instruction ratios exceed one (1.928–8.190). These are instrumented backend observations, not qualified wall-speed claims.</p><p>The first graph cycle is insufficient in 1,244/1,260 actual-order genus cases. PKM includes generic identity returns on tiny targets; its mechanism counts are disclosed. Binary finite coverage approaches 3/8 in the primary fixtures but is not implied by the trace theorem alone. <a href=\"https://github.com/aburan28/crypto/blob/main/research/three_leads_e2e_20261018/CLAIMS.md\">Every original claim and verdict</a> · <a href=\"https://github.com/aburan28/crypto/blob/main/research/three_leads_e2e_20261018/README.md\">Frozen report and replay</a></p></section>\n");
    let page_path = "docs/index-calculus-scoreboard.html";
    let mut page = fs::read_to_string(page_path).unwrap();
    if let Some(start) = page.find("<section class=\"dash-section\" id=\"three-leads-e2e\"") {
        let end = start + page[start..].find("</section>").unwrap() + "</section>".len();
        page.replace_range(start..end, "");
    }
    while let Some(at) = page.find("<td>Three-leads fresh ") {
        let start = page[..at].rfind("<tr>").unwrap();
        let end = at + page[at..].find("</tr>").unwrap() + "</tr>".len();
        page.replace_range(start..end, "");
    }
    let marker = "<section class=\"dash-library-intro\" id=\"evidence-search\"";
    let at = page.find(marker).unwrap();
    page.insert_str(at, &panel);
    let at = page
        .find("<summary>Every plotted point as a table</summary>")
        .unwrap();
    let end = at + page[at..].find("</tbody>").unwrap();
    page.insert_str(end, &table);
    let start_marker = "<script type=\"application/json\" id=\"progress-data\">";
    let start = page.find(start_marker).unwrap() + start_marker.len();
    let end = start + page[start..].find("</script>").unwrap();
    write("docs/ic/progress-timeline.json", &timeline);
    page.replace_range(
        start..end,
        &fs::read_to_string("docs/ic/progress-timeline.json").unwrap(),
    );
    fs::write(page_path, page).unwrap();
}
