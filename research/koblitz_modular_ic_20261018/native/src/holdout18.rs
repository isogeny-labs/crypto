//! Independent F1024 incidences from actual7-torsion over F2^30, then finite-fit rejection.
use super::*;
fn point(c: &Curve, x: u32) -> Option<Pt> {
    let f = c.f;
    let d = f.mul(c.a(), x) ^ c.b;
    let rhs = f.mul(x, f.sq(x)) ^ f.mul(c.a2(), f.sq(x)) ^ c.a6();
    let y = if d == 0 {
        f.frob(rhs, f.m - 1)
    } else {
        f.mul(d, f.quadratic(f.mul(rhs, f.inv(f.sq(d))))?)
    };
    let p = Pt::A(x, y);
    assert!(c.valid(p));
    Some(p)
}
fn count(c: &Curve, s: &crate::gf2k::SmallField) -> i64 {
    let mut n = 1;
    for x in 0..s.q as u16 {
        let d = s.mul(c.a() as u16, x) ^ c.b as u16;
        let x2 = s.sqr(x);
        let rhs = s.mul(x, x2) ^ s.mul(c.a2() as u16, x2) ^ c.a6() as u16;
        n += if d == 0 {
            1
        } else if s.trace(s.mul(rhs, s.inv(s.sqr(d)))) == 0 {
            2
        } else {
            0
        };
    }
    n
}
pub fn run(dir: &Path) {
    let fits = crate::round04::read(&dir.join("cm21-affine-linear-elliptic-map-search.json"));
    let sf = crate::gf2k::SmallField::new(10);
    let f = Field {
        m: 10,
        modulus: sf.poly as u64,
    };
    assert!(f.irreducible());
    let small = Field {
        m: 5,
        modulus: fits["modulus"].as_u64().unwrap(),
    };
    let from32 = f.embed(small);
    let ext = Field::find(30);
    let em = ext.embed(f);
    let (circuit, _, _) = circuit(false);
    let mut holds = vec![];
    let mut scanned = 0;
    let mut bases = 0;
    let mut skipped = vec![];
    let mut seen = BTreeSet::new();
    'outer: for b in 1u32..1024 {
        for cc in 1u32..1024 {
            scanned += 1;
            let a = 1 ^ cc;
            if b ^ f.mul(cc, f.mul(a, f.sq(a))) == 0 {
                continue;
            }
            if f.frob(b, 5) == b && f.frob(cc, 5) == cc {
                continue;
            }
            if seen.contains(&(b, cc)) {
                continue;
            }
            let c = Curve {
                f,
                b,
                c: cc,
                twist: false,
            };
            if c.times(Pt::A(0, 0), 11) != Pt::O {
                continue;
            }
            bases += 1;
            let n = count(&c, &sf);
            assert_eq!(n % 11, 0);
            let t = 1025 - n;
            let t3 = t * t * t - 3 * 1024 * t;
            let n3 = (1i64 << 30) + 1 - t3;
            if n3 % 7 != 0 {
                continue;
            }
            let mut cofactor = n3 as usize;
            let mut val = 0;
            while cofactor % 7 == 0 {
                cofactor /= 7;
                val += 1;
            }
            let ce = Curve {
                f: ext,
                b: em[b as usize],
                c: em[cc as usize],
                twist: false,
            };
            let mut accepted = None;
            let mut attempts = 0;
            for k in 1u32..=256 {
                attempts += 1;
                let Some(p) = point(&ce, k) else {
                    continue;
                };
                let mut q = ce.times(p, cofactor);
                if q == Pt::O {
                    continue;
                }
                let mut reduced = 0;
                while ce.times(q, 7) != Pt::O {
                    q = ce.times(q, 7);
                    reduced += 1;
                    assert!(reduced < val);
                }
                let frob = |p: Pt| match p {
                    Pt::O => p,
                    Pt::A(x, y) => Pt::A(ext.frob(x, 10), ext.frob(y, 10)),
                };
                let mut triple = [q, ce.times(q, 2), ce.times(q, 4)];
                if !triple.contains(&frob(q)) {
                    q = ce.add(frob(q), ce.neg(q));
                    if q == Pt::O {
                        continue;
                    }
                    triple = [q, ce.times(q, 2), ce.times(q, 4)];
                    if !triple.contains(&frob(q)) {
                        continue;
                    }
                }
                let (Pt::A(x, y), Pt::A(xx, yy)) = (q, ce.times(q, 2)) else {
                    unreachable!()
                };
                let l = ext.mul(y ^ yy, ext.inv(x ^ xx));
                let nu = y ^ ext.mul(l, x);
                let Some(li) = em.iter().position(|v| *v == l) else {
                    continue;
                };
                let Some(ni) = em.iter().position(|v| *v == nu) else {
                    continue;
                };
                let vars = [b, cc, li as u32, ni as u32];
                assert!(eval(&circuit, f, vars).is_some());
                accepted = Some(
                    json!({"input":vars,"curve_cardinality_F1024":n,"trace_F1024":t,"curve_cardinality_F2_30":n3,"seven_primary_valuation":val,"cofactor":cofactor,"point_attempts":attempts,"seven_torsion_Q":match q{Pt::A(x,y)=>vec![x,y],_=>unreachable!()},"Q2_Q4":triple.iter().map(|v|match v{Pt::A(x,y)=>vec![*x,*y],_=>unreachable!()}).collect::<Vec<_>>(),"independent_order11_and_order7_checked":true,"incidence_circuit_checked":true}),
                );
                break;
            }
            if let Some(row) = accepted {
                for j in 0..10 {
                    seen.insert((f.frob(b, j), f.frob(cc, j)));
                }
                holds.push(row);
                if holds.len() == 8 {
                    break 'outer;
                }
            } else {
                skipped.push(json!({"b":b,"c":cc,"reason":"256 actual torsion-point attempts exhausted","attempts":attempts}));
            }
        }
    }
    assert_eq!(holds.len(), 8, "holdout construction cap exhausted");
    let mut records = vec![];
    for rr in fits["records"].as_array().unwrap() {
        let a = rr["a"].as_u64().unwrap() as u32;
        let mut accepted = vec![];
        let mut rejected = vec![];
        let mut constants = 0;
        for (id, cand) in rr["finite_fit_candidates"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
        {
            let xc = std::array::from_fn::<_, 5, _>(|j| {
                from32[cand["x_coefficients"][j].as_u64().unwrap() as usize]
            });
            let yc = std::array::from_fn::<_, 5, _>(|j| {
                from32[cand["y_coefficients"][j].as_u64().unwrap() as usize]
            });
            let mut fail = None;
            for (i, h) in holds.iter().enumerate() {
                let v = &h["input"];
                let basis = [
                    1,
                    v[0].as_u64().unwrap() as u32,
                    v[1].as_u64().unwrap() as u32,
                    v[2].as_u64().unwrap() as u32,
                    v[3].as_u64().unwrap() as u32,
                ];
                let x = (0..5).fold(0, |z, j| z ^ f.mul(xc[j], basis[j]));
                let y = (0..5).fold(0, |z, j| z ^ f.mul(yc[j], basis[j]));
                let residual = f.sq(y) ^ f.mul(x, y) ^ f.mul(x, f.sq(x)) ^ f.mul(a, f.sq(x)) ^ 1;
                if residual != 0 {
                    fail = Some(
                        json!({"candidate_id":id,"first_holdout":i,"x":x,"y":y,"nonzero_residual":residual}),
                    );
                    break;
                }
            }
            if let Some(v) = fail {
                rejected.push(v);
            } else if cand["constant_coefficients_only"] == true {
                constants += 1;
            } else {
                accepted.push(json!({"candidate_id":id,"candidate":cand}));
            }
        }
        println!("CM22:a{a},8 independent holdouts reject{} finite fits;{constants} constants retained;{} nonconstant survivors",rejected.len(),accepted.len());
        records.push(json!({"a":a,"rejected_finite_fits":rejected,"constant_maps_retained":constants,"nonconstant_survivors":accepted}));
    }
    crate::round02::save(
        dir,
        "cm22-independent-elliptic-map-holdouts.json",
        &json!({"schema":"koblitz.round18/cm22-v1","field_F1024_modulus":f.modulus,"extension_F2_30_modulus":ext.modulus,"embedding_F32_into_F1024":from32,"embedding_F1024_into_F2_30":em,"Tate_pairs_scanned":scanned,"order11_bases":bases,"skipped_torsion_caps":skipped,"holdouts":holds,"records":records,"scope":"independent geometric point constructions refute finite-fit affine-linear candidates with exact residuals;does not exclude higher-degree or rational maps"}),
    );
}

pub fn proof(dir: &Path) {
    let doc = crate::round04::read(&dir.join("cm22-independent-elliptic-map-holdouts.json"));
    let poly = doc["field_F1024_modulus"].as_u64().unwrap();
    let mut cases = vec![];
    let mut counts = vec![];
    for row in doc["records"].as_array().unwrap() {
        let a = row["a"].as_u64().unwrap();
        let mut n = 0;
        for r in row["rejected_finite_fits"].as_array().unwrap() {
            let x = r["x"].as_u64().unwrap();
            let y = r["y"].as_u64().unwrap();
            let z = r["nonzero_residual"].as_u64().unwrap();
            cases.push(format!("({a},{x},{y},{z})"));
            n += 1;
        }
        counts.push(n);
    }
    assert_eq!(counts, vec![90, 12]);
    let lean=format!("import KoblitzRound18\nnamespace KoblitzRound18MapCounterexamples\ndef mul (a b:Nat) : Nat :=\n let raw:=(List.range 10).foldl (fun z i => if b.testBit i then Nat.xor z (a*2^i) else z) 0\n ((List.range 10).reverse).foldl (fun z i => if z.testBit (10+i) then Nat.xor z ({poly}*2^i) else z) raw\ndef residual (a x y:Nat) : Nat := Nat.xor (Nat.xor (Nat.xor (Nat.xor (mul y y) (mul x y)) (mul x (mul x x))) (mul a (mul x x))) 1\ndef cases : List (Nat×Nat×Nat×Nat) := [{}]\ndef check (v:Nat×Nat×Nat×Nat) : Bool := let (a,x,y,r):=v;decide (residual a x y=r ∧ r≠0)\ntheorem all_actual_rejections : cases.all check=true := by decide +kernel\ntheorem actual_rejection_count : cases.length=102 := by decide +kernel\ntheorem actual_sign_counts : (cases.countP fun v => v.1=0)=90 ∧ (cases.countP fun v => v.1=1)=12 := by decide +kernel\nend KoblitzRound18MapCounterexamples\n#print axioms KoblitzRound18MapCounterexamples.all_actual_rejections\n#print axioms KoblitzRound18MapCounterexamples.actual_rejection_count\n#print axioms KoblitzRound18MapCounterexamples.actual_sign_counts\n",cases.join(","));
    let path = dir.join("KoblitzRound18MapCounterexamples.lean");
    assert!(!path.exists());
    std::fs::write(path, lean).unwrap();
    println!("CM22 proof:102 exact nonzero F1024 bit-polynomial residuals exported;geometric point construction and search exhaustion remain native");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trace_cardinality_matches_full_tate_point_enumeration() {
        let s = crate::gf2k::SmallField::new(4);
        let f = Field {
            m: 4,
            modulus: s.poly as u64,
        };
        for b in 1..16 {
            for cc in 1..16 {
                let a = 1 ^ cc;
                if b ^ f.mul(cc, f.mul(a, f.sq(a))) == 0 {
                    continue;
                }
                let c = Curve {
                    f,
                    b,
                    c: cc,
                    twist: false,
                };
                let n = 1
                    + (0..16)
                        .map(|x| (0..16).filter(|y| c.valid(Pt::A(x, *y))).count() as i64)
                        .sum::<i64>();
                assert_eq!(count(&c, &s), n);
            }
        }
    }
}
