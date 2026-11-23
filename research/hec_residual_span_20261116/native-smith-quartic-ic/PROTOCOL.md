# Native original-target quartic IC correctness gate [pcc:unverified]

Fixed original F61 mixed-term H, ell124459, both original fixtures source multiples1/17 and frozen sequential seed2026111800 xor61 xor(3<<32). Both canonical models and exact public mapper imported unchanged from native-smith-transport. No known scalar passed to ic.rs; main.rs alone recreates fixtures and verifies answers.

Per model enumerate all66rational curve points and3783projective lines, including point-at-infinity intersection multiplicities. Retain only fully rational degree4line divisors and verify every accepted line as a full Pic class relation to a common K-4P0. Add L_P0=0. Output all line coefficients/support/multiplicities. No relation subset tuning; all lines retained, shared across two cases explicitly.

Per fixture check nonzero G and ell-annihilation of G,T. Walk by seeded additions of G orT using seed2026112400 xor map xor(case<<32), cap4096steps. Accept typical decompositions only when the canonical conics have3distinct rational common zeroes; use the exact exceptional formula otherwise. At each accepted position append actual target-last relation and run frozen Fast sparse elimination. Require a returned k, verify [k]G=T natively, then main verifies [k]D=T on original H and equality to its independent driver label. Retain all failures/exhaustion/no unchanged rerun.

Record relation setup Pic counters, cumulative partial native counters, target walk/smooth/row/solve statistics and core Pic counters. These are NOT a complete cost ledger: native setup/transport, constructor internals and mixed-module operation coverage require later auditing. No timing/10x gate, no asymptotic performance claim. This implementation exhausts all lines (quadratic in field size), not an optimized quartic relation scheduler. Independent signed review and full-genus objective remain open.

One600CPU process, overflow checks enabled. Git custody while S3 down: push source/protocol/input/binary hashes before execution; retain stdout/stderr/exit/output hashes afterward. NoS3polling/cloud/merge. Pre-execution compile checks only; no scientific execution before freeze.
