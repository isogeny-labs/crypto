# VERIFY: LEAD-A-fall-effective-variable-law (lane A, Lead A3), skeptic pass, 2026-11-12

Every statement below is `[pcc:unverified]`: it is one skeptic agent on the same shared container
and the same tool (`fallclose.c`) as the producer. Nothing was replayed on a second host.

## Re-run of the key measurement

The cheapest falsifier cell is (i), T-type with n=35, k=17. I re-ran it, together with the T33
stall control that shares N = v = 34.

- **Build.** I built `fallclose.c` (sha256 `f8d33898…ad68b52f`) in a scratch directory outside the
  repository: `gcc -O3 -march=native -o fallclose fallclose.c $(pkg-config --cflags --libs m4ri)`.
- **Instances.** I regenerated them fresh from the committed emitter
  (`research/lfd_lower_bound_20261111/emit_instance_T.py 35 1 17` and `... 33 2 17`).
  - The T35 seed-1 line matches the line recorded in `out/instances.jsonl` byte for byte.
  - `emin count` on it gives 0 solutions.
- **Batch size.** I used 4096 where the producer used 8192. This changes the order in which
  products are added, which tests the claim that the result does not depend on order.

| instance | my result | producer's result |
|:--|:--|:--|
| T n35 k17 seed 1, D=4 | one_in=1, W_dims (1,35,596,6580,52956), gen_rank 34, 186 s, 0.81 GB | identical (141.7 s) |
| T n33 k17 seed 2, D=4 | stall, W_dims (0,0,64,2968,48068), codim 4888, 156 s | identical; equals treatise T33a/b |

**The key number reproduces:** cell (i) falls, the falsifier fires, and the result is unchanged by
batch order.

## Code review of `fallclose.c` (read in full)

- **Generators.** The coefficients of a_i b_j, a_i, b_j and the constant are correct for
  S = X1²X2² + z²(X1²+X2²) + zX1X2 + a6. They use Frobenius-linearity on Boolean a_i, b_j.
- **Products.** x_i·m is computed as m|x_i, which is correct in the Boolean ring.
- **Column ordering.** Columns are ordered degree-D first. In reduced row-echelon form, a row whose
  pivot is in a low-degree column therefore has no degree-D part.
- **Back-reduction.** A low-pivot row can only be back-reduced by new rows that also have low
  pivots, and those rows are queued as well. So the queue's span equals W ∩ B_{<D}, and the
  semi-naive closure is sound.
- **Pivot check.** Pivot clashes abort (`internal: bad pivot`).
- **Bugs.** I found none that could create a spurious 1.
- **Consistency checks against spurious falls.**
  - The satisfiable controls at the same sizes stall, with codim equal to the number of solutions
    (2 and 4).
  - A fall implies the instance is unsatisfiable, which agrees with `emin count` = 0.
- **Cross-check file.** `out/crosscheck.jsonl` has 72 comparison rows plus a summary line, with 0
  disagreements against the independent Python `lfd_small.closure`.
  - Caveat: 30 of the 42 stall comparisons are satisfiable instances. Only 12 are unsatisfiable
    stalls.

## Verdict against the preregistration (lane_A_falldegree.md, Lead A3)

**Claim tested:** "an unsatisfiable instance has 1 in W_4 iff v <= 33", for n in {2k-1, 2k, 2k+1}.

**Scope check.**

- Cell (i) is T35 with k=17, so n = 2k+1.
- Cell (iii) is R36 with k=18, so n = 2k.
- Both are inside the claim's stated scope.

**Falsifiers.**

- The listed falsifier "(i) with 1 in W_4" fired.
  - The producer's two instances show it.
  - My re-run of seed 1 shows it again.
- Cell (iii) also contradicts the claim, since v=35 falls.
  - It is not named in the falsification line.
  - It is still a direct counterexample to the "iff" claim and to the prediction for (iii).
- The producer applied the preregistered reading for "(i) falls" unchanged: "equation count
  matters".

**Thresholds.**

- No threshold was moved.
- The excess law e = 2v-m is labelled post-hoc. `POSTHOC_PREDICTION.md` is timestamped 04:02,
  before the first follow-up log at 04:20.
- RESULT.md correctly reports that its D=2 clause was falsified. It does not promote the law.

**Positive-control gap.** The independent stall certificate checker (Prop. `cert`) was not run,
because it is in the VLT1-encrypted sibling repository. The verdict does not need it: the
falsification rests on falls, not on stalls.

## Default unchanged

- `git status` shows no modification under `research/lfd_lower_bound_20261111/`. The only
  untracked item there, `out/lead_A1_syzygies/`, belongs to lead A1.
- `emit_instance_T.py` is committed (f74b40015a, lead A2).
- No tracked source file was edited by this lead.
- The modified files `examples/jv_cover.rs`, `src/cryptanalysis/f4_fp.rs` and
  `src/cryptanalysis/gaudry_quartic_bin.rs` belong to other lanes.
- The `fallclose` binary is not in the directory.

## Minor issues (none changes the verdict)

1. **Cross-check composition.** "42 full-dim stall comparisons" are mostly satisfiable instances:
   30 satisfiable, 12 unsatisfiable.
2. **Same-tool evidence.** The falls at N=34 and 36 are verified only by the same tool, with
   identical code. There is no independent large-N closure, no certificate (an explicit expression
   of 1) was extracted, and nothing was run on a second host. Exporting the combination that yields
   1 and checking it independently would close this.
3. **Infrastructure.** No bench record, S3 hash or EKS run, because AWS is blocked. This is stated
   in RESULT.md.

**Conclusion:** I agree that the v-law is falsified as preregistered `[pcc:unverified]`.
