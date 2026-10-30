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
//! The switches are the solver's own thread-local ones (`set_gate_first`,
//! and the eager-rewrite, word-XOR-unit, harvest and calibration switches added
//! beside them): the solve runs on the calling thread, so a test running
//! another solve on another thread is unaffected.  They are restored when
//! the guard drops.

use crate::cryptanalysis::ic_boundary::set_legacy_calibration_before_1640;
use crate::cryptanalysis::ic_framework::set_legacy_exact_word_xor_unit;
use crate::cryptanalysis::inherited_f4::set_legacy_lazy_rewrite;
use crate::cryptanalysis::koblitz_groebner::{
    set_gate_first, set_legacy_no_linear_harvest, set_legacy_pre_1646,
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
        set_legacy_no_linear_harvest(self.no_linear_harvest);
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
        set_legacy_no_linear_harvest(false);
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

/// What a `rho.gls` / `rho.frobenius_orbit` replay reports of its uncharged
/// endomorphism counters, as the binaries did before amendment A1 of
/// `research/ext_endo_rho_20261023` (2026-10-23) changed
/// `EndomorphismClasses` in place: no `endomorphism_field_muls_uncharged`,
/// and on GLS three applications of `ψ` per canonicalisation where the class
/// needs one (`ψ² = −1`; the subfield family's two per step is unchanged).
/// Only uncharged counters differ: the totals and every group-operation
/// count are the same.
///
/// The view is taken from the replay alone (the new counter is dropped from
/// the counters and the unpriced list, GLS applications are multiplied by
/// three) and is offered only for a record that lacks the new counter, so a
/// record cannot choose any figure; every other field must still match.
pub fn pre_endomorphism_a1(
    rep: &crate::cryptanalysis::ecbench::methods::SolveReport,
    r: &crate::cryptanalysis::ecbench::record::Record,
) -> Option<crate::cryptanalysis::ecbench::methods::SolveReport> {
    const FIELD_MULS: &str = "endomorphism_field_muls_uncharged";
    const APPLICATIONS: &str = "endomorphism_applications_uncharged";
    if !matches!(r.method.id.as_str(), "rho.gls" | "rho.frobenius_orbit")
        || r.counters.contains_key(FIELD_MULS)
        || !rep.counters.contains_key(FIELD_MULS)
    {
        return None;
    }
    let mut view = rep.clone();
    view.counters.remove(FIELD_MULS);
    view.unpriced.retain(|name| name != FIELD_MULS);
    if r.method.id == "rho.gls" {
        *view.counters.get_mut(APPLICATIONS)? *= 3;
    }
    Some(view)
}

/// How the replay detail names [`pre_endomorphism_a1`].
pub const PRE_ENDOMORPHISM_A1: &str = "pre-A1 endomorphism counters";

// ── Who may be replayed under a legacy accounting ──────────────────────
//
// A legacy accounting excuses a difference, so it must not be available to
// a record that merely claims it.  As for the class set-up
// (`audit::PRE_CLASS_SETUP_BINARIES`), each profile is offered only to a
// record written by a binary on that profile's closed list, found by
// scanning the committed sessions that needed it.  Each entry is
// `(binary_sha256, commit, sessions)`: the full SHA-256 of the ecbench
// binary the record names, the commit the session names (`session.json`
// `git_commit`; empty where it names none, and informational where one
// binary served several sessions: matching is by binary hash alone), and
// where the records are (paths under `research/`).  Anything else, and
// every record the current harness writes, must replay exactly.

/// `(binary_sha256, commit, sessions)`.
pub type Producer = (&'static str, &'static str, &'static str);

/// Binaries whose records replay under [`PRE_2043`].
pub const PRE_2043_BINARIES: [Producer; 7] = [
    (
        "1e972df4d8e769629ad4fd4a023214b05ee513a2cfea6f3ef51f2ca9e61fe531",
        "2245b7596f3fe04586c33b53a7cac97185fadba2",
        "ecbench_rootfind_engine_20261012/{sessions/claim-n13,sessions/claim-n19,sessions/smoke-koblitz,abandoned/claim-n13-interrupted}",
    ),
    (
        "394e0c1bfe705f2bb0795425099beb02c50f04abe5027fe8b4020b91d40ec973",
        "7cc12f568eb0a9c93b1430d348af9fd4383041dd",
        "ecbench_rootfind_improve_20261013/sessions/{claim-n13,select,window-d4}",
    ),
    (
        "1622e3fa8e638031c8406b651a8e2e596573189bd2c78aa8b83d5e979fa372dc",
        "5c14590755da5a9cad11caee33b40eb2b42b7970",
        "ecbench_rootfind_parity_20261013/sessions/{claim-n13,claim-n13-v2,cycle2,cycle4,cycle4b,cycle5}",
    ),
    (
        "de2e66bcf3ed770d5ea6fdf87a4c24bb16fd3c4f554cee5a65dc41699dcb320c",
        "2fad98b0bdaedc5735156ba7e3188c729e5f8c03",
        "ecbench_rootfind_parity_20261013/sessions/cycle1",
    ),
    (
        "b978f0e4a427e4a822234e263a8ec93bbc2c83bfe16f208505140f499d5cccdf",
        "cd0228d4c8e7a6b8d0037e7504b3e3e18b13059f",
        "ecbench_yield_sweep_20261004/sessions/koblitz (the da-f4 arms)",
    ),
    (
        "712a58bb4ae2ba9da32ed4170d6b4c775453a3b0cdf46fd28760559a249cebad",
        "",
        "f6_ic_ecbench_ladder_20261005/sessions/n7 (exhausted runs)",
    ),
    (
        "9bb9a1a64ea01bf5f447e0c87980207d26fb4f9fa8961270edc66ce37bfb8b68",
        "",
        "ecbench_factor_bases_20261013/sessions/koblitz17-algebraic (exhausted runs)",
    ),
];

/// Binaries whose records replay under [`PRE_2043_PRICED`].
pub const PRE_2043_PRICED_BINARIES: [Producer; 1] = [(
    "27fd2653b706b6e82e8da1c39a8c26c8f686e3f6e4831953b85eb9798610b76c",
    "f67fc6dbd06d5a295696cf36c339181d4b92efae",
    "ecbench_rootfind_faster_20261014/sessions/{claim-n13-v3,cycle1,cycle3}",
)];

/// Binaries whose records replay under [`PRE_1646`].
pub const PRE_1646_BINARIES: [Producer; 3] = [
    (
        "712a58bb4ae2ba9da32ed4170d6b4c775453a3b0cdf46fd28760559a249cebad",
        "",
        "f6_ic_ecbench_ladder_20261005/sessions/{n13,n19}",
    ),
    (
        "b2596e9db8b79a11479ae22b4d12570114501b841c22716929f61012fd9d3845",
        "678d7e5fa67777b6791ebe7fa6fe9a275d0bcc2f",
        "ecbench_all_candidates_20261003/sessions/koblitz",
    ),
    (
        "b978f0e4a427e4a822234e263a8ec93bbc2c83bfe16f208505140f499d5cccdf",
        "cd0228d4c8e7a6b8d0037e7504b3e3e18b13059f",
        "ecbench_yield_sweep_20261004/sessions/koblitz",
    ),
];

/// Binaries whose records predate the `oracle_setup` descriptors
/// ([`SETUP_DESCRIPTORS`]) and the `window_div` parameter.
pub const SETUP_DESCRIPTOR_BINARIES: [Producer; 3] = [
    (
        "1e972df4d8e769629ad4fd4a023214b05ee513a2cfea6f3ef51f2ca9e61fe531",
        "2245b7596f3fe04586c33b53a7cac97185fadba2",
        "ecbench_rootfind_engine_20261012/{sessions/claim-n13,sessions/claim-n19,sessions/smoke-koblitz,abandoned/claim-n13-interrupted}",
    ),
    (
        "394e0c1bfe705f2bb0795425099beb02c50f04abe5027fe8b4020b91d40ec973",
        "7cc12f568eb0a9c93b1430d348af9fd4383041dd",
        "ecbench_rootfind_improve_20261013/sessions/{claim-n13,select,window-d4}",
    ),
    (
        "de2e66bcf3ed770d5ea6fdf87a4c24bb16fd3c4f554cee5a65dc41699dcb320c",
        "2fad98b0bdaedc5735156ba7e3188c729e5f8c03",
        "ecbench_rootfind_parity_20261013/sessions/cycle1 (rootfind-window-negfold without window_div)",
    ),
];

/// Binaries whose `rho.gls` / `rho.frobenius_orbit` records predate
/// amendment A1 ([`pre_endomorphism_a1`]).
pub const PRE_ENDOMORPHISM_A1_BINARIES: [Producer; 1] = [(
    "87bac3448d63e3c1dcd841a2d23b565c09477c855eda0febf11a03d9931d148f",
    "6bf8d5c2f51dbdd1b134b11a02396e1e4ac76b68",
    "ext_endo_rho_20261023/sessions/{gls-fp2,subfield-fp3}",
)];

fn listed(list: &[Producer], r: &crate::cryptanalysis::ecbench::record::Record) -> bool {
    list.iter()
        .any(|&(binary, _, _)| r.binary_sha256.as_deref() == Some(binary))
}

/// Whether `r` may be replayed under `profile`.  Every condition must
/// hold, so anything unforeseen fails closed to an exact comparison: the
/// record names a binary on the profile's list; it is an index-calculus
/// pipeline record (the profiles change nothing else); and it carries no
/// `bitop/v1` work counters (every binary since 2026-10-16 writes them).
pub fn admits(profile: &Profile, r: &crate::cryptanalysis::ecbench::record::Record) -> bool {
    let list: &[Producer] = match profile.name {
        "pre-2043" => &PRE_2043_BINARIES,
        "pre-2043-priced" => &PRE_2043_PRICED_BINARIES,
        "pre-1646" => &PRE_1646_BINARIES,
        _ => return false,
    };
    r.method.id.starts_with("ic.")
        && r.phases.iter().all(|p| p.work_native.is_empty())
        && listed(list, r)
}

/// Whether `r` may be compared without the set-up descriptors it predates:
/// it names a binary on [`SETUP_DESCRIPTOR_BINARIES`].
pub fn admits_setup_descriptors(r: &crate::cryptanalysis::ecbench::record::Record) -> bool {
    listed(&SETUP_DESCRIPTOR_BINARIES, r)
}

/// Whether `r` may be replayed under [`pre_endomorphism_a1`]: it names a
/// binary on [`PRE_ENDOMORPHISM_A1_BINARIES`] (the record must also lack
/// the counter amendment A1 added, which the function checks).
pub fn admits_pre_endomorphism_a1(r: &crate::cryptanalysis::ecbench::record::Record) -> bool {
    listed(&PRE_ENDOMORPHISM_A1_BINARIES, r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn producer_lists_hold_full_digests_and_commits() {
        let lists: [&[Producer]; 5] = [
            &PRE_2043_BINARIES,
            &PRE_2043_PRICED_BINARIES,
            &PRE_1646_BINARIES,
            &SETUP_DESCRIPTOR_BINARIES,
            &PRE_ENDOMORPHISM_A1_BINARIES,
        ];
        for list in lists {
            for &(binary, commit, sessions) in list {
                assert!(binary.len() == 64 && binary.bytes().all(|b| b.is_ascii_hexdigit()));
                assert!(
                    commit.is_empty()
                        || (commit.len() == 40 && commit.bytes().all(|b| b.is_ascii_hexdigit())),
                    "{commit}"
                );
                assert!(!sessions.is_empty());
            }
        }
    }

    #[test]
    fn every_profile_has_a_producer_list() {
        for profile in PROFILES {
            assert!(
                matches!(profile.name, "pre-2043" | "pre-2043-priced" | "pre-1646"),
                "{} has no list in `admits`",
                profile.name
            );
        }
    }
}
