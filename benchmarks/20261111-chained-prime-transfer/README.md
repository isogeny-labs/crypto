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

## Lead runs (2026-11-12)

The report is `REPORT_leads_20261112.pdf`, and the graphs are `amortized.svg`
and `tower_large.svg`. The records are:

- `20261112T0341Z-chained-prime-transfer-v3`: the real instance, with
  certificates.
- `20261112T0402Z-chained-amortized-ic`: amortized IC against plain and
  precomputed rho.
- `20261112T0421Z-chained-tower-large-p`: genus 2 and 3 up to p = 64,007.

Each run was reproduced from the integrated source
(`research/chained_prime_transfer_20261111/repro/REPRO.md`). There is no
advantage over the matched rho in any of them [pcc:unverified]. They are not
admitted by the validation gate.

## Cover mechanism and endomorphism census (2026-11-12)

The report is `REPORT_cover_census_20261112.pdf`, and the graphs are
`pushforward.svg` and `census.svg`. The records are
`20261112T1550Z-cover-pushforward` and `20261112T1558Z-endo-degree-census`.

Genus-2 IC on the transferred subgroup reduces to a birthday search on E.
Low-degree endomorphisms occur only on curves built to have them. Neither
result shows an advantage over rho [pcc:unverified]. Neither is validation-gate
admitted.

## Small D_E with an endomorphism-adapted factor base (2026-11-12)

The report is `REPORT_small_de_20261112.pdf` and the graph is `small_de.svg`;
the record is `20261112T1753Z-small-de-endo-fb`. Solving work per trial is
unchanged. The saving is column folding, and a generic scalar-folded control
matches it. No arm comes in below the matched rho [pcc:unverified]. This run is
not admitted by the validation gate.
