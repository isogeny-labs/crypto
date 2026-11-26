# Bounded EC point-relation screens for proposals 6, 16, 21, 29

Frozen 2026-11-24 (America/Los_Angeles), before execution, for draft crypto PR #5165. Scope is public synthetic curves over `F_1009`; no external target or cloud spend. The latest user instruction resumes parallel `E(F_p)` runs after an earlier stop. The user had also authorized local execution after disclosure of the failed S3 checkpoint; the parent confirmed that this renewed instruction admits bounded local checks. `scripts/checkpoint.sh gate` still fails because `.resume/RESUME.md` is absent; the prior S3 upload returned `InvalidAccessKeyId`. This is a **local-only user exception**, not a successful checkpoint.

Sources checked: Semaev 2004 Theorem 1 (exact point relations); `docs/ic/ECDLP_RESEARCH_TECHNIQUES.md` and `research/prime_field_100_ideas_20261123/WORKSTREAM_001_034.md` (existing boundaries). Sources checked (KB): revision `750797a540fb4c38837c67b12df84d5b`, refreshed 2026-11-25T03:19:48Z; searches for PGL2 EC orbits, isogeny-coordinate relations, and prime-field decomposition scheduling returned prior internal proposals/evidence but no established map that turns a generic coordinate product or integer smoothness predicate into EC point sums. This batch uses exact group arithmetic rather than claiming novelty.

## Frozen inputs and checks

- Main curve: `E/F_1009: y²=x³+x+14`, cardinality 1013 prime, `G=(0,425)` of order 1013. Reuse the first 16 nonzero liftable abscissae in ascending order as the factor base, choosing PARI/GP's deterministic square root. Targets for the scheduler screen are the *next eight* liftable abscissae in ascending order, with the same root rule; none belongs to the base.
- **#6, generic PGL2 orbit claim:** evaluate the fixed Möbius translation `x↦x+1` on every liftable affine abscissa; count outputs that are not liftable. Identity `x↦x` is the positive membership control. A single failure falsifies this proposed full action on the point set; it does not rule out special-curve automorphisms. Positive special-curve control: `E0/F_1009:y²=x³+3`, a prime-order-79 subgroup, and a nontrivial cube root `ζ`; check `φ(x,y)=(ζx,y)` preserves point addition on fixed `G0,2G0` and has an orbit of size three.
- **#16 and #21, coordinate relations:** on the main curve set `P=G`, `Q=[2]G`, `R=P+Q=[3]G`. Check the positive group identity exactly, then compare `x(R)` with `x(P)x(Q)` and `x(P)+x(Q)` modulo 1009. This is a counterexample to treating abscissa multiplication or addition as a generic point-group homomorphism, including under the identity isogeny. It does not test every supersingular path or rational-reconstruction heuristic.
- **#29, scheduling prerequisite:** enumerate every signed three-base-point sum once: `16³×8=32768` candidate sign patterns. Divide the first factor-base index into four contiguous blocks of four; each block has the same candidate cost. For each of eight deterministic targets, count exact signed group hits in each block. Use targets 1–4 only to select the block with highest training hits (tie: lowest block index); compare that fixed block on targets 5–8 against predeclared static block 1 and the mean of all four blocks. Report all 8×4 counts, training and holdout totals. This tests stable yield heterogeneity, **not** a trained RL policy or complete relation rank.

The native GP source has no randomness. It must guard curve cardinality, subgroup order, all chosen points, and every reported point identity. Preserve stdout, stderr, and shell exit separately. Stop on a GP error, incorrect group guard, or any unexpected input size. Accept a result only if stderr is empty and all summaries appear. No silent retry or parameter change. Counts are deterministic component results, not wall-time or full IC comparisons. A negative or null result remains in the PR.

## One-shot command

Frozen native source: `EC_LEADS_001_034_GP.gp`, SHA-256 `4b89585eaf32543ecfeea9b1c8e9eefd4fee6d3628e37d97d98079abe6f89341`. The local-bypass admission and unchanged failed checkpoint state are recorded above. Run once from repository root:

```sh
gp -q < research/prime_field_100_ideas_20261123/EC_LEADS_001_034_GP.gp > research/prime_field_100_ideas_20261123/EC_LEADS_001_034.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_LEADS_001_034.stderr.txt
```

Record the shell exit immediately in `EC_LEADS_001_034.exit.txt`. Append source/raw hashes, exact outputs, and failure status below after the run. No output is yet admitted.

## Observed local result — 2026-11-24 (America/Los_Angeles)

The frozen command was invoked once with PARI/GP 2.17.3. Shell exit was 0; stderr was empty; stdout contained eight target rows and the final sentinel. This is a local producer result `[pcc:unverified]`, admitted by the user's renewed local-run instruction while the S3 gate still fails. There was no cloud run, external target, wall-time comparison, or result-driven input change.

| Check | Exact result | Scope |
|---|---|---|
| #6 generic `x↦x+1` | Of 506 liftable affine abscissae, identity retained all 506; the fixed translation sent 240 to nonliftable abscissae. First counterexample was point `(2,348)`. | Refutes this full PGL2 generator as a point-set action on the main curve; does not exclude smaller automorphism groups. |
| #6 special-curve control | On `y²=x³+3`, `#E=948`; a point of order 79 had a 3-element abscissa orbit `[378,112,519]` under cube root `ζ=374`, and the tested additivity identity held. | Positive EC automorphism control; no relation-yield gain measured. |
| #16/#21 coordinate map | On the prime-order main curve, `P=G` has `x=0`, `Q=[2]G` has `x=991`, and `P+Q=[3]G` has `x=162`. Product `x(P)x(Q)=0` and sum `x(P)+x(Q)=991` modulo 1009 differ from 162; the group addition check held. | Counterexample to using abscissa multiplication/addition as a generic group homomorphism, including under identity isogeny. Does not test a special supersingular path or rational-reconstruction search cost. |
| #29 training/holdout | Base `x=[1,2,4,5,10,11,13,16,17,23,25,29,30,34,37,40]`; held-out target `x=[48,49,54,55]`. Training hits per block `[10,14,20,19]` selected block 3. Holdout hits `[27,23,31,30]`: selected 31, fixed block 1 gave 27, all-block mean `111/4=27.75`. | Exact signed-point hit counts with equal candidate work per block. Four training and four holdout targets are too few to infer stable advantage; no RL policy, rank, or complete cost measured. |

The full 8×4 per-target count table is in [stdout](EC_LEADS_001_034.stdout.txt). It sums to the reported training and holdout totals. The selected block had 4 more holdout hits than the fixed block-1 control and 3.25 more than the all-block mean at equal candidate count. This is a small target-dependent component comparison; it does not establish a scheduling advantage on other curves or target sets. `[pcc:unverified]`

| Preserved artifact | SHA-256 |
|---|---|
| `EC_LEADS_001_034_GP.gp` | `4b89585eaf32543ecfeea9b1c8e9eefd4fee6d3628e37d97d98079abe6f89341` |
| `EC_LEADS_001_034.stdout.txt` | `ab4e92a1762ad1782b40966f50604bf2ef78a742ba9995fdb3886acb6d14fbe0` |
| `EC_LEADS_001_034.stderr.txt` (empty) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `EC_LEADS_001_034.exit.txt` | `bde294368bfed77c2cddf8cec271d398aee9cdbab3b26e1059281bd33adb0120` |

Other proposals in 1–34 retain their per-item mathematical obstruction or missing EC construction in [workstream 1–34](WORKSTREAM_001_034.md). This batch adds executed controls only for #6, #16, #21, and #29, alongside the earlier separate #24 S4 run. Proposal #8 has a related CM automorphism control here, but its stated CM *lift filtering* is not instantiated by that control. No result is silently promoted from one proposal to another.
