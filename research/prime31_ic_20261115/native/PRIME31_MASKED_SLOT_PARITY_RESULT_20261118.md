# Prime31 independent-slot half-density masks: exact support obstruction and Rust result

**18 November 2026.** Pinned original Prime31 ordinary cofactor-one curve, source-specific CI [#44286173248](https://github.com/isogeny-labs/crypto/actions/runs/44286173248) **PASS**; 40 original native Rust library tests + one exhaustive B4 mask test **PASS**, separately coded Python BigInt verifier **PASS**, but **[pcc:unverified]** (no signed independent PCC). The preregistered acceptance threshold failed. No unknown ECDLP logarithm was recovered.

## Decisive new algebraic audit: arbitrary subset masks cannot repair the eight-slot yield

Let G be an abelian group and A a fixed B-point subset. Suppose each of m ordered slots contains **any** subset S_i of the *same* A. The support (both existence support and parity support in F2[G]) of

`(sum_{P in S_1}[P]) ... (sum_{P in S_m}[P])`

is bounded by **C(B+m-1,m)**. Proof: expand the ordered m-tuples. Since the group operation is commutative, their sums depend only on the multiset of chosen members of A. There are C(B+m-1,m) such multisets, regardless of how the different S_i are chosen. Passing to characteristic-two coefficients can only cancel group sums, never create new ones.

In particular, for all independently masked positive eight-slot bases chosen within **the same B31 original base**,

`|support| <= C(31+8-1,8) = C(38,8) = 48,903,492`.

The original unchanged group has n=2,147,446,831 points. Thus even a **perfect existence** oracle on these masked slots covers at most **2.27728535%** of uniformly distributed group residuals. Exact parity is bounded by the same figure and can have much smaller support. A uniform residual-shift strategy would need at least **43.91 independent trials on average**, even with an ideal membership oracle (for fixed, uniformly random shifts). For the 12 preregistered independent uniform target model, the expected hits are at most 0.2733; the actual 12 deterministic seeded targets are not claimed to be a random statistical sample. This theorem does not require identical slot subsets and is **strictly stronger in scope** than the identical-slot Frobenius-collapse lemma from #4714.

The distinction is important: **independent binary masks of one common B31 positive point base are not equivalent to independent point maps or hashes with a larger possible image**. Replacing each S_i by a publicly different scalar image [c_i]A may break this common-multiset restriction, as in #4714, but does not by itself make a fast extractor. Adding mere per-slot translations to these shared-base subsets does not enlarge their support: all sums shift by the same total translation.

Exact multiset ceilings for the three rungs: B8: C(15,8)=6,435 (at most 0.0002997% of G); B16: C(23,8)=490,314 (at most 0.02283%); B31: C(38,8)=48,903,492 (at most 2.2773%). These are *upper* bounds on support, not measured support counts.

## Frozen experiment: measured original-curve Rust candidate

From the [preregistered protocol](PRIME31_MASKED_SLOT_PARITY_PROTOCOL_20261118.md): B={8,16,31}, the first B rational-x points with canonical ordinate, eight independently selected half-density masks (4,8,16 per slot), fixed public seeds `0x4d41534b50333131` and `0x57534c4f54503131`, and the same 12 public residuals as #4714. All masks and targets were fixed **before** source execution. No target scalar supplied to the extractor. It uses the original native Rust Arithmetic, exact 4+4 XOR-parity table, and signed-free group sums.

| B / points per slot | Left half odd support | Natural odd residuals / 12 | Candidate setup EC additions | 12 target query EC additions |
|---|---:|---:|---:|---:|
| 8 / 4 | 112 | 0 | 596 | 1,344 |
| 16 / 8 | 1,476 | 0 | 8,488 | 17,712 |
| 31 / 16 | 14,824 | **0** | **132,176** | **177,888** |

All costs above are native operation-counter values, not complete ECDLP costs. Target-fixture construction and original-curve/base construction are charged separately in the raw JSON but excluded from the displayed candidate setup, as for the previous weighted-slot comparator. The only original-curve natural parity witnesses would have come from successful target rows; there were **none**. Three deliberately planted group-sum controls were independently replayed; planted existence does not guarantee odd parity.

Against the prior B31 *distinct-weight* control [#4714](https://github.com/isogeny-labs/crypto/pull/4714), whose 12 queries cost 11,065,988 EC additions plus 1,852,467 setup additions and yielded **7/12** odd residuals, this mask experiment charges only **310,064** setup-plus-query EC additions, **41.66x fewer**, but yields **0/12** odd residuals. This is **not a 41.66x solver improvement**: the new stage produces no natural relations. Its 12 query cost alone is **62.21x smaller** than the weighted control, showing why relation yield must be included in complete costs. The preregistered >=4/12 acceptance gate failed, so mask-only subset reshuffling should be retired at fixed B31.

## Validation details and links

- [Source-bound Actions run #44286173248](https://github.com/isogeny-labs/crypto/actions/runs/44286173248) on source commit `b5b3537e9c55c46e2f45503552f3de6f99d9f16b`: the pinned original Rust 1.100.0 library passes **40/40**, exhaustive independently masked B4 positive/negative parity checks **1/1**, full B8/B16/B31 candidate run succeeds.
- The producer-independent **Python BigInt arithmetic** separately checked the original curve and generator, all 12 frozen public targets in each B rung (36 records), every chosen slot index, exact EC witness identities for 3 planted controls, and **all 24 B8/B16 parity decisions** via an independently implemented full four-sum convolution. It did **not** recompute full B31 parity: the B31 zero outcome is source-tested, not independent arithmetic enumeration. As there were zero natural odd hits, no natural witness mutation test was possible. **[pcc:unverified]**.
- [Original run artifact #14418798701](https://github.com/isogeny-labs/crypto/actions/runs/44286173248/artifacts/14418798701) contains the full 36-row candidate JSON and independent verifier JSON. GitHub ZIP receipt `sha256:378d0a7afed84eddad6b9c923f43ef5323d1db727fec53c946cf5cabebb4f913`. Artifact retention is 30 days; source and exact protocol remain in this PR.
- Workflow syntax passed in the run (the separate workflow syntax job passed). A shared 'Workflow files parse' job failed *before parsing* on the pre-existing artifact-store pull, not on this experiment.

## Prior lead checks and next experiments

Checked #4623/#4635/#4647 (generic-equivalent Rank31 complete IC), #4678 (compact/mixed-slot candidates), #4692 (nonenumerative RR9 remainder tower, active elsewhere), #4698 and [Bochum's 2026 preprint repository](https://github.com/bochum-crypto/subexponential-ecdl) (independent hashing, different hypothesis), #4714 (weighted parity control), #4596/#4711 (matched baselines). Primary existing summation-polynomial line: Semaev (2004), Gaudry (2009), Diem (2011). The repository research-KB MCP/CLI is unavailable to this connector runtime; review is bounded and novelty clearance is **not** claimed.

**Ranked next experiments, not executed here:**

1. **Public-residual-only algebraic RR9/F4 or tensor hash extraction:** reuse the active #4692 pipeline to recover natural positive/signed relations without a generic four-sum table. Confirm that each slot map actually expands the *image* beyond one shared B31 pool and independently reconstruct signs/witnesses. No secret scalar in extraction; B4/B8 exact positive and negative fibre controls first.
2. **Witness-preserving weighted/hash comparison:** using **different public point maps**, not just subset masks, freeze the same source curve and 12 residuals, measure full exact parity, nonempty witness rows, linear-algebra coefficient rank and cost. Reject schemes that are bounded by C(B+7,8) or collapse via group-algebra Frobenius; do not choose the best seed post hoc.
3. **Full 1,024-log recovery only after nonenumerative extraction:** feed independently found relations to existing native Rust rank31/individual-log solver; charge setup, failed searches, inverse stages, modular rank, every shifted target and verification, then compare same-host cold IC frontier, optimized shared BSGS and rho. Do not relabel generic search as a non-generic speedup.

**Disposition:** fixed B31 mask-only parity sublead falsified by a mathematical support ceiling *and* its preregistered exact-source Rust panel. Main nonenumerative prime-field ECDLP objective remains open.
