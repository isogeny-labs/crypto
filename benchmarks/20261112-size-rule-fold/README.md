# Size rule `fes-then-inherited` and the `inherited-f4` fold, 2026-11-12

Stage diagnostic (the algebraic index-calculus oracle end to end), not a
leaderboard row: no table row in `benchmarks/README.md`, no VALIDATION.json.
Study, change, protocol and pre-stated expectation:
[`research/size_rule_fold_20261112/`](../../research/size_rule_fold_20261112/).
Record `benchmarks/records/20261112T1612Z-size-rule-fold-e2e.json` (measured).

`ic bench --sweep` on the six instances of the e2e ladder, 3 repeats, one
thread, local container `[pcc:unverified]`:

- `fes-then-inherited` (fes-f2 up to 14 unknowns, inherited-f4 above) is the
  best algebraic row on every instance: at degree-13 Koblitz S 101.9
  (83.7–116.3) against `inherited-f4`'s 138.0 (117.8–167.6), 0.74×, and
  exactly `inherited-f4`'s S on the other five.
- The folded `inherited-f4` and `inherited-f4-legacy` have identical S on all
  six instances.
- All 24 configurations and all matched rho runs recovered their logarithms.

Files: `REPORT.pdf`, `results.svg`, `results.png`.
