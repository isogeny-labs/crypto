# Prime-field ECDLP: state of the leads and an independent assessment

Date: 2026-11-07. Scope: `isogeny-labs/crypto`, `isogeny-labs/cryptanalysis`,
and the lead atlas in `isogeny-labs/crypto-autoresearcher`
(`knowledge/leads/ecdlp/`, 160 leads, 27 in `ic-prime`). Read-only survey; no
experiment was run and no research state was changed. Everything quoted below
is from committed files; where a file itself says a record is "not found" or
"unverified", that is repeated here. Standing of every number: `[pcc:unverified]`
(quoted from the repositories, not re-derived here).

---

## 1. The one-paragraph picture

The program has converged, correctly in my view, on a single open question for
prime fields: **KN-OPEN-020** (and its parent KN-OPEN-001). Every method that
treats `E(F_p)` as a group plus coordinate hashing is bounded by Shoup
(`T ≳ √N`) and Corrigan-Gibbs–Kogan (`S·T² ≳ N`). The only non-generic handle
over `F_p` is that coordinates are integers mod `p`. Two algebraic levers exist:

1. **Small roots of summation polynomials** (Coppersmith/Jochemsz–May). Closed
   with a strategy-independent reach bound for convex monomial sets:
   `(n! vol Newt(S_3))^{-1/n}` = 0.329 (3 unknowns) / 0.354 (2 unknowns) against
   a demand of 8/9 at the first viable arity `(m, w) = (4, 2)`. Margin about
   −0.56 proved, −0.39 numerically at best. Probes B/C measured the same
   threshold (`δ ≈ 1/6` for `S_3`) at 40/56 bits. Only non-convex monomial sets,
   joint multi-block lattices and a non-Coppersmith small-root solver remain.
2. **A factor base with sub-`B`-degree membership.** Two results pin it:
   mean relation yield is `C(B+m−1, m)/N` for *every* size-`B` base
   (KN-FIND-007, measured exactly in 288 cells, ratio 1.000000), and a
   target-independent algebraic locus of degree `d` holds `O(d)` points, so a
   size-`B` algebraic base has description degree `≥ B/3`. Yield is therefore
   dead as a lever; only the *solving degree* of a structured presentation can
   move, and every measured presentation (PKM towers, digit/base-B, split-graph
   secant) shows degree growing with system size. The digit presentation's
   first-fall degree `d_ff = m·2^{m−1} + ⌊(s − 2^{m−1})/2⌋ + 1` is
   curve-independent and reproduced by a curve-free block-factored null
   (EV-PFDR-1394a4): the Gröbner instrument is seeing presentation
   combinatorics, not the curve.

With the linear algebra included, arity `m = 4` is the *first* arity that could
beat rho at all, and it needs an `S_5` decomposition oracle cheap at
`B ≈ p^{1/5}`. Nothing in the atlas supplies one; the k-SUM-conjecture row
(LEAD-EXO-ksum-conjecture-s5-oracle) argues no structureless oracle can.

Measured distance to rho today (all generic-track, prime fields):

| row | counted S_IC / S_rho | note |
|---|---:|---|
| Table B prime ladder | 5.47× at log2 r 11.9 → 13.5× at 23.4 | widens with size, as `S·T²` predicts |
| 20-bit holdout, N 32→45 | 9.47× → 7.75× [6.92, 8.70] at w=0 | second-host replay reproduced 1,008/1,008 |
| k-sum table k=5, 40 bits | 9.7× rho-neg, 7.6× bsgs-neg | exponent 0.547 vs generic optimum 5/9 |
| fp28 (layout tournament v6) | 23.0 / 22.9 / 22.3 at w = 0 / 0.1 / 1 | candidate 12 % slower, not an improvement |
| residual collector `res2`, 36 bits | ≈ 2.6× rho (S 2.94 vs 1.13) | smallest prime gap on the progress table; a birthday search with rho's exponent, no VALIDATION.json seen |
| bitop/v1 ladder 16–40 bits | 6.2× → 78× rho-neg; exponent 0.69–0.72 vs rho 0.55 | 25 a=−3 prime-order curves |
| prime m=4 (S_5) cells, 16–28 bits | 8.2×–29.5× rho | validated, "REGRESSES"; own F4 arm ~ |F|^11 |
| single-target counted IC, 2026-11-07 logs | 25× / 41× / 53× rho at 20/22/24 bits | an earlier "wall-time parity" claim was withdrawn by its author (weak in-binary rho) |
| multi-target IC v3 vs anchored batch rho, T = 4096–16384 | 1.03–1.07 at w=0, 0.79–0.92 at w=1 | order flips across the §5a band, so unranked; no independent validator yet |

None has a fitted exponent below rho's. The IC/rho gap *widens* with size on
every prime ladder. The boundary ledger (docs/ic/BOUNDARY_TARGETS.md, regime C)
records vs_rho as "Not achieved" with the 16–24-bit a=−3 gap widening 19× → 403×.

---

## 1b. What is implemented (crypto and cryptanalysis), in brief

Full inventories: `crypto_survey.md`, `cryptanalysis_survey.md` and `atlas_survey.md` in this directory.

- **crypto** (`src/cryptanalysis`, about 584k lines): Pollard rho (negation,
  automorphism-class, preprocessing/BL, GPU), BSGS, kangaroo; the `ic.*`
  pipeline (pair-table / mitm / k-sum / root-finding decomposition, planner,
  CADO-style filtering, sparse LA); Semaev S_3/S_4 and PKM towers; F4/F5/F6
  engines; small-root lattice probes; isogeny/volcano tooling
  (`isogeny_algos`, 20.7k lines); APS single-relation solver (2,382 lines,
  correctness-only on p ≤ 65,521); hashed-factor-base tensor crates
  (`hashed-ecdl-tensor` 22.6k, `ecdl-tensor-ic` 6.9k). The `ecbench` harness
  (about 69k lines) measures every method in S = GAE/√r with the §5a native-work
  band, replay certificates, and a Postgres/S3-backed results database; a
  leaderboard row needs a `VALIDATION.json`.
- **cryptanalysis**: the C11 reference library (rho, BSGS, kangaroo, grumpy
  giants, BL precomputation, GLV dispatch for j=0/1728, Cheon, Z_p^* index
  calculus, distributed and GPU rho) over 64-bit primes, the Rust attack suite,
  `tools/ecdlp-surfaces`, and the experiments tree (P-256 isogeny class is
  fundamental: no vertical levels, no j=0/1728 member; volcano IC on ECC2K-130
  all null; xedni cross-field and lift-optimizer studies; Hecke D=−39 cells).
- **cryptanalysis specifics worth knowing**: no prime-field IC baseline row
  exists there (docs/ic/baselines.json is binary Koblitz only); the only
  Certicom targets encoded are ECCp-131 / ECC2-131 (unsolved; ECCp-79/89/97/109
  are not defined); the draft paper `papers/ecdlp-constant` quotes a prime
  full-pipeline cell at 23.4 bits of S = 56.8, 14.5× matched rho, exponent 0.60;
  `research/coordinate-leverage-bound-20261103` shows no IC row reaches rho
  parity through cheaper coordinates alone (the p36 res2 row would need a
  per-op factor c* = 2.59 against a realistic 1.05–1.30); measured positives are
  constant-factor rho engineering (negation-map group-ops ratio 0.748 pooled;
  GLV cycle-escape 0.658 / 0.768 of legacy ops on j=1728 / j=0; Z_p^* IC
  instruction halving 2.3×), several with wall-time results that go both ways.
- **Atlas bookkeeping**: REVIEW_STATE.json is pinned at 2026-10-30 (153 leads);
  the checkout holds 173 lead files and `--delta` lists 164 changed watched
  paths in crypto-autoresearcher alone, so the atlas is one review behind.
  `--brief` and `--coverage` need `orchestration/research-priority.yaml`,
  absent from a sparse checkout.
- **Theory notes** (crypto root): `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md`
  (the §1 map and Probes A–C), `RESEARCH_PRIME_FAST_ECDLP.md` (native rewrite:
  rho 357–466×, IC-2 704× faster, no vs-rho crossover),
  `RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md` (root finding 100–205× faster than
  msolve end to end at 16–24 bits; asymptotics unchanged),
  `RESEARCH_ISOGENY_FIBERS_ECDLP.md` (3,500 lines; every isogeny mechanism gives
  constants only; deployed prime-order curves have no hidden crater),
  `RESEARCH_P256_BACKDOOR_SPECULATION.md` (seed explained: drand48 after
  srand48(871696354); published b is a median candidate on six probes).
- **Recent prime studies worth knowing** (research/): `pkm_matched_prime_20261106`
  (D = 4,5,5,5,5 for t = 5–9 with p grown with depth, identical to the
  random-coefficient null); `semaev_alternatives_audit_20261104` (the
  18-idea program's "e < 1 at m=4" gate is wrong: a generic 2+2 MITM oracle
  passes it yet costs p^{2/3}; correct gate e < (m−3)/4);
  `prime_ic_elimination_20261101`; `prime_fall_degree_t14_20261101` (cap-6
  cells blocked by a 3,600 s request cap vs 86,400 s needed);
  `curve25519_algebraic_ecbench_scale_20261105` (S1–S4 scale-ups censored by
  wall caps at k = 24/28).

---

## 2. Lead-by-lead: prime-field index calculus (`ic-prime`, 27 rows)

Status key is the atlas's: `open_next_step`, `open_blocked`, `open_unexamined`,
`exhausted_at_scale`, `closed_dominated`, `closed_known`, `closed_obstructed`.
"Cost" is my estimate of the next step, labelled as such.

### 2a. The only exponent-relevant open rows

| lead | status | what is measured | obstruction / what decides it | next-step cost (mine) |
|---|---|---|---|---|
| `LEAD-ICP-pkm-tower-algebraic-oracle` | open_blocked | F4 solving degree grows with N on every tower family: m=2: 3,4,4,4,4,5,5,5,5,6 (N=2..20); m=3: 5,6,6,7; m=4: 6,7. Shape-matched null gives the same D. P-256 2-isogeny-chain encoding 2×–654× slower than dense. | Preregistered prime fall-degree study (crypto `research/prime_fall_degree_20261027`): stage A must reproduce Kummer m=2 medians, then score t ≥ 11 against the Amendment-6 prediction `d_F = 4 + ⌊(t−2)/4⌋`. Raw stage A/C runs exist only on the owner's Mac (`RESTART.md`). Route blocked: isolab accepts only ecbench specs. m=3 N=21 / m=4 N=20 need 130–380 GB hosts. | **Medium.** 14 CPU / 56 GiB / 4 h per job for stage A; large-memory host for m ≥ 4. |
| `LEAD-ICP-split-graph-secant-presentation` | open_next_step | Secant presentation has homogeneous degree d+1 vs 2d+1 for the dense same-base system (theorem; 14,220/14,220 saved root sets verified offline). Stock F4 handled degree 3 vs 6 on all 18 F17 targets. Toy candidate used fewer branch-summed rows than the nongraph control but *more* than its own dense presentation. | No complete cost per independent relation yet; 26 F127/d5 extraction caps censored. Needs a native triangular/FGLM extractor and a three-cell prospective comparison with low-x / hash-x / nongraph controls. | **Low–medium.** Toy fields; mostly implementation. |
| `LEAD-ICP-digit-presentation-solving-degree` | open_next_step | `d_ff` closed form, curve-independent, equals a curve-free null; σ > 1 at every s; no syzygy beyond Koszul; d_last = p + 2 for the plain presentation. Registered exhausted at m=2 s≤8, m=3 s∈{4,5}. | The m=3 CDCL(T)/F_p screen (cryptanalysis PR #1347 protocol) with gates e < 0.5 (natural queries) and a < 1.5 (reusable setup) has never been run. | **Medium.** Requires building the CaDiCaL F_p trail theory. |
| `LEAD-ICP-coppersmith-smallroot-semaev` | open_next_step (revisit only) | See §1. HG-rigorous reach never past δ = 0.10; heuristic fades 0.14–0.16; symmetric lattice same δ at 3–10× lower cost. S_4/S_5 reach caps 1/21.8 and 1/70.8 vs targets 1/4 and 1/5. One exceptional specialisation (z = 0) reaches 7/41 > 1/6. | Reopens only for a joint three-block lattice, a non-convex monomial set, a rank ≥ 2 GAP substitution, or a non-Coppersmith solver. | **Low** (desk) but low expected value: the exponent gap is ≥ 2×. |
| `LEAD-ICP-shared-coefficient-membership` | open_next_step | Nothing measured; theory only (norm form `A² − B²f = (X − r)U` in L(6O), shared Graeffe/quotient-ring coefficient tower). | Must route through the hardened m=3 protocol; m=2 cells can never be exponent evidence (gate e < m/2 − 1 impossible at m=2). | **Medium.** |
| `LEAD-ICP-algebraic-membership-factor-base` | open_next_step (revisit only) | Subgroup, coset, cubes, quadratic-character, Dickson, rational-map images: all within 0.0003 bits of a size-matched random base at m=2, N up to 2^26; learned selector 0.0011 bits headroom. msolve on subgroup bases grows deg^4.4 at m=3. | Theorem-kind: KN-FIND-007 conservation + bounded-degree theorem. Reopens only on a positive answer to KN-OPEN-020. | Closed in all but name. |

### 2b. Generic-track rows (bounded by √r; improvements count under the owner's iterative criterion but cannot beat rho)

| lead | status | measured | note |
|---|---|---|---|
| `LEAD-ICP-generic-table-decomposition` | closed_obstructed | Exhaustive IC exponent 0.99 (m=2), 1.01 (m=3); mitm 0.67/0.75/0.60 at m=3/4/5 vs rho 0.50. k-sum k=5 at 40 bits 9.7× rho-neg. | Theorem (CGK + Shoup). Root-finding is generic too (owner ruling R1, Option C: two tracks). |
| `LEAD-ICP-prime-ic-constant-engineering` | open_next_step | N 32→45 at 20 bits: 0.818 [0.773, 0.866]; packed filter −4 % wall at 32 bits only; layout=lines v6 at fp28: `invalid_measurement`, 12 % slower in valid sessions. Hash-table floor ≈ 53 % of a relation trial. | Software prefetch still open. Reference-only under the 2026-10-24 criterion unless D-R1 lands. |
| `LEAD-ICP-residual-partial-relations` | open_next_step (revisit) | Single-large-prime sweep is a birthday search: slope −0.025 [−0.034, −0.017], same as rho/BSGS/kangaroo; S 2.28 (B=2) to 11.89 (B=128) at 36 bits vs 1.29 rho-neg. | Double-large-prime / cycle variant untested on prime fields; F_{p^4} analogue cut LA exponent but whole solve 2.31× costlier. |
| `LEAD-ICP-coincidence-harvest-structured-base` | open_next_step | Native break-even ladder 16–32 bits: SUB_BREAK_EVEN_AT_SCALE, every slope d ln κ / d ln N in [−0.006, +0.008]; κ in [0.92, 1.08]; planted controls recovered. 44/44 powered toy cells null. | Effectively closed by law; the Coordinator has not yet discharged the ladder step. |
| `LEAD-ICP-full-width-arity-grid` | open_blocked | Prime complete-solve ladder stops at 32 bits, m=7; 63-bit panel: six time-censored planted controls. | Infrastructure (launcher admission, RSS monitor). Would give the first measured distance to rho at 63 bits. |
| `LEAD-ICP-retained-setup-online` | closed_dominated | Online IC 13.9× (n61) / 18.5× (n83) was vs *empty-table* rho; BL preprocessing rho at equal advice is 53–317× faster online. | Accounting correction, not a result. |
| `LEAD-ICP-isogeny-transported-relation-yield` | open_blocked | Crater/floor IC 0.975 [0.603, 1.451] and 1.127 [0.922, 1.342]: no separation; 17 of 20 arity-3 arms lost to infrastructure. | Conservation predicts no effect. |
| `LEAD-ICP-representation-subset-sum`, `LEAD-ICP-self-pairing-prime-order` | closed_obstructed | — | Theorem-kind. |

### 2c. Lifting / transfer / correspondence rows (prime fields)

| lead | status | measured | obstruction |
|---|---|---|---|
| `LEAD-ICP-char0-lifting-xedni-smoothness` | open_next_step (revisit) | p=353 Cheon example reproduced; 2/2 historical pairs independent; Xedni V2: 0 dependencies in 20 pairs. | JKSST obstruction is conditional on its premise chain; not a theorem over every lift rule. |
| `LEAD-ICP-xedni-function-field-sections` | open_next_step (revisit) | Exact: `P_lift = (1 − 2/p + p^{−3})/(2p^3)`, α = 3, checked on 5.96·10^8 slots, zero disagreements. Viability needs α < 1/2. | Proof-kind for rational elliptic surfaces. |
| `LEAD-ICP-lattice-ic-relation-construction` | open_blocked | Hecke+Atkin-Lehner rank closure on D=−39 reproduced by doubling rows alone. | No certified pipeline on even the F43/F97/F101 fixtures. |
| `LEAD-ICP-hecke-cm-class-point-rank-completion` | open_next_step | Hecke marginal rank over the pair-index-reuse control: 0, 10, 19, 0 on four cells. Owner's cited D79/support-seven/query-order records **not found** in any repo or open PR (2026-11-06). | Pair-reuse control already leaves 1 free log; no complete setup pricing. |
| `LEAD-ICP-prym-witness-relation-diversity` | open_blocked | Owner-brief figures (4.59 M attempts, 97/67 witnesses → 13/9 equations) **not found**. | No Q-attachment mechanism. |
| `LEAD-ICP-elliptic-code-list-decoding` | open_next_step | Nothing run. Desk: GS agreement threshold √(Bk) > k, so minimum-weight codewords are unreachable by decoding around a fixed word. | Zhang–Liu full text unread (eprint blocked the session). Author's own slide: "not even of square-root time". |
| `LEAD-ICP-extension-field-embedding` | open_next_step (revisit) | — | Gaudry cost `p^{2−2/n}` ≥ p; the embedding enlarges the problem faster than the subfield helps. |
| `LEAD-ICP-gfp5-*` (EcGFp5 / EcMasFp5, F_{p^5}) | mixed | m=5 symmetrised quotient solved at 3 primes: D = 2^16 confirming `2^{(m−1)^2}`, c_cyc 2^46.3–2^46.6 vs G1 CLOSE bar 2^46 (margin < 1 bit). | Extension-field, not F_p; included because the quotient-degree law is the cleanest measured Gröbner fact in the atlas. |
| `LEAD-ICP-xline-symmetry-special-j`, `LEAD-ICP-construction-normal-form` | open_unexamined | — | Cheap desk/toy items; the second is methodology (dedup proposed systems by normal form). |

---

## 3. Structure, transfer, generic and exotic rows that touch prime fields

Nearly all carry proof-kind or theorem-kind obstructions; I list them so they are
not re-proposed.

**Closed by theorem or measured invariance (do not re-propose):**
isogeny-class generic invariance (26-bit census, 403,168 runs, 33/35 level
ratios TOST-equivalent, rho floor/crater 1.004 [0.990, 1.016]); isogeny-walk to
weak neighbour (weaknesses are class invariants); isogeny-transport
preprocessing (reduces to single-curve BL, measured 1.5–21× loss); JMV
self-reduction (hardness equivalence, not speedup); endo-scalar action
(End(G) = Z/N); P-192 CM relations (103/103 scalar); secp256k1 CM audit (only
√6); small-CM non-unit endomorphisms (orbit h ≥ log N / log n); torsion-module
leak; Lattès dynamics; p-adic jets / canonical lifts (Teichmüller contrast at
chance, power 0.90 for 3×); elliptic nets (k-recovery exponent 1.97–2.80 vs
BSGS 1.17–1.32); torsor/deck period; split-Jacobian projection; same-field
genus-g embedding (Tate multiplicativity; `q^{2−2/g} ≥ q`); small-genus covers
in odd characteristic (genus floors 9/49/41 at k = 6/10/14); correlated rho
walks (0.073×); curve-model representation (d_reg = 2 invariant; Kummer and
u = x/y classes tie at `D = 2^{(m−1)^2}`); Semaev cover monodromy (uniform
`(Z/2)^{m−2} ⋊ S_{m−1}`); generic-oracle arity cost floor (free oracle at m=3
on ECC2K-130 still 2^68.6 vs rho 2^60.8).

**Open and cheap (desk or toy), mostly closure work:**
- `LEAD-EXO-coordinate-revealing-oracles` (split 2026-11-04): the GGM gives no
  bound once the solver sees coordinates; two x-queries plus free local work give
  the log. Honest conclusion already in the row: "the open question is index
  calculus itself".
- `LEAD-EXO-low-degree-query-lower-bound`: H1 withdrawn (false under a
  full-rank condition, checked exactly at p = 251–2039, 39/39 cells);
  H1′ proved with Δ in place of D. What remains is a description-length or
  local-work-charged bound: a complexity question no GGM argument decides.
- `LEAD-EXO-ksum-conjecture-s5-oracle`: write the conditional reduction
  (k-SUM conjecture) that covers every free-S_5-oracle proposal in one row.
- `LEAD-EXO-minors-guess-and-determine` (Mahalanobis line): never charged
  against matched rho. Four arXiv papers to read, then a 20–40-bit
  implementation with the scaling exponent fitted.
- `LEAD-EXO-hashed-factor-base-tensor-ecdl` (Akshima et al., Bochum v0.2):
  the only externally *claimed* subexponential prime-field algorithm in the
  atlas; conditional on an unverified `#k-SAT`/ETH preprint. Native
  implementation exists in both crypto and cryptanalysis (34 correctness tests
  pass); the 84-case speed panel is blocked on execution access. The
  clustering-loophole desk note closes one escape by non-admission
  (`λ ≥ 2^85` needed vs margins −16 bits). ECC2K-130 / ECCp-131 attempts
  exceed their exact-leaf budget without a relation.
- `LEAD-EXO-quadratic-phase-small-x-leakage`: NULL at 2^12–2^16 with valid
  controls; the instrument is uncalibrated on weak curve classes.
- `LEAD-EXO-lange-winterhof-interpolation`, `LEAD-EXO-invertible-hint-subset`:
  read the theorems (behind publisher logins), then decide if they give a
  proof-kind closure for bounded-degree statistics and advice.
- `LEAD-EXO-ml-learned-models`: a learnability control with a planted
  homomorphic leak; prediction is chance on real curves.
- `LEAD-STR-special-j-ic-semaev`: matched-order j=0 / j=1728 charged-S study
  (S_3 support 9/10 vs 13 generic; callgrind says the order-6 base costs +4 %).
- `LEAD-STR-class-representative-ic-selection`: 7.2 % model effect and 5.8 %
  l=2 yield effect at 24 bits (constant-factor; IC/rho 17–170× remains).
- `LEAD-STR-level-structure-torsor` (X_1(N) diamond torsor): expected closure;
  one proof obligation (separating-function degree over F_p when N ~ p).

**Rho-side rows that matter for honest accounting:**
- `LEAD-RHO-matched-rho-calibration`: toy prime rho.negation is 1.453× the
  floor with fitted α 0.398 [0.308, 0.476]; **no prime medium-tier rho bound
  exists**. Every prime-field "gap to rho" number has this in its denominator.
- `LEAD-RHO-native-step-cost`: 1,196 → 718 instructions/step; floor ~6–7 field
  multiplications.
- `LEAD-RHO-prime-gpu-rho`: P-256 1.632 G it/s on one RTX PRO 6000
  (descriptive; no admitted CPU/GPU comparison).

---

## 4. My assessment

### 4a. What the evidence says, read cold

1. **No live prime-field lead has a credible exponent mechanism.** The atlas's
   own pursuit audit (2026-11-01) keeps 139 of 160 leads "pursued", but that is
   a *scope policy* (only `closed_obstructed` leaves the queue). Reading the
   rows rather than the counts: of the 27 `ic-prime` rows, three are
   theorem-closed, five are closed in substance behind a `revisit_when`, and
   the remaining open rows are either generic-track constant engineering or
   solving-degree questions where every measurement so far says "degree grows,
   curve-independent".
2. **The strongest single fact in the atlas is a negative control.** The
   digit-presentation first-fall degree is reproduced by a *curve-free* null
   (EV-PFDR-1394a4), and the PKM tower D is reproduced by a shape-matched null.
   The Gröbner/F4 instrument is not seeing the elliptic curve. Any further
   "solving degree" experiment that lacks a positive control (an instrument
   that *does* distinguish anomalous / small-embedding-degree / supersingular
   curves from random ones) is measuring presentation combinatorics.
   `LEAD-EXO-weak-class-instrument-calibration` exists for exactly this; I would
   make it a gate, not a lead.
3. **Yield is a conserved quantity; only decomposition cost can move.**
   KN-FIND-007 plus the sub-break-even coincidence ladder close every
   "better factor base" idea at the level of mean yield, coverage and
   variance. The only escape is a decomposition test sub-linear in `|F|`, i.e.
   KN-OPEN-020. The program knows this; the atlas says it in five places.
4. **The owner's two tracks are the right accounting.** Generic-track gains
   (N 32→45, k-sum k=5) are real, replicated, and count under the iterative
   criterion, but they shrink a constant on a curve whose gap to rho *widens*
   with size. They should not be confused with progress on the open problem.
5. **Several cited evidence records do not exist in the repositories.** The
   Hecke D79 / support-seven / query-order figures and the Prym witness counts
   are quoted from an owner brief and were not found on any main or open PR as
   of 2026-11-06. Those two leads should stay blocked until the records are
   located.

### 4b. Where I would put effort, in order

1. **Finish the preregistered PKM fall-degree study** (`LEAD-ICP-pkm-tower-algebraic-oracle`).
   It is the owner's stated algebraic lane, it has a frozen protocol with a
   registered prediction (`d_F = 4 + ⌊(t−2)/4⌋`), the stage A/C data exist only
   on the owner's Mac, and the route is blocked by an access-contract item
   (D-ROUTE Option 1) rather than by science. Expected outcome: H0 confirmed,
   lead moves to `exhausted_at_scale` at the reached N. That is still the
   single most valuable closure available, because it is the one lane that
   meets the 2026-10-24 criterion. Recover the Mac files first (D-MAC).
2. **Seal a prime medium-tier rho bound** (`LEAD-RHO-matched-rho-calibration`).
   ecbench rho.negation / rho.plain / bsgs.negation at log2 r 32–48, ≥ 24 runs
   per size, replay certificates, §5a band. Cheap on isolab-eks, and every
   prime-field gap number is currently divided by a toy rho that sits 1.45×
   above its floor. If medium-tier rho lands near √(π/4), the gaps *grow*.
3. **Run the three cheap charged closures as Sonnet/Haiku task cards:**
   minors guess-and-determine at 20–40 bits against matched rho; the
   elliptic-code list-decoding toy (p ~ 2^16, B ~ 2^8, k = 2..6) once someone
   with eprint access reads Zhang–Liu; the ML learnability control with a
   planted leak. Each is a day of compute at toy scale and each removes a
   recurring re-proposal.
4. **Audit the hashed-factor-base tensor claim** (`LEAD-EXO-hashed-factor-base-tensor-ecdl`)
   as a desk task: does the Boolean encoding's variable count make the counting
   bound lose to enumeration once the CNF has ω(l√log l) variables, as it did for
   descended Semaev systems? This is the only externally claimed subexponential
   prime-field result in play and the program already owns an implementation.
5. **Split-graph secant complete-cost study** (`LEAD-ICP-split-graph-secant-presentation`),
   only after (1): the d+1 vs 2d+1 theorem is real, but conservation and the
   curve-free degree law both predict no complete-cost gain. Run it with the
   nongraph, low-x and hash-x controls and let it close.
6. **Generic-track constant engineering** only if leaderboard movement is the
   goal for its own sake. Software prefetch on the hash-table floor (53 % of a
   trial) is the one open item with a diagnosed mechanism; layout=lines is
   closed at fp28.

### 4c. What I would stop or demote

- All xedni / global-lift variants (α = 3 exact; JKSST), Hecke/CM and Prym
  (records missing; marginal rank 0 vs the pair-reuse control), isogeny /
  volcano / class-group transport (invariance theorems plus a 403k-run census),
  coincidence harvest (slopes inside ±0.008), residual large primes at m = 2
  (rho exponent), Lattès / p-adic jets / elliptic nets / torsor rows
  (proof-kind obstructions). Keep their `revisit_when` clauses; stop spending
  sessions on them.
- Extension-field embedding and same-field covers: theorem-closed.

### 4d. Directions I do not see in the atlas (offered as questions, not leads)

- **Sum-product structure of the small-x set.** The k-SUM framing asks whether
  `S_B = {P : x(P) < p^{1/5}}` has additive structure under the group law; the
  measured nulls say no. A Bourgain-type sum-product or Weil-bound argument
  for `x`-coordinate windows under addition would turn "measured null" into a
  theorem-kind closure of the whole free-oracle class, and would say *exactly*
  which pseudorandomness property a new oracle must break. Closure work, not an
  attack, but it would retire a dozen rows at once.
- **Positive controls as a hard gate for every algebraic-degree experiment.**
  See §4a(2). Any Gröbner/SAT/lattice instrument should first show it
  separates a planted-weak curve class from random before its null on P-256-like
  curves counts.
- **Multi-target accounting for deployed curves.** Preprocessing rho (Probe A:
  online steps track `2√(n/S)`, break-even 1–116 targets at 32–44 bits) is the
  only place sub-√n per target is real. For secp256k1 / P-256 the realistic
  adversary is multi-target. This is not a breakthrough and the atlas says so;
  but it is the one row where a deployed-curve number would change a threat
  model, and no prime medium-tier BL frontier is sealed.

### 4e. Bottom line

The repositories contain a complete, well-instrumented prime-field IC pipeline
and a lead atlas that has done the hard work of closing the classical levers
with theorems and controlled measurements. The honest reading is that classical
index calculus over `F_p` beating Pollard rho has no open lead with a credible
exponent mechanism; what remains open is the solving-degree question
(KN-OPEN-020), and every measurement so far points the same way. I would spend
the next cycle finishing the preregistered PKM study, fixing the rho
denominator, and converting the cheap exotic rows into charged closures by
delegation, rather than opening new mechanism families.

---

## Appendix: pointers

- Atlas: `crypto-autoresearcher:knowledge/leads/ecdlp/README.md`, `tools/lead_atlas.py --pursuit|--match`
- Prime-field analysis: `crypto:RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` (§2 derivation, §3 Probes A–C, §4 levers)
- Owner rulings: `crypto:research/prime_ic_program_20261029/DECISIONS.md` (D1, D2, R1 Option C, D-ROUTE, D-MAC)
- Sweeper and tracks: `crypto:docs/ic/SWEEPER.md`
- Open problems: `crypto-autoresearcher:knowledge/open-problems/KN-OPEN-001.md`, `KN-OPEN-020.md`
- Most recent prioritisation: `crypto-autoresearcher:docs/reports/hecke-cm-prioritization-20261106/REPORT.md`
- Round-2 desk work: `crypto-autoresearcher:docs/reports/leads-round2-20261104/README.md`, `ecdlp-leads-desk-20261104/README.md`
