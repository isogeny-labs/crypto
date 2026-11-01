//! Actual rotation-lift test for the selected odd diagonal cover.
use super::*;
fn scalar(a: &M, n: &Z) -> M {
    a.iter()
        .map(|r| r.iter().map(|x| x * n).collect())
        .collect()
}
fn remmat(a: &M, m: i64) -> M {
    a.iter()
        .map(|r| r.iter().map(|x| Z::from(red(x, m))).collect())
        .collect()
}
pub fn run(dir: &Path) {
    let doc = read(&dir.join("g16-diagonal-cover-section-bases.json"));
    let b = doc["records"][0]["B_integral8"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            r.as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().parse::<Z>().unwrap())
                .collect()
        })
        .collect::<M>();
    assert_eq!(
        doc["records"][0]["B_integral8"],
        doc["records"][2]["B_integral8"]
    );
    let d = det(b.clone());
    let ba = adj(&b);
    assert_eq!(mm(&ba, &b), scalar(&ident(8), &d));
    assert_eq!(mm(&b, &ba), scalar(&ident(8), &d));
    let mut r = vec![vec![Z::zero(); 8]; 8];
    for i in 0..2 {
        for j in 0..4 {
            r[i][2 * j + i] = (-1).into();
        }
        for j in 0..3 {
            r[2 * (j + 1) + i][2 * j + i] = 1.into();
        }
    }
    let mut r5 = ident(8);
    for _ in 0..5 {
        r5 = mm(&r5, &r);
    }
    assert_eq!(r5, ident(8));
    let a = mm(&ba, &mm(&r, &b));
    assert_eq!(mm(&b, &a), scalar(&mm(&r, &b), &d));
    let gcd = a.iter().flatten().fold(d.clone(), |u, v| u.gcd(v));
    let den = &d / &gcd;
    let num = a
        .iter()
        .map(|r| r.iter().map(|x| x / &gcd).collect())
        .collect::<M>();
    let entries=(0..8).flat_map(|i|{let a=&a;let d=&d;(0..8).map(move|j|{let g=d.gcd(&a[i][j]);json!({"row":i,"column":j,"numerator":(&a[i][j]/&g).to_string(),"denominator":(d/&g).to_string()})})}).collect::<Vec<_>>();
    let mut witness = None;
    for p in [3i64, 5, 7] {
        let mut m = p;
        while (&d % Z::from(m)).is_zero() {
            if let Some((i, j)) = (0..8)
                .flat_map(|i| (0..8).map(move |j| (i, j)))
                .find(|(i, j)| red(&a[*i][*j], m) != 0)
            {
                witness = Some((m, i, j));
                break;
            }
            m *= p;
        }
        if witness.is_some() {
            break;
        }
    }
    let (m, i, j) = witness.expect("selected diagonal cover has a nonintegral rotation lift");
    let l = vec![ba[i].clone()];
    assert_eq!(remmat(&mm(&l, &b), m), vec![vec![Z::zero(); 8]]);
    assert_ne!(red(&mm(&l, &mm(&r, &b))[0][j], m), 0);
    let lean=format!("import KoblitzRound18\nnamespace KoblitzRound18Frobenius\ndef B : Matrix (Fin 8) (Fin 8) (ZMod {m}) := {}\ndef R : Matrix (Fin 8) (Fin 8) (ZMod {m}) := {}\ndef L : Matrix (Fin 1) (Fin 8) (ZMod {m}) := {}\ntheorem actual_rotation_obstruction : L*B=0 ∧ L*(R*B)≠0 := by decide +kernel\ntheorem no_rotation_lift : ¬ ∃ A:Matrix (Fin 8) (Fin 8) (ZMod {m}),B*A=R*B := by\n rintro ⟨A,hA⟩\n have hz : L*(R*B)=0 := by rw [←hA,←Matrix.mul_assoc,actual_rotation_obstruction.1,Matrix.zero_mul]\n exact actual_rotation_obstruction.2 hz\nend KoblitzRound18Frobenius\n#print axioms KoblitzRound18Frobenius.actual_rotation_obstruction\n#print axioms KoblitzRound18Frobenius.no_rotation_lift\n",lit(&remmat(&b,m),false),lit(&remmat(&r,m),false),lit(&remmat(&l,m),false));
    let path = dir.join("KoblitzRound18Frobenius.lean");
    assert!(!path.exists());
    std::fs::write(path, lean).unwrap();
    println!("G20:odd B rotation conjugate denominator{den};mod{m} row{i}/column{j} excludes an integral lift on this selected cover");
    crate::round02::save(
        dir,
        "g20-odd-cover-rotation-lift-obstruction.json",
        &json!({"schema":"koblitz.round18/g20-v1","graphs":[0,2],"B":matv(&b),"R":matv(&r),"detB":d.to_string(),"rotation_order":5,"R5_identity_checked":true,"B_adjugate":matv(&ba),"lift_numerator":matv(&num),"lift_denominator":den.to_string(),"rational_entries":entries,"obstruction_modulus":m,"obstruction_row":i,"obstruction_column":j,"left_annihilator":matv(&remmat(&l,m)),"L_B_zero_checked":true,"L_R_B_nonzero_checked":true,"selected_E4_cover_rotation_lift_exists":false,"quotient_F2_descent_excluded":false,"scope":"exact integer no-lift certificate for selected diagonal B and actual augmentation R;does not exclude quotient/bundle descent or other covers"}),
    );
}
