---
title: "Amendment 1: analyze refuses an incomplete ladder; raw rows to the artifact store"
date: 2026-10-29
---

# Amendment 1

Made after the main run, in response to two review findings on PR #2897. It adds a refusal check to the analysis step and does not touch any measured number. Every frozen file (`PROTOCOL.md`, `REGISTRATION.sha256`, `run.sh`, `BINARY.sha256`, `design.json`, `pilot/`, `analysis/`, `audits/selftest.txt`) is unchanged.

## 1. What changed

- **`examples/coincidence_ladder.rs`.** `analyze` now returns an error, with nonzero exit and no output on stdout, unless:
  - all five registered rungs (16, 20, 24, 28, 32) are present;
  - both θ values have at least 40 curves (the pilot size) in every rung;
  - every one of the 8 series exists at both θ, with no infeasible plant;
  - the gates see exactly the registered slopes (PROTOCOL.md §7): six real, four null-type, two Z/N and four plant.

  Before, a missing θ or rung was skipped silently and the remaining slopes could still print a reading. The census, design, Stage 1 code and every formula are unchanged.
- **New check `check-refusal.sh`.** It confirms that `analyze` refuses (a) the committed pilot TSVs filtered to θ index 0, (b) the pilot TSVs without the 32-bit rung and (c) the 32-bit main rung alone, and that on the complete ladder it reproduces `analysis/analysis.json` byte for byte. Output: `audits/refusal.txt`.
- **Selftest** has an 11th check (`validate` refuses a missing θ, rung or curve count); output: `audits/selftest-amendment-1.txt`. The original 10-check output stays in `audits/selftest.txt`.
- **Raw rows.** `main/main-16.tsv` … `main-32.tsv` are now committed, byte for byte the files `run.sh` produced (`shasum -a 256 -c main/SHA256SUMS` passes on them). They are data files over 64 KiB, so the repository's artifact bot moves them to the store and leaves the `.artifacts.json` manifest.

## 2. Effect on the evidence

None. The amended binary reproduces `analysis/analysis.json` from `main/*.tsv` byte for byte, and the reading is still `SUB_BREAK_EVEN_AT_SCALE`. The sentence of the README's §3 that says the raw per-curve outputs are not committed is superseded by this amendment.

## 3. Hashes

- `REGISTRATION-AMENDMENT-1.sha256`: the amended source and `check-refusal.sh`. The original source hash (`16d048d15655abb901b5a9db017ec492d55d6fa15aa97dd64a8d2dd3cd270f1a`) is in `REGISTRATION.sha256`; the amended one is `164cf9c75f0b1617fa94be18d05d09a56d8924a92abf566548e612a67862c9fa`.
- `BINARY-amendment-1.sha256`: the binary built from the amended source on the local host.
