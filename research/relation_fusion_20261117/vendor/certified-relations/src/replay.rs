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
    /// Checked Buchberger product criterion; never a producer-only assertion.
    #[serde(default, skip_serializing_if = "is_false")]
    pub product_criterion: bool,
    pub proof: Vec<CofactorTerm>,
}
fn is_false(value: &bool) -> bool {
    !*value
}
fn coprime(a: &Mono, b: &Mono) -> bool {
    a.iter().zip(b).all(|(x, y)| *x == 0 || *y == 0)
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
    produce_impl(system, compact, false)
}
/// Opt-in shorter replay, with the same constructor and complete answer.
pub fn produce_product(system: &System, compact: bool) -> Result<Witness, String> {
    produce_impl(system, compact, true)
}
#[derive(Serialize)]
pub struct SparseWitness {
    pub witness: Witness,
    pub constructor: quotient::compact::Info,
    pub subset_succeeded: bool,
    pub subset_error: Option<String>,
}
pub fn produce_sparse(system: &System) -> Result<SparseWitness, String> {
    let input = system.decode()?;
    let output = quotient::compact::solve_sparse(&input, system.variables.len(), system.prime)?;
    let certificate = output.output.certificate;
    Ok(SparseWitness {
        witness: complete_product(
            system,
            certificate.definitions.iter().map(member).collect(),
            certificate.basis.iter().map(member).collect(),
        )?,
        constructor: output.output.info,
        subset_succeeded: output.subset_succeeded,
        subset_error: output.subset_error,
    })
}
#[derive(Serialize)]
pub struct NativeWitness {
    pub witness: Witness,
    pub constructor: serde_json::Value,
}
/// Opt-in native pair-verification optimization, keeping witness completion and
/// its validation unchanged. The external consumer is a separate caller choice.
pub fn produce_native_product(system: &System, kind: &str) -> Result<NativeWitness, String> {
    native_product_impl(system, kind, CompletionExports::Both)
}
/// Native-owned rows only: omit the preliminary discarded text export, retaining
/// the final structural export. Constructor and consumer obligations are unchanged.
pub fn produce_native_single_export(system: &System, kind: &str) -> Result<NativeWitness, String> {
    native_product_impl(system, kind, CompletionExports::Final)
}
/// Unaccepted serialized evidence. This type contains no checked basis and must
/// pass a complete arithmetic consumer before any answer is exposed.
#[must_use = "An unverified native wire requires complete arithmetic replay"]
pub struct UnverifiedNativeWire {
    wire: Vec<u8>,
    constructor: serde_json::Value,
}
/// Exact native-kernel ablations. All modes retain the same complete checks.
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeKernels {
    Legacy,
    Ordered,
    Cached,
    Both,
    BothUpdates,
    BothUpdatesSum,
    BothUpdatesLead,
    BothUpdatesFused,
    BothUpdatesFrontier,
    FrontierPackedProducts,
    FrontierPackedBatch,
}
impl NativeKernels {
    fn bits(self) -> u16 {
        match self {
            Self::Legacy => 0,
            Self::Ordered => 1,
            Self::Cached => 2,
            Self::Both => 3,
            Self::BothUpdates => 7,
            Self::BothUpdatesSum => 15,
            Self::BothUpdatesLead => 23,
            Self::BothUpdatesFused => 39,
            Self::BothUpdatesFrontier => 103,
            Self::FrontierPackedProducts => 231,
            Self::FrontierPackedBatch => 487,
        }
    }
}
/// Scoped native-owned construction only; the opaque wire still requires full
/// independent-code-path arithmetic replay before any accepted basis exists.
pub fn produce_native_for_rust_with_kernels(
    system: &System,
    kind: &str,
    kernels: NativeKernels,
) -> Result<UnverifiedNativeWire, String> {
    with_native_kernel_bits(kernels.bits(), || produce_native_for_rust(system, kind))
}
impl UnverifiedNativeWire {
    /// Transfer proof bytes and diagnostic metadata, never an accepted basis.
    pub fn into_parts(self) -> (Vec<u8>, serde_json::Value) {
        (self.wire, self.constructor)
    }
}
/// Complete native-owned, constructor-verified rows without rendering unused CAS
/// text. The p31 Rust consumer must validate the entire serialized proof; this
/// function does not accept imported rows or perform that final acceptance.
pub fn produce_native_for_rust(
    system: &System,
    kind: &str,
) -> Result<UnverifiedNativeWire, String> {
    if system.prime != 2147483647 {
        return Err("native Rust wire: consumer requires p2147483647".into());
    }
    let produced = native_product_impl(system, kind, CompletionExports::RustConsumer)?;
    let wire = serde_json::to_vec(&produced.witness).map_err(|e| e.to_string())?;
    if wire.len() > 16 * 1024 * 1024 {
        return Err("native Rust wire: serialized proof byte cap".into());
    }
    Ok(UnverifiedNativeWire {
        wire,
        constructor: produced.constructor,
    })
}

/// Full native construction and complete wire for the separate packed checker.
/// This remains unverified bytes; no accepted basis is exposed by this API.
/// Native checks and every ideal-equality/Buchberger obligation are retained.
pub fn produce_native_for_packed(
    system: &System,
    kind: &str,
    kernels: NativeKernels,
) -> Result<(Vec<u8>, serde_json::Value), String> {
    if ![2147483647, 9007199254740881].contains(&system.prime) {
        return Err("packed wire supports only the two fixed primes".into());
    }
    let produced = with_native_kernel_bits(kernels.bits(), || {
        native_product_impl(system, kind, CompletionExports::RustConsumer)
    })?;
    let wire = serde_json::to_vec(&produced.witness).map_err(|e| e.to_string())?;
    if wire.len() > 16 * 1024 * 1024 {
        return Err("packed serialized proof byte cap".into());
    }
    Ok((wire, produced.constructor))
}

/// Construct the same full grevlex output using direct cubics and a
/// provenance-carrying Buchberger engine. No resultant, FGLM or reference
/// answer is used. The returned complete wire is still unverified bytes.
pub fn produce_cubic_for_packed(system: &System, kernels: NativeKernels) -> Result<Vec<u8>, String> {
    with_native_kernel_bits(kernels.bits(), || cubic_for_packed_impl(system))
}
fn cubic_for_packed_impl(system: &System) -> Result<Vec<u8>, String> {
    let input = system.decode()?;
    let n = system.variables.len();
    let p = system.prime;
    if ![2147483647,9007199254740881].contains(&p)
        || !(3..=quotient::MAX_VARIABLES).contains(&n)
        || input.len()<4 || input.len()>32
        || input.iter().any(|f| f.len()>32 || degree(f)>4)
    {return Err("cubic constructor field/input budget".into());}
    let pairs=cheap::consequences(&input[..4],n,p);
    if pairs.len()!=2 {return Err("cubic constructor needs two recognized initial pairs".into());}
    let definitions:Vec<_>=pairs.into_iter().map(|r|r.cubic_syzygy).collect();
    let mut augmented=input.clone();augmented.extend(definitions.iter().map(|r|r.poly.clone()));
    let basis=reference_criteria(&augmented,n,p)?;
    let witness=complete_impl(system,definitions.iter().map(member).collect(),basis.iter().map(member).collect(),true,CompletionExports::RustConsumer)?;
    let wire=serde_json::to_vec(&witness).map_err(|e|e.to_string())?;
    if wire.len()>16*1024*1024 {return Err("cubic constructor wire byte cap".into());}
    Ok(wire)
}
/// Version2 original-generator DAG basis, with full native construction checks.
/// Bytes remain unverified until a separate explicit DAG consumer accepts them.
pub fn produce_cubic_dag_for_packed(system:&System,kernels:NativeKernels)->Result<Vec<u8>,String> {
    let output=with_native_kernel_bits(kernels.bits(),||super::dag::solve(&system.decode()?,system.variables.len(),system.prime))?;
    let witness=complete_impl(system,output.definitions.iter().map(member).collect(),output.basis.iter().map(member).collect(),true,CompletionExports::PackedDag)?;
    let wire=serde_json::to_vec(&witness).map_err(|e|e.to_string())?;
    if wire.len()>16*1024*1024 {return Err("dag complete wire byte cap".into());}Ok(wire)
}
pub fn produce_cubic_dag_pruned_for_packed(system:&System,kernels:NativeKernels)->Result<(Vec<u8>,Value),String> {
    let (output,trace)=with_native_kernel_bits(kernels.bits(),||super::dag::solve_pruned(&system.decode()?,system.variables.len(),system.prime))?;
    let witness=complete_impl(system,output.definitions.iter().map(member).collect(),output.basis.iter().map(member).collect(),true,CompletionExports::PackedDag)?;
    let wire=serde_json::to_vec(&witness).map_err(|e|e.to_string())?;if wire.len()>16*1024*1024 {return Err("pruned dag wire byte cap".into());}Ok((wire,trace))
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum CompletionExports {
    Both,
    Final,
    RustConsumer,
    PackedDag,
}
// Only this private native factory may select Final/RustConsumer. Every branch
// obtains its rows from a constructor with its complete native checks intact.
fn native_product_impl(
    system: &System,
    kind: &str,
    exports: CompletionExports,
) -> Result<NativeWitness, String> {
    let input = system.decode()?;
    let n = system.variables.len();
    let p = system.prime;
    let (certificate, constructor) = match kind {
        "compact" => {
            let output = quotient::compact::solve_product(&input, n, p)?;
            (output.certificate, serde_json::json!({"info":output.info}))
        }
        "compact-sparse" => {
            let output = quotient::compact::solve_sparse_product(&input, n, p)?;
            (
                output.output.certificate,
                serde_json::json!({"info":output.output.info,"subset_succeeded":output.subset_succeeded,"subset_error":output.subset_error}),
            )
        }
        "expanded" => {
            let (basis, info) = quotient::solve_product(&input, n, p)?;
            (
                quotient::compact::Certificate {
                    definitions: Vec::new(),
                    basis,
                },
                serde_json::json!({"info":info}),
            )
        }
        _ => return Err("native product constructor kind".into()),
    };
    let witness = complete_impl(
        system,
        certificate.definitions.iter().map(member).collect(),
        certificate.basis.iter().map(member).collect(),
        true,
        exports,
    )?;
    Ok(NativeWitness {
        witness,
        constructor,
    })
}
fn produce_impl(system: &System, compact: bool, product: bool) -> Result<Witness, String> {
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
    complete_impl(
        system,
        certificate.definitions.iter().map(member).collect(),
        certificate.basis.iter().map(member).collect(),
        product,
        CompletionExports::Both,
    )
}
/// Complete witnesses for independently produced membership rows. No supplied
/// identity is accepted arithmetically until the exported checker executes.
pub fn complete(
    system: &System,
    definitions: Vec<Member>,
    basis: Vec<Member>,
) -> Result<Witness, String> {
    complete_impl(system, definitions, basis, false, CompletionExports::Both)
}
/// Opt-in completion for mature producers; product-criterion acceptance is
/// checked by the same consumer used for locally constructed witnesses.
pub fn complete_product(
    system: &System,
    definitions: Vec<Member>,
    basis: Vec<Member>,
) -> Result<Witness, String> {
    complete_impl(system, definitions, basis, true, CompletionExports::Both)
}
fn complete_impl(
    system: &System,
    definitions: Vec<Member>,
    basis: Vec<Member>,
    product: bool,
    exports: CompletionExports,
) -> Result<Witness, String> {
    let n = system.variables.len();
    let p = system.prime;
    if !(3..=quotient::MAX_VARIABLES).contains(&n)
        || definitions.len() > if exports == CompletionExports::PackedDag {512} else {n}
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
                product_criterion: false,
                proof: Vec::new(),
            });
        }
    }
    let mut witness = Witness {
        version: if exports == CompletionExports::PackedDag {2} else {1},
        definitions,
        basis,
        input_reductions: vec![Vec::new(); input.len()],
        pair_reductions: pairs,
    };
    // Validate untrusted imported rows before division. Empty placeholders are
    // not acceptance witnesses and this intermediate script is never executed.
    if exports == CompletionExports::Both {
        singular_script(system, &witness)?;
    }
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
        if product
            && coprime(
                lead(&reducers[row.left].poly),
                lead(&reducers[row.right].poly),
            )
        {
            row.product_criterion = true;
            continue;
        }
        let target = s_polynomial(&reducers[row.left], &reducers[row.right], p).poly;
        row.proof = quotient_witness(target, &reducers, p)?;
    }
    if !matches!(exports,CompletionExports::RustConsumer|CompletionExports::PackedDag) {
        singular_script(system, &witness)?;
    }
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
    singular_script_impl(system, witness, false,false)
}
/// Two fixed prime residue rings, including the 53-bit GMP coefficient backend.
/// This checks identities only; it makes no standard-basis solver speed claim.
pub fn singular_script_residue(system: &System, witness: &Witness) -> Result<String, String> {
    if ![2147483647, 9007199254740881].contains(&system.prime) {
        return Err("residue checker supports only the two fixed primes".into());
    }
    singular_script_impl(system, witness, true,false)
}
/// Explicit version2 shared-row DAG replay, same full identity obligations.
pub fn singular_script_dag(system:&System,witness:&Witness)->Result<String,String> {
    if ![2147483647,9007199254740881].contains(&system.prime) {return Err("dag residue prime roster".into());}
    singular_script_impl(system,witness,true,true)
}
fn singular_script_impl(system: &System, witness: &Witness, residue: bool,dag:bool) -> Result<String, String> {
    let n = system.variables.len();
    let p = system.prime;
    if !(3..=quotient::MAX_VARIABLES).contains(&n)
        || (!residue && p > i32::MAX as u64)
        || p < 3
        || !prime(p)
        || witness.version != if dag {2} else {1}
        || witness.definitions.len() > if dag {512} else {n}
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
    let coefficients = if residue && p > i32::MAX as u64 {
        format!("(integer,{p})")
    } else {
        p.to_string()
    };
    let mut script = format!(
        "ring r={coefficients},({}),dp;\n",
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
            if row.product_criterion {
                if !row.proof.is_empty() || !coprime(a, b) {
                    return Err("replay: invalid product criterion".into());
                }
                // The actual leading monomials are already bound above. Check
                // the product criterion again in the arithmetic consumer.
                script += &format!(
                    "if(leadmonom(g{left})*leadmonom(g{right})!={}){{print(\"REPLAY_FAIL:product\");quit;}}\n",
                    mono(&lcm)
                );
                continue;
            }
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
    #[test]
    fn native_completion_variants_preserve_exact_wire_bytes() {
        for n in [3, quotient::MAX_VARIABLES] {
            let edges: Vec<_> = (1..n).map(|i| (0, i)).collect();
            #[cfg(feature = "scale-screen")]
            let system = workload::generate_scale_graph(2147483647, n, 7, &edges)
                .unwrap()
                .system;
            #[cfg(not(feature = "scale-screen"))]
            let system = workload::generate_graph(2147483647, n, 7, &edges)
                .unwrap()
                .system;
            for kind in ["compact", "expanded", "compact-sparse"] {
                let old = produce_native_product(&system, kind).unwrap();
                let single = produce_native_single_export(&system, kind).unwrap();
                let (wire, metadata) = produce_native_for_rust(&system, kind).unwrap().into_parts();
                let expected = serde_json::to_vec(&old.witness).unwrap();
                assert_eq!(serde_json::to_vec(&single.witness).unwrap(), expected);
                assert_eq!(wire, expected);
                assert_eq!(metadata, old.constructor);
                assert_eq!(single.constructor, old.constructor);
                for kernels in [
                    NativeKernels::Legacy,
                    NativeKernels::Ordered,
                    NativeKernels::Cached,
                    NativeKernels::Both,
                    NativeKernels::BothUpdates,
                    NativeKernels::BothUpdatesSum,
                    NativeKernels::BothUpdatesLead,
                    NativeKernels::BothUpdatesFused,
                    NativeKernels::BothUpdatesFrontier,
                ] {
                    let (wire, metadata) =
                        produce_native_for_rust_with_kernels(&system, kind, kernels)
                            .unwrap()
                            .into_parts();
                    assert_eq!(wire, expected);
                    assert_eq!(metadata, old.constructor);
                    assert_eq!(native_kernel_bits(), 0);
                }
            }
        }
    }
    #[test]
    fn unverified_native_wire_rejects_unsupported_consumer_scope() {
        let wide = workload::generate(9007199254740881, 3, 7).unwrap().system;
        assert!(produce_native_for_rust(&wide, "compact").is_err());
        let small = workload::generate(2147483647, 2, 7).unwrap().system;
        assert!(produce_native_for_rust(&small, "compact").is_err());
        let system = workload::generate(2147483647, 3, 7).unwrap().system;
        assert!(produce_native_for_rust(&system, "imported").is_err());
    }
    #[test]
    fn opt_in_native_product_verification_keeps_complete_original_ideals() {
        for n in [3, 6] {
            let s = workload::generate(2147483647, n, 7).unwrap().system;
            let input = s.decode().unwrap();
            let legacy = quotient::compact::solve(&input, n, s.prime).unwrap();
            for kind in ["compact", "compact-sparse", "expanded"] {
                let output = produce_native_product(&s, kind).unwrap();
                let actual: Vec<_> = output
                    .witness
                    .basis
                    .iter()
                    .map(|r| decode_poly(&r.polynomial, n, s.prime).unwrap())
                    .collect();
                assert_eq!(
                    actual,
                    legacy
                        .certificate
                        .basis
                        .iter()
                        .map(|r| r.poly.clone())
                        .collect::<Vec<_>>()
                );
                assert!(singular_script(&s, &output.witness).is_ok());
            }
            let mut bad = legacy.certificate.clone();
            bad.definitions[0].proof.clear();
            assert!(!quotient::compact::verify_product(&bad, &input, n, s.prime));
            let bad_basis = vec![Row {
                poly: BTreeMap::from([(vec![0; n], 1)]),
                proof: Proof::new(),
            }];
            assert!(!check_reference_product(&bad_basis, &input, n, s.prime));
        }
    }
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
    #[test]
    fn product_criterion_preserves_legacy_wire_and_complete_pair_coverage() {
        let (system, legacy) = fixture();
        let short = produce_product(&system, true).unwrap();
        assert!(!serde_json::to_string(&legacy)
            .unwrap()
            .contains("product_criterion"));
        assert_eq!(
            serde_json::to_value(&legacy.basis).unwrap(),
            serde_json::to_value(&short.basis).unwrap()
        );
        assert_eq!(legacy.pair_reductions.len(), short.pair_reductions.len());
        assert!(short.pair_reductions.iter().any(|r| r.product_criterion));
        assert!(short
            .pair_reductions
            .iter()
            .filter(|r| r.product_criterion)
            .all(|r| r.proof.is_empty()));
        assert!(singular_script(&system, &short)
            .unwrap()
            .contains("REPLAY_FAIL:product"));
        let mut bad = short.clone();
        bad.pair_reductions[0].right = 0;
        assert!(singular_script(&system, &bad)
            .unwrap_err()
            .contains("index"));
        let mut bad = short.clone();
        let row = bad
            .pair_reductions
            .iter_mut()
            .find(|r| r.product_criterion)
            .unwrap();
        row.proof.push(CofactorTerm {
            generator: 0,
            exponents: vec![0; 3],
            coefficient: 1,
        });
        assert!(singular_script(&system, &bad)
            .unwrap_err()
            .contains("product criterion"));
    }
    #[test]
    fn noncoprime_product_marker_is_rejected_by_consumer() {
        let (system, mut w) = fixture();
        w.basis.truncate(2);
        w.pair_reductions.truncate(1);
        // Placeholders reach the product-shape boundary without stale reducer
        // indices from the six-row fixture; this script is never executed.
        w.input_reductions = vec![Vec::new(); system.polynomials.len()];
        for (row, m) in w
            .basis
            .iter_mut()
            .zip([vec![2, 0, 0], vec![1, 1, 0], vec![0, 0, 1]])
        {
            row.polynomial = vec![CanonicalTerm {
                exponents: m,
                coefficient: 1,
            }];
        }
        w.pair_reductions[0].product_criterion = true;
        w.pair_reductions[0].proof.clear();
        assert!(singular_script(&system, &w)
            .unwrap_err()
            .contains("product criterion"));
    }
}
