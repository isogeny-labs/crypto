//! Explicit identity witnesses for a separately executed arithmetic checker.
//! No standard-basis solve occurs in the exported Singular program.
use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalTerm {
    pub exponents: Mono,
    pub coefficient: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CofactorTerm {
    pub generator: usize,
    pub exponents: Mono,
    pub coefficient: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Member {
    pub polynomial: Vec<CanonicalTerm>,
    pub proof: Vec<CofactorTerm>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PairReduction {
    pub left: usize,
    pub right: usize,
    pub proof: Vec<CofactorTerm>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Witness {
    pub version: u32,
    pub definitions: Vec<Member>,
    pub basis: Vec<Member>,
    pub input_reductions: Vec<Vec<CofactorTerm>>,
    pub pair_reductions: Vec<PairReduction>,
}
fn polynomial(f: &Poly) -> Vec<CanonicalTerm> {
    f.iter()
        .map(|(m, c)| CanonicalTerm {
            exponents: m.clone(),
            coefficient: *c,
        })
        .collect()
}
fn proof(p: &Proof) -> Vec<CofactorTerm> {
    p.iter()
        .map(|((i, m), c)| CofactorTerm {
            generator: *i,
            exponents: m.clone(),
            coefficient: *c,
        })
        .collect()
}
fn member(row: &Row) -> Member {
    Member {
        polynomial: polynomial(&row.poly),
        proof: proof(&row.proof),
    }
}
fn quotient_witness(target: Poly, reducers: &[Row], p: u64) -> Result<Vec<CofactorTerm>, String> {
    let mut remainder = normal_form(
        Row {
            poly: target,
            proof: Proof::new(),
        },
        reducers,
        p,
    );
    if !remainder.poly.is_empty() {
        return Err("replay: nonzero division remainder".into());
    }
    for c in remainder.proof.values_mut() {
        *c = minus(*c, p);
    }
    Ok(proof(&remainder.proof))
}
/// Build all witnesses from a constructor's already checked basis, with no oracle.
pub fn produce(system: &System, compact: bool) -> Result<Witness, String> {
    let input = system.decode()?;
    let n = system.variables.len();
    let p = system.prime;
    let certificate = if compact {
        quotient::compact::solve(&input, n, p)?.certificate
    } else {
        quotient::compact::Certificate {
            definitions: Vec::new(),
            basis: quotient::solve(&input, n, p)?.0,
        }
    };
    complete(
        system,
        certificate.definitions.iter().map(member).collect(),
        certificate.basis.iter().map(member).collect(),
    )
}
/// Complete witnesses for independently produced membership rows. No supplied
/// identity is accepted arithmetically until the exported checker executes.
pub fn complete(
    system: &System,
    definitions: Vec<Member>,
    basis: Vec<Member>,
) -> Result<Witness, String> {
    let n = system.variables.len();
    let p = system.prime;
    if !(3..=quotient::MAX_VARIABLES).contains(&n)
        || definitions.len() > n
        || basis.is_empty()
        || basis.len() > 256
    {
        return Err("replay: completion shape budget".into());
    }
    let input = system.decode()?;
    let mut pairs = Vec::new();
    for right in 0..basis.len() {
        for left in 0..right {
            pairs.push(PairReduction {
                left,
                right,
                proof: Vec::new(),
            });
        }
    }
    let mut witness = Witness {
        version: 1,
        definitions,
        basis,
        input_reductions: vec![Vec::new(); input.len()],
        pair_reductions: pairs,
    };
    // Validate untrusted imported rows before division. Empty placeholders are
    // not acceptance witnesses and this intermediate script is never executed.
    singular_script(system, &witness)?;
    let polynomials = witness
        .basis
        .iter()
        .map(|r| decode_poly(&r.polynomial, n, p))
        .collect::<Result<Vec<_>, _>>()?;
    let reducers = input_rows(&polynomials, n);
    witness.input_reductions = input
        .into_iter()
        .map(|f| quotient_witness(f, &reducers, p))
        .collect::<Result<_, _>>()?;
    for row in &mut witness.pair_reductions {
        let target = s_polynomial(&reducers[row.left], &reducers[row.right], p).poly;
        row.proof = quotient_witness(target, &reducers, p)?;
    }
    singular_script(system, &witness)?;
    Ok(witness)
}
fn checked_monomial(m: &Mono, n: usize) -> Option<usize> {
    if m.len() != n {
        return None;
    }
    m.iter()
        .try_fold(0usize, |s, e| s.checked_add(*e))
        .filter(|d| *d <= 64)
}
fn decode_poly(f: &[CanonicalTerm], n: usize, p: u64) -> Result<Poly, String> {
    if f.is_empty() || f.len() > 5000 {
        return Err("replay: polynomial term budget/zero row".into());
    }
    let mut result = Poly::new();
    for term in f {
        if checked_monomial(&term.exponents, n).is_none()
            || term.coefficient == 0
            || term.coefficient >= p
            || result
                .insert(term.exponents.clone(), term.coefficient)
                .is_some()
        {
            return Err("replay: noncanonical polynomial".into());
        }
    }
    Ok(result)
}
fn mono(m: &Mono) -> String {
    let factors: Vec<_> = m
        .iter()
        .enumerate()
        .filter(|(_, e)| **e != 0)
        .map(|(i, e)| format!("x{i}^{e}"))
        .collect();
    if factors.is_empty() {
        "1".into()
    } else {
        factors.join("*")
    }
}
fn expression(f: &Poly) -> String {
    if f.is_empty() {
        return "0".into();
    }
    f.iter()
        .map(|(m, c)| format!("{c}*{}", mono(m)))
        .collect::<Vec<_>>()
        .join("+")
}
struct Budget {
    terms: usize,
    work: usize,
    n: usize,
    p: u64,
}
impl Budget {
    fn cofactor(
        &mut self,
        terms: &[CofactorTerm],
        generators: &[Poly],
        names: &[String],
        bound: Option<&Mono>,
        script: &mut String,
    ) -> Result<String, String> {
        let n = self.n;
        let p = self.p;
        if terms.len() > 50000 {
            return Err("replay: identity cofactor budget".into());
        }
        self.terms = self
            .terms
            .checked_add(terms.len())
            .ok_or("replay: count overflow")?;
        if self.terms > 100000 {
            return Err("replay: total cofactor budget".into());
        }
        let mut seen = BTreeSet::new();
        let mut work = 0usize;
        let mut summands = Vec::new();
        for term in terms {
            let g = generators
                .get(term.generator)
                .ok_or("replay: future/out-of-range reference")?;
            if term.coefficient == 0
                || term.coefficient >= p
                || checked_monomial(&term.exponents, n).is_none()
                || !seen.insert((term.generator, term.exponents.clone()))
                || degree(g) + term.exponents.iter().sum::<usize>() > 64
            {
                return Err("replay: malformed/duplicate cofactor".into());
            }
            work = work.checked_add(g.len()).ok_or("replay: work overflow")?;
            let item = format!(
                "{}*{}*{}",
                term.coefficient,
                mono(&term.exponents),
                names[term.generator]
            );
            if let Some(bound) = bound {
                let product: Mono = lead(g)
                    .iter()
                    .zip(&term.exponents)
                    .map(|(a, b)| a + b)
                    .collect();
                if cmp(&product, bound) != Ordering::Less {
                    return Err("replay: nonstandard S-pair representation".into());
                }
                script.push_str(&format!(
                    "if(leadmonom({item})>={}){{print(\"REPLAY_FAIL:bound\");quit;}}\n",
                    mono(bound)
                ));
            }
            summands.push(item);
        }
        self.work = self.work.checked_add(work).ok_or("replay: work overflow")?;
        if work > 1000000 || self.work > 5000000 {
            return Err("replay: identity/total work budget".into());
        }
        Ok(if summands.is_empty() {
            "0".into()
        } else {
            summands.join("+")
        })
    }
}
fn identity(script: &mut String, target: &str, rhs: &str) {
    script.push_str(&format!(
        "if(({target})-({rhs})!=0){{print(\"REPLAY_FAIL:identity\");quit;}}\n"
    ));
}
/// Validate shape/coverage/order, then export arithmetic checks, without solving.
/// Successful execution must separately report CERTIFICATE_REPLAY_OK.
pub fn singular_script(system: &System, witness: &Witness) -> Result<String, String> {
    let n = system.variables.len();
    let p = system.prime;
    if !(3..=quotient::MAX_VARIABLES).contains(&n)
        || p > i32::MAX as u64
        || p < 3
        || !prime(p)
        || witness.version != 1
        || witness.definitions.len() > n
        || witness.basis.is_empty()
        || witness.basis.len() > 256
    {
        return Err(
            "replay: unsupported field/shape/version (Singular characteristic <=2^31-1)".into(),
        );
    }
    let mut generators = system.decode()?;
    if generators.is_empty()
        || generators.len() > 32
        || generators.iter().any(|g| degree(g) > 4 || g.len() > 32)
    {
        return Err("replay: original input budget".into());
    }
    let input_count = generators.len();
    let mut names: Vec<_> = (0..input_count).map(|i| format!("o{i}")).collect();
    let mut script = format!(
        "ring r={p},({}),dp;\n",
        (0..n)
            .map(|i| format!("x{i}"))
            .collect::<Vec<_>>()
            .join(",")
    );
    for (name, g) in names.iter().zip(&generators) {
        script += &format!("poly {name}={};\n", expression(g));
    }
    let mut budget = Budget {
        terms: 0,
        work: 0,
        n,
        p,
    };
    for (i, row) in witness.definitions.iter().enumerate() {
        let poly = decode_poly(&row.polynomial, n, p)?;
        let rhs = budget.cofactor(&row.proof, &generators, &names, None, &mut script)?;
        let name = format!("d{i}");
        script += &format!("poly {name}={};\n", expression(&poly));
        identity(&mut script, &name, &rhs);
        generators.push(poly);
        names.push(name);
    }
    let mut basis = Vec::new();
    let mut basis_names = Vec::new();
    for (i, row) in witness.basis.iter().enumerate() {
        let poly = decode_poly(&row.polynomial, n, p)?;
        let rhs = budget.cofactor(&row.proof, &generators, &names, None, &mut script)?;
        if poly[lead(&poly)] != 1 {
            return Err("replay: non-monic basis".into());
        }
        let name = format!("g{i}");
        script+=&format!("poly {name}={};\nif(leadcoef({name})!=1 || leadmonom({name})!={}){{print(\"REPLAY_FAIL:leading\");quit;}}\n",expression(&poly),mono(lead(&poly)));
        identity(&mut script, &name, &rhs);
        basis.push(poly);
        basis_names.push(name);
    }
    for (i, g) in basis.iter().enumerate() {
        for (j, h) in basis.iter().enumerate() {
            if i != j && g.keys().any(|m| divides(lead(h), m)) {
                return Err("replay: non-reduced basis".into());
            }
        }
    }
    if witness.input_reductions.len() != input_count
        || witness.pair_reductions.len() != basis.len() * (basis.len() - 1) / 2
    {
        return Err("replay: missing input/S-pair coverage".into());
    }
    for (i, terms) in witness.input_reductions.iter().enumerate() {
        let rhs = budget.cofactor(terms, &basis, &basis_names, None, &mut script)?;
        identity(&mut script, &format!("o{i}"), &rhs);
    }
    let mut pair_index = 0;
    for right in 0..basis.len() {
        for left in 0..right {
            let row = &witness.pair_reductions[pair_index];
            pair_index += 1;
            if row.left != left || row.right != right {
                return Err("replay: missing/duplicate S-pair index".into());
            }
            let a = lead(&basis[left]);
            let b = lead(&basis[right]);
            let lcm: Mono = a.iter().zip(b).map(|(x, y)| (*x).max(*y)).collect();
            let rhs = budget.cofactor(&row.proof, &basis, &basis_names, Some(&lcm), &mut script)?;
            identity(
                &mut script,
                &format!(
                    "{}*g{left}-{}*g{right}",
                    mono(&difference(&lcm, a)),
                    mono(&difference(&lcm, b))
                ),
                &rhs,
            );
        }
    }
    script += "print(\"CERTIFICATE_REPLAY_OK\");\nquit;\n";
    if script.len() > 16 * 1024 * 1024 {
        return Err("replay: checker script byte cap".into());
    }
    Ok(script)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (System, Witness) {
        let system = workload::generate(2147483647, 3, 7).unwrap().system;
        let witness = produce(&system, true).unwrap();
        (system, witness)
    }
    #[test]
    fn original_input_and_both_constructor_certificates_have_complete_witnesses() {
        let (system, compact) = fixture();
        let expanded = produce(&system, false).unwrap();
        assert_eq!(
            compact
                .basis
                .iter()
                .map(|r| decode_poly(&r.polynomial, 3, system.prime).unwrap())
                .collect::<Vec<_>>(),
            expanded
                .basis
                .iter()
                .map(|r| decode_poly(&r.polynomial, 3, system.prime).unwrap())
                .collect::<Vec<_>>()
        );
        assert_eq!(compact.input_reductions.len(), system.polynomials.len());
        for witness in [compact, expanded] {
            let script = singular_script(&system, &witness).unwrap();
            assert!(script.ends_with("print(\"CERTIFICATE_REPLAY_OK\");\nquit;\n"));
            for forbidden in [
                "std(",
                "slimgb(",
                "reduce(",
                "division(",
                "lift(",
                "liftstd(",
            ] {
                assert!(!script.contains(forbidden));
            }
        }
    }
    #[test]
    fn reject_future_nodes_missing_coverage_and_malformed_shapes() {
        let (system, w) = fixture();
        let mut bad = w.clone();
        bad.definitions[0].proof[0].generator = system.polynomials.len();
        assert!(singular_script(&system, &bad)
            .unwrap_err()
            .contains("reference"));
        let mut bad = w.clone();
        bad.input_reductions.pop();
        assert!(singular_script(&system, &bad)
            .unwrap_err()
            .contains("coverage"));
        let mut bad = w.clone();
        bad.pair_reductions.pop();
        assert!(singular_script(&system, &bad)
            .unwrap_err()
            .contains("coverage"));
        let mut bad = w.clone();
        bad.basis[0].polynomial[0].exponents.push(0);
        assert!(singular_script(&system, &bad)
            .unwrap_err()
            .contains("polynomial"));
        let mut bad = w.clone();
        bad.basis.push(w.basis[0].clone());
        assert!(singular_script(&system, &bad)
            .unwrap_err()
            .contains("non-reduced"));
        let mut bad = w.clone();
        bad.definitions[0].proof[0].coefficient = system.prime;
        assert!(singular_script(&system, &bad)
            .unwrap_err()
            .contains("cofactor"));
        let mut wide = system;
        wide.prime = 9007199254740881;
        assert!(singular_script(&wide, &w)
            .unwrap_err()
            .contains("characteristic"));
    }
    #[test]
    fn reject_true_but_nonstandard_trivial_s_pair_identity() {
        let (system, mut w) = fixture();
        let basis: Vec<_> = w
            .basis
            .iter()
            .map(|r| decode_poly(&r.polynomial, 3, system.prime).unwrap())
            .collect();
        let row = &mut w.pair_reductions[0];
        let a = lead(&basis[row.left]);
        let b = lead(&basis[row.right]);
        let lcm = a.iter().zip(b).map(|(x, y)| (*x).max(*y)).collect();
        row.proof = vec![
            CofactorTerm {
                generator: row.left,
                exponents: difference(&lcm, a),
                coefficient: 1,
            },
            CofactorTerm {
                generator: row.right,
                exponents: difference(&lcm, b),
                coefficient: system.prime - 1,
            },
        ];
        assert!(singular_script(&system, &w)
            .unwrap_err()
            .contains("nonstandard"));
    }
}
