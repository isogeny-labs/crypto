# Five follow-up ideas on PKM smooth-map factor bases

**DRAFT (not yet frozen; awaiting smoke tests).** 2026-11-15. Study id `EXP-PKM-IDEAS-001`. Instrument
[`examples/pkm_ideas.rs`](../../examples/pkm_ideas.rs); runner [`run.sh`](run.sh); rows `runs/`. Counted metrics and
exact counts only; no clock is read. Context: EXP-PKM-ODD-001 (`research/pkm_odd_tower_f4_20261114/`).

The user asked whether the auxiliary smooth isogeny kernel should be faster than a random factor base, and for other
ideas. Five ideas, each with its cheapest decisive test:

| # | Idea | Test | Pre-registered prediction | Falsified by |
|:--|:--|:--|:--|:--|
| 1 | A smooth tower is far cheaper to *solve* than a random `V` of the same size | F4 on refutation systems with membership written as one dense polynomial `∏(x − v)` of degree `S`: random `V`, and the Kummer coset `g·μ_S` as a control. Compared with the committed tower counts of EXP-PKM-ODD-001 at the same `p`, `m` and `S` | random dense ÷ tower `≥ 2` at every completed `S` (`loss`), the ratio growing with `S` | any completed `S` with ratio `< 2` |
| 2 | On `j = 0` curves, a Kummer tower with `3 \| S` is closed under the automorphism `ω: (x, y) ↦ (ζ₃x, y)` | Exact check on prime-order `y² = x³ + b` at small `p`: `ω(F) = F`; `ω(P) = [λ]P` on every factor-base point; orbits under `⟨ω, −1⟩`; decomposition count invariant under `R ↦ ω(R)`; random-set control | all checks hold, columns `= |F|/6` (one third of the negation-only `|F|/2`), random sets not closed | any check failing |
| 3 | Replaying a recorded F4 trace on new targets costs less than running F4 afresh | Record on one refutation target, replay on 3–4 others; `f4_replay` vs fresh `f4` on the same system | every replay agrees with the fresh verdict, no trace mismatch, replay ÷ fresh `≤ 0.5` | a mismatch on a refutation target, or ratio `> 0.5` |
| 4 | One trace serves every curve, and every prime, of the same shape | Record on (`P1`, seed A); replay on (`P1`, seed B) and on (`P2`, seed A) | no mismatch, ratios as in idea 3 | a mismatch |
| 5 | A separate coset per point changes coverage, not F4 work | (a) F4 counts, shared vs per-point coset, same targets policy; (b) exact `m = 2` coverage `|{P₁ + P₂}|` at small `p`, shared vs per-point vs random sets | (a) per-point ÷ shared in `(0.5, 2)`; (b) per-point covers about twice the shared count, like random pairs | (a) outside the band; (b) per-point ÷ shared `< 1.5` |

**Fixed.** `P1 = 1399680001`, `P2 = 3265920001` (as EXP-PKM-ODD-001); small primes `1001089` (coverage, `192 \| p − 1`)
and `1000081` (`aut`, `p ≡ 1 mod 3`, `36 \| p − 1`); grevlex, degree bound 64, budget 1,800 s per system,
`RAYON_NUM_THREADS=4`, one binary (SHA-256 in `binary.sha256`), seed `20261115` unless a cell says otherwise.

**Ladders** (all in `run.sh`). Idea 1: `m = 2`, `S = 16, 32, 64, 128, 256`, random and coset, 3 draws; `m = 3`,
`S = 4, 8, 16`, 3 draws. Idea 3: `m = 2` degrees `2^4 … 2^8` and `3^3 … 3^5`; `m = 3` degrees `2^2 … 2^4`; 3 replays
each. Idea 4: `m = 2` `2^6`, `3^4`; `m = 3` `2^3`; cross-curve (seed 77) and cross-prime (`P2`). Idea 5a: `m = 2`,
`2^4 … 2^8`, 3 draws per layout. Idea 5b: `S = 16, 32, 64`. Idea 2: `S = 18, 36`, 2,000 targets.

**Rules.** A row that times out is censored and reported as a lower bound, never imputed. Verdicts come from the
prediction column above; every one is reported. No ladder, budget or band changes after a row is seen without a dated
amendment. Idea 1 compares against committed rows of another study (same instrument family, same `p`, `m`, `S`; a
different `V`), stated as such. Nothing here concerns cryptographic sizes or Pollard rho [pcc:unverified].

**Execution.** Local (4 vCPU, 15 GiB), as EXP-PKM-ODD-001: no EKS route here; checkpoint opt-out
`CHECKPOINT_SKIP=no-s3-credentials`. Cells needing more memory are censored and listed, not moved.

## Amendments

None at freeze.
