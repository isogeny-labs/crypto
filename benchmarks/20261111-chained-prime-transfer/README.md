# 2026-11-11 — chained prime-field transfer (E1 → E1′ → J(C)), measured boundary

Report: [REPORT.pdf](REPORT.pdf). Figure: [cost-vs-p.svg](cost-vs-p.svg).
Study: [`research/chained_prime_transfer_20261111/`](../../research/chained_prime_transfer_20261111/).
Record: `benchmarks/records/20261111T0540Z-chained-prime-transfer.json`.

Verified chain (Vélu isogeny, then bielliptic pull-back φ*) at p = 211 to 1619.
Rho on each destination stays near the source rho (n = 1 per cell). Genus-2
index calculus on J(C) costs 9.8× to 42.9× the matched rho, with slope about
1.0 in p. This is a boundary, not an improvement. Unvalidated (no validation
gate run). All values are [pcc:unverified].

## v2 rerun after independent replay (2026-11-12)

Record `benchmarks/records/20261112T0316Z-chained-prime-transfer-v2.json`;
report `REPORT_v2.pdf`; graph `band-v2.svg`. This run uses the corrected IC
counter, checks rho against the planted log, and emits the LA mulmods.
IC/rho is 8.6–39.2× at 0 GAE, 12–110× at 0.1 GAE and 43–752× at 1 GAE, so
there is no advantage at any price [pcc:unverified]. This is not
validation-gate admitted: there is no replay certificate under
`VALIDATION.json`.
