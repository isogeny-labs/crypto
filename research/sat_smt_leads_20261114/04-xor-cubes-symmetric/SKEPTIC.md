# SKEPTIC review — idea 4, XOR cubes on symmetric functions

Independent review, 2026-11-14, one core, about 15 minutes. Every number here
is `[pcc:unverified]`. Scratch scripts and outputs:
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-04-xor-cubes-symmetric/`
(`indep.py`, `alg.py`, `aa.py`, `an.py`, `cmp.py`). No producer file was edited.

## Outcome

- **Reproduced:** yes (smallest cell bit-for-bit; three l=5 UNSAT instances too).
- **Producer's verdict "inconclusive":** supported by the pre-written criterion.
- **Corrected verdict:** inconclusive, unchanged. In substance the idea is dead
  at these sizes: clause 1 of the kill criterion holds, and clause 2 misses
  only by 0.3%, which is inside the noise I measured.

## 1. Instance generator and encoding (l=3, n=11, all 8 seeds)

Checked with my own carry-less F_{2^11} arithmetic, my own group law and a
brute force over real curve points (not over S4). Producer code was used only
to obtain `(basis, xR, planted)` and the CNF under test.

| check | result |
|---|---|
| field modulus found independently | equals the producer's (`0b100000000101`) |
| my group law, closure and associativity | 200 random triples pass |
| planted tuple is a real decomposition (some signs, x(sum) = xR) | 4 of 4 |
| all CNF models, enumerated with Glucose4 and blocking clauses | planted: exactly the 6 permutations of one tuple; random: 0 |
| CNF x-models vs real-point brute force | identical sets on all 8 (no missed, no spurious) |
| `y_j = x1_j ^ x2_j ^ x3_j` in every model | holds |
| full cube sweeps (lit-3, xor-3) vs brute force | agree; each SAT cube's model lies in the asserted cube |
| SAT cubes per planted target | 3 for literal, 1 for XOR, as expected |

The symbolic e-form (`symbolic_e.json`), evaluated with my arithmetic, equals
the producer's S4 on all of V^3 for the 16 instances at l=3 and l=4. A
different rank test (span of second differences of the residual map) gives the
same non-affine rank as `algebra_rows.json` on every cube compared (all cubes
at l=3, two per k at l=4): full rank n for every k < l.

## 2. Rerun

`python3 run_sat.py <scratch>/rerun_l3.json 3 11 8 --cms` took 2 s. All 80
rows match `sat_l3.json` on conflicts, decisions, propagations, SAT-cube
counts, truth counts and CNF size, and `results.json` holds the same rows. My
own loop reproduces the l=5 numeric-order conflicts for seeds 5004 to 5006
(xor-5/lit-5 = 0.972, 0.972, 0.990, as reported).

## 3. Fairness

What holds:

- Baseline and treatment use the same instances, seeds and CNF; they differ
  only in assumptions and are interleaved.
- Every arm found in the data files is reported.
- Slopes use 3 sizes, on UNSAT targets only; SAT and UNSAT are tabulated separately.
- The enumeration, MITM and linear 2+1 baselines are stated as counts, and the
  README says plainly that all SAT arms sit near 2^{3l} against 2^l.
- File times are consistent with the protocol preceding the data
  (PROTOCOL.md 08:23:42, first data 08:24:32). The post-protocol `pdplib.py`
  edit (08:25:47) is disclosed. I could not verify ordering beyond mtimes;
  there is no git history.

Problems:

1. **The sub-1 ratio at l=5 is a cube-order artefact.** With numeric cube
   order xor-5/lit-5 is 0.972, 0.972, 0.990 on seeds 5004 to 5006. With two
   shuffled orders it is 1.007, 1.025, 1.004, 1.008, 0.993, 1.026 (geometric
   mean about 1.01). The same arm moves by up to 5% across orders at l=5
   (geometric mean 3%) and up to 25% on SAT targets at l=4. The reported 0.98
   (UNSAT) and 0.997 (pooled) are therefore not below 1 in any meaningful
   sense. The 0.14 slope gap is likewise noise plus the l=3 penalty.
2. **Clause 2 was not evaluated as written.** It asks for "the best k of each
   arm", but l=5 ran only k=2 and k=5. Among those, both arms are best at k=2
   and the pooled ratio is 1.04, which satisfies the clause; k=l gives 0.997,
   which fails it on the letter.
3. **The SAT arm does not test the idea as stated.** The CNF is the x-bit ANF
   with definitional XORs for e1. The symmetrised system in (e1,e2,e3) was
   never given to a solver, and no symmetry breaking was applied. A null
   result here does not cover the e-variable encoding.
4. **The slope is weak.** Three points, 3 to 4 targets each, and n/l varies
   (11/3, 13/4, 17/5), so "bits per l" mixes l and n.
5. **One mislabelled target.** Seed 4005 is described as "one random target is
   decomposable". It has 6 S4 zeros, none of which lifts to curve points
   (`truth_nsol_lifting = 0`); it is the source of all 32 non-lifting models
   (16 in the incremental runs, and by count the other 16 in the fresh runs). It sits in the truth-SAT class at l=4 although it
   is not decomposable on the curve. It does not affect the UNSAT slope
   population or the l=5 cell.
6. **Per-cube overhead.** Conflicts are the primary metric, but XOR cubes cost
   1.7x the propagations at equal conflicts (I see the same at l=4 and l=5).
   The README reports this; on a work metric the XOR arm is clearly worse.
7. **Encoding cost is uncounted for all arms.** The CNF is built from the full
   2^{3l} evaluation table and its Moebius transform, which is already
   enumeration cost. It is equal across arms, so it does not bias xor against
   lit, but no SAT arm here can be compared with the 2^l scan on total work.
8. The CryptoMiniSat arm has wall time only and was not checked beyond the
   rerun. No session log, pcc claim or benchmark record exists, as stated.

## 4. Does the verdict follow?

- Clause 1 (non-affine rank never 0 for k <= l-1): holds, independently
  confirmed at l=3 and l=4. The prediction is refuted.
- Clause 2 (ratio >= 1.0 at the largest l, at k=l and at best k): 0.997 at
  k=l, 1.04 at best k. It fails on the letter by 0.3%, with same-arm noise of
  3 to 5%.
- With clause 1 holding and clause 2 not met, the protocol allows only
  "alive (SAT constant only)" or "inconclusive". The slope gap (0.14) is under
  the 0.15 threshold and the ratio is not a real constant gain, so
  "inconclusive" is the right reading and "alive" would not be.

My shuffled-order runs suggest clause 2 would be met under a randomised cube
order, but they cover 3 UNSAT instances at one k, were run after the fact, and
are not the protocol's procedure. They do not justify upgrading to
"kill-criterion-met". A fresh run with an A/A arm, randomised cube order and
all k at l=5 would settle it, most likely as a kill.
