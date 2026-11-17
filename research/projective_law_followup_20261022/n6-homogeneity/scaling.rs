//! Exact integer illustration of the Lean W-scaling identities at a fixed witness.
fn main() {
    println!("mu,aa_squared_w,aa_linear_w,linear_defect");
    for mu in 1_i128..=5 {
        let squared = -mu.pow(3);
        let linear = -mu.pow(2);
        let defect = mu.pow(2) * (mu - 1);
        assert_eq!(linear - squared, defect);
        println!("{mu},{squared},{linear},{defect}");
    }
}
