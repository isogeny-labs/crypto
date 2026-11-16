# Ratcheting Elliptic-Curve Index Calculus Toward Pollard Rho

`ic_ledger.tex` (built: [`ic_ledger.pdf`](ic_ledger.pdf), 18 pages) is the
ledger of the verified, measured improvements made to the repository's
elliptic-curve index calculus between July and 23 October 2026. It states
each improvement against its own incumbent, then the distance to a matched
rho that remains, the claims that were corrected, and where the validation
gate stands. It follows the owner's success criterion (AGENTS.md, "Success
criterion: iterative improvement of the index-calculus baseline").

**Status**: research draft, anonymized, not peer-reviewed, not submitted,
not cleared. A synthesis of committed evidence: **no new measurement** was
run for it. Every number cites a committed artefact (Appendix A of the PDF).
Plotted values and their sources are in [`figures/data.json`](figures/data.json).

**Relation to [`../ecdlp-index-calculus-program/`](../ecdlp-index-calculus-program/)**:
that record surveys the literature, the measurement infrastructure and the
chronology through 20 October. This paper is organised by pipeline stage and
by the ratchet, and it adds the work of 20–23 October: rule v8 at a matched
rho, the K1107 base confirmation, the cache-blocked pair table, the planner
validation, the ECCp-131 probes, the BMI2 root cause of the replay
regression, and the gate status.

**Change class**: none of its own. Every improvement it reports is
**engineering** or **accounting**, or an exponent change against the IC
incumbent (genus 3, k = 4) that has not crossed rho.

## What it says

- **Cost model** (§2): which term of the IC cost each lever acts on; set-up
  and relation collection now dominate (set-up is 98.5–99.7% of cold S on
  the Koblitz descent).
- **The ratchet** (§§4–7, Figure 1, Table 2): oracle (root finding
  383–1,535× vs matrix F4; inherited F4; f4_fp at msolve parity; gate-first;
  harvest gate), factor base and yield (free relations 0.960; stride-renew
  0.531; K1107 −1.195 s at n = 83), set-up and LA (stacked n83 producer
  3.24×; cache-blocked set-up 0.24; CADO-style LA 10.4×), and the two
  families with sub-rho exponents (genus 3: q^1.910 → local q^1.356;
  k = 4 D₄: 370–481× per decomposition, gap 5,080 → 6.06 vs the rho model).
- **Distance** (§8): Koblitz rule v8 at a matched rho 4.67–18.07×; prime
  10.5–101.8×; planner 12.6–25.8×; n83 cold 55.4×; nearest genus 3
  (1.15×) and k = 4 char 2 (4.27× measured, one target).
- **Audit** (§9): every online win retired (Bernstein–Lange rho 53–317×
  faster with equal advice); pricing and reference matching raised gaps;
  replay regression traced to the BMI2 path in `inherited_f4.rs`.
- **Gate** (§10): 0 of 28 IC leaderboard rows validated; what each row needs.
- **Next** (§11): validate landed rows; test the k = 4 crossover at
  m = 15–17; measure genus-3 rho in its unit; Koblitz yield per probe.

## What it does not say

No ECDLP speed-up on any deployed curve, no key recovery, no asymptotic
sub-rho algorithm for prime fields. Stage ratios are not end-to-end
speed-ups. L0/L1 wall ratios are descriptive. No row here is validated in the
sense of `docs/VALIDATION_GATE.md`.

## Build

```sh
python3 figures/make_figures.py   # PDF, SVG and 150 dpi PNG from data.json
make                              # pdflatex + bibtex
make check                        # fail on errors, unresolved refs, overfull boxes
```

## Before submission

- [ ] Re-read every cited artefact at the commit of submission.
- [ ] Verify every bibliography entry against the publisher's record.
- [ ] Re-state the gate status (§10) if any row has since validated.
- [ ] `make deanonymize`, `make check`, then the `iacr-eprint` workflow.
