# Graph-IC lead sweep, 2026-11-18: summary

Two rounds of parallel lanes over the open leads of the graph-based
extension-field index calculus (`research/extension_ic_graph_20261115/`).
Round 1 landed in PR #4654; round 2 is in PR #4741. Every number here is a
producer result labelled [pcc:unverified]; none is a validated row, and no
paid compute was used. Red-team reviews: round 1 findings are folded into
the lane READMEs and `docs/lessons/`; round 2 is verbatim in
[CRITIC_ROUND2.md](CRITIC_ROUND2.md).

## Results by lane

| Lane | Status | Result |
| --- | --- | --- |
| [run51-outcome](run51-outcome/README.md) | partial | No Run51 result exists in reachable stores. Pod `7zki4wnz56fzad` is no longer on the Runpod account (read-only listing, 2026-11-18T17:09Z). Originals were LOCAL on the dispatching host. |
| [descent-finder](descent-finder/README.md) | done | Descent ≈77% F4, ≈22% linear stage; 99.9% of descent work is spent on residuals that never decompose. Opt-in F4 trace replay cuts F4 multiplications 2.29x on recorded streams with exact agreement. |
| [descent-walltime](descent-walltime/README.md) | done | Same replay: local wall time 1.26–1.29x faster (indicative, non-isolated, not counterbalanced). Corrected counted ratio 1.68x / 1.60x; the round-1 "conservative 1.62–1.71x" label was wrong. Amendment 13's 3–16x loss does not apply to this replay design. |
| [complete-accounting](complete-accounting/README.md) | partial | Counted linear-stage patch with exact conservation tests (108 random systems, all 3,415 Run50 residuals). Hooks missed 9.43e11 bitops (+27.6% of hooked descent). Run50 lower bounds recounted at 0/0.1/1 GAE. Null-count gaps on both IC and rho still forbid any §5a ranking. |
| [rho-fold](rho-fold/README.md) | done | Negation (fold 2) is the strongest legitimate fold and is already registered. The reference is weaker than it could be in another way: schoolbook F_p6 pricing; a Karatsuba/Chung–Hasan tower would cut rho cost ≈1.58x (op-count model, not measured). The "fastest rho 36.9 s" is a lucky single walk (0.387×E[steps]); expected best ≈76 s. |
| [fresh-scaling](fresh-scaling/README.md) | done (draft) | Six-size fresh-target protocol draft and a tested Rust target generator; ≈55 CPU-h modelled. **Not freezable as written**: see critic F1, F2, F4, F8. |
| [independent-validation](independent-validation/) | done | Everything recomputable checks out (Q=[d]G, statistics and bootstrap bit-for-bit, Run50 repricing, descent-finder rerun). All sets are still `insufficient_evidence`: six claims are missing from the PCC ledger (dropped by merges) and LOCAL raw archives are not in custody. |
| [root-build](root-build/README.md) | done | f4_fp.rs and metrics errors were already fixed on main. Three merge-duplicate compile errors removed (plugins.rs duplicate test, a duplicate `channels` field, a duplicate `fn main`); scoped cargo checks pass. Full workspace, clippy and test execution not run. |
| [scout-graph-ic](scout-graph-ic/LEADS.md) | done | New leads L1–L6, led by using exact-log quadratic classes as a descent factor base. |
| [scout-repo-wide](scout-repo-wide/LEADS.md) | done | Lab-wide leads, stale lines and orphan candidates. |

## Distance to rho

All values are lower bounds over lower bounds on one exposed p1009 target, so
no ranking is permitted. Against the cheapest registered rho in
expectation (negation, batch 1, 1.549e14 bitops), the deferred graph arm's
recounted cost is 0.044–0.050 of rho across w = 0, 0.1, 1. Against a
Karatsuba-priced rho (9.78e13, a model), it is 0.070–0.079. Earlier figures
quoted against rho plain-b16 (0.0625–0.0705) used a single lucky walk as
the reference and should not be cited (critic F3).

## New leads, ranked by expected value per cost

1. Land the counted linear-stage patch in shared `src/` (complete-accounting
   next step 1). This is an accounting change, not an improvement, and it
   raises future descent subtotals by about 28%.
2. L1 (scout-graph-ic): descent over exact-log quadratic classes, with
   factorization patterns (1^4,2), (1^2,2^2) and (2^3). The first step is a
   local, ~30 s pattern tally on the recorded stream.
3. Integrate F4 trace replay into the pipeline, opt-in, and run a counted
   local test. Isolated wall-time confirmation needs paid compute (M1).
4. Amend and then freeze the fresh-scaling protocol. Fixes are needed first:
   - the exponent test (F1);
   - full G-cov coverage (F2);
   - add the quadratic-table IC arm and a stronger rho (F4);
   - a p=2017 u64 overflow pilot (F8).
5. Fill the remaining null-count gaps (graph containers, RREF scans, PRNG
   internals, and rho setup/RNG), then attempt the first legitimate §5a band
   ranking.
6. Lab-wide (scout-repo-wide):
   - repair the malformed bench record blocking 16 others;
   - rotate dead AWS keys, which block every launcher and S3 path;
   - validate the tw2l Table-B lead;
   - clean up stale `docs/ACTIVE.md` lines.

## Owner decisions

- Runpod pod `replead-r1-20261116`: $1.04/h, running since 2026-11-16. It may
  hold the only copy of `research/representation_leads_20261109` output.
  Copy the data, then terminate it. The pods `hky-*` and `bl-ladder` belong
  to draft PR #4735, whose lifetime cap does not work, so they need explicit
  deadlines.
- Adopt the operation-level linear-stage counting convention in shared
  `src/`, and choose how 1 GAE is priced: the formula (331,250 bitops) or a
  measured walk step.
- Rho reference policy:
  - adopt a Karatsuba tower;
  - report expected cost with an interval rather than the minimum of single
    runs;
  - authorize an N≥20-seed rho run.
- Paid compute, in two blocks:
  - the isolated EKS A/B for trace replay;
  - the about 55 CPU-h fresh-scaling run, after the protocol is amended.
  Both are blocked until the AWS credentials are rotated.
- Whether `pcc ledger verify` should fail when claim files lack ledger
  entries (2,588 of 3,584 claim files currently do), and whether to re-append
  the dropped entries, labelled post-hoc.
- The two root-build fixes outside `src/`: one in `tests/` and one in
  `examples/`. Both are one-line duplicate removals.

## Round 3 (2026-11-19)

All results are [pcc:unverified]. No paid compute was used.

- [l1-pattern-tally](l1-pattern-tally/README.md): **L1 step 1 keeps the lead.**
  - On the recorded Run50 stream (3,419 solutions), (1^4,2) occurs 69 times against a kill threshold of 35 or fewer, and (1^2,2^2) 238 times against 110 or fewer.
  - Observed/predicted ratios are 0.97–1.11.
  - This confirms only the pattern-frequency assumption. The exact-log match rate, valid signed classes and any descent saving are still unmeasured.
  - Next: step 2 (an exact |D_universe| count), then the step-3 matched-rows test.
  - The 92 KB seed-389 rows file is LOCAL pending a store push; see the custody note.
- [fresh-scaling-v2](fresh-scaling-v2/README.md): the amended draft protocol fixes critic findings F1–F5 and F8 ([CHANGES.md](fresh-scaling-v2/CHANGES.md)).
  - The new exponent rule FAILs the lane's own e=3 model, which v1 passed.
  - G-cov now requires a count for every inventory row.
  - The table IC arm is primary, with a priced Karatsuba rho.
  - p2017 is dropped: the frozen worker refuses it, and several unreduced u64 sums mod l were found. The ladder is now p101–p1511.
  - Modelled compute is about 23 CPU-h, down from about 55.
  - It is still a draft. The owner decisions are listed in its README.
- CI hygiene on #4741:
  - clippy debt fixed in 9 examples, plus the library errors;
  - the n83 example now reads its store-managed CSV at run time;
  - the oversized data file was removed from Git.

## Round 4 (2026-11-19): L1 steps 2-3 and L5

All results are [pcc:unverified]. No paid compute was used.

- [l1-steps-2-3](l1-steps-2-3/README.md): **L1 gets a weak PROMOTE under the rule as written. A longer run of the same stream is inconclusive.**
  - **Step 2.** |D_universe| = 254,530 classes up to sign, out of 508,536 monic irreducible U over F_1009. f = 20,510 / 254,530 = 0.0806. The halving rule does not trigger.
  - **Step 3, prespecified seed-389 stream.**
    - Matched rows: 6 (Poisson 95% 2.2–13.1). Full splits: 2.
    - Ratio 3.0x, with a 95% interval of 0.54x–30x. This is PROMOTE by the ≥2x rule.
    - Against the model full-split rate it is 1.26x (0.46x–2.75x).
  - **Not prespecified: the same walk run to 30,000 residuals.**
    - 57 matched rows (47 from (1^4,2), 10 from (1^2,2^2)) and 31 full splits.
    - Ratio 1.84x (1.17x–2.95x), which is inconclusive. Against the model rate it is 1.36x (1.03x–1.76x).
    - The model itself expects about 1.51x at this f, so the 1.5x kill line falls on the model.
  - **Control.** The local rerun reproduces Run50's deferred collection exactly: graph stats, rank 508, every phase's bitops, the scalar.
  - **Checks.** All matched rows pass the group identity and the anchor witnesses. The affine path and elliptic replay for matched rows are not yet done.
  - **Next.** Only an M3-style paired protocol on fresh targets is justified. It needs ≥100 pooled successes per arm and a decision rule preregistered against the measured full-split rate. L2 (more anchors after rank readiness) is what raises f.
- [l5-kernel-split](l5-kernel-split/README.md): **C4 is closed.**
  - Krylov plus BM is 87.60% of linear-stage macs, above the >70% rule.
  - NF does 0 multiplications.
  - Per-kernel totals conserve exactly against complete-accounting.
  - The Krylov products alone are 17.4% of descent; they are the only linear-stage target worth optimizing.

## Round 5 (2026-11-19): L1 frozen paired protocol

All results are [pcc:unverified].

- [l1-paired/results](l1-paired/results/README.md): **INCONCLUSIVE**, because the interval straddles 1.5.
  - 96 fresh p1009 targets; F = 555, M = 789.
  - ρ̂ = 1.42, 95% interval 1.27–1.59.
  - Every control passed, and every L1 row passed the full affine and elliptic replay.
- **Established:** on fresh targets, the verified descent successes per solution are 2.42x those of full splits alone, matching the model.
- **Not resolved:** the preregistered 1.5 line, which sits on the model's own expectation. No descent-cost or wall-time saving is claimed yet.
