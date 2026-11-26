# Validator report: research/quartic_diem_nofb_20261126

The harness stopped the validator subagent from writing Markdown in `validation/`. This file transcribes its final report, as relayed to the producing session. The authoritative structured record is `validator-report.json` (sha256 bound in `VALIDATION.json`).

**Verdict: validated with errata.** Validated commit: cc7e5c5488a. The gate checker returns `valid`.

**Claims.**
1. **IMPROVEMENT [pcc:unverified], with the order holding across the band.** Confirmed, citing **0.832**.
   - The held-out pooled GAE₁ ratio is 0.8324606. The exact cluster bootstrap gives [0.7591, 0.9076]; the producer's Monte Carlo interval was [0.7580, 0.9066].
   - At w = 0 the ratio is 0.99708, with exact interval [0.99595, 0.99840].
   - Dominance holds on every counter, on all 80 pairs.
2. **0 rescues of 16 fallbacks.** Confirmed three ways:
   - by the counter;
   - by equal attempts on all 80 pairs;
   - by the rank diagnosis, which shows t undetermined on all 32 failed attempts. A rescue was impossible.
3. **IC/ideal rho at 131101, w = 1.** nofb 1.98447 [1.835, 2.163] against sge 2.62990. All 8 cells are UNRESOLVED.

**Requirements.**
- **Hashes and freeze order:** pass.
- **Binary and source binding:** pass, from cargo build records; this is mtime evidence, weaker than a rebuild. The code review confirms exact pairing and no uncounted work.
- **Replay:** 268 of 268 rows byte-identical with wall fields masked.
- **Independent analysis:** every point estimate matches exactly, and the frozen `analyze.py` regenerates `analysis.json` byte-identically.
- **Certificates:** 160 of 160 rows on 20 curves checked with independent arithmetic [pcc:unverified], with negative controls.
- **C0:** 40 of 40 rows on all 83 shared fields.
- **Fallback-pair la ratios:** 0.20230–0.27283.
- **Document numbers:** 38 groups checked; errata E1–E5 as in `../ERRATA.md`.

**Process findings P1–P7:** see `../ERRATA.md`.

**Lessons:**
- **New lessons:** L-bf3ebea55103, L-6cac0c58a699, L-0b46f9d886ae.
- **Tests:** 14, all confirm.
- **Outcome:** O-4eff96f4ed25. These records are in `docs/lessons/entries/`.
