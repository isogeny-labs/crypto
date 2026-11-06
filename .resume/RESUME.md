# Resume: crypto @ claude/lift-dense-char2-base-8y3bht

## Goal
Check independently whether 2-torsion translation (x -> sqrt(b)/x) pairs the
columns and relations of the Oct 27 lift-dense char-2 lines u*F_q when
u in b^(1/4) F_q, and correct the Oct 27 lead's u-count / yield if so.

## Lead / hypothesis
Lead: research/lift_dense_factor_base_20261027 (2^n relations per trial on
all-lift lines). Claim from another session (unverified): on L_T = b^(1/4) F_q
columns pair, rows repeat 2^(n-1)-fold, gain ~2x not 2^n. Confirm: P1-P4 of
the protocol pass. Kill: P3 or P4 fails.

## Spec
research/lift_dense_torsion_pairing_20261106/PROTOCOL.md, frozen at c7878791e4.
Instrument: research/lift_dense_torsion_pairing_20261106/pairing/ (std-only
Rust; bins pairing, c2pnb_pairing). Seeds are fixed strings in the code.

## State
Protocol frozen; instrument built; no data yet.

## Next steps
1. cd research/lift_dense_torsion_pairing_20261106/pairing && cargo build --release
2. ./target/release/pairing run ../reports/run-<UTC>/ ../../lift_dense_factor_base_20261027/reports/run-20261027T0356Z/lines_amend3/rows.jsonl
3. repeat into ../reports/run-<UTC>-aa/ and compare sha256 (A/A)
4. ./target/release/pairing summarize ../reports/run-<UTC>/
5. ./target/release/c2pnb_pairing ../../lift_dense_factor_base_20261027/reports/run-20261027T0356Z/c2pnb_input_binary-curve-params.txt ../reports/run-<UTC>/c2pnb.jsonl
6. README, REPORT.pdf, Oct 27 README correction, PR.

## Runs in flight
none

## Stop and success conditions
Exact counts on toy fields; runs take minutes locally. Success = verdicts
P1-P6 recorded either way.

## Open questions
Orbit-parity criterion and the conjugate-pair family (odd char) are later
pieces, not this one.
