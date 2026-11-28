// Isolated compilation of the *actual* root-finder source. These stubs
// only supply external module types and are NEVER used for performance
// measurements or correctness paths selected in the workflow.
pub mod prime_root_trace {
    pub const MAX_PRIME: u64 = (1u64 << 61) - 1;
    pub const MAX_WORK: u64 = 1_000_000;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RootStop { Complete }
    #[derive(Clone, Debug)]
    pub struct RootLimits {
        pub field_work: u64,
        pub split_attempts: u64,
        pub events: u64,
        pub degree: usize,
    }
    #[derive(Clone, Debug)]
    pub struct RootReport {
        pub stop: RootStop,
        pub roots: Vec<u64>,
    }
    pub fn solve<F: FnMut(&()) -> Result<(), String>>(
        _polynomial: &[u64], _p: u64, _seed: u64,
        _limits: RootLimits, _sink: &mut F,
    ) -> Result<RootReport, String> {
        Err("prime-root-trace fallback deliberately unavailable in isolated test".into())
    }
}
pub mod residual_walk {
    // The selected tests only construct known prime fields, so no primality
    // claim is made by this stub. Run full repository tests for that coverage.
    pub fn is_prime_u64(p: u64) -> bool { p > 3 && p % 2 != 0 }
}

#[path = "../../../../../src/cryptanalysis/prime_radical_root.rs"]
pub mod prime_radical_root;
