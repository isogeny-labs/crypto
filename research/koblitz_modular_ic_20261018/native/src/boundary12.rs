//! Counterpoints to the actual full quotient homogenized circuits.
use super::*;
#[path = "saturation13.rs"]
pub(crate) mod saturation13;
#[derive(Clone)]
struct Lin {
    n: usize,
    rows: Vec<Option<(Vec<u64>, u8)>>,
}
impl Lin {
    fn new(n: usize) -> Self {
        Self {
            n,
            rows: vec![None; n],
        }
    }
    fn equation(&mut self, vars: &[usize], v: u8) -> bool {
        let mut a = vec![0u64; self.n.div_ceil(64)];
        for i in vars {
            a[i / 64] ^= 1u64 << (i % 64);
        }
        self.insert(a, v)
    }
    fn insert(&mut self, mut a: Vec<u64>, mut v: u8) -> bool {
        loop {
            let Some(p) = a
                .iter()
                .enumerate()
                .find(|(_, x)| **x != 0)
                .map(|(i, x)| i * 64 + x.trailing_zeros() as usize)
            else {
                return v == 0;
            };
            if let Some((r, b)) = &self.rows[p] {
                for (x, y) in a.iter_mut().zip(r) {
                    *x ^= *y;
                }
                v ^= *b;
            } else {
                self.rows[p] = Some((a, v));
                return true;
            }
        }
    }
    fn model(&self) -> Vec<u8> {
        let mut z = vec![0u8; self.n];
        for i in (0..self.n).rev() {
            if let Some((r, b)) = &self.rows[i] {
                let mut v = *b;
                for (j, x) in r.iter().enumerate() {
                    let mut x = *x;
                    while x != 0 {
                        let k = j * 64 + x.trailing_zeros() as usize;
                        if k != i {
                            v ^= z[k];
                        }
                        x &= x - 1;
                    }
                }
                z[i] = v;
            }
        }
        z
    }
}
fn search(l: Lin, products: &[(usize, usize)], visited: &mut usize) -> Option<Vec<u8>> {
    *visited += 1;
    if *visited > 100000 {
        return None;
    }
    let z = l.model();
    if let Some((a, b)) = products.iter().find(|(a, b)| z[*a] == 1 && z[*b] == 1) {
        for i in [a, b] {
            let mut ll = l.clone();
            if ll.equation(&[*i], 0) {
                if let Some(z) = search(ll, products, visited) {
                    return Some(z);
                }
            }
        }
        None
    } else {
        Some(z)
    }
}
pub fn run(dir: &Path) {
    let mut records = vec![];
    for graph in 0..4 {
        let input = crate::round04::read(&dir.join(format!("q8-graph-{graph}-F2-relations.json")));
        let p: Program = serde_json::from_value(input["program"].clone()).unwrap();
        let mut l = Lin::new(p.nodes.len());
        let mut products = vec![];
        let mut candidates = vec![];
        for (i, op) in p.nodes.iter().enumerate() {
            match op {
                Op::Input(_) => {}
                Op::Constant(_) => assert!(l.equation(&[i], 0)),
                Op::Add(a, b) => assert!(l.equation(&[i, *a, *b], 0)),
                Op::CoefficientFrobenius(a) => assert!(l.equation(&[i, *a], 0)),
                Op::Inverse(a) => products.push((*a, i)),
                Op::Mul(a, b) => {
                    products.push((*a, *b));
                    for (c, x) in [(*a, *b), (*b, *a)] {
                        if matches!(&p.nodes[c],Op::Constant(s) if s=="1") {
                            candidates.push((i, c, x));
                        }
                    }
                }
            }
        }
        for r in &p.residuals {
            assert!(l.equation(&[*r], 0));
        }
        for (a, b) in &products {
            if a == b {
                assert!(l.equation(&[*a], 0));
            }
        }
        let mut attempts = vec![];
        let mut answer = None;
        for (out, c, x) in candidates {
            let mut ll = l.clone();
            let mut visited = 0;
            if ll.equation(&[out, x], 1) {
                if let Some(z) = search(ll, &products, &mut visited) {
                    answer = Some((out, c, x, z, visited));
                    break;
                }
            }
            attempts.push(json!({"mul_node":out,"constant_one_node":c,"other_input":x,"search_nodes":visited,"cap_reached":visited>100000}));
        }
        let (out, c, x, z, visited) =
            answer.expect("no actual boundary counterpoint in frozen search");
        let mut checked = 0;
        for (i, op) in p.nodes.iter().enumerate() {
            let r = match op {
                Op::Input(_) => continue,
                Op::Constant(_) => z[i],
                Op::Add(a, b) => z[i] ^ z[*a] ^ z[*b],
                Op::Mul(a, b) => z[*a] * z[*b],
                Op::CoefficientFrobenius(a) => z[i] ^ z[*a],
                Op::Inverse(a) => z[*a] * z[i],
            };
            for j in 0..360 {
                let scalar = if j == 0 { r } else { 0 };
                assert_eq!(scalar, 0);
                checked += 1;
            }
        }
        for i in &p.residuals {
            for j in 0..360 {
                assert_eq!(if j == 0 { z[*i] } else { 0 }, 0);
                checked += 1;
            }
        }
        assert_eq!(
            checked as u64,
            input["individual_F2_equations"].as_u64().unwrap()
        );
        assert_eq!(z[c], 0);
        assert_eq!(z[out] ^ z[x], 1);
        records.push(json!({"graph":graph,"h":0,"extension_block_values_in_F2":z,"nodes":p.nodes.len(),"scalar_equations_checked":checked,"missing_linear_relation":{"out":out,"constant_one_node":c,"input":x,"value":1,"membership_certificate":"h*(out-input)=(h*out-c*input)+input*(c-h)"},"successful_search_nodes":visited,"failed_candidates":attempts,"interpretation":"all360 coefficient indeterminates per block vanish except coefficient0 equal to the displayed bit;not Boolean equations in the original scheme"}));
        println!("Q10:graph{graph} actual false boundary point,{checked} scalar equations checked,missing linear relation node{out}-node{x}");
    }
    round02::save(
        dir,
        "q10-full-homogenized-boundary-counterpoints.json",
        &json!({"schema":"koblitz.round12/q10-v1","graphs":records,"naive_full_systems_rejected":4,"full_saturated_ideals_computed":false,"whole_proper_quotient_embedding":false,"scope":"actual full circuit homogenization counterpoints and explicit saturated linear relations;no complete projective boundary repair"}),
    );
}

pub fn certify(dir: &Path) {
    let r = crate::round04::read(&dir.join("q10-full-homogenized-boundary-counterpoints.json"));
    let mut source=String::from("import Mathlib.Tactic\nset_option maxRecDepth 100000\nset_option maxHeartbeats 0\nnamespace KoblitzRound12Boundary\ninductive Gate where\n | input | constant | add (a b : Nat) | mul (a b : Nat) | frob (a : Nat) | inv (a : Nat)\n deriving Inhabited, DecidableEq\ndef eval (g : Gate) (i : Nat) (z : Array Bool) : Bool :=\n match g with\n | .input => true\n | .constant => !(z[i]!)\n | .add a b => z[i]! == ((z[a]!).xor (z[b]!))\n | .mul a b => !((z[a]!) && (z[b]!))\n | .frob a => z[i]! == z[a]!\n | .inv a => !((z[a]!) && (z[i]!))\ndef check (p : Array Gate) (z : Array Bool) (r : Array Nat) : Bool :=\n (p.toList.zipIdx.all fun gi => eval gi.1 gi.2 z) &&\n (r.toList.all fun i => !(z[i]!))\n");
    for graph in 0..4 {
        let input = crate::round04::read(&dir.join(format!("q8-graph-{graph}-F2-relations.json")));
        let p: Program = serde_json::from_value(input["program"].clone()).unwrap();
        let z: Vec<u8> =
            serde_json::from_value(r["graphs"][graph]["extension_block_values_in_F2"].clone())
                .unwrap();
        source.push_str(&format!("def program{graph} : Array Gate := #[\n"));
        for op in &p.nodes {
            let s = match op {
                Op::Input(_) => ".input".into(),
                Op::Constant(_) => ".constant".into(),
                Op::Add(a, b) => format!(".add {a} {b}"),
                Op::Mul(a, b) => format!(".mul {a} {b}"),
                Op::CoefficientFrobenius(a) => format!(".frob {a}"),
                Op::Inverse(a) => format!(".inv {a}"),
            };
            source.push_str(&format!("{s},\n"));
        }
        source.push_str("]\n");
        source.push_str(&format!(
            "def model{graph} : Array Bool := #[{}]\n",
            z.iter()
                .map(|z| if *z == 1 { "true" } else { "false" })
                .collect::<Vec<_>>()
                .join(",")
        ));
        source.push_str(&format!(
            "def residual{graph} : Array Nat := #[{}]\n",
            p.residuals
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(",")
        ));
        source.push_str(&format!("theorem graph{graph}_all_boundary_gates : check program{graph} model{graph} residual{graph}=true := by decide +kernel\n"));
        let out = r["graphs"][graph]["missing_linear_relation"]["out"]
            .as_u64()
            .unwrap();
        let x = r["graphs"][graph]["missing_linear_relation"]["input"]
            .as_u64()
            .unwrap();
        source.push_str(&format!("theorem graph{graph}_missing_linear_relation : model{graph}[{out}]! ≠ model{graph}[{x}]! := by decide +kernel\n"));
    }
    source.push_str("end KoblitzRound12Boundary\n");
    for graph in 0..4 {
        source.push_str(&format!("#print axioms KoblitzRound12Boundary.graph{graph}_all_boundary_gates\n#print axioms KoblitzRound12Boundary.graph{graph}_missing_linear_relation\n"));
    }
    let path = dir.join("KoblitzRound12Boundary.lean");
    assert!(!path.exists());
    std::fs::write(path, source).unwrap();
    println!("Q10b:four actual boundary circuits and eight kernel-checkable Lean claims exported");
}

pub fn constants(dir: &Path) {
    let f = F::new(360, 45);
    let mut records = vec![];
    for graph in 0..4 {
        let input = crate::round04::read(&dir.join(format!("q8-graph-{graph}-F2-relations.json")));
        let p: Program = serde_json::from_value(input["program"].clone()).unwrap();
        let mut known: Vec<Option<(E, usize)>> = vec![];
        let mut relations = vec![];
        let mut constants = vec![];
        let mut maxdepth = 0;
        for (i, op) in p.nodes.iter().enumerate() {
            let v = match op {
                Op::Input(_) => None,
                Op::Constant(s) => Some((
                    E::from_biguint(&BigUint::parse_bytes(s.as_bytes(), 16).unwrap(), 360),
                    0,
                )),
                Op::Add(a, b) => known[*a]
                    .as_ref()
                    .zip(known[*b].as_ref())
                    .map(|((a, k), (b, l))| (a.add(b), (*k).max(*l))),
                Op::CoefficientFrobenius(a) => known[*a].as_ref().map(|(a, k)| (f.sq(a), *k)),
                Op::Inverse(a) => known[*a].as_ref().map(|(a, k)| (f.inv(a), k + 1)),
                Op::Mul(a, b) => {
                    if let Some((c, k)) = &known[*a] {
                        relations.push(json!({"out":i,"input":b,"scalar":c.to_biguint().to_str_radix(16),"h_power_membership_depth":k+1,"kind":"scale","certificate":"h*(out-c*input)=(h*out-a*input)+input*(a-c*h)"}));
                    } else if let Some((c, k)) = &known[*b] {
                        relations.push(json!({"out":i,"input":a,"scalar":c.to_biguint().to_str_radix(16),"h_power_membership_depth":k+1,"kind":"scale","certificate":"h*(out-c*input)=(h*out-b*input)+input*(b-c*h)"}));
                    }
                    match (&known[*a], &known[*b]) {
                        (Some((a, k)), Some((b, l))) => Some((f.mul(a, b), 1 + (*k).max(*l))),
                        (Some((a, k)), _) if *a == f.z() => Some((f.z(), k + 1)),
                        (_, Some((b, k))) if *b == f.z() => Some((f.z(), k + 1)),
                        _ => None,
                    }
                }
            };
            if let Some((c, k)) = &v {
                constants.push(json!({"node":i,"scalar":c.to_biguint().to_str_radix(16),"h_power_membership_depth":k}));
                if !matches!(op, Op::Constant(_)) {
                    relations.push(json!({"out":i,"scalar":c.to_biguint().to_str_radix(16),"h_power_membership_depth":k,"kind":"constant","linear_relation":"out_i+scalar_i*h=0"}));
                }
                maxdepth = maxdepth.max(*k);
            }
            known.push(v);
        }
        let originalconstants = p
            .nodes
            .iter()
            .filter(|o| matches!(o, Op::Constant(_)))
            .count();
        let scalarlinear = relations.len() * 360;
        maxdepth = maxdepth.max(
            relations
                .iter()
                .map(|r| r["h_power_membership_depth"].as_u64().unwrap() as usize)
                .max()
                .unwrap_or(0),
        );
        records.push(json!({"graph":graph,"evaluated_constant_nodes":constants,"original_constant_nodes":originalconstants,"new_linear_relations":relations,"new_scalar_linear_equations":scalarlinear,"maximum_h_power_depth":maxdepth,"scale_expansion":"out_i+sum_j(sum_k scalar_k*tensor(k,j)_i)*input_j=0;shared basis tensor in q8-F2-polynomial-templates.json","original_equations_retained":true,"same_h_localization":true,"h_saturation_complete":false}));
        println!(
            "Q10c:graph{graph} {} new360-coefficient linear relations,h-depth<={maxdepth}",
            scalarlinear / 360
        );
    }
    round02::save(
        dir,
        "q10c-constant-gate-saturation-step.json",
        &json!({"schema":"koblitz.round12/q10c-v1","field_modulus":"z^360+z^5+z^3+z^2+1","records":records,"full_h_saturation":false,"scope":"exact constant propagation and intermediate linear saturation relations on all four actual circuits;not a proper quotient model"}),
    );
}
