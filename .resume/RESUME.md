# Resume: crypto @ claude/novel-ecdlp-algorithm-iams4j

## Goal
User request (2026-11-07): run every next step of research/split_prime_ecdlp_20261107 (#3654, #3655
merged): proposals P1 (Riemann-Roch system with a Dickson-torus tower), P2 (twist-pure coset
selection), P3 (cross-trace covariate test of the split-prime statistic) under frozen protocols on toy
prime fields, plus a Lean statement of the obstruction.

## Lead / hypothesis
P1: round7_rr's D_RR = 4 < D_T = 6 (m3 t4, Kummer) transfers to the Dickson torus tower (proposer 0.55
stage-level). P2: F4 cost is coset-independent and coverage scales as c_E^m, so selecting the most
E-pure coset pays at toy scale (0.85 toy, 0.05 cold end-to-end). P3: no Frobenius-field covariate
moves rho cost or exact m = 2 coverage by >= 1 % (null expected, 0.98).

## Spec
research/split_prime_ecdlp_20261107/followups/{README.md, p1_rr_dickson/PROTOCOL.md,
p2_twist_coset/PROTOCOL.md, p3_cross_trace/PROTOCOL.md}, frozen in commit 535d9da826 (PR #3657).
Instrument commands pinned in each protocol's Addendum A before any claim cell.

## State
Protocols committed and pushed (PR #3657, auto-merge on). Builders writing standalone Rust crates under
followups/p1_rr_dickson/p1tool, followups/p2_twist_coset/p2coset, followups/p3_cross_trace/p3census
(uncommitted until they report). msolve 0.6.5 from apt at /usr/bin/msolve. aws CLI from
/tmp/report-venv (pip) linked at /usr/local/bin/aws; caller is account 104186811093.
Lean: aburan28/crypto-lean PR #3 (CryptoLean.SplitPrime.Obstruction), local module build passed,
standard axioms only. Toolchain: ~/.elan with lean-4.35.0 linked from the release archive (elan cannot
resolve "latest" through the proxy).

## Next steps
1. Read builders' reports; pin Addendum A per protocol; commit; new PR (or same branch if #3657 open).
2. scripts/checkpoint.sh push; tools/benchrec.py init per run.
3. experiment-director compile mode -> card per experiment; relay verbatim to executor-mechanical;
   director assess mode.
4. Analyse per the frozen decision rules; report + figures + Typst PDF per experiment; pcc claims;
   session log; PR; track to merge.

## Runs in flight
None. No claim cell has run.

## Stop and success conditions
Per protocol decision rules. Budget: P1 1,200 s per msolve job, one at a time; P2 same limits;
P3 about 4 CPU-hours total. Stop a protocol on its control-failure rule.

## Open questions
Whether the director accepts the local L0 route (stated deviation from AGENTS.md section 9a).
