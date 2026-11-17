// Additional exact validation only; included by the native analyzer after the
// frozen measurement campaign. It does not change a solver or an estimand.
fn b_checks(out: &Path) {
    use crypto_lib::cryptanalysis::identity_certificate::{
        check_with_extra, issue, Expr, IdentityStatement, Verdict,
    };
    let l = Expr::var("lambda");
    let q = Expr::var("q");
    let z = Expr::var("z");
    let mul = |a, b| Expr::Mul(vec![a, b]);
    let neg = |a| Expr::Neg(Box::new(a));
    let poly = Expr::Add(vec![mul(l.clone(), l.clone()), neg(l.clone()), q.clone()]);
    let statement = IdentityStatement {
        name: "orbit-recurrence-factorization".into(),
        source: "conductor_gap_replication_20261025: distributivity underlying the conditional group recurrence".into(),
        variables: vec!["lambda".into(),"q".into(),"z".into()], degree_bound: 3,
        lhs: mul(z.clone(),poly),
        rhs: Expr::Add(vec![mul(l.clone(),mul(l.clone(),z.clone())),neg(mul(l,z.clone())),mul(q,z)]),
    };
    let certificate = issue(
        statement,
        2026102524,
        64,
        "native issuer; additional domain-separated checker seed and 128 points",
        None,
    )
    .unwrap();
    save(
        analysis_dir().join("recurrence-factorization.json"),
        &serde_json::to_value(&certificate).unwrap(),
    );
    let identity_verdict = check_with_extra(&certificate, Some((2026102525, 128))).unwrap();
    let accepted = matches!(&identity_verdict, Verdict::Accept { .. });
    let verdict_json = match &identity_verdict {
        Verdict::Accept {
            points,
            extra_points,
            false_accept_log2,
        } => {
            json!({"accepted":true,"points":points,"extra_points":extra_points,"false_accept_log2":false_accept_log2})
        }
        other => json!({"accepted":false,"detail":format!("{other:?}")}),
    };
    save(
        analysis_dir().join("recurrence-identity-check.json"),
        &json!({"id":certificate.id,
        "verdict":verdict_json,"scope":"evaluated polynomial identity over 2^61-1; not a complete CM geometry proof"}),
    );
    assert!(accepted, "recurrence factoring identity certificate failed");
    let manifest = load(out.join("manifest.json"));
    let mut by_workload: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
    for s in manifest["specs"].as_array().unwrap() {
        let session = Path::new(ROOT)
            .join("sessions")
            .join(s["name"].as_str().unwrap());
        for r in lines(session.join("records.jsonl")) {
            if r["arm"] == "rho-neg" {
                continue;
            }
            let wid = r["workload"]["workload_id"].as_str().unwrap().to_string();
            let arm = r["arm"].as_str().unwrap().to_string();
            assert!(by_workload.entry(wid).or_default().insert(arm, r).is_none());
        }
    }
    let mut recurrence_pairs = 0;
    let mut velu_pairs = 0;
    let mut mismatches = Vec::new();
    let mut base_costs: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    let nonbase = |r: &Value| {
        r["phases"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|p| p["name"] != "factor_base")
            .map(|p| {
                json!({"name":p["name"],"adds":p["adds"],"doubles":p["doubles"],
            "scalar_mults":p["scalar_mults"],"gae":p["gae"],"native":p["native"]})
            })
            .collect::<Vec<_>>()
    };
    for (wid, arms) in &by_workload {
        let scalar = &arms["ic-scalar"];
        for (arm, r) in arms {
            for p in r["phases"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|p| p["name"] == "factor_base")
            {
                base_costs
                    .entry(arm.clone())
                    .or_default()
                    .push(p["gae"].as_f64().unwrap());
            }
            if arm == "ic-scalar" {
                continue;
            }
            if arm == "ic-recurrence" {
                recurrence_pairs += 1;
            } else {
                assert_eq!(arm, "ic-velu");
                velu_pairs += 1;
            }
            if scalar["algorithm_seed"] != r["algorithm_seed"]
                || scalar["outcome"] != r["outcome"]
                || scalar["counters"] != r["counters"]
                || nonbase(scalar) != nonbase(r)
            {
                mismatches.push(json!({"workload":wid,"arm":arm}));
            }
        }
    }
    assert_eq!(recurrence_pairs, 512);
    assert_eq!(velu_pairs, 128);
    let cost_summary = base_costs.iter().map(|(arm,xs)| json!({"arm":arm,"records":xs.len(),
        "factor_base_gae_sum":xs.iter().sum::<f64>(),"mean_factor_base_gae":xs.iter().sum::<f64>()/xs.len() as f64})).collect::<Vec<_>>();
    save(
        analysis_dir().join("b-mode-invariants.json"),
        &json!({
        "recurrence_pairs":recurrence_pairs,"velu_pairs":velu_pairs,"mismatches":mismatches,
        "equal_algorithm_seeds_outcomes_counters_and_nonbase_phases":mismatches.is_empty(),
        "factor_base_costs":cost_summary,"timing_excluded_from_equality":true}),
    );
    assert!(
        mismatches.is_empty(),
        "mode comparison changed more than factor-base construction; inspect retained diagnostics"
    );
}
