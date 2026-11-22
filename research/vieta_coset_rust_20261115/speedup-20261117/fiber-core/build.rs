use std::{env, fs, path::PathBuf};
mod original {
    include!("../transform-core/build.rs");
    pub fn generate() {
        main();
    }
}
fn main() {
    original::generate();
    println!("cargo:rerun-if-changed=../transform-core/build.rs");
    println!("cargo:rerun-if-changed=quadratic_method.rs");
    println!("cargo:rerun-if-changed=root_tests.rs");
    let path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("vieta_product_seam.rs");
    let mut s = fs::read_to_string(&path).unwrap();
    let anchor = "    fn reset_polynomial_cache(&self) {}";
    assert_eq!(s.matches(anchor).count(), 1);
    s = s.replace(
        anchor,
        &format!("{anchor}\n    fn root_shortcuts(&self) -> bool {{ false }}"),
    );
    let anchor = "    fn roots(&self, a: &[F::E], seed: &mut u64) -> Result<Vec<F::E>> {";
    assert_eq!(s.matches(anchor).count(), 1);
    s = s.replace(
        anchor,
        &format!(
            "{}\n{anchor}",
            fs::read_to_string("quadratic_method.rs").unwrap()
        ),
    );
    let anchor =
        "        let xp = self.powmod(&[self.zero(), self.one()], &self.field.modulus(), a)?;";
    assert_eq!(s.matches(anchor).count(), 1);
    s=s.replace(anchor,&format!("        if self.field.root_shortcuts() && a.len() <= 3 {{ return self.short_roots(a); }}\n{anchor}"));
    let anchor = "            if g.len() == 2 {";
    assert_eq!(s.matches(anchor).count(), 1);
    s=s.replace(anchor,&format!("            if self.field.root_shortcuts() && g.len() <= 3 {{ out.extend(self.short_roots(&g)?); continue; }}\n{anchor}"));
    s.push_str(&fs::read_to_string("root_tests.rs").unwrap());
    fs::write(path, s).unwrap();
}
