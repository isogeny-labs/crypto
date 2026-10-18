# Additive verifier amendment and corrected Fp2 replication

Original sources, protocols and scientific records remain frozen. This directory contains a review-authored repair and replication; it is not an independent external review.

`verify_v2.cpp` binds every certificate to its exact source result and enforces complete coverage. It checks strict distinct-abscissa constraints, prime-field bounds, Fp2 projected-base membership, correct discriminants, exact curve point counts and subgroup assertions, then replays sums with Jacobian formulas. Finite-field arithmetic remains shared with the original generator. The JSON parser is pinned nlohmann/json v3.12.0 under its MIT license; duplicate keys are rejected.

The original dataset passes: **79 certificates over 999 actual target rows**; two explicit unavailable controls remain excluded. All nine adversarial controls are rejected. See `runs/run-20261018-verifier-v2/attempts.tsv` and its binding receipts. The old verifier's acceptance gaps are demonstrated separately under `../../review/`.

`diagnostic_v2.cpp` corrects the prime-subfield embedding of discriminant coefficient 27 and runs only the same 24-fixture Fp2 panel. An explicit singular F17² fixture (A=14,B=2) is rejected. The fresh panel has the **same 24 accepted identities and identical full result records**, with **23/24 table hits and 24/24 rho successes**. Its full candidate trace records 49 draws: 24 accepted, 25 subgroup-condition rejections, zero singular rejections. This does not retroactively replace the original cohort. See `runs/run-20261018-fp2-replication-v2/cohort-comparison.json`.

Frozen protocols precede their respective runs. Executed commands, pre-run hashes, outputs, exit statuses and failures are retained under `runs/`. The review report and figures are at `../../review/REPORT.md` and `../../review/REPORT.pdf`.

Build from the repository root:

```sh
g++ -std=c++20 -O2 -Wall -Wextra -Wpedantic research/ideas_execution_20261018/targets_prime/verification_v2/verify_v2.cpp -lcrypto -o research/ideas_execution_20261018/targets_prime/verification_v2/verify_v2
g++ -std=c++20 -O2 -Wall -Wextra -Wpedantic research/ideas_execution_20261018/targets_prime/verification_v2/controls.cpp -o research/ideas_execution_20261018/targets_prime/verification_v2/controls
g++ -std=c++20 -O2 -Wall -Wextra -Wpedantic research/ideas_execution_20261018/targets_prime/verification_v2/diagnostic_v2.cpp -o research/ideas_execution_20261018/targets_prime/verification_v2/diagnostic_v2
g++ -std=c++20 -O2 -Wall -Wextra -Wpedantic research/ideas_execution_20261018/targets_prime/verification_v2/compare_replication.cpp -o research/ideas_execution_20261018/targets_prime/verification_v2/compare_replication
```

The execution scripts refuse existing output directories. Copy a script to a fresh run name before repeating; do not overwrite committed records. `run_verification.sh` executes the 79-certificate amendment and adversarial suite; `run_replication.sh` executes the separately frozen corrected 24-fixture panel. Both preserve the original source artifacts.

This amendment establishes neither a Gaudry index-calculus oracle nor Q-curve/GLS folding, double-large-prime gains, full ECDLP extraction, speedup or FourQ extrapolation. Items 1 and 14 remain partial.
