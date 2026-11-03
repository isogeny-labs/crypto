//! Generate literal Lean certificates from the actual round15 records.
use crate::round04::read;
use serde_json::Value;
use std::path::Path;
fn matrix(v: &Value) -> String {
    format!(
        "!![{}]",
        v.as_array()
            .unwrap()
            .iter()
            .map(|r| r
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(","))
            .collect::<Vec<_>>()
            .join(";")
    )
}
fn bits(b: u64) -> String {
    format!(
        "![{}]",
        (0..4)
            .map(|i| ((b >> i) & 1).to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}
fn save(dir: &Path, name: &str, s: String) {
    let p = dir.join(name);
    assert!(!p.exists());
    std::fs::write(p, s).unwrap();
}
fn expr(p: &[num_bigint::BigInt]) -> String {
    use num_traits::Zero;
    p.iter()
        .enumerate()
        .filter(|(_, c)| !c.is_zero())
        .map(|(i, c)| format!("({c}:R)*x^{i}"))
        .collect::<Vec<_>>()
        .join(" + ")
}
pub fn run(dir: &Path) {
    let data = read(&dir.join("g15-theta-class-descent.json"));
    let mut s=String::from("import KoblitzRound15\nset_option maxRecDepth 100000\nset_option maxHeartbeats 0\nnamespace KoblitzRound15Theta\n");
    let mut names = vec![];
    s.push_str("def rotation : Matrix (Fin 4) (Fin 4) ℤ := !![-1,-1,-1,-1;1,0,0,0;0,1,0,0;0,0,1,0]\ntheorem rotation_order : rotation^5=1 := by decide +kernel\n");
    names.push("rotation_order".into());
    for (g, r) in data["records"].as_array().unwrap().iter().enumerate() {
        s.push_str(&format!("def pic{g} : Matrix (Fin 4) (Fin 4) (ZMod 2) := {}\ndef corr{g} : Fin 4 → ZMod 2 := {}\ndef drift{g} : Fin 4 → ZMod 2 := {}\ntheorem actual_theta_correction{g} : Matrix.mulVec pic{g} corr{g}+drift{g}=corr{g} := by decide +kernel\ntheorem picard_inverse{g} : (1-pic{g})*(pic{g}+pic{g}^3)=1 ∧ (pic{g}+pic{g}^3)*(1-pic{g})=1 := by decide +kernel\n",matrix(&r["Picard_Frobenius_mod2"]),bits(r["actual_theta_correction_bits"].as_u64().unwrap()),bits(r["actual_theta_drift_bits"].as_u64().unwrap())));
        s.push_str(&format!("def charFrob{g} : Matrix (Fin 2) (Fin 2) (ZMod 19) := {}\ndef charInv{g} : Matrix (Fin 2) (Fin 2) (ZMod 19) := {}\ntheorem actual_character_inverse{g} : (1-charFrob{g})*charInv{g}=1 ∧ charInv{g}*(1-charFrob{g})=1 := by decide +kernel\n",matrix(&r["Cartier_character_Frobenius"]),matrix(&r["I_minus_C_inverse"])));
        for n in [
            "actual_theta_correction",
            "picard_inverse",
            "actual_character_inverse",
        ] {
            names.push(format!("{n}{g}"));
        }
    }
    s.push_str("end KoblitzRound15Theta\n");
    for n in names {
        s.push_str(&format!("#print axioms KoblitzRound15Theta.{n}\n"));
    }
    save(dir, "KoblitzRound15Theta.lean", s);
    let cm = read(&dir.join("cm15-one-prime-carriers.json"));
    let mut s = String::from("import KoblitzRound15\nnamespace KoblitzRound15CM\n");
    let mut names = vec![];
    for r in cm["winners"].as_array().unwrap() {
        let n = r["n"].as_u64().unwrap();
        let p = r["p"].as_u64().unwrap();
        let g = r["carrier_genus"].as_u64().unwrap();
        let b = r["base_X0_genus"].as_u64().unwrap();
        let c = r["independent_coset_genus_check"]["cycles_S_ST_T"]
            .as_array()
            .unwrap();
        let index = r["independent_coset_genus_check"]["index"]
            .as_u64()
            .unwrap();
        s.push_str(&format!("theorem auxiliary_prime_{n} : Nat.Prime {p} := by norm_num\ntheorem genus_{n} : 2*{n}*({b}-1)+1={g} ∧ 2*{g}+( {}+{}+{})={index}+2 := by norm_num\n",c[0],c[1],c[2]));
        let roots = r["roots_of_X2_minus_X_plus2"].as_array().unwrap();
        s.push_str(&format!("theorem split_two_{n} : (({}:ZMod {p})^2-{}+2)=0 ∧ (({}:ZMod {p})^2-{}+2)=0 := by decide +kernel\n",roots[0],roots[0],roots[1],roots[1]));
        for name in ["auxiliary_prime", "genus", "split_two"] {
            names.push(format!("{name}_{n}"));
        }
    }
    let minus = crate::cmray15::an(5, -1);
    let plus = crate::cmray15::an(5, 1);
    let product = cm["winners"][0]["exact_Frobenius_polynomial_X_ascending"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().parse().unwrap())
        .collect::<Vec<_>>();
    s.push_str(&format!("theorem actual_n5_Euler_product {{R : Type*}} [CommRing R] (x:R) : ({})*({})={} := by ring\n",expr(&minus),expr(&plus),expr(&product)));
    names.push("actual_n5_Euler_product".into());
    s.push_str("theorem odd_character_involution {R : Type*} [CommRing R] (a b : R) : (-a)+(-b)=-(a+b) := by ring\nend KoblitzRound15CM\n");
    names.push("odd_character_involution".into());
    for n in names {
        s.push_str(&format!("#print axioms KoblitzRound15CM.{n}\n"));
    }
    save(dir, "KoblitzRound15CM.lean", s);
    println!("Round15:generated actual theta/Picard/Cartier and CM finite certificates");
}
