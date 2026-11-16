//! Checked catalog join, projected principal rows and separate deck-fiber rows.
use crate::{
    round02,
    round04::{read, Curve, Field, Pt},
};
use num_bigint::BigInt;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
fn point(v: &Value) -> Pt {
    if v.is_array() {
        Pt::A(v[0].as_u64().unwrap() as u32, v[1].as_u64().unwrap() as u32)
    } else {
        assert_eq!(v["infinity"], true);
        Pt::O
    }
}
fn ranks(matrix: &[Vec<u64>], masks: &[Vec<usize>]) -> Vec<usize> {
    masks
        .iter()
        .map(|mask| {
            let m = matrix
                .iter()
                .map(|r| mask.iter().map(|i| r[*i]).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let a = crate::gluing::rref(m.clone(), 11).1.len();
            let b = round02::rank(
                m.into_iter()
                    .map(|r| r.into_iter().map(BigInt::from).collect())
                    .collect(),
                &BigInt::from(11),
            );
            assert_eq!(a, b);
            a
        })
        .collect()
}
fn lean_point(p: Pt) -> String {
    match p {
        Pt::O => "none".into(),
        Pt::A(x, y) => format!("some ({x},{y})"),
    }
}
fn minor(matrix: &[Vec<u64>], rows: &[usize], cols: &[usize]) -> Vec<Vec<u64>> {
    let m = rows
        .iter()
        .map(|i| cols.iter().map(|j| matrix[*i][*j]).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for (i, r) in m.iter().enumerate() {
        for (j, v) in r.iter().enumerate() {
            assert_eq!(*v, u64::from(i == j));
        }
    }
    m
}
pub fn run(dir: &Path) {
    let old = read(&dir.join("m4-direct-modular-relations.json"));
    let m8 = read(&dir.join("m8-full-map-principal-row.json"));
    let m11 = read(&dir.join("m11-fixed-infinity-row.json"));
    let i1 = read(&dir.join("i1-degree210-images.json"));
    let support = read(&dir.join("o24-five-source-supports.json"));
    let middles = read(&dir.join("o21-five-middle-candidates.json"));
    let cusp = read(&dir.join("m13-coupled-cusp-valuations.json"));
    let ss = read(&dir.join("s19-complete-supersingular-divisor-strata.json"));
    let oldzero = read(&dir.join("o17-ordinary-projected-images.json"));
    assert_eq!(m11["total_columns"], 1308);
    assert_eq!(old["closed_points"].as_array().unwrap().len(), 544);
    assert_eq!(m8["columns"].as_array().unwrap().len(), 60);
    assert_eq!(m11["columns_from648"].as_array().unwrap().len(), 660);
    assert_eq!(i1["columns"].as_array().unwrap().len(), 44);
    assert_eq!(ss["all_target_values_killed_by8"], true);
    assert_eq!(ss["new_11_primary_nonzero_images"], 0);
    assert_eq!(oldzero["new_nonzero_11_primary_columns"], 0);
    let f = Field {
        m: 5,
        modulus: i1["F32_modulus"].as_u64().unwrap(),
    };
    assert_eq!(f.modulus, 37);
    let cs = [Curve { f, a: 1, b: 1 }, Curve { f, a: 0, b: 1 }];
    let mut matrix: Vec<Vec<u64>> = old["principal_relations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            let mut v: Vec<u64> = serde_json::from_value(r["row_mod11"].clone()).unwrap();
            v.resize(1308, 0);
            v
        })
        .collect();
    for range in [544..588, 588..648] {
        let mut r = vec![0; 1308];
        for i in range {
            r[i] = 1;
        }
        matrix.push(r);
    }
    for r in m11["rows"].as_array().unwrap() {
        let r: BTreeMap<usize, u64> = serde_json::from_value(r.clone()).unwrap();
        let mut v = vec![0; 1308];
        for (i, x) in r {
            v[i] = x % 11;
        }
        matrix.push(v);
    }
    let mut masks = [vec![], vec![]];
    let mut olddegrees = BTreeSet::from([210usize]);
    for (sign, mask) in masks.iter_mut().enumerate() {
        for (i, p) in old["closed_points"].as_array().unwrap().iter().enumerate() {
            olddegrees.insert(p["degree"].as_u64().unwrap() as usize);
            if p["images"][sign]["nonzero"] == true {
                mask.push(i);
            }
        }
        for p in i1["columns"].as_array().unwrap() {
            assert_eq!(point(&p["images"][sign]["image_F32"]), Pt::O);
        }
        for (i, p) in m8["columns"].as_array().unwrap().iter().enumerate() {
            olddegrees.insert(p["degree"].as_u64().unwrap() as usize);
            if p["images"][sign]["nonzero"] == true {
                mask.push(588 + i);
            }
        }
        for (i, p) in m11["columns_from648"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
        {
            olddegrees.insert(p["degree"].as_u64().unwrap() as usize);
            if p["images"][sign]["nonzero"] == true {
                mask.push(648 + i);
            }
        }
    }
    assert!(olddegrees.iter().all(|d| d % 6 == 0));
    let oldranks = ranks(
        &matrix,
        &[(0..1308).collect(), masks[0].clone(), masks[1].clone()],
    );
    assert_eq!(oldranks, [12, 11, 11]);
    assert_eq!([masks[0].len(), masks[1].len()], [1165, 1160]);
    let oldrows = matrix.len();
    let mut columns = vec![];
    let mut images = [vec![], vec![]];
    let mut deck = vec![];
    let mut principal = vec![vec![]; 8];
    let mut newdegrees = BTreeSet::new();
    let mut keys = BTreeSet::new();
    let mut ordinary = [0usize; 8];
    let mut ordinary_entries = vec![vec![]; 8];
    for c in oldzero["columns"].as_array().unwrap() {
        let fi = if c["function_index"] == 0 { 0 } else { 5 };
        ordinary[fi] += c["closed_degree"].as_u64().unwrap() as usize;
        assert_eq!(c["even_projected_image"], "zero");
        assert_eq!(c["odd_projected_image"], "zero");
        ordinary_entries[fi].push(json!({"source":"round13 O17","closed_degree":c["closed_degree"],"multiplicity":1,"even":"zero","odd":"zero"}));
    }
    for r in support["records"].as_array().unwrap() {
        let index = r["index"].as_u64().unwrap() as usize;
        let fi = [1usize, 2, 3, 6, 7][r["function_index"].as_u64().unwrap() as usize];
        let ps = r["source_points"].as_array().unwrap();
        for p in ps {
            ordinary[fi] += p["closed_degree"].as_u64().unwrap() as usize;
            assert_eq!(p["multiplicity"], 1);
        }
        if ps.len() == 1 {
            assert_eq!(ps[0]["even_projected_image"], "zero");
            assert_eq!(ps[0]["odd_projected_image"], "zero");
            ordinary_entries[fi].push(json!({"source_point_index":index,"source_scalar_factor_index":0,"degree":ps[0]["closed_degree"],"multiplicity":1,"even":"zero","odd":"zero","reason":"whole scalar deck fiber"}));
            continue;
        }
        assert_eq!(ps.len(), 5);
        let data = read(&dir.join(if index == 15 {
            "o25-degree7-individual-images.json".into()
        } else {
            format!("o26-point-{index}-images.json")
        }));
        assert_eq!(data["individual_closed_points_evaluated"], 5);
        assert_eq!(data["commuting_squares"], 120);
        let m = &middles["records"][index];
        let basekey = format!(
            "{}:{}",
            m["field_modulus_bits"].as_str().unwrap(),
            m["source_target"][1].as_str().unwrap()
        );
        assert!(keys.insert(basekey.clone()), "different base closed points");
        let d = r["base_degree"].as_u64().unwrap() as usize;
        assert_ne!(d % 6, 0);
        newdegrees.insert(d);
        let mut factors = BTreeSet::new();
        let mut row = vec![];
        for c in data["columns"].as_array().unwrap() {
            let scalar = c["source_scalar_factor_index"].as_u64().unwrap() as usize;
            assert!(factors.insert(scalar));
            assert!(scalar < 5);
            assert_eq!(ps[scalar]["closed_degree"], d);
            let col = columns.len();
            row.push(col);
            principal[fi].push(col);
            let mut vals = vec![];
            for sign in 0..2 {
                let p = point(&c["images"][sign]["image_F32"]);
                assert!(cs[sign].valid(p));
                assert_eq!(cs[sign].times(p, 11), Pt::O);
                images[sign].push(p);
                if p != Pt::O {
                    masks[sign].push(1308 + col);
                }
                vals.push(c["images"][sign]["image_F32"].clone());
            }
            columns.push(json!({"index":1308+col,"source_point_index":index,"source_scalar_factor_index":scalar,"degree":d,"base_point_key":basekey,"scalar_factor":ps[scalar]["extension_polynomial_coefficients"],"images_F32":vals}));
            ordinary_entries[fi].push(json!({"source_point_index":index,"source_scalar_factor_index":scalar,"degree":d,"multiplicity":1,"column":1308+col}));
        }
        assert_eq!(factors.len(), 5);
        deck.push(row);
    }
    assert_eq!(columns.len(), 40);
    assert_eq!(deck.len(), 8);
    for sign in 0..2 {
        for row in deck.iter().chain(principal.iter()) {
            let sum = row
                .iter()
                .fold(Pt::O, |a, i| cs[sign].add(a, images[sign][*i]));
            assert_eq!(sum, Pt::O);
        }
    }
    let mut complete = vec![];
    let mut degree_table = vec![];
    for fi in 0..8 {
        let mut pos = 0usize;
        let mut neg = 0usize;
        let mut entries = vec![];
        for (i, c) in cusp["cusp_records"].as_array().unwrap().iter().enumerate() {
            let v = c["valuations"][fi].as_i64().unwrap();
            if v > 0 {
                pos += v as usize;
            }
            if v < 0 {
                neg += (-v) as usize;
            }
            if v != 0 {
                entries.push(json!({"geometric_cusp_index":i,"signed_multiplicity":v}));
            }
        }
        let ss_entries = &ss["eight_supersingular_divisors"][fi];
        let sd = ss_entries
            .as_array()
            .unwrap()
            .iter()
            .map(|p| {
                p["degree"].as_u64().unwrap() as usize
                    * p["multiplicity"].as_u64().unwrap() as usize
            })
            .sum::<usize>();
        assert_eq!(
            sd,
            ss["supersingular_zero_degrees"][fi].as_u64().unwrap() as usize
        );
        assert_eq!(pos + sd + ordinary[fi], neg);
        assert_eq!(
            neg,
            cusp["functions"][fi]["pole_degree"].as_u64().unwrap() as usize
        );
        degree_table.push([pos, sd, ordinary[fi], neg]);
        complete.push(json!({"function":cusp["functions"][fi]["function"],"geometric_signed_cusps":entries,"supersingular_closed_zeros":ss_entries,"ordinary_closed_zeros":ordinary_entries[fi],"degree_partition_cusp_SS_ordinary_poles":[pos,sd,ordinary[fi],neg],"geometric_signed_divisor_complete_under_written_bridges":true,"full_F2_closed_cusp_enumeration_complete":fi==4,"projected_principal_row":principal[fi]}));
    }
    for r in &mut matrix {
        r.resize(1348, 0);
    }
    let principal_start = matrix.len();
    for row in &principal {
        let mut r = vec![0; 1348];
        for j in row {
            r[1308 + j] = 1;
        }
        matrix.push(r);
    }
    let allmasks = vec![(0..1348).collect(), masks[0].clone(), masks[1].clone()];
    let pranks = ranks(&matrix, &allmasks);
    let deck_start = matrix.len();
    for row in &deck {
        let mut r = vec![0; 1348];
        for j in row {
            r[1308 + j] = 1;
        }
        matrix.push(r);
    }
    let dranks = ranks(&matrix, &allmasks);
    let active = |rows: &[Vec<usize>], sign: Option<usize>| {
        rows.iter()
            .enumerate()
            .filter_map(|(i, r)| {
                r.iter()
                    .find(|j| sign.is_none_or(|s| images[s][**j] != Pt::O))
                    .map(|j| (i, 1308 + *j))
            })
            .collect::<Vec<_>>()
    };
    let raw = active(&deck, None);
    let even = active(&deck, Some(0));
    let odd = active(&deck, Some(1));
    assert_eq!(
        dranks,
        [
            oldranks[0] + raw.len(),
            oldranks[1] + even.len(),
            oldranks[2] + odd.len()
        ]
    );
    let pm = active(&principal, None);
    let pe = active(&principal, Some(0));
    let po = active(&principal, Some(1));
    assert_eq!(
        pranks,
        [
            oldranks[0] + pm.len(),
            oldranks[1] + pe.len(),
            oldranks[2] + po.len()
        ]
    );
    let mut minors = vec![];
    for (label, rows, start) in [
        ("deck", raw, deck_start),
        ("deck_even", even, deck_start),
        ("deck_odd", odd, deck_start),
        ("principal", pm, principal_start),
        ("principal_even", pe, principal_start),
        ("principal_odd", po, principal_start),
    ] {
        let rs = rows.iter().map(|(i, _)| start + i).collect::<Vec<_>>();
        let cols = rows.iter().map(|(_, j)| *j).collect::<Vec<_>>();
        minors.push(
            json!({"label":label,"rows":rs,"columns":cols,"matrix":minor(&matrix,&rs,&cols)}),
        );
    }
    let mut lean=String::from("import KoblitzRound14\nopen scoped Matrix\nset_option maxRecDepth 100000\nset_option maxHeartbeats 0\nnamespace KoblitzRound14Relations\nabbrev Point := Option (Nat × Nat)\ndef step (a : Nat) := if a.testBit 4 then Nat.xor ((a<<<1)%32) 5 else (a<<<1)%32\ndef mulAux : Nat → Nat → Nat → Nat → Nat\n | 0,_,_,z => z\n | n+1,a,b,z => mulAux n (step a) (b>>>1) (if b.testBit 0 then Nat.xor z a else z)\ndef mul (a b : Nat) := mulAux 5 a b 0\ndef pow (a : Nat) (n : Nat) := (List.range n).foldl (fun z _ => mul z a) 1\ndef inv (a : Nat) := pow a 30\ndef add (a : Nat) (p q : Point) : Point :=\n match p,q with\n | none,q => q\n | p,none => p\n | some (x,y),some (u,v) =>\n   if x=u ∧ (y≠v ∨ x=0) then none else\n    let slope := if x=u then Nat.xor x (mul y (inv x)) else mul (Nat.xor y v) (inv (Nat.xor x u))\n    let nx := Nat.xor (Nat.xor (Nat.xor (Nat.xor (mul slope slope) slope) x) u) a\n    some (nx,Nat.xor (Nat.xor (mul slope (Nat.xor x nx)) y) nx)\ndef scalar (a n : Nat) (p : Point) := (List.range n).foldl (fun z _ => add a z p) none\ndef valid (a : Nat) : Point → Bool\n | none => true\n | some (x,y) => (x<32 && y<32) && (Nat.xor (mul y y) (mul x y)==Nat.xor (Nat.xor (mul (mul x x) x) (mul a (mul x x))) 1)\ndef rowSum (a : Nat) (p : List Point) (r : List Nat) := r.foldl (fun z i => add a z (p[i]!)) none\n");
    lean.push_str("instance : Fact (Nat.Prime 11) := ⟨by decide⟩\n");
    for (sign, name) in ["even", "odd"].into_iter().enumerate() {
        lean.push_str(&format!(
            "def {name}Points : List Point := [{}]\n",
            images[sign]
                .iter()
                .map(|p| lean_point(*p))
                .collect::<Vec<_>>()
                .join(",")
        ));
    }
    let lit = |rows: &Vec<Vec<usize>>| {
        format!(
            "[{}]",
            rows.iter()
                .map(|r| format!(
                    "[{}]",
                    r.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(",")
                ))
                .collect::<Vec<_>>()
                .join(",")
        )
    };
    lean.push_str(&format!(
        "def principalRows : List (List Nat) := {}\ndef deckRows : List (List Nat) := {}\n",
        lit(&principal),
        lit(&deck)
    ));
    let mut names = vec![];
    for (name, a) in [("even", 1), ("odd", 0)] {
        let n = format!("{name}_points_valid_order11");
        lean.push_str(&format!("theorem {n} : ({name}Points.all fun p => valid {a} p && (scalar {a} 11 p==none))=true := by decide +kernel\n"));
        names.push(n);
    }
    for row in ["principal", "deck"] {
        let n = format!("{row}_rows_zero");
        lean.push_str(&format!("theorem {n} : ({}Rows.all fun r => (rowSum 1 evenPoints r==none) && (rowSum 0 oddPoints r==none))=true := by decide +kernel\n",row));
        names.push(n);
    }
    for m in &minors {
        let label = m["label"].as_str().unwrap();
        let rows: Vec<Vec<u64>> = serde_json::from_value(m["matrix"].clone()).unwrap();
        let n = rows.len();
        let value = format!(
            "!![{}]",
            rows.iter()
                .map(|r| r
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(","))
                .collect::<Vec<_>>()
                .join(";")
        );
        lean.push_str(&format!("def {label}Minor : Matrix (Fin {n}) (Fin {n}) (ZMod 11) := {value}\ntheorem {label}_minor_one : {label}Minor=1 := by decide +kernel\ntheorem {label}_minor_rank : Matrix.rank {label}Minor={n} := by rw [{label}_minor_one];simpa using (Matrix.rank_one (R := ZMod 11) (n := Fin {n}))\n"));
        names.extend([format!("{label}_minor_one"), format!("{label}_minor_rank")]);
    }
    lean.push_str(&format!("theorem actual_degrees_disjoint : (({} : List Nat).all fun d => d%6==0)=true ∧ (({} : List Nat).all fun d => d%6!=0)=true := by decide +kernel\n",format!("[{}]",olddegrees.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")),format!("[{}]",newdegrees.iter().map(ToString::to_string).collect::<Vec<_>>().join(","))));
    names.push("actual_degrees_disjoint".into());
    lean.push_str("theorem fresh_block_coefficients {K ι : Type*} [Field K] [Fintype ι] [DecidableEq ι] (c : ι → K) (v : ι → ι → K) (hv : ∀ i j,v i j=if i=j then 1 else 0) (h : ∀ j,∑ i,c i*v i j=0) : ∀ j,c j=0 := by\n intro j\n simpa [hv] using h j\nend KoblitzRound14Relations\n");
    names.push("fresh_block_coefficients".into());
    for n in &names {
        lean.push_str(&format!("#print axioms KoblitzRound14Relations.{n}\n"));
    }
    let path = dir.join("KoblitzRound14Relations.lean");
    assert!(!path.exists());
    std::fs::write(path, lean).unwrap();
    let out = json!({"schema":"koblitz.round14/o27-v1","old_rank_controls":oldranks,"old_columns":1308,"old_rows":oldrows,"old_degree_set":olddegrees,"new_degree_set":newdegrees,"new_columns":columns,"new_columns_count":40,"complete_ordinary_closed_points":63,"all_ordinary_images_complete":true,"coupled_geometric_divisors":complete,"degree_table_cusp_SS_ordinary_poles":degree_table,"projected_principal_rows":principal,"deck_fiber_rows":deck,"principal_only_joined_ranks":pranks,"with_deck_joined_ranks":dranks,"all_columns":1348,"nonzero_column_counts":[masks[0].len(),masks[1].len()],"nonzero_masks":masks,"complete_joined_matrix_mod11":matrix,"identity_minors":minors,"new_named_Lean_theorems":names.len(),"geometric_divisors_complete_under_written_bridges":true,"projected_principal_rows_complete":true,"all_F2_closed_cusp_divisors_complete":false,"full_h_saturation":false,"proper_embedding_equations":false,"scope":"exact finite catalog and rank join;all40 new images checked independently in u32 F32 arithmetic;deck rows distinguished from principal rows;no IC efficiency conclusion"});
    let saved = dir.join("o27-complete-projected-relations.json");
    if saved.exists() {
        assert_eq!(read(&saved), out);
    } else {
        round02::save(dir, "o27-complete-projected-relations.json", &out);
    }
    println!("O27:all63 ordinary images complete;principal ranks{pranks:?},with deck{dranks:?},columns1348,nonzero counts{:?};{} new named Lean results",[out["nonzero_column_counts"][0].clone(),out["nonzero_column_counts"][1].clone()],names.len());
}
