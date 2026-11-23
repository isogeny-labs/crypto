//! Opt-in preprocessing, never enabled for frozen historical arms.
//! Substitute only pivots with nonzero FIELD CONSTANT coefficients. In
//! particular x*y=1 cannot eliminate x. A pivot x-h has a unique lift x=h;
//! reverse-order reconstruction preserves the entire affine solution scheme.
//! Input degree is bounded here; F4 solving degree is an empirical question.
use super::*;

pub(super) fn substitute(p: &MP, f: F, v: usize, rhs: &MP) -> MP {
    let mut result = MP::zero(p.n);
    for (powers, &c) in &p.t {
        let mut term = MP::konst(p.n, c);
        let mut residual = powers.clone();
        residual[v] = 0;
        term.t.clear();
        term.addterm(f, residual, c);
        for _ in 0..powers[v] {
            term = term.mul(f, rhs);
        }
        result = result.add(f, &term);
    }
    result
}

// Return h from a*x + b = 0, where a is a nonzero field constant,
// and b contains no x. This imposes no nonvanishing chart condition.
pub(super) fn pivot(p: &MP, f: F, v: usize) -> Option<MP> {
    let mut coeff = 0;
    let mut rest = MP::zero(p.n);
    for (e, &c) in &p.t {
        if e[v] == 0 {
            rest.addterm(f, e.clone(), c);
        } else if e[v] == 1 && e.iter().map(|&k| k as usize).sum::<usize>() == 1 {
            coeff = c;
        } else {
            return None;
        }
    }
    (coeff != 0).then(|| rest.scale(f, f.neg(f.inv(coeff))))
}

fn project(p: &MP, active: &[usize]) -> MP {
    let mut r = MP::zero(active.len());
    for (e, &c) in &p.t {
        // Assert that no removed variable is silently dropped.
        assert!(e
            .iter()
            .enumerate()
            .all(|(v, &k)| k == 0 || active.contains(&v)));
        r.t.insert(active.iter().map(|&v| e[v]).collect(), c);
    }
    r
}

pub(super) fn reduce(sys: System, f: F, mode: &str) -> (System, Value) {
    if !matches!(mode, "affine" | "quadratic") {
        die("--reduce affine|quadratic");
    }
    let original_names = sys.names.clone();
    let original_plant = sys.plant.clone();
    let original_n = sys.names.len();
    let original_sha256 = sha256_hex(ms_text(&sys, f.p).as_bytes());
    let original_equations: Vec<Value> = sys.eqs.iter().map(terms).collect();
    let original_degree = sys.eqs.iter().map(MP::deg).max().unwrap_or(0);
    assert!(
        original_degree <= 2,
        "degree-capped reduction requires input degree <= 2"
    );
    let mut eqs = sys.eqs;
    let mut active: Vec<usize> = (0..original_n).collect();
    let mut steps: Vec<(usize, MP)> = vec![];
    loop {
        let mut selected = None;
        // Keep deterministic source-equation/variable order. Test every candidate
        // against ALL equations before accepting it, not only the pivot equation.
        'scan: for (k, p) in eqs.iter().enumerate() {
            if mode == "affine" && p.deg() > 1 {
                continue;
            }
            for &v in &active {
                let Some(rhs) = pivot(p, f, v) else { continue };
                let next: Vec<MP> = eqs
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != k)
                    .map(|(_, q)| substitute(q, f, v, &rhs))
                    .filter(|q| !q.is_zero())
                    .collect();
                if next
                    .iter()
                    .all(|q| q.deg() <= original_degree && q.t.len() <= 100_000)
                {
                    selected = Some((v, rhs, next));
                    break 'scan;
                }
            }
        }
        let Some((v, rhs, next)) = selected else {
            break;
        };
        active.retain(|&w| w != v);
        steps.push((v, rhs));
        eqs = next;
    }
    let reduced = System {
        names: active.iter().map(|&v| original_names[v].clone()).collect(),
        eqs: eqs.iter().map(|p| project(p, &active)).collect(),
        plant: active.iter().map(|&v| original_plant[v]).collect(),
    };
    // Store sparse exponent vectors as well as readable equations: no parser or
    // knowledge of the planted solution is needed for reconstruction.
    let cert = json!({
        "schema": "p1tool.constant-pivot-reduction/v1", "mode": mode,
        "p": f.p, "original_ms_sha256": original_sha256,
        "reduced_ms_sha256": sha256_hex(ms_text(&reduced, f.p).as_bytes()),
        "original_equations": original_equations,
        "original_names": original_names, "retained_indices": active,
        "original_N": original_n, "reduced_N": reduced.names.len(),
        "input_degree_cap": original_degree,
        "reconstruction_order": "reverse steps; seed retained coordinates, evaluate RHS, assign pivot",
        "steps": steps.iter().map(|(v, rhs)| json!({
            "variable_index": v, "variable": original_names[*v],
            "rhs": rhs.fmt(&original_names),
            "rhs_terms": rhs.t.iter().map(|(e,c)| json!({"exponents": e, "coefficient": c})).collect::<Vec<_>>()
        })).collect::<Vec<_>>()
    });
    (reduced, cert)
}

fn terms(p: &MP) -> Value {
    json!(p
        .t
        .iter()
        .map(|(e, c)| json!({"exponents":e,"coefficient":c}))
        .collect::<Vec<_>>())
}

fn eval_terms(f: F, terms: &Value, v: &[u64]) -> Result<u64, String> {
    let mut sum = 0;
    for term in terms.as_array().ok_or("missing terms")? {
        let mut c = term["coefficient"]
            .as_u64()
            .filter(|&x| x < f.p)
            .ok_or("invalid coefficient")?;
        let e = term["exponents"].as_array().ok_or("missing exponents")?;
        if e.len() != v.len() {
            return Err("exponent dimension mismatch".into());
        }
        for (k, &x) in e.iter().zip(v) {
            c = f.mul(c, f.pow(x, k.as_u64().ok_or("invalid exponent")?));
        }
        sum = f.add(sum, c);
    }
    Ok(sum)
}

/// Lift ANY reduced solution, then check every original equation. The caller
/// need not use or even supply a planted assignment. Certificate files must
/// come from the pinned emitter/input; this is not a signed proof backend.
pub(super) fn lift(cert: &Value, coordinates: &[u64]) -> Result<Vec<u64>, String> {
    if cert["schema"] != "p1tool.constant-pivot-reduction/v1" {
        return Err("unsupported map schema".into());
    }
    let p = cert["p"].as_u64().ok_or("missing p")?;
    if p <= 3 || p >= 1u64 << 32 || !is_prime(p) {
        return Err("invalid prime modulus".into());
    }
    let f = F::new(p);
    let n = cert["original_N"].as_u64().ok_or("missing N")? as usize;
    if n > 100_000 {
        return Err("oversized map".into());
    }
    let names: Vec<String> = cert["original_names"]
        .as_array()
        .ok_or("missing names")?
        .iter()
        .map(|v| {
            v.as_str()
                .map(String::from)
                .ok_or("invalid name".to_string())
        })
        .collect::<Result<_, _>>()?;
    if names.len() != n {
        return Err("name dimension mismatch".into());
    }
    let mut equations = vec![];
    for equation in cert["original_equations"]
        .as_array()
        .ok_or("missing original equations")?
    {
        let mut poly = MP::zero(n);
        for term in equation.as_array().ok_or("missing terms")? {
            let c = term["coefficient"]
                .as_u64()
                .filter(|&x| x < p)
                .ok_or("invalid coefficient")?;
            let e: Vec<u16> = term["exponents"]
                .as_array()
                .ok_or("missing exponents")?
                .iter()
                .map(|e| {
                    e.as_u64()
                        .and_then(|k| u16::try_from(k).ok())
                        .ok_or("invalid exponent".to_string())
                })
                .collect::<Result<_, _>>()?;
            if e.len() != n {
                return Err("exponent dimension mismatch".into());
            }
            poly.addterm(f, e, c);
        }
        equations.push(poly);
    }
    let original = System {
        names,
        eqs: equations,
        plant: vec![],
    };
    if cert["original_ms_sha256"].as_str()
        != Some(sha256_hex(ms_text(&original, p).as_bytes()).as_str())
    {
        return Err("original equation digest mismatch".into());
    }
    let retained = cert["retained_indices"]
        .as_array()
        .ok_or("missing retained indices")?;
    if retained.len() != coordinates.len() || coordinates.iter().any(|&x| x >= p) {
        return Err("coordinates must be canonical residues for every retained variable".into());
    }
    let mut v = vec![0; n];
    let mut assigned = vec![false; n];
    for (i, &x) in retained.iter().zip(coordinates) {
        let i = i.as_u64().ok_or("invalid index")? as usize;
        if i >= n || assigned[i] {
            return Err("invalid or duplicate retained index".into());
        }
        v[i] = x;
        assigned[i] = true;
    }
    for step in cert["steps"]
        .as_array()
        .ok_or("missing steps")?
        .iter()
        .rev()
    {
        let i = step["variable_index"]
            .as_u64()
            .ok_or("invalid pivot index")? as usize;
        if i >= n || assigned[i] {
            return Err("invalid or duplicate pivot index".into());
        }
        for term in step["rhs_terms"].as_array().ok_or("missing RHS")? {
            let exps = term["exponents"].as_array().ok_or("missing exponents")?;
            if exps.len() != n {
                return Err("exponent dimension mismatch".into());
            }
            for (j, e) in exps.iter().enumerate() {
                if e.as_u64().ok_or("invalid exponent")? > 0 && !assigned[j] {
                    return Err("RHS depends on an unassigned variable".into());
                }
            }
        }
        v[i] = eval_terms(f, &step["rhs_terms"], &v)?;
        assigned[i] = true;
    }
    if assigned.iter().any(|&a| !a) {
        return Err("incomplete reconstruction map".into());
    }
    for eq in cert["original_equations"]
        .as_array()
        .ok_or("missing original equations")?
    {
        if eval_terms(f, eq, &v)? != 0 {
            return Err("lift does not satisfy original equations".into());
        }
    }
    Ok(v)
}

pub(super) fn cmd_lift(args: &HashMap<String, String>) {
    let path = args
        .get("map")
        .unwrap_or_else(|| die("lift needs --map FILE.plant.json"));
    let values = args
        .get("values")
        .unwrap_or_else(|| die("lift needs --values FILE.json (array in retained variable order)"));
    let full: Value = serde_json::from_str(
        &std::fs::read_to_string(path).unwrap_or_else(|e| die(&e.to_string())),
    )
    .unwrap_or_else(|e| die(&e.to_string()));
    let coords: Vec<u64> = serde_json::from_str(
        &std::fs::read_to_string(values).unwrap_or_else(|e| die(&e.to_string())),
    )
    .unwrap_or_else(|e| die(&e.to_string()));
    let cert = &full["reduction"];
    let v = lift(cert, &coords).unwrap_or_else(|e| die(&e));
    let names = cert["original_names"]
        .as_array()
        .unwrap_or_else(|| die("missing names"));
    if names.len() != v.len() {
        die("name dimension mismatch");
    }
    let vals: Map<String, Value> = names
        .iter()
        .zip(v)
        .map(|(n, x)| {
            (
                n.as_str().unwrap_or_else(|| die("invalid name")).to_owned(),
                json!(x),
            )
        })
        .collect();
    println!(
        "{}",
        json!({"values":vals,"original_equations_vanish":true,
        "original_ms_sha256":cert["original_ms_sha256"],"standing":"producer-unverified"})
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unchecked_lift(f: F, cert: &Value, coordinates: &[u64]) -> Vec<u64> {
        let mut v = vec![0; cert["original_N"].as_u64().unwrap() as usize];
        for (a, &x) in cert["retained_indices"]
            .as_array()
            .unwrap()
            .iter()
            .zip(coordinates)
        {
            v[a.as_u64().unwrap() as usize] = x;
        }
        for step in cert["steps"].as_array().unwrap().iter().rev() {
            let mut rhs = 0;
            for term in step["rhs_terms"].as_array().unwrap() {
                let mut c = term["coefficient"].as_u64().unwrap();
                for (e, &x) in term["exponents"].as_array().unwrap().iter().zip(&v) {
                    c = f.mul(c, f.pow(x, e.as_u64().unwrap()));
                }
                rhs = f.add(rhs, c);
            }
            v[step["variable_index"].as_u64().unwrap() as usize] = rhs;
        }
        v
    }

    #[test]
    fn exhaustive_solution_bijection_and_nonconstant_pivot_refusal() {
        let f = F::new(5);
        let x = MP::var(3, 0);
        let y = MP::var(3, 1);
        let z = MP::var(3, 2);
        // x=y^2 and z=x+y, with y*z=1. Nonlinear substitution would make
        // the last equation cubic and must be refused by the degree cap.
        let eqs = vec![
            x.sub(f, &y.mul(f, &y)),
            z.sub(f, &x.add(f, &y)),
            y.mul(f, &z).sub(f, &MP::konst(3, 1)),
        ];
        assert!(pivot(&eqs[2], f, 1).is_none());
        for mode in ["affine", "quadratic"] {
            let sys = System {
                names: vec!["x".into(), "y".into(), "z".into()],
                eqs: eqs.clone(),
                plant: vec![0; 3],
            };
            let (r, cert) = reduce(sys, f, mode);
            assert!(r.eqs.iter().all(|p| p.deg() <= 2));
            assert!(r.names.len() < 3);
            for a in 0..125 {
                let v = vec![a % 5, (a / 5) % 5, a / 25];
                let w: Vec<u64> = cert["retained_indices"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|i| v[i.as_u64().unwrap() as usize])
                    .collect();
                let valid = eqs.iter().all(|p| p.eval(f, &v) == 0);
                let reduced_valid = r.eqs.iter().all(|p| p.eval(f, &w) == 0);
                assert_eq!(valid, reduced_valid && unchecked_lift(f, &cert, &w) == v);
                assert_eq!(lift(&cert, &w).is_ok(), reduced_valid);
                if reduced_valid {
                    assert!(eqs
                        .iter()
                        .all(|p| p.eval(f, &lift(&cert, &w).unwrap()) == 0));
                }
            }
        }
    }

    #[test]
    fn inconsistent_constant_survives_and_no_chart_is_saturated() {
        let f = F::new(5);
        let x = MP::var(2, 0);
        let y = MP::var(2, 1);
        let eqs = vec![x.clone(), x.sub(f, &MP::konst(2, 1)), y.mul(f, &y)];
        let (r, _) = reduce(
            System {
                names: vec!["x".into(), "y".into()],
                eqs,
                plant: vec![0; 2],
            },
            f,
            "quadratic",
        );
        assert!(r.eqs.iter().any(|p| p.deg() == 0 && !p.is_zero()));
        assert!(r.names.contains(&"y".to_string())); // y^2 is not a constant pivot
    }

    #[test]
    fn rr_dickson_fixture_lifts_without_degree_growth() {
        // Development fixture, not a measured claim cell or a cost comparison.
        let inst = sample_instance(1151, 294, 772, Kind::Dickson, 3, 4, None, 1);
        let f = F::new(inst.p);
        for mode in ["affine", "quadratic"] {
            let orig = build_rr(&inst);
            let (r, cert) = reduce(build_rr(&inst), f, mode);
            assert!(r.names.len() < orig.names.len());
            assert!(r.eqs.iter().all(|p| p.deg() <= 2));
            planted_check(&r, f);
            assert_eq!(lift(&cert, &r.plant).unwrap(), orig.plant);
        }
    }

    #[test]
    fn nonunit_coefficients_dependent_chain_and_malformed_maps() {
        let f = F::new(7);
        let n = 3;
        let x = MP::var(n, 0);
        let y = MP::var(n, 1);
        let z = MP::var(n, 2);
        let eqs = vec![
            x.scale(f, 2).sub(f, &y),
            y.scale(f, 3).sub(f, &z),
            z.sub(f, &MP::konst(n, 6)),
        ];
        let (r, cert) = reduce(
            System {
                names: vec!["x".into(), "y".into(), "z".into()],
                eqs,
                plant: vec![1, 2, 6],
            },
            f,
            "quadratic",
        );
        assert!(r.names.is_empty());
        assert_eq!(lift(&cert, &[]).unwrap(), vec![1, 2, 6]);
        let mut bad = cert.clone();
        bad["steps"][0]["variable_index"] = json!(999);
        assert!(lift(&bad, &[]).is_err());
        let mut bad = cert.clone();
        bad["original_equations"][0][0]["coefficient"] = json!(7);
        assert!(lift(&bad, &[]).is_err());
        let mut bad = cert.clone();
        bad["original_ms_sha256"] = json!("unrelated-input");
        assert!(lift(&bad, &[]).is_err());
    }
}
