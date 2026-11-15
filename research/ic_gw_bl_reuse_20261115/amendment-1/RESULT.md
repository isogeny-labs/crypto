# Amendment 1: result (2026-11-15)

Guarded build BLR1 (`src/`, sha256 e236855d7fef…) ran the 30 BLR units of `../units.tsv` through the study's own
`run-local.sh` (REF units reused from `../runs-blreuse-20261115.tar.gz`; REF rebuilt bit-identically, 00355034924e…).
Raw records: `runs-blr1-20261115.tar.gz`. A first rerun with an ad-hoc runner lacked the `unit` exit-code records and
was discarded.

**Not identical, so the study's numbers are re-derived from BLR1 (pre-registered rule).** In the 23 units where the
single narrowing happens in the final iteration, the guard also uses the direct V^T V_0 gram in that iteration, where
the recurrence was still exact: DL `linear_algebra.lmul` rises by 3,352–13,800 (at most 0.04%). Every other counted
field, `bl_iters`, `bl_narrowed` and every logarithm are identical; the 7 units without narrowing are identical.

`python3 analyze.py <runs>` (this directory's copy of the frozen `../analyze.py`, repository path adjusted) →
`analysis.json`, `tables.md`: G1–G4 pass, decision **SUPPORTED**, below 1 at every hash price. DL cold M-eq BLR1/REF,
worst seed at additions 0 / 0.5 / 1 M-eq: k = 18 0.8223 / 0.8656 / 0.8859; k = 19 0.8136 / 0.8651 / 0.8863;
k = 20 0.8163 / 0.8663 / 0.8870; k = 21 0.8194 / 0.8700 / 0.8905; k = 22 0.8129 / 0.8687 / 0.8905 (largest change from
the frozen BLR: 0.0002). k = 22 mean over seeds 1–3: 8.4401e8 / 1.01401e9 / 2.54399e9 M-eq at w = 0 / 0.1 / 1.
These are the numbers the leaderboard uses. `[pcc:unverified]`
