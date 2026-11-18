# H112 recovery handoff — 2026-11-17

## Completed evidence

- Full deterministic replay: 28 rows, 9,096 targets, 144 frozen predicates.
- Stable scientific digest reproduced exactly:
  `3f0d58b381df5d308537942c29f80e826217105bdd7c8db4129885c4e91ea832`.
- Independent verifier accepted every target and found 0 exact rows.
- Eight focused Python tests pass across the canonical and standalone suites.
- `H112P3SecondDigitCarry.lean` compiles with Lean 4.34.1 and pinned mathlib.
- Result SVG/PNG and the two-page PDF were inspected page by page.
- Exact recovered raw JSON is `p3-second-digit-carry.json.xz`; decompression
  yields SHA-256
  `304d84d1602b8278436775719a2b235506b613df907c4540a418c9b80698e759`.

## Publication

- Canonical bundle: crypto PR #4603.
- Independent source-only bundle: cryptanalysis PR #1731.
- Earlier certificate-only PRs: crypto #4076 and cryptanalysis #1525, merged.
- `RECOVERY_PROVENANCE.json` records original-lost and recovered hash sets.

## Verification commands

```bash
python -m unittest -v test_h112_p3_second_digit_carry.py
python -m unittest -v test_h112_reference_only.py
python h112_reference_verify.py p3-second-digit-carry.json
lake env lean H112P3SecondDigitCarry.lean
xz -dc p3-second-digit-carry.json.xz | sha256sum
```

## Open administrative gates

- Crypto artifact migration cannot probe new S3 keys because its workflow gets
  `HeadObject` 403 without `ListBucket`; use the reviewed `large-file-ok`
  exception or repair the workflow permission. The two affected files are only
  256 KiB xz and 70 KiB PNG.
- Crypto `names` and cryptanalysis `gate`/`source-receipts` are red on
  pre-existing storage or self-test fixtures outside the H112 diff. Job logs
  name no H112 file.
- Both repositories allow merge commits, not squash. Merge with an expected
  current head after required gates are evaluated.

## Next research step

Do not repeat H111/H112 by merely adding another `p`-adic digit. Freeze a
mechanism that cancels section noise before scoring—e.g. a binary/trinary
carry-curl or another proof-relevant invariant—with matched controls and a
paired PR before measurement.
