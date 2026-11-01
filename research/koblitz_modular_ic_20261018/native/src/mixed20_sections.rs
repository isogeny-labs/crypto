//! Complete elliptic factor bases and sparse full-weight orbit sections.
use super::*;
#[cfg(test)]
use std::collections::BTreeSet;
fn basis(n: usize) -> Vec<(usize, usize)> {
    let mut v = vec![(0, 0)];
    v.extend((1..=n / 2).map(|i| (i, 0)));
    v.extend((0..=(n - 3) / 2).map(|i| (i, 1)));
    assert_eq!(v.len(), n);
    v
}
fn eval(f: &F, q: &[Vec<P>], bs: &[Vec<(usize, usize)>]) -> Option<Vec<E>> {
    let mut v = vec![f.o()];
    let mut top = f.o();
    for (row, functions) in q.iter().zip(bs) {
        let mut values = vec![vec![]; functions.len()];
        for p in row {
            let P::Affine { x, y } = p else { return None };
            let mut powers = vec![f.o()];
            for i in 1..=functions.iter().map(|v| v.0).max().unwrap() {
                powers.push(f.mul(&powers[i - 1], x));
            }
            for (i, (a, b)) in functions.iter().enumerate() {
                values[i].push(if *b == 0 {
                    powers[*a].clone()
                } else {
                    f.mul(&powers[*a], y)
                });
            }
        }
        for (i, value) in values.iter().enumerate().skip(1) {
            let e = es(f, value);
            v.extend(e.into_iter().skip(1));
            if i + 1 == functions.len() {
                top = f.mul(&top, &value.iter().fold(f.o(), |z, x| f.mul(&z, x)));
            }
        }
    }
    v.push(top);
    Some(v)
}
fn points(
    f: &F,
    c: &crate::binary_ecc::BinaryCurve,
    p: &[P],
    rows: &[[[Cm; 4]; 5]; 4],
    a: u64,
) -> Vec<Vec<P>> {
    rows.iter()
        .map(|r| r.iter().map(|r| act(f, c, p, r, a)).collect())
        .collect()
}
fn digest(v: &[E]) -> String {
    hex::encode(crate::sha256_impl::sha256(
        &serde_json::to_vec(&values(v)).unwrap(),
    ))
}
fn rank_rows(mut rows: Vec<Vec<u64>>, columns: usize) -> (usize, Vec<usize>) {
    let mut rank = 0;
    let mut pivots = vec![];
    for j in 0..columns {
        let Some(i) = (rank..rows.len()).find(|i| rows[*i][j / 64] >> (j % 64) & 1 != 0) else {
            continue;
        };
        rows.swap(i, rank);
        for i in rank + 1..rows.len() {
            if rows[i][j / 64] >> (j % 64) & 1 != 0 {
                for k in j / 64..rows[i].len() {
                    rows[i][k] ^= rows[rank][k];
                }
            }
        }
        pivots.push(j);
        rank += 1;
        if rank == rows.len() {
            break;
        }
    }
    (rank, pivots)
}
fn eval_rank(v: &[Vec<E>]) -> (usize, Vec<usize>) {
    let n = v[0].len();
    let mut rows = vec![vec![0u64; n.div_ceil(64)]; v.len() * 168];
    for (i, point) in v.iter().enumerate() {
        for (j, z) in point.iter().enumerate() {
            let z = z.to_biguint();
            for bit in 0..168 {
                if z.bit(bit as u64) {
                    rows[168 * i + bit][j / 64] |= 1 << (j % 64);
                }
            }
        }
    }
    rank_rows(rows, n)
}
pub fn run(dir: &Path) {
    let data = read(&dir.join("g25-correct-polarization-orbit-sections.json"));
    let kernel = read(&dir.join("g17-odd-cover-kernel-and-commutators.json"));
    let inputs = read(&dir.join("g22-mixed-F2-orbit-sections.json"));
    let rows = decode_rows(&data);
    let weights = (0..4)
        .map(|i| data["weights"][i].as_u64().unwrap() as usize)
        .collect::<Vec<_>>();
    let bases = weights.iter().map(|n| basis(*n)).collect::<Vec<_>>();
    let count = 2 + 5 * bases.iter().map(|b| b.len() - 1).sum::<usize>();
    assert!(count <= 4096);
    let mut pairs = vec![];
    for i in 0..20 {
        for j in i + 1..20 {
            let a = rows[i / 5][i % 5];
            let b = rows[j / 5][j % 5];
            let witness = (0..4)
                .flat_map(|k| (k + 1..4).map(move |l| (k, l)))
                .find_map(|(k, l)| {
                    let x = cm(a[k], b[l]);
                    let y = cm(a[l], b[k]);
                    (x != y).then_some(json!({"columns":[k,l],"wedge":[x.0-y.0,x.1-y.1]}))
                })
                .expect("registered pairwise independence trigger");
            pairs.push(json!({"rows":[i,j],"witness":witness}));
        }
    }
    assert_eq!(pairs.len(), 190);
    let f = F::new(168, 95);
    let mut records = vec![];
    for sign in 0..2 {
        let a = data["records"][sign]["ordinary_a"].as_u64().unwrap();
        let c = f.curve(f.e(a), f.o());
        let mut probes = vec![];
        let mut vectors = vec![];
        let mut shifts = vec![];
        let mut poles = vec![];
        for (id, t) in inputs["records"][sign]["tuples"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
        {
            let p = t["input"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| point(&f, v))
                .collect::<Vec<_>>();
            let q = points(&f, &c, &p, &rows, a);
            let Some(s) = eval(&f, &q, &bases) else {
                poles.push(id);
                continue;
            };
            assert_eq!(s.len(), count);
            let rot = rotation(&c, &p);
            assert_eq!(
                eval(&f, &points(&f, &c, &rot, &rows, a), &bases).unwrap(),
                s
            );
            let fp = rotation(&c, &p.iter().map(|v| f.frobp(v, 1)).collect::<Vec<_>>());
            assert_eq!(
                eval(&f, &points(&f, &c, &fp, &rows, a), &bases).unwrap(),
                s.iter().map(|z| f.sq(z)).collect::<Vec<_>>()
            );
            if probes.is_empty() {
                for j in 0..4 {
                    for t in kernel["records"][sign]["torsion_bases"].as_array().unwrap() {
                        for key in ["P", "Q"] {
                            let mut shifted = p.clone();
                            shifted[j] = lf::plus(&c, &shifted[j], &point(&f, &t[key]));
                            let after = eval(&f, &points(&f, &c, &shifted, &rows, a), &bases);
                            if let Some(after) = after {
                                let difference = s.iter().zip(&after).position(|(x, y)| x != y);
                                shifts.push(json!({"coordinate":j,"prime":t["prime"],"basis":key,"shifted_input":shifted.iter().map(lf::pj).collect::<Vec<_>>(),"first_difference":difference,"difference_residue":difference.map(|i|s[i].add(&after[i]).to_biguint().to_str_radix(16)),"after_first_difference_value":difference.map(|i|after[i].to_biguint().to_str_radix(16)),"after_vector_sha256":digest(&after)}));
                            } else {
                                shifts.push(json!({"coordinate":j,"prime":t["prime"],"basis":key,"pole":true}));
                            }
                        }
                    }
                }
            }
            probes.push(json!({"input_probe":id,"section_values":values(&s),"vector_sha256":digest(&s),"top_product_value":s.last().unwrap().to_biguint().to_str_radix(16)}));
            vectors.push(s);
        }
        assert!(!vectors.is_empty());
        let (rank, pivots) = eval_rank(&vectors);
        let noninvariant = shifts
            .iter()
            .filter(|s| !s["first_difference"].is_null())
            .count();
        println!("G29:a{a},{count} ordinary orbit sections,{} tuples,binary evaluation rank{rank},{noninvariant} noninvariance witnesses,190 nonproportional row pairs",probes.len());
        records.push(json!({"graph":data["records"][sign]["graph"],"ordinary_a":a,"probes":probes,"pole_rejections":poles,"translation_diagnostics":shifts,"binary_evaluation_rank":rank,"binary_rank_pivot_columns":pivots,"binary_evaluation_rows":vectors.len()*168,"rank_is_only_a_section_independence_lower_bound":true}));
    }
    let ordinary_dimension = weights
        .iter()
        .fold(BigUint::from(1u32), |z, n| z * BigUint::from(*n).pow(5));
    let full_dimension = &ordinary_dimension * BigUint::from(57u32);
    let source_h0 = BigUint::from(87723u32) * BigUint::from(105u32).pow(8);
    let lean=String::from("import KoblitzRound20Retraction\nnamespace KoblitzRound20Sections\ndef degrees : Fin 5→Nat := ![45,315,63,171,57]\ndef basis (n:Nat):List (Nat×Nat) := [(0,0)]++(List.range (n/2)).map (fun i=>(i+1,0))++(List.range ((n-3)/2+1)).map (fun i=>(i,1))\ntheorem actual_basis_dimensions : ∀i:Fin 5,(basis (degrees i)).length=degrees i := by decide +kernel\ntheorem actual_affine_coordinates : ∀i:Fin 5,(basis (degrees i)).contains (1,0)=true ∧ (basis (degrees i)).contains (0,1)=true := by decide +kernel\ntheorem actual_pole_bounds : ∀i:Fin 5,(basis (degrees i)).all (fun (a,b)=>2*a+3*b≤degrees i)=true := by decide +kernel\ntheorem actual_infinity_coordinates : ∀i:Fin 5,(basis (degrees i)).contains ((degrees i-1)/2,0)=true ∧ (basis (degrees i)).contains ((degrees i-3)/2,1)=true := by decide +kernel\ntheorem actual_consecutive_poles : ∀i:Fin 5,2*((degrees i-1)/2)=degrees i-1 ∧ 2*((degrees i-3)/2)+3=degrees i := by decide +kernel\ntheorem actual_nonproportional_rows : ∀i j:Fin 20,i≠j→∃a b:Fin 4,KoblitzRound19OrbitCover.cmul (KoblitzRound20Retraction.row i a) (KoblitzRound20Retraction.row j b)≠KoblitzRound19OrbitCover.cmul (KoblitzRound20Retraction.row i b) (KoblitzRound20Retraction.row j a) := by decide +kernel\nend KoblitzRound20Sections\n#print axioms KoblitzRound20Sections.actual_basis_dimensions\n#print axioms KoblitzRound20Sections.actual_affine_coordinates\n#print axioms KoblitzRound20Sections.actual_pole_bounds\n#print axioms KoblitzRound20Sections.actual_infinity_coordinates\n#print axioms KoblitzRound20Sections.actual_consecutive_poles\n#print axioms KoblitzRound20Sections.actual_nonproportional_rows\n");
    save_lean(dir, "KoblitzRound20Sections.lean", lean);
    crate::round02::save(
        dir,
        "g29-full-weight-orbit-sections.json",
        &json!({"schema":"koblitz.round20/g29-v1","weights":weights,"elliptic_factor_bases":bases,"companion_degree":57,"companion_basis":basis(57),"ordinary_sparse_sections":count,"with_companion_factorized_sparse_sections":count*57,"ordinary_full_factorized_tensor_coordinates":ordinary_dimension.to_string(),"with_companion_full_factorized_tensor_coordinates":full_dimension.to_string(),"actual_source_bundle_h0":source_h0.to_string(),"full_tensor_coordinates_materialized":false,"complete_native_section_space_constructed":false,"records":records,"all_row_pair_independence_witnesses":pairs,"ordinary_sparse_system_has_no_fixed_divisor_under_written_bridge":true,"ordinary_sparse_system_basepoint_free_asserted":false,"factorized_complete_tensor_system_gives_closed_source_embedding_under_written_bridge":true,"canonical_quotient_243_sections_constructed":false,"scope":"explicit full elliptic bases and factorized closed embedding of the scalar cover;2952 ordinary sparse orbit sections and native evaluation ranks;not a243-coordinate quotient embedding"}),
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binary_rank_matches_independent_small_span() {
        for seed in 0u64..128 {
            let r = (0..4)
                .map(|i| vec![(seed * (2 * i + 1) + 7 * i) & 31])
                .collect::<Vec<_>>();
            let mut s = BTreeSet::from([0]);
            for v in &r {
                let more = s.iter().map(|x| x ^ v[0]).collect::<Vec<_>>();
                s.extend(more);
            }
            let (rank, _) = rank_rows(r, 5);
            assert_eq!(s.len(), 1usize << rank);
        }
    }
}
