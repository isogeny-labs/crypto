//! Actual square/cube source residuals, excluding the four Q26 models.
use super::*;
pub fn run(dir: &Path) {
    let f = F::new(360, 45);
    assert!(f.irreducible());
    let old = read(&dir.join("q26-after-companion-cube-repair.json"));
    let mut records = vec![];
    let mut lean = String::from("import KoblitzRound18Boundary\nnamespace KoblitzRound18Squares\n");
    for g in 0..4 {
        let doc = read(&dir.join(format!("q8-graph-{g}-F2-relations.json")));
        let p = &doc["program"];
        let mut family = vec![];
        for input in [0usize, 2, 4, 6, 8] {
            let mut memo = BTreeMap::new();
            let mut square = None;
            let mut cube = None;
            for i in 0..p["nodes"].as_array().unwrap().len() {
                if let Some(v) = form(&f, p, i, input, &mut memo) {
                    if v.d == 2
                        && v.p.len() == 1
                        && v.p.get(&(0, 2)) == Some(&f.o())
                        && square.is_none()
                    {
                        square = Some((i, v.clone()));
                    }
                    if v.d == 3
                        && v.p.len() == 1
                        && v.p.get(&(0, 3)) == Some(&f.o())
                        && cube.is_none()
                    {
                        cube = Some((i, v));
                    }
                    if square.is_some() && cube.is_some() {
                        break;
                    }
                }
            }
            let (u, s) = square.unwrap();
            let (v, c) = cube.unwrap();
            let depth = 3 + s.k.max(c.k);
            let reject = old["records"][g]["model"][u] == 1;
            let j = input / 2;
            lean.push_str(&format!("theorem square_separator_boundary_{g}_{j} (V:ZMod 2) : ((1:ZMod 2)^3-0*V^2)=1 := by simp\n"));
            if reject {
                lean.push_str(&format!("theorem actual_Q26_unit_{g} : KoblitzRound18Boundary.point{g}.contains {u}=true := by decide +kernel\n"));
            }
            family.push(json!({"input":input,"square_node":u,"cube_node":v,"square_depth":s.k,"cube_depth":c.k,"membership_depth":depth,"rejects_Q26":reject,"source_certificate":memo.iter().filter_map(|(i,z)|z.as_ref().map(|z|json!({"node":i,"gate":p["nodes"][*i],"degree":z.d,"h_depth":z.k,"terms":z.p.iter().map(|(m,c)|json!([m.0,m.1,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>()}))).collect::<Vec<_>>()}));
        }
        assert!(family.iter().any(|r| r["rejects_Q26"] == true));
        println!(
            "Q27:graph{g},five actual square/cube source residuals;Q26 companion square excluded"
        );
        records
            .push(json!({"graph":g,"family":family,"Q26_excluded":true,"full_saturation":false}));
    }
    lean.push_str("end KoblitzRound18Squares\n");
    for g in 0..4 {
        for j in 0..5 {
            lean.push_str(&format!(
                "#print axioms KoblitzRound18Squares.square_separator_boundary_{g}_{j}\n"
            ));
        }
        lean.push_str(&format!(
            "#print axioms KoblitzRound18Squares.actual_Q26_unit_{g}\n"
        ));
    }
    let path = dir.join("KoblitzRound18Squares.lean");
    assert!(!path.exists());
    std::fs::write(path, lean).unwrap();
    crate::round02::save(
        dir,
        "q27-square-cube-source-separators.json",
        &json!({"schema":"koblitz.round18/q27-v1","records":records,"universal_residual":"KoblitzRound18.square_cube_residual","scope":"source-bound h^(3+max depths) cubic saturation relations;radical boundary square zeros;not whole saturation"}),
    );
}
