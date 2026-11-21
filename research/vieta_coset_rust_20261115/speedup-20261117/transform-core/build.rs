use std::{env, fs, path::PathBuf};
fn main() {
    let source = PathBuf::from("../../../../src/cryptanalysis/vieta_coset.rs");
    println!("cargo:rerun-if-changed={}", source.display());
    let mut s = fs::read_to_string(&source).unwrap();
    let rational = source
        .parent()
        .unwrap()
        .join("vieta_coset/rational_tower.rs")
        .canonicalize()
        .unwrap();
    println!("cargo:rerun-if-changed={}", rational.display());
    s = s.replace(
        "#[path = \"vieta_coset/rational_tower.rs\"]",
        &format!("#[path = {:?}]", rational),
    );
    s = s
        .lines()
        .map(|l| {
            if l.starts_with("//!") {
                l.replacen("//!", "//", 1)
            } else {
                l.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let anchor = "    fn canonical(&self, a: &Self::E) -> bool;";
    assert_eq!(s.matches(anchor).count(), 1);
    s = s.replace(anchor, &format!("{anchor}\n    fn reset_polynomial_cache(&self) {{}}\n    fn convolution(&self, _a: &[Self::E], _b: &[Self::E], _limits: &Limits) -> Result<Option<(Vec<Self::E>, Work)>> {{ Ok(None) }}"));
    let anchor = "        if self.polynomials != PolynomialBackend::Schoolbook {\n            return self.pmul_karatsuba(a, b);\n        }";
    assert_eq!(s.matches(anchor).count(), 1);
    s = s.replace(anchor, "        if self.polynomials != PolynomialBackend::Schoolbook {\n            if let Some((v, work)) = self.field.convolution(a, b, &self.limits)? {\n                self.count(|w| { w.muls += work.muls; w.adds += work.adds; });\n                self.check()?;\n                return Ok(self.trim(v));\n            }\n            return self.pmul_karatsuba(a, b);\n        }");
    let anchor = "        Ok(Self {\n            field,";
    assert_eq!(s.matches(anchor).count(), 1);
    s = s.replace(
        anchor,
        "        field.reset_polynomial_cache();\n        Ok(Self {\n            field,",
    );
    fs::write(
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("vieta_product_seam.rs"),
        s,
    )
    .unwrap();
}
