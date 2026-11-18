pub mod f4_fp_tower {
    // verbatim: src/cryptanalysis/f4_fp_tower.rs, `SquareRule`
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct SquareRule {
        pub next: Option<u8>,
        pub a: u64,
        pub b: u64,
        pub c: u64,
        pub d: u64,
    }
}
#[path = "../../../../../src/cryptanalysis/hky_fp_tower.rs"]
pub mod hky_fp_tower;
#[path = "../../../../../src/cryptanalysis/hky_fp_tower_fast.rs"]
pub mod hky_fp_tower_fast;
#[path = "../../../../../src/cryptanalysis/hky_fp_tower_fast2.rs"]
pub mod hky_fp_tower_fast2;
#[path = "../../../../../src/cryptanalysis/hky_closure_io.rs"]
pub mod hky_closure_io;
