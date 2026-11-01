//! Actual normalized Tate cusp Frobenius and complete F2 closed signed divisors.
use crate::{
    round02,
    round04::{read, Curve, Field, Pt},
};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
fn norm(b: usize, d: usize) -> [usize; 2] {
    [b, d].min([(11 - b) % 11, (11 - d) % 11])
}
fn point(v: &Value) -> Pt {
    if v.is_array() {
        Pt::A(v[0].as_u64().unwrap() as u32, v[1].as_u64().unwrap() as u32)
    } else {
        assert_eq!(v, "O");
        Pt::O
    }
}
fn fp(f: Field, p: Pt) -> Pt {
    match p {
        Pt::O => Pt::O,
        Pt::A(x, y) => Pt::A(f.sq(x), f.sq(y)),
    }
}
fn product_key(base: &str, labels: Vec<[usize; 2]>, degree: usize, mult: i64) -> String {
    let set: BTreeSet<_> = labels.into_iter().collect();
    format!("{base}:{set:?}:{degree}:{mult}")
}
fn lean_point(p: Pt) -> String {
    match p {
        Pt::O => "none".into(),
        Pt::A(x, y) => format!("some ({x},{y})"),
    }
}
pub fn run(dir: &Path) {
    let input = read(&dir.join("m13-coupled-cusp-valuations.json"));
    let old = read(&dir.join("m13b-complete-product-divisor.json"));
    let complete = read(&dir.join("o27-complete-projected-relations.json"));
    let bases = input["base_cusps"].as_array().unwrap();
    let labels: Vec<[usize; 2]> =
        serde_json::from_value(input["level_cusp_vectors"].clone()).unwrap();
    let rows = input["cusp_records"].as_array().unwrap();
    assert_eq!(bases.len(), 8);
    assert_eq!(labels.len(), 60);
    assert_eq!(rows.len(), 480);
    let f = Field::find(3);
    assert_eq!(f.modulus, 11);
    let ec = Curve { f, a: 1, b: 1 };
    let points = bases.iter().map(|p| point(&p["point"])).collect::<Vec<_>>();
    assert!(points.iter().all(|p| ec.valid(*p)));
    let base_next = points
        .iter()
        .map(|p| points.iter().position(|q| *q == fp(f, *p)).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(base_next.iter().copied().collect::<BTreeSet<_>>().len(), 8);
    let lookup: BTreeMap<_, _> = labels.iter().enumerate().map(|(i, v)| (*v, i)).collect();
    assert_eq!(lookup.len(), 60);
    let mut perm = vec![];
    let mut checked = 0;
    for (i, r) in rows.iter().enumerate() {
        assert_eq!(r["base_cusp"], i / 60);
        assert_eq!(r["vector_mod_sign"], json!(labels[i % 60]));
        let [b, d] = labels[i % 60];
        let next = 60 * base_next[i / 60] + lookup[&norm(b, 6 * d % 11)];
        let target = &rows[next];
        for j in 0..2 {
            assert_eq!(
                fp(f, point(&r["target_pair"][j])),
                point(&target["target_pair"][j])
            );
            checked += 1;
        }
        for j in 0..8 {
            assert_eq!(r["valuations"][j], target["valuations"][j]);
            checked += 1;
        }
        for key in ["ramification_phi0", "ramification_phiinf"] {
            assert_eq!(r[key], target[key]);
            checked += 1;
        }
        let oldline = r["canonical_line"].as_u64().unwrap() as usize;
        assert_eq!(
            target["canonical_line"],
            if oldline == 11 { 11 } else { 2 * oldline % 11 }
        );
        perm.push(next);
    }
    assert!(checked <= 10000);
    assert_eq!(perm.iter().copied().collect::<BTreeSet<_>>().len(), 480);
    let mut seen = BTreeSet::new();
    let mut orbits = vec![];
    let mut closed = vec![];
    let mut histogram = BTreeMap::new();
    let mut product = BTreeSet::new();
    for i in 0..480 {
        if seen.contains(&i) {
            continue;
        }
        let mut j = i;
        let mut orbit = vec![];
        loop {
            assert!(seen.insert(j));
            orbit.push(j);
            j = perm[j];
            if j == i {
                break;
            }
            assert!(orbit.len() <= 30);
        }
        let degree = orbit.len();
        *histogram.entry(degree).or_insert(0usize) += 1;
        let valuations: Vec<i64> = serde_json::from_value(rows[i]["valuations"].clone()).unwrap();
        for member in &orbit {
            assert_eq!(rows[*member]["valuations"], json!(valuations));
        }
        if valuations[4] != 0 {
            let base = bases[i / 60]["name"].as_str().unwrap();
            assert!(["O", "T"].contains(&base));
            product.insert(product_key(
                base,
                orbit.iter().map(|j| labels[j % 60]).collect(),
                degree,
                valuations[4],
            ));
        }
        closed.push(json!({"index":closed.len(),"degree":degree,"geometric_members":orbit,"signed_multiplicities":valuations,"definition":"transitive normalized Frobenius orbit of actual level49/Tate11 cusp labels"}));
        orbits.push(orbit);
    }
    assert_eq!(seen.len(), 480);
    assert_eq!(orbits.iter().map(Vec::len).sum::<usize>(), 480);
    let expected = old["closed_divisor"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| {
            product_key(
                p["base_cusp"].as_str().unwrap(),
                serde_json::from_value(p["exponent_orbit"].clone()).unwrap(),
                p["closed_degree"].as_u64().unwrap() as usize,
                p["multiplicity"].as_i64().unwrap(),
            )
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(product, expected);
    assert_eq!(product.len(), 22);
    let mut divisors = vec![];
    let mut coeffs = vec![];
    let mut ssordinary = vec![];
    for fi in 0..8 {
        let row = closed
            .iter()
            .map(|p| p["signed_multiplicities"][fi].as_i64().unwrap())
            .collect::<Vec<_>>();
        let mut pos = 0i64;
        let mut neg = 0i64;
        let entries = closed
            .iter()
            .zip(&row)
            .filter(|(_, m)| **m != 0)
            .map(|(p, m)| {
                let d = p["degree"].as_i64().unwrap();
                if *m > 0 {
                    pos += m * d;
                } else {
                    neg -= m * d;
                }
                json!({"cusp_index":p["index"],"degree":d,"signed_multiplicity":m})
            })
            .collect::<Vec<_>>();
        let prior = &complete["coupled_geometric_divisors"][fi];
        let table = &prior["degree_partition_cusp_SS_ordinary_poles"];
        assert_eq!(pos, table[0].as_i64().unwrap());
        assert_eq!(neg, table[3].as_i64().unwrap());
        let so = table[1].as_i64().unwrap() + table[2].as_i64().unwrap();
        assert_eq!(pos + so - neg, 0);
        ssordinary.push(so);
        coeffs.push(row);
        divisors.push(json!({"function":prior["function"],"closed_cusp_signed_entries":entries,"supersingular_closed_zeros":prior["supersingular_closed_zeros"],"ordinary_closed_zeros":prior["ordinary_closed_zeros"],"degree_partition_cusp_SS_ordinary_poles":table,"signed_degree":0,"complete_F2_closed_signed_divisor_under_written_bridges":true,"projected_principal_row":prior["projected_principal_row"]}));
    }
    let degrees = orbits.iter().map(Vec::len).collect::<Vec<_>>();
    let vecnat = |v: &[usize]| {
        format!(
            "[{}]",
            v.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
        )
    };
    let mut lean=String::from("import KoblitzRound14Relations\nset_option maxRecDepth 100000\nset_option maxHeartbeats 0\nnamespace KoblitzRound14Cusps\ntheorem tate_determinant_residual {R : Type*} [CommRing R] (a b c d i : R) : (2*a)*(i*d)-b*c-(a*d-b*c)=a*d*(2*i-1) := by ring\ntheorem tate_normalized_determinant {R : Type*} [CommRing R] (a b c d i : R) (h : 2*i=1) : (2*a)*(i*d)-b*c=a*d-b*c := by\n have h' := tate_determinant_residual a b c d i\n rw [h,sub_self,mul_zero] at h'\n exact sub_eq_zero.mp h'\nabbrev Point := Option (Nat × Nat)\ndef step8 (a : Nat) := if a.testBit 2 then Nat.xor ((a<<<1)%8) 3 else (a<<<1)%8\ndef mul8Aux : Nat → Nat → Nat → Nat → Nat\n | 0,_,_,z => z\n | n+1,a,b,z => mul8Aux n (step8 a) (b>>>1) (if b.testBit 0 then Nat.xor z a else z)\ndef mul8 (a b : Nat) := mul8Aux 3 a b 0\ndef frob8 : Point → Point\n | none => none\n | some (x,y) => some (mul8 x x,mul8 y y)\ndef valid8 : Point → Bool\n | none => true\n | some (x,y) => (x<8 && y<8) && (Nat.xor (mul8 y y) (mul8 x y)==Nat.xor (Nat.xor (mul8 (mul8 x x) x) (mul8 x x)) 1)\ndef norm (b d : Nat) : Nat × Nat :=\n let nb := (11-b)%11\n let nd := (11-d)%11\n if b<nb || (b==nb && d≤nd) then (b,d) else (nb,nd)\n");
    lean.push_str(&format!("def bases : List Point := [{}]\ndef baseNext : List Nat := {}\ndef labels : List (Nat × Nat) := [{}]\ndef nativePermutation : List Nat := {}\ndef orbits : List (List Nat) := [{}]\ndef degrees : List Int := [{}]\n",points.iter().map(|p|lean_point(*p)).collect::<Vec<_>>().join(","),vecnat(&base_next),labels.iter().map(|v|format!("({},{})",v[0],v[1])).collect::<Vec<_>>().join(","),vecnat(&perm),orbits.iter().map(|v|vecnat(v)).collect::<Vec<_>>().join(","),degrees.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")));
    lean.push_str("def cuspFrob (i : Nat) : Nat :=\n let v := labels[i%60]!\n baseNext[i/60]!*60+labels.idxOf (norm v.1 (6*v.2%11))\ntheorem actual_base_curve_and_frobenius : ((bases.all valid8) && ((List.range 8).all fun i => frob8 bases[i]! == bases[baseNext[i]!]!))=true := by decide +kernel\ntheorem actual_normalized_permutation : ((List.range 480).all fun i => cuspFrob i==nativePermutation[i]!)=true := by decide +kernel\ntheorem actual_orbits_cover : orbits.flatten.length=480 ∧ ((List.range 480).all fun i => (orbits.flatten.filter (fun j => j==i)).length==1)=true := by decide +kernel\ntheorem actual_orbits_frobenius : (orbits.all fun o => !o.isEmpty && ((o.zip (o.drop 1 ++ [o[0]!])).all fun ij => cuspFrob ij.1==ij.2))=true := by decide +kernel\n");
    let hist = histogram
        .iter()
        .map(|(d, n)| format!("({d},{n})"))
        .collect::<Vec<_>>()
        .join(",");
    lean.push_str(&format!("theorem actual_degree_histogram : (([{}] : List (Nat × Nat)).all fun dn => (orbits.filter (fun o => o.length==dn.1)).length==dn.2)=true := by decide +kernel\n",hist));
    let mut names = vec![
        "tate_determinant_residual".to_string(),
        "tate_normalized_determinant".into(),
        "actual_base_curve_and_frobenius".into(),
        "actual_normalized_permutation".into(),
        "actual_orbits_cover".into(),
        "actual_orbits_frobenius".into(),
        "actual_degree_histogram".into(),
    ];
    for fi in 0..8 {
        lean.push_str(&format!("def multiplicities{fi} : List Int := [{}]\ntheorem divisor{fi}_signed_degree : (degrees.zipWith (fun d m => d*m) multiplicities{fi}).sum+{}=0 := by decide +kernel\n",coeffs[fi].iter().map(ToString::to_string).collect::<Vec<_>>().join(","),ssordinary[fi]));
        names.push(format!("divisor{fi}_signed_degree"));
    }
    lean.push_str("theorem old_product_closed_control : (multiplicities4.filter (fun m => m != 0)).length=22 ∧ ((degrees.zip multiplicities4).foldl (fun s dm => if 0<dm.2 then s+dm.1*dm.2 else s) 0)=2640 ∧ ((degrees.zip multiplicities4).foldl (fun s dm => if dm.2<0 then s+dm.1*dm.2 else s) 0)= -2640 := by decide +kernel\nend KoblitzRound14Cusps\n");
    names.push("old_product_closed_control".into());
    for name in &names {
        lean.push_str(&format!("#print axioms KoblitzRound14Cusps.{name}\n"));
    }
    let path = dir.join("KoblitzRound14Cusps.lean");
    assert!(!path.exists());
    std::fs::write(path, lean).unwrap();
    round02::save(
        dir,
        "o28-complete-closed-coupled-divisors.json",
        &json!({"schema":"koblitz.round14/o28-v1","base_Frobenius":base_next,"normalized_cusp_Frobenius":perm,"Tate_action":"(b,d)->(b,6d) modulo11 and simultaneous sign;coefficient Frobenius on the actual F8 level49 cusp","geometric_cusps":480,"closed_cusps":closed,"closed_cusp_count":orbits.len(),"degree_histogram":histogram,"finite_covariance_checks":checked,"old_product22_exactly_reproduced":true,"eight_closed_signed_divisors":divisors,"all_eight_F2_closed_signed_divisors_complete_under_written_bridges":true,"new_named_Lean_theorems":names.len(),"full_h_saturation":false,"proper_quotient_equations_constructed":false,"bridge":"normalized Tate11 exponents and level49 cusp moduli;written global bridge,not a complete Lean modular-curve construction"}),
    );
    println!("O28:{} closed cusps exhaust480 labels;old22 product entries agree;all8 F2 closed signed divisors assembled;{} new named Lean results",orbits.len(),names.len());
}
