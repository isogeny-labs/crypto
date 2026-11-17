# Independent validation of four chained-transfer results (2026-11-13)

Validator role, against origin/main `c5678a98ae`, with code independent of the
producers: `scripts/vlib.py` provides affine EC arithmetic, Vélu, genus-2/3 Cantor
arithmetic and the bielliptic pull-back. The tree was exported with
`git archive`, and the release build used rustc 1.97.0. Raw outputs of every
check are in `out/t1..t4/`, and the resume checklist is `PROGRESS.md`. Every
figure is `[pcc:unverified]`; no pcc claim was filed. Two earlier attempts were
lost to container restarts; this third run completed.

## Verdicts

| target | admitted | reason |
|---|---|---|
| 1. v3 (`results_v3.json`, `certs_v3.jsonl`) | no | rho reference is plain (A = 1) though negation is available; rho native work not emitted |
| 2. `tower_large` | no | plain rho reference; rho native work not emitted; no frozen protocol of its own |
| 3. `cover_pushforward` | no | plain rho reference |
| 4. `small_de_endo_fb` | **yes, with caveats** | `validation_gate.py evidence` returns `valid`; `VALIDATION.json` written |

All four measurements reproduce [pcc:unverified]. Targets 1–3 fail on the
gate's rho-reference requirement, not on the data. Context:
`research/instrument_fixes_20261112` found the plain reference 5–9× too
expensive in set-up, so their recorded IC/rho distances are understated, not
overstated.

## What was checked independently

1. **v3.**
   - Hashes match.
   - 16 of 16 certificates pass (`[r]G = 0`, `[k]G = T`, `#E1` by point count,
     Vélu images, the pull-back), and all 68 single-field tampers are rejected.
   - A Python re-run of the IC arm matches group ops (3,419 / 8,475 / 11,353 /
     34,521), the LA counter and the logs at all four p. All 1,544 relations
     pass the relation check [pcc:unverified].
   - The native replay is byte-identical, and all 20 record metrics match.
   - Not independent: rho op counts, which were reproduced only by
     re-running producer code.
2. **tower_large.**
   - Hashes match.
   - All 234 table numbers and all 45 metrics recompute.
   - The p = 250 and 500 replay is identical outside timing (0 of 533
     values differ).
   - All 1,680 solver logs at p = 251–16001 are certified by vlib.
   - Not checked: p = 32003 and 64007.
3. **cover_pushforward.**
   - Hashes and the three-stage protocol hashes match.
   - The replays differ in 0 values outside timing (3,205 and 14,125 values).
   - All 882 logs are certified on E1 and J(C).
   - A Python re-run of the J(C) IC matches all 21 primary cells.
   - All 217 table numbers and 48 metrics match.
4. **small_de.**
   - Hashes match, and the pre-data draft is the byte prefix of the frozen
     protocol.
   - The full replay re-renders `small_de_derived.json`, `tables.md` and
     `small_de.svg` byte-identical to the committed files.
   - 1,920 IC logs and 2,240 rho logs are certified; CM discriminant, `j`,
     `λ` and `ψ(G) = [λ]G` hold for all 30 curves.
   - 720 per-arm-cell values recompute.
   - IC is above the best matched rho (plain, negation or automorphism) in
     120 of 120 arm-cells at 0, 0.1 and 1 GAE (minimum 2.325 / 7.616 /
     40.156). The order holds for every `w` in [0, 1].

## Discrepancies found (none changes a verdict)

- **Commit bindings.** Records bind dirty commits that predate the measuring
  code:
  - v3 and tower: `1d0afb7` (code first in `2796d7f`);
  - cover_pushforward: `e9d5dbd` (code first in `999d82e`);
  - small_de: `b7e8c68` (code first in `e0f476b`).
  This is the replay's F13 finding again.
- **`V3.md` LA wording.** It said the LA count "equals the column count times
  the row count". It is `(fb+2)×rows+1` (corrected in this PR).
- **v3 protocol.** `PROTOCOL.md` was committed together with the v1 results,
  so a pre-measurement freeze is shown only for v3's parent protocol.
- **cover_pushforward figure.** The committed `pushforward.svg` (lines 45–46)
  cannot be regenerated from committed source, because the writer prints
  " / " where the figure has " , ". The record's `s3_uri` for it still names
  the old hash.
- **small_de "to the operation".** `RESULTS.md` says per-trial solving cost is
  the same "to the operation" in every cell. In 12 of 60 cells (m = 3) it
  varies by up to 0.85% between repetitions, and A2/A1 is 1.0007 in one cell
  (corrected in this PR).
- **small_de raw data.** The store copy of `small_de.json` was not fetched,
  because S3 credentials were rejected.

## What would admit targets 1–3

Re-measure with the strongest available rho (the negation map) in the same
process, with its native work emitted, and bind the record to a clean commit
that contains the measuring code. `instrument_rerun` already provides the
negation reference.
