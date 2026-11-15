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
