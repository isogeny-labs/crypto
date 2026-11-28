//! Text-only assembly of recorded candidate witnesses into Lean proof obligations.
//! No field operations, curve calls, metrics or acceptance verdicts are computed.
use std::fs;
use std::path::Path;
fn degree_of_text(s:&str)->usize {
    let b=s.as_bytes();let mut max=0;let mut i=0;
    while i<b.len(){if b[i]==b'X'{let mut n=1;let mut j=i+1;
        if j<b.len()&&b[j]==b'^'{j+=1;n=0;while j<b.len()&&b[j].is_ascii_digit(){n=n*10+(b[j]-b'0')as usize;j+=1;}}
        max=max.max(n);i=j;
    }else{i+=1;}}max
}
fn monic(term:&str)->String {
    format!("(by\n    apply Polynomial.monic_of_degree_le {}\n    · compute_degree!\n    · norm_num [Polynomial.coeff_one, Polynomial.coeff_X])",degree_of_text(term))
}
fn exact_degree(term:&str,n:usize,natural:bool)->String {
    let conclusion=if natural{format!("({term}:K[X]).natDegree={n}")}else{format!("({term}:K[X]).degree=({n}:WithBot ℕ)")};
    let lemma=if natural{"Polynomial.natDegree_eq_of_le_of_coeff_ne_zero"}else{"Polynomial.degree_eq_of_le_of_coeff_ne_zero"};
    format!("have hU : {conclusion} := by\n      apply {lemma}\n      · compute_degree!\n      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]")
}
fn remainder_degree(v:&str,u:&str)->String {
    let du=degree_of_text(u);let dv=degree_of_text(v);let setup=exact_degree(u,du,false);
    if v.trim()=="0" {format!("(by\n    {setup}\n    rw [hU]\n    simp)")}
    else {format!("(by\n    {setup}\n    rw [hU]\n    apply lt_of_le_of_lt (show ({v}:K[X]).degree≤({dv}:WithBot ℕ) from by compute_degree!)\n    norm_num)")}
}
fn reduced_degree(u:&str)->String {
    let du=degree_of_text(u);let setup=exact_degree(u,du,true);
    let bound=if du>2{format!("2<({du}:ℕ)")}else{format!("({du}:ℕ)≤2")};
    format!("(by\n    {setup}\n    simpa only [hU] using (by decide : {bound}))")
}
fn replace_obligations(s:&str,d:&str,u:&str,v:&str,high:bool)->String {
    let mut result=String::new();let mut n=0;let mut rest=s;
    while let Some(i)=rest.find("?_"){
        result.push_str(&rest[..i]);let replacement=match n {
            0=>monic(d),1=>monic(u),8=>remainder_degree(v,u),10=>reduced_degree(u),
            11 if high=>"(by decide)".into(),_=>"?_".into()};
        result.push_str(&replacement);rest=&rest[i+2..];n+=1;
    }result.push_str(rest);result
}

fn main(){
    let args:Vec<_>=std::env::args().collect();assert_eq!(args.len(),3,"candidate directory and output Lean path");
    let base=Path::new(&args[1]);let choices=fs::read_to_string(base.join("bezout-choices.lean.part")).unwrap();
    let fragments=fs::read_to_string(base.join("witness-refinements.txt")).unwrap();
    let mut out=String::from("import MumfordStollProofRules\n\n/-! Candidate certificates are accepted only by the Lean kernel below.\nBezout choices are fixed before defining the deterministic arithmetic. -/\nnamespace MumfordStollFullCertificatesAudit\nnoncomputable section\nopen Polynomial MumfordStollAudit MumfordStollGeneralAudit\nset_option backward.isDefEq.respectTransparency false\nset_option maxHeartbeats 1200000\n\nattribute [local mumford_stoll_coeff] Polynomial.coeff_C Polynomial.coeff_one Polynomial.coeff_zero\n  Polynomial.coeff_X_pow Polynomial.coeff_X Polynomial.coeff_mul_C Polynomial.coeff_C_mul\n  Polynomial.coeff_add Polynomial.coeff_sub Polynomial.coeff_neg\n  MumfordStollCantorAudit.coefficient_constant_power\n  MumfordStollProofRulesAudit.coeff_constant_product\n  MumfordStollProofRulesAudit.coeff_right_constant_power\n  MumfordStollProofRulesAudit.coeff_left_constant_power\n\ndef repPerm (i : Fin 10) : Fin 10 :=\n  match i with\n  | 0 => 0\n  | 1 => 6\n  | 2 => 2\n  | 3 => 8\n  | 4 => 4\n  | 5 => 5\n  | 6 => 1\n  | 7 => 9\n  | 8 => 3\n  | _ => 7\n\ndef expectedIndex (i j : Fin 10) : Fin 10 := repPerm (repPerm i+repPerm j)\n\n");
    out.push_str(&choices);
    out.push_str("macro \"stoll_full_poly\" : tactic =>\n  `(tactic| (ring_nf <;> ((try simp only [mul_assoc,←map_ofNat (Polynomial.C : K →+* K[X]),←map_mul (Polynomial.C : K →+* K[X]),←map_pow (Polynomial.C : K →+* K[X])]) <;> (first | exact Or.inl True.intro | (ext n <;> ((try simp only [mumford_stoll_coeff]) <;> ((try split_ifs) <;> norm_num <;> (try simp_all) <;> decide)))))))\n\n");let mut count=0;
    for block in fragments.split("-- Inputs ").skip(1){
        let (head,body)=block.split_once('\n').unwrap();let inputs=head.split(';').next().unwrap();
        let (i,j)=inputs.split_once(',').unwrap();let i:usize=i.trim().parse().unwrap();let j:usize=j.trim().parse().unwrap();
        assert!(i<10&&j<10);let body=body.trim();assert!(body.starts_with("refine ⟨"));
        let tuple=body.strip_prefix("refine ⟨").unwrap();let mut fields=tuple.split(',');let d=fields.next().unwrap();let u=fields.next().unwrap();let v=fields.next().unwrap();
        let refine=replace_obligations(body,d,u,v,body.contains("Or.inr"));
        out.push_str(&format!("theorem cert_{i}_{j} : GeneralCantorRun (bezoutA {i} {j})\n    (bezoutB {i} {j}) (bezoutC {i} {j}) (point {i}) (point {j})\n    (point (expectedIndex {i} {j})) := by\n"));
        for line in refine.lines(){out.push_str("  ");out.push_str(line);out.push('\n');}
        out.push_str("  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,\n    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]\n");
        out.push_str("  all_goals stoll_full_poly\n");out.push('\n');count+=1;
    }
    assert_eq!(count,100,"Require every complete-model input pair, no partial proof assembly.");
    out.push_str("theorem all_pairs (i j : Fin 10) :\n    GeneralCantorRun (bezoutA i j) (bezoutB i j) (bezoutC i j)\n      (point i) (point j) (point (expectedIndex i j)) := by\n  fin_cases i <;> fin_cases j\n");
    for i in 0..10{for j in 0..10{out.push_str(&format!("  · exact cert_{i}_{j}\n"));}}
    out.push_str("\n#print axioms all_pairs\nend\nend MumfordStollFullCertificatesAudit\n");
    fs::write(&args[2],out).unwrap();println!("Assembled100Lean proof obligations; kernel checking still required.");
}
