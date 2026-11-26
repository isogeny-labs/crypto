# Execution workstream: proposals 1–34

Date: 2026-11-24 (America/Los_Angeles). This file belongs to draft crypto PR #5165. Every numerical outcome here is a producer result `[pcc:unverified]` until an independent signed verdict is attached.

## Sources checked

- Semaev, *Summation Polynomials and the Discrete Logarithm Problem on Elliptic Curves* ([primary paper](https://eprint.iacr.org/2004/031.pdf)), Theorem 1: the exact `S3` formula, resultant recurrence, and point-sum condition. A local original PDF was read through research-kb because the web PDF returned 403.
- [Prime-field technique inventory](../../docs/ic/ECDLP_RESEARCH_TECHNIQUES.md), especially the dense `S4`/`S5` support diagnostic and prime-field solvers; [prior 100-item ledger](README.md) and [first execution receipt](PROGRESS.md).
- Sources checked (KB): revision `752f945c662745959e4516205d027b5e`, refreshed 2026-11-22; `paper:1cab0ae675996cdba6393b1e369a22113fd2e74bcf1223595743dc140631c7d6` (original Semaev paper), `source:a9212eb47b2dee6ab0db114f8b9c40a4` (`crypto-autoresearcher:knowledge/open-problems/KN-OPEN-004.md`, sparse-elimination question), `source:de5553a8be093839651c483e49cd1ba7` (`crypto-autoresearcher:experiments/EXP-BKK-001/specification.yaml`, mixed-volume controls). These records motivate a matched specialization test, not a novelty claim.

## Frozen bounded #24 comparison before execution

Hypothesis: the 88/125 specialized coefficient support at `z=0` lowers exact decomposition work relative to a matched ordinary target on the same prime-order curve. Fixed inputs: `p=1009`, `a=1`, `b=14`, `#E=1013` prime, `S4(x1,x2,x3,z)=Res_t(S3(x1,x2,t),S3(t,x3,z))`, and targets at valid abscissae `z=0` and `z=320`. The factor base is the first 16 *nonzero* liftable `x` coordinates in ascending order, with the deterministic PARI/GP `sqrt(Mod(f(x),p))` point choice. Both targets use this identical base and exactly 16³ ordered triples. No random sampling or post hoc target replacement.

The reference is direct signed group addition of all base triples; the algebraic route evaluates exact `S4` and enumerates every zero on that grid. Check equality of algebraic and signed solution sets; preserve affine target point and sign. Count coefficient support, multidegree, polynomial evaluations, exact zeros, signed witness rows, and matrix rank over `F_1013` of rows `[base coefficients | -1 target]` separately. The only low-level work proxy is the number of nonzero terms in dense monomial evaluation; it is **not** wall time or a solver benchmark. Stop on any algebraic/signed mismatch or GP error. Cap: two target specializations and 16³ triples each; local exact GP only, no cloud spend. No full ECDLP solve is attempted. Output and exact command will be appended below.

## Results

The matched #24 comparison was **not launched**. The mandatory `scripts/checkpoint.sh gate` failed on both supported transports. Existing [GP support counts](S4_SUPPORT_GP.txt) remain structural observations; exact decomposition count, independent rank, and elimination cost remain unmeasured. The earlier ordinary `z=7` control is not an on-curve abscissa for `E:y²=x³+x+14`; this protocol deliberately chooses on-curve `z=320` instead.

### Sanitized checkpoint failure

At 2026-11-25T02:02–02:04Z, the gate's presigned-token upload returned HTTP 403. A diagnostic POST with the same token returned S3 XML code `InvalidAccessKeyId`. The supported AWS-profile fallback, invoked by `CHECKPOINT_TOKEN_FILE=/tmp/checkpoint-token-absent-ideas-001-034 scripts/checkpoint.sh gate`, returned `InvalidAccessKeyId` from `PutObject` as well. The complete fallback stdout/stderr is retained locally at `/tmp/ideas_001_034_checkpoint_aws.log`; the raw XML was kept only in local temp storage because it contains an access-key identifier. The token file and embedded policy expire 2026-12-01T17:10:14Z, after the attempted run; its S3 host, bucket and `checkpoints/` prefix match the intended upload. This isolates invalid or revoked key IDs rather than token expiry or a wrong endpoint. No gate bypass was used. `[pcc:unverified]`

### Item-specific EC outcomes

Each row gives an exact derivation or the concrete missing construction reached by source/data inspection. These are producer analyses `[pcc:unverified]`, not formal Lean proofs. An `Fp×` mechanism is not promoted to an `E(Fp)` point relation without a certified translation. Source inspection covered the submitted [100-item list](IDEAS.md), [existing assessment](README.md), [technique inventory](../../docs/ic/ECDLP_RESEARCH_TECHNIQUES.md), `src/cryptanalysis/{semaev_prime_poly.rs,semaev_higher.rs,isogeny_walk/,jacobian_transfer/}`, and `docs/transfer-assessment/MEASUREMENT.md`.

| # | Outcome or precise blocker |
|---:|---|
| 1 | **Exact:** every valuation `v:Fp×→Z` is zero: `(p−1)v(u)=v(u^(p−1))=0`. A tropical relation must use a lift and prove a point-sum map. |
| 2 | **Search:** no lift-independent p-adic integral predicate for Semaev point relations is defined in the proposal or checked prime-field inventory. Specify lift, acceptance rule, and soundness. |
| 3 | **Exact:** a finite-order isometry of the hyperbolic plane has translation length zero, so this natural metric cannot rank smoothness. A different metric needs measured EC decomposition correlation. |
| 4 | **Exact:** blowing up a closed point of smooth `P¹` blows up an invertible ideal and is isomorphic to `P¹`; the proposed exceptional factor-base divisors do not arise. |
| 5 | **Search:** Arakelov height needs an arithmetic lift and archimedean data; no map from low height to `P1+...+Pm=R` is specified. The repository transfer contract requires that map and cost. |
| 6 | **Exact:** origin-preserving automorphisms of an elliptic curve in characteristic >3 have order at most 6, while `|PGL₂(Fp)|=p(p²−1)`; full PGL₂ orbit reduction cannot preserve EC addition. |
| 7 | **Search:** a sheaf of recorded local states could check compatibility, but there is no map from its global sections to a new exact signed point identity. |
| 8 | **Source-backed:** an isogeny can transport subgroup DLPs; CM lift filtering alone has no stated decomposition predicate. `isogeny_walk/` implements transport, not a universal CM smoothness filter. |
| 9 | **Search:** no computable knot-invariant-to-EC-group homomorphism or signed point certificate is specified. |
| 10 | **Exact scope mismatch:** Tate–Shafarevich obstructions concern global-field torsors; no torsor encoding or map to finite-field factor-base point sums is supplied. |
| 11 | **Exact:** on prime-order `<P>`, a character normalized by `χ(P)=ζ_r` satisfies `χ(dP)=ζ_r^d`; useful evaluation encodes the unknown log. Duality alone does not give an SVP shortcut. |
| 12 | **Search:** Hecke coefficients are scalars, and no correspondence maps them to exact relations in the chosen target subgroup at a charged cost. |
| 13 | **Source-backed:** `jacobian_transfer/` and the transfer contract require a certified subgroup map, compatible Jacobian solver, projection, and target recovery. A split divisor alone is insufficient. |
| 14 | **Domain gate:** a Voronoi sieve can organize multiplicative NFS candidates; the proposed boundary rule supplies no EC point equation. Compare verified Semaev relations per cost only after defining EC cells. |
| 15 | **Search:** no finite-field state, differentiable score, ODE projection, or exact point-relation validator is defined. |
| 16 | **Exact:** isogenies satisfy `φ(P+Q)=φ(P)+φ(Q)`; products of image abscissae do not generally represent this sum. Existing path transport does not establish coordinate-multiplicative relations. |
| 17 | **Exact:** small Archimedean or p-adic norms do not guarantee bounded-prime smoothness: an integer prime may exceed any chosen factor-base bound. LLL still needs a point-relation translation. |
| 18 | **Domain gate:** braid-group exponent-sum identities are not certificates of EC point sums; no homomorphism making them so is supplied. |
| 19 | **Search:** no Penrose-tile-to-EC-candidate map or smoothness-correlated statistic is defined; an equal-budget collector comparison cannot be instantiated. |
| 20 | **Search:** no associated surface or trace-formula statistic is linked to Semaev solution density; this is the missing prerequisite for an executable filter. |
| 21 | **Exact:** rational reconstruction controls integer numerator/denominator for a residue, but `x:E(Fp)→Fp` is not a group homomorphism. It certifies no point decomposition. |
| 22 | **Exact:** Teichmüller lifting preserves `Fp×` multiplication but not EC addition; factorization of its p-adic expansion gives no signed point relation. |
| 23 | **Exact:** for the obvious coefficient group, Hilbert 90 gives `H¹(Fp,G_m)=0`; that obstruction rejects nothing. A nontrivial coefficient module and sound EC predicate must be specified. |
| 24 | **Blocked controlled run:** prior support is 88/125 at `z=0` versus 125/125 at on-curve `z=320` on a prime-order curve; both multidegrees are `(4,4,4)`. The frozen decomposition/rank/cost comparison above did not launch because S3 rejected both credential paths. |
| 25 | **Exact:** with a fixed finite integer prime base, its smooth products up to `X` grow only polynomially in `log X`, so their density tends to zero. Szemerédi's positive-density premise does not follow; EC point relations need a separate construction. |
| 26 | **Exact:** `Fp` has no proper subfield, and the norm of `a∈Fp` viewed in `Fp^k` is `a^k`; the tower does not expose a smaller EC relation source. |
| 27 | **Search:** the proposed GNN graph needs labeled EC candidates available before decomposition; none is defined. A held-out curve and charged verified-rank baseline are prerequisites. |
| 28 | **Search:** the GAN's integer-smoothness discriminator does not validate EC point relations; no generator-to-exact-relation map or matched collector arm is specified. |
| 29 | **Search:** no predeclared EC interval feature/reward trace appears in the proposal. RL scheduling needs measured yield heterogeneity and static/random same-stream controls. |
| 30 | **Domain gate:** shared integer factors of residue representatives are not invariant under EC addition. A learned embedding must predict verified point decomposition on held-out curves. |
| 31 | **Exact:** predicted integer smoothness cannot replace exact verification; false positives are invalid rows and false negatives can lose rank. An EC classifier requires a sound point predicate and full cost. |
| 32 | **Search:** no time-parametrized EC sieve trajectory, vector field, observable, or exact root-recovery rule is specified for a neural ODE. |
| 33 | **Domain gate:** a hyperbolic embedding needs a labeled EC relation graph and held-out exact decomposition test; prime-factor proximity itself is an `Fp×` feature. |
| 34 | **Exact/domain:** `Fp` has trivial field automorphism group; number-field Galois orbit savings do not directly act on `E(Fp)`. An auxiliary-field transfer and relation-preserving action are required. |

## Next action

Restore valid checkpoint credentials and pass `scripts/checkpoint.sh gate`, then run the frozen #24 comparison. Record algebraic solution-set agreement, exact point witnesses, multidegree, independent row rank, and cost as separate fields. Any subsequent EC adaptation of an `Fp×` proposal requires a group-relation map and a fully charged translation.
