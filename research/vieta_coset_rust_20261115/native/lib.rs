// Compile exactly the crypto library implementation with a small dependency
// graph. No copy of arithmetic or solver is maintained in this harness.
#[path = "../../../src/cryptanalysis/vieta_coset.rs"]
pub mod vieta_coset;
