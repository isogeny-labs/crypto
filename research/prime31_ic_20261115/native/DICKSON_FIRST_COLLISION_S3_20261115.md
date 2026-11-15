# EXP-R31-E — first-collision decomposition of the fixed Dickson/S3 bottleneck (2026-11-15)

**Standing: producer-local exact arithmetic; [pcc:unverified].** The Rust code and CI gate are checked in; new Rust/F4 results must be read from the exact-head workflow. No complete ECDLP or cost win is claimed.

## Motivation

EXP-R31-D (M8_NATURAL_F4_CI_OUTCOME_20261115.md) ran existing Rust F4 on 16 naturally sampled prime31 public targets: all 16 stopped at degree 6 with a 26,618-entry dense reducer above the 15,000 cap. The same failure occurred with only S3(x6,x7,w) and its two required six-level Dickson fibre blocks. The obstruction is in the target-independent pair system, so this experiment changes that presentation rather than increasing the entire m8 ideal.

## Exact mathematical decomposition

Let f(X)=X^2-2 and T_j=f composed j times, T_0(X)=X. Over every commutative ring:

    T_d(X)-T_d(Y) = (X-Y) * product_{j=0}^{d-1} (T_j(X)+T_j(Y)).

Proof: iterate f(U)-f(V)=(U-V)(U+V). For two distinct roots of a separable degree-2^d common fibre, a unique first collision occurs at a level j from 0 to d-1, where T_j(X)=-T_j(Y). Include the diagonal X=Y as a seventh case. The disjointness assertion is about **rational point assignments with first-collision exclusions**, not an unsaturated ideal decomposition.

The new Rust F4 gate adds a low-degree constraint T_j(x6)+T_j(x7)=0 alongside the **same** S3 polynomial and two Dickson tower blocks for each of six levels; the diagonal arm adds x6-x7=0. All seven arms are required to preserve finite rational pair coverage. Extra constraints can help or hurt Groebner reduction. This is a hypothesis to measure, not a predicted speedup.

## Exact producer-local prime31 arithmetic

Source: p=2147483647, ordinary E:y^2=x^3+847514846x+2087444724, prime group n=2147446831, cofactor one, generator (625037715,2100627251). Deterministic Dickson depth 6 has seed 3, terminal 933677148, 64 distinct fibre x coordinates and 31 curve-liftable canonical x coordinates.

| First collision j | 0 | 1 | 2 | 3 | 4 | 5 | Diagonal |
|---|---:|---:|---:|---:|---:|---:|---:|
| Full fibre unordered pairs | 32 | 64 | 128 | 256 | 512 | 1024 | 64 |
| Liftable base unordered pairs | **8** | **10** | **31** | **62** | **120** | **234** | **31** |
| Liftable S3 finite abscissa roots | **16** | **20** | **62** | **124** | **240** | **468** | **31** |

The distinct-pair rows total 2016=C(64,2) and 465=C(31,2). For each distinct liftable pair the S3 quadratic in w has two roots. Each diagonal has one finite doubled-point root; P+(-P)=O is not represented by a finite w and needs a separately guarded exception. Thus **961 finite roots with multiplicity, 961 distinct finite x values** for this specific base. The discriminant identity Delta_w S3=16*(x^3+a*x+b)*(y^3+a*y+b) was verified in field arithmetic for all 465 pairs, then an independently implemented EC addition path checked x(P+Q), x(P-Q) and all diagonal doublings. EC additions in this step are **validator controls**, not a relation-finding oracle.

Source and reproduction:
- Rust: [prime31_dickson_pair_branches.rs](examples/prime31_dickson_pair_branches.rs) reuses the actual source-tree prime31 Arithmetic/Base and Rust F4.
- Separate independently authored [Python arithmetic replay](replay_dickson_pair_branches.py), no external packages.
- Focused development workflow: [.github/workflows/prime31-algebraic-ic.yml](../../../.github/workflows/prime31-algebraic-ic.yml).

## Frozen seven-arm F4 test and advancement criterion

1. Replay all 64- and 31-element branch counts and every rational S3 root; fail closed on any mismatch. Run the existing Rust F4 on each pair (S3, T_j(x)+T_j(y)) and (S3, x-y) with the unchanged degree-12, 15,000-NNZ/dense, five-second cooperative caps. Record every failure, completed basis, matrix maximum and total work across **all seven arms**. A cheap branch alone is not a full oracle.
2. If summed preconditioning improves, prove saturated coverage with repeated-abscissa, infinity and y-sign guards; run on the **frozen 16 natural public masks**. Require algebraically extracted signed witnesses, canonical projective rows and incremental independent rank; split test labels cannot drive the oracle.
3. Complete cold prime31 public-target recovery in existing Rust, including branch generation, all failed F4 calls, root extraction, sparse LA, final scalar verification and matched same-host optimized rho/BSGS. Independent replay, complete counters, bigger-field holdouts and PCC are required before a solver advantage is stated. A basis without extracted roots is not a relation.
4. Falsify under this cap if every branch still oversizes or if total seven-arm cost exceeds the previous F4 strategy. Then try the certified paired-S3 quotient work in crypto #4228 rather than silently raising caps.

## Prior work, novelty boundary and execution

- Petit–Kosters–Messeng, PKC 2016 (https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf) established composed rational-map prime-field algebraic IC and Semaev relations. Dickson/Chebyshev composition and the difference-of-squares identity are established mathematics (https://encyclopediaofmath.org/wiki/Dickson_polynomial); neither is claimed novel here.
- Rechecked prior crypto #4077 whole-call useful-output gate, #4275 dense-vs-tower positive experiments, #4228 paired-S3 preprocessing, #4188 natural F4 abort and signed-support controls, #3877 chained transfers; cryptanalysis #1536 Rust auxiliary-map IC; crypto-autoresearcher #3305 lead atlas. This is not an exhaustive novelty search. Research-KB MCP and repository KB wrapper were unavailable in this runtime; the fallback was targeted primary GitHub files/PRs and literature.
- No EKS experiment, checkpoint admission, signed independent PCC verification or benchmark ranking was performed; the GitHub Actions job is a development diagnostic.

**Decision:** Test the mathematically exact seven-way first-collision preconditioner before increasing the unchanged F4 matrix cap; do not equate factor-base algebraic support with useful independent relation rank.


## GitHub Actions seven-arm result — full two-tower branch constraints (2026-11-15)

Actual Rust workflow [43721541632](https://github.com/isogeny-labs/crypto/actions/runs/43721541632) passed all development checks. The [exact-source, raw-counter evidence](DICKSON_FIRST_COLLISION_S3_CI_EVIDENCE_20261115.json) pins head 71328023788fc86f1904eb3c3381d9370914a877 and job 132834395751. The independent Python replay and the same-fibre Rust branch replay also passed. **Standing remains [pcc:unverified], not independent scientific validation.**

| First-collision arm | F4 degree | Max dense B' | F4 multiply-adds | Outcome at 15k cap |
|---|---:|---:|---:|---|
| j=0 | 7 | 4,056 | 40,966 | Basis, no extraction |
| j=1 | 6 | 19,473 | 52,688 | Oversize |
| j=2 | 6 | 22,619 | 47,543 | Oversize |
| j=3 | 6 | 27,727 | 30,820 | Oversize |
| j=4 | 6 | 26,618 | 30,947 | Oversize |
| j=5 | 6 | 26,618 | 30,944 | Oversize |
| Diagonal | 6 | 7,886 | 304,781 | Basis, no extraction |

All seven: **538,689 measured F4 multiply-adds**, including work on censored branches, and 16.385 ms summed internal F4 clocks, excluding polynomial construction, cold setup, allocation, lifts and scalar recovery. Five arms remain undetermined and zero roots were extracted. This **falsifies a cheap seven-arm complete fixed-S3 preconditioner under the original full-two-tower encoding and 15k cap**; completion of individual branches does not make the required union complete.

### Next exact algebraic reduction: remove redundant fibre towers

For j>=1, rather than add T_j(x)+T_j(y)=0 to two complete depth-six towers, retain one complete six-level x chain and only j y-variables, with terminal y_(j-1)^2 = 2-x_j. All original levels y_j through y_5 become consequences of the branch and the x chain. This yields 6+j tower variables, versus 12. For j=0 and the diagonal, use y=-x and y=x directly, requiring only six tower variables. Feed the **same S3** into this smaller quotient ring. The union of seven branch solution sets is exactly the rational finite pair fibre (with the stated first-collision/separability and identity exclusions); unlike simply dropping equations, this is an algebraically constrained presentation, not a group enumerator.

A new source file [prime31_dickson_collapsed_s3.rs](examples/prime31_dickson_collapsed_s3.rs) and focused workflow gate compare all seven quotients with the same Rust F4 engine and 15k cap. This is a distinct proposed encoding test, not yet a benchmark. No end-to-end ECDLP or matched-rho performance inference is justified from its development counters.


## Second Rust CI result — collapsed towers, exact branch coverage

Actual exact-source GitHub Actions [43722021158](https://github.com/isogeny-labs/crypto/actions/runs/43722021158) **passed all Rust and independent Python development steps**, including new [collapsed-tower Rust example](examples/prime31_dickson_collapsed_s3.rs). The [machine-readable seven-arm CI results](DICKSON_COLLAPSED_S3_CI_EVIDENCE_20261115.json) pin the compiled source head a61bf2050a8a47d4a39cad770c52552a199fcffd and job 132835864395. Its independently cross-checked rational pair enumeration still covers exactly all 961 signed-pair S3 finite roots. This is development evidence, [pcc:unverified].

| Arm | Reduced tower vars | Full-tower B' peak | Collapsed B' peak | Collapsed F4 muladds | Collapsed verdict |
|---|---:|---:|---:|---:|---|
| j=0 | 6 | 4,056 | 4,056 | 40,020 | Basis only |
| j=1 | 7 | 19,473 | 18,282 | 259,813 | Oversize |
| j=2 | 8 | 22,619 | **15,577** | 39,259 | Oversize (just over cap) |
| j=3 | 9 | 27,727 | 18,515 | 28,421 | Oversize |
| j=4 | 10 | 26,618 | 26,618 | 30,943 | Oversize |
| j=5 | 11 | 26,618 | 26,618 | 30,943 | Oversize |
| Diagonal | 6 | 7,886 | 7,886 | 303,074 | Basis only |

**Conclusion at the frozen cap:** true quotient-ring elimination lowers certain peak dense reducer sizes substantially (j=2: 22,619 to 15,577), with identical correct rational root coverage, but does not change the number of completed arms (2 of 7), and **total recorded F4 multiply-adds rise from 538,689 to 732,473 (about 1.36x)**. The summed engine-only clock is 12.44 ms versus 16.39 ms on separate runs, **not an admissible end-to-end speedup** (no cold setup, witness extraction or controlled paired runtime); the operation count and censoring defeat a positive whole-call claim. No new algebraic witness was extracted, and no scalar recovered.

This rejects further uncapped scaling of either seven-arm raw F4 construction as a primary solver strategy. It also gives a more surgical question: the j=2 cap miss is only 577 entries above 15,000, but satisfying *one* branch is insufficient. The next candidate should avoid F4 for the fixed pair stage altogether: compute the finite signed pair-abscissa fibres using the quadratic S3 discriminant and field operations, construct a certified univariate or split-fibre membership for w, and charge that algebraic preprocessing and the target-dependent S4/F4/finite-field witness extraction in full. This must not use a generic EC group table or hidden target logs. Test against existing dense membership experiments (#4275), not as an unsearched innovation.

**No cheaper complete ECDLP solution has been established on prime31.** These measurements are specific to fixed S3 preprocessing, not a full index-calculus call, and remain [pcc:unverified].
