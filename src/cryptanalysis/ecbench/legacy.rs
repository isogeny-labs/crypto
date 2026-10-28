//! Accounting profiles under which committed records still replay.
//!
//! `docs/ecbench/README.md` §9: a method's counts are frozen by its id, and
//! every committed session must replay bit for bit.  Several changes to the
//! inherited-F4 / F6-IC solver and to the pipeline's pricing were merged
//! after sessions had been recorded and altered what the *same* method id
//! counts, so the records made before them stopped replaying:
//!
//! | change | effect on a replay |
//! |---|---|
//! | #1774 (`c8b1594f6`, 2026-10-14): the qualified unit `word XORs (…)` is priced by count at the pinned `ns_per_word_xor` | the solver term stays in the total instead of being taken out as an unpriced lower bound |
//! | #1646 (`e49afb98f`, 2026-10-11): the root's row echelon form goes through the Four Russians kernel of `gf2_elim`, and a child asks the node oracle before specialising its bases | the root elimination charges different word XORs (matrices from 128 x 256); the F6-IC gate refutes branches the reduction used to |
//! | #2043 (`d72e8b04d`, 2026-10-17): the shift-run eager rewrite for hosts without BMI2 | every aarch64 host charges every kept row's rewrite, as x86-64 already did; before, it rewrote lazily and charged only the rows read |
//! | #1640 (`31dd89b67`, 2026-10-11): `docs/ic/calibration.json` gains the row `koblitz/K_0 / GF(2^23)` | native units of that curve are priced by count; before, they were listed unpriced |
//! | linear harvest on by default (`d1d034d94`, `9e69f57ef`, 2026-10-15) | the root basis' linear rows are appended and the node re-eliminated, so the search and its word count change |
//! | F6-IC gate-first on by default (`d72e8b04d`) | refuted nodes skip their specialisations |
//!
//! Both generations are committed evidence under the same id and the same
//! parameters, so the id cannot tell them apart.  The *record* can: it is
//! the only thing that knows which accounting produced it.  So a replay
//! that does not reproduce under the current accounting is tried once more
//! under [`PRE_2043`], the accounting of the code through 2026-10-14, and is
//! accepted only if it then reproduces **every** compared field exactly
//! (answer, totals, phases, counters, native counters, factor base,
//! unpriced list), in which case the audit says so
//! ([`PRE_2043`]'s name appears in the replay detail).  The profile is a
//! fixed, named set of switches defined here; nothing in a record selects
//! a value, only whether the whole record reproduces.  A new session is
//! always recorded and replayed under the current accounting.
//!
//! The switches are the solver's own thread-local ones (`set_linear_harvest`,
//! `set_gate_first`, and the eager-rewrite and word-XOR-unit switches added
//! beside them): the solve runs on the calling thread, so a test running
//! another solve on another thread is unaffected.  They are restored when
//! the guard drops.

use crate::cryptanalysis::ic_boundary::set_legacy_calibration_before_1640;
use crate::cryptanalysis::ic_framework::set_legacy_exact_word_xor_unit;
use crate::cryptanalysis::inherited_f4::set_legacy_lazy_rewrite;
use crate::cryptanalysis::koblitz_groebner::{
    set_gate_first, set_legacy_pre_1646, set_linear_harvest,
};

/// A named accounting of the past.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Profile {
    pub name: &'static str,
    /// Only the bare `word XORs` unit is priced by count (before #1774).
    pub exact_word_xor_unit: bool,
    /// Lazy row rewrite, as every aarch64 host did before #2043.
    pub lazy_rewrite: bool,
    /// No in-engine linear harvest (before 2026-10-15).
    pub no_linear_harvest: bool,
    /// Specialise-then-ask order of the F6-IC gate (before #2043).
    pub no_gate_first: bool,
    /// The inherited engine as it counted before #1646 (2026-10-11): the
    /// root's echelon form on the column-at-a-time kernel, and the node
    /// oracle asked after the specialisation.
    pub pre_1646: bool,
    /// The pinned calibration table before #1640 added rows to it.
    pub calibration_before_1640: bool,
}

/// The accounting of the code through 2026-10-14, when the committed
/// root-finding, parity, improve and faster sessions were recorded on
/// Apple silicon, and the F6-IC ladder before it.
pub const PRE_2043: Profile = Profile {
    name: "pre-2043",
    exact_word_xor_unit: true,
    lazy_rewrite: true,
    no_linear_harvest: true,
    no_gate_first: true,
    pre_1646: false,
    calibration_before_1640: false,
};

/// The same accounting but for the one switch #1774 flipped, for records
/// made after it and before #2043.
pub const PRE_2043_PRICED: Profile = Profile {
    name: "pre-2043-priced",
    exact_word_xor_unit: false,
    ..PRE_2043
};

/// The accounting of the code before #1646 (2026-10-11), when the F6-IC
/// ladder and the first descent-algebraic sweeps were recorded.
pub const PRE_1646: Profile = Profile {
    name: "pre-1646",
    pre_1646: true,
    calibration_before_1640: true,
    ..PRE_2043
};

/// Profiles tried, in order, after the current accounting fails.
pub const PROFILES: [Profile; 3] = [PRE_2043, PRE_2043_PRICED, PRE_1646];

/// A profile in force; the previous state returns when it drops.
pub struct Guard {
    _private: (),
}

impl Profile {
    /// Put this profile in force until the guard drops.
    pub fn enter(&self) -> Guard {
        set_legacy_exact_word_xor_unit(self.exact_word_xor_unit);
        set_legacy_lazy_rewrite(self.lazy_rewrite);
        set_legacy_pre_1646(self.pre_1646);
        set_legacy_calibration_before_1640(self.calibration_before_1640);
        if self.no_linear_harvest {
            set_linear_harvest(Some(None));
        }
        if self.no_gate_first {
            set_gate_first(Some(false));
        }
        Guard { _private: () }
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        set_legacy_exact_word_xor_unit(false);
        set_legacy_lazy_rewrite(false);
        set_legacy_pre_1646(false);
        set_legacy_calibration_before_1640(false);
        set_linear_harvest(None);
        set_gate_first(None);
    }
}

/// Counters that describe the shape of the oracle's set-up and were added
/// to the `oracle_setup` phase after some sessions were recorded.  They are
/// descriptors, never priced (`full_price.rs`), so a record that lacks one
/// is compared without it; every other counter, and these where the record
/// has them, compare exactly.
pub const SETUP_DESCRIPTORS: [&str; 3] = ["pair_table_kind", "pair_table_points", "window_div"];

/// Whether `name` is a set-up descriptor a record may predate.
pub fn is_setup_descriptor(name: &str) -> bool {
    SETUP_DESCRIPTORS.contains(&name)
}
