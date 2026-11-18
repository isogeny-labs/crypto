// Real sources (unchanged files of the repository).
// f4_fp.rs and f4_fp/ are symlinks to the real sources (the file has
// out-of-line submodules, which a #[path] file cannot carry).
pub mod f4_fp;
#[path = "../../../../../src/cryptanalysis/f4_fp_tower.rs"]
pub mod f4_fp_tower;
#[path = "../../../../../src/cryptanalysis/f4_tower_generic.rs"]
pub mod f4_tower_generic;
#[path = "../../../../../src/cryptanalysis/fp_backend.rs"]
pub mod fp_backend;
#[path = "../../../../../src/cryptanalysis/fp_backend_ifma.rs"]
pub mod fp_backend_ifma;
#[path = "../../../../../src/cryptanalysis/fx_hash.rs"]
pub(crate) mod fx_hash;
#[path = "../../../../../src/cryptanalysis/hky_closure_io.rs"]
pub mod hky_closure_io;
#[path = "../../../../../src/cryptanalysis/hky_fp_tower.rs"]
pub mod hky_fp_tower;
#[path = "../../../../../src/cryptanalysis/hky_fp_tower_fast.rs"]
pub mod hky_fp_tower_fast;
#[path = "../../../../../src/cryptanalysis/hky_fp_tower_fast2.rs"]
pub mod hky_fp_tower_fast2;
#[path = "../../../../../src/cryptanalysis/residual_walk.rs"]
pub mod residual_walk;
#[path = "../../../../../src/cryptanalysis/sig_fp_tower.rs"]
pub mod sig_fp_tower;

// Stubs (this directory): what the files above name in modules whose own
// dependencies reach the files of the root crate that do not parse.
pub mod glv_invariant_base;
pub mod groebner_f4;
pub mod ic_boundary;
pub mod jv_bitops;
pub mod pq_groebner_f2;
