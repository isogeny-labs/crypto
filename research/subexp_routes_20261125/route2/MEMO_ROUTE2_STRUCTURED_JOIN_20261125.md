# Route 2 — is there a structured join for m = 2?

**Verdict: no.** No factor base with |V| > p^(1/2) pays for itself, and no join over
either PKM tower costs less than Θ(|V|) per relation. The best join that exists
(the sumset table) reaches p^(2/3), measured exponent 0.75 over p = 1009…40961;
rho is p^(1/2). Route 2 as posed is closed for m = 2, and it is closed by an
*identity*, not by an implementation gap (§2). Sections §3–§5 give the three
measured structured-join attempts and the structural reason each fails.

All numbers below are from `route2_join.py` (pure Python, public toy instances,
p ≤ 40961), reproduced by the commands in `RUN.md`. Store: `route2_results.json`,
log: `run_all.log`. Total wall 10–13 s.

---

## 1. What V is, and how large it can actually be

| tower | V | hard condition | measured |
|:--|:--|:--|:--|
| Kummer | `g·μ_{2^t}` | `2^t \| p−1` | `|V| = 2^t` exactly; t ≤ v₂(p−1) |
| isogeny | `x(R' + ⟨Q⟩)` on `E'/F_p` | some `E'` with `2^t \| #E'` and `2R' ∉ ⟨Q⟩` | `|V| = 2^t` exactly (checked, 0 failures) |
| interval | `[1, 2^t)` | none | `|V| = 2^t`, no build cost |

Measured reachable sizes (`caps` stage; 240–600 random curves per prime):

| p | v₂(p−1) | √p | largest built &#124;V&#124; | &#124;V&#124;/√p | curves tried ≈ 2^t |
|---:|---:|---:|---:|---:|---:|
| 1009 | 4 | 31.8 | 256 (t=8) | 8.06 | 86 |
| 4099 | 1 | 64.0 | 1024 (t=10) | 15.99 | 67 |
| 12289 | 12 | 110.9 | 2048 (t=11) | 18.47 | 600 |
| 40961 | 13 | 202.4 | 256 (t=8, draw-limited) | 1.26 | 80 |

So **|V| > p^(1/2) is not the obstruction** — the isogeny tower reaches 8–18×√p
in these runs, because a random curve has `2^t | #E'` with probability ~2^−t and
the Hasse window is wide. But note the last column: reaching |V| = 2^t costs
~2^t curve-order computations, and then 2^t point additions to enumerate
`R' + ⟨Q⟩`. **The build is Θ(|V|) group operations.** That single fact already
kills the "make |V| bigger" idea (§2.3).

Analytic 2-adic caps for the primes the towers are attached to in the
encyclopedia (`caps` stage):

| prime | bits | v₂(p−1) | β_Kummer | v₂(p+1) | β_isogeny (supersingular `y²=x³+x`) |
|:--|---:|---:|---:|---:|---:|
| P-192 | 192 | 1 | 0.005 | 64 | 0.333 |
| P-224 | 224 | 96 | **0.429** | 1 | 0.004 |
| P-256 | 256 | 1 | 0.004 | 96 | 0.375 |
| P-384 | 384 | 1 | 0.003 | 32 | 0.083 |
| P-521 | 521 | 1 | 0.002 | 521 | 1.000 (Mersenne: p+1 = 2^521) |
| secp256k1 | 256 | 1 | 0.004 | 4 | 0.016 |

P-224's Kummer base is capped at `2^96 = p^0.4286` — **below p^(1/2)**, so on the
one NIST prime whose 2-adicity is the whole point of the Kummer tower, the base
*can never reach the size the random model asks for*. That is an independent,
prime-specific closure of the Kummer route.

---

## 2. The identity that decides the question

### 2.1 Derivation

Let `B = |V|`, `nF = |F|` (points of `E` whose abscissa is in V; about half of V
lifts, so `nF ≈ B`), `N = #E`. One enumerative join pass over a target T costs
`nF` probes and finds `nF²/N` ordered hits, i.e. `nF²/(2N)` relations.

A square system needs `B` relations (one per column). Hence

```
probes = B · (2N/nF)            # "ops per relation" × "relations needed"
       ≈ 2N       when nF ≈ B   # independent of B
```

**The relation phase of m = 2 index calculus is Θ(N) = Θ(p), for every base size
and every join that costs one probe per base element.** Increasing B buys
nothing: the number of relations you must collect grows exactly as fast as the
relation density falls.

### 2.2 Measured (`join` stage, 35 configurations, 3 primes, 3 towers, t = 3…8)

`ops/rel = 2N/|F|` predicted, measured median ratio **1.005** [0.72, 1.59] over
35 rows. Total probes to collect |V| relations, directly simulated
(`join_direct`, probe-until-found): median **2.57·p** [1.64, 3.43].

Selected rows (full table in `run_all.log`, §join):

| p | tower | t | &#124;V&#124; | &#124;F&#124; | hits/pass | ops/rel | pred 2N/&#124;F&#124; | tot/p | meas/pred |
|---:|:--|---:|---:|---:|---:|---:|---:|---:|---:|
| 1009 | interval | 3 | 8 | 12 | 0.142 | 168.4 | 168.2 | 1.34 | 1.00 |
| 1009 | interval | 6 | 64 | 72 | 4.965 | 29.0 | 28.0 | 1.84 | 1.03 |
| 12289 | kummer | 8 | 256 | 250 | 4.790 | 104.4 | 99.4 | 2.17 | 1.05 |
| 40961 | interval | 5 | 32 | 28 | 0.020 | 2800 | 2930 | 2.19 | 0.96 |
| 40961 | kummer | 8 | 256 | 244 | 1.433 | 340.7 | 336.3 | 2.13 | 1.01 |
| 40961 | isogeny | 8 | 256 | 242 | 1.363 | 355.2 | 339.0 | 2.22 | 1.05 |

Flatness across a 32× range of |V| (median tot/p per |V| bucket):

| &#124;V&#124; ≈ | 8 | 16 | 32 | 64 | 128 | 256 |
|:--|---:|---:|---:|---:|---:|---:|
| median tot/p | 2.25 | 1.97 | 2.36 | 2.22 | 2.00 | 2.15 |

(`tot/p ≈ 2` rather than 1 because only ~half of V lifts to E, so nF ≈ B; the
model value is `2N·B/nF`.)

### 2.3 Why "|V| > p^(1/2)" cannot help

* Raising B does not change the total (§2.1) — it is flat, measured.
* For any join with `C = B^θ`, θ ≥ 0, the total `B^{1+θ} + N·B^{θ−2} + B` is
  *increasing* in B once `B > N^{1/3}` (the balance point of §5). So |V| above
  p^(1/2) is strictly dominated by |V| ≈ p^(1/3), for every join that exists.
* Both towers *build* V in Θ(|V|) operations (§1). At |V| > p^(1/2) the base
  alone costs ~B ≈ 0.8·(rho's 1.25√p) before a single relation is found. The
  interval base is the only free one (implicit description), and its structure is
  destroyed by the lift (§4), so it is enumerated like any other set.
* Pure m = 2 has a second, independent defect: each relation has exactly two
  nonzeros, so the relation matrix is an unsigned graph incidence matrix and
  `rank = n − #(bipartite components)`; a bipartite component leaves a ±1 sign
  kernel, so those logs are fixed only up to sign — an extra relation with an odd
  cycle (≥ 3 summands) is needed. *(Corrected in Part B §B.4: the kernel vector is
  the bipartition sign vector, not `1ᵀ`; measured 200/200.)*

---

## 3. Three structured joins, each costed (`structured` stage, p = 40961)

| join | idea | measured cost | vs hash |
|:--|:--|:--|:--|
| (a) multiplicative membership | `x ∈ g·μ_{2^t} ⟺ x^(2^t) = g^(2^t)` | 0.30 µs/test, t=7, |V|=128 | **14× slower** than a dict hit |
| (b) Vélu tower membership | `L_t∘…∘L_1(x) = c` (exact: agrees with the set on 100.0 % of 2000 draws) | 4–27 µs/test, t=6 | **188×–1320× slower** |
| (c) sumset table | store all `P1+P2` of F (½|F|² group ops), then one lookup per target | see below | **wins** |

(a) and (b) are the two ways of *using the tower structure* in the join, and both
lose to hashing the same set by 1–3 orders of magnitude. The tower's algebraic
description of V is a cost, not a saving: at |V| = 2^t the membership test needs
t squarings or t field inversions, while the alternative is one array probe.

(c) is the only join that beats the enumerative pass, and by a full exponent
step:

| p | best t | &#124;V&#124; | &#124;F&#124; | enumerative total (&#124 V&#124;·2N/&#124 F&#124;) | sumset-table total | gain |
|---:|---:|---:|---:|---:|---:|---:|
| 1009 | 3 | 8 | 12 | 1345 | **190** | 7.1× |
| 12289 | 5 | 32 | 26 | 30575 | **1527** | 20.0× |
| 40961 | 6 | 64 | 54 | 97240 | **3286** | 29.6× |

(*Correction, same day, after the m = 3 addendum: an earlier version of this
table divided by the ordered hit count instead of by the number of distinct
relations, which is half of it. The totals above are the corrected ones; the
exponent is unaffected, the constant and the gains are 1.9× smaller.*)

(the enumerative column is the measured law `|V|·2N/|F|` of §2; the flat `≈2p`
of §2.2 is its size-independent value at |F| ≈ |V|.)

Fitted exponent of the sumset-table total over these three primes: **0.769**
(asymptote 2/3: total = ½|F|² + 2B·N/|F|² ⟹ `B³ = 2N`, total ≈ 2.4·N^(2/3)).
At p = 40961 the best total is 3286 ≈ **16.2·√p** — sixteen times rho, not below it.

This is the one actionable item: the current pipeline's join is the enumerative
`rootfind-subtract` pass (cost N); a sumset/pair table costs N^(2/3). It does
**not** reach rho (2/3 > 1/2) and it needs N^(2/3) memory, but it is a measured
7–30× over the three toy sizes, growing as p^(1/3).

---

## 4. The structural reason: the join lives in E, the structure lives in F_p

V is a set of *field elements*. The join is a statement about *points of E*. The
lift `x ↦ {±P}` is a 2-valued, non-homomorphic map (it is `y = ±√(x³+ax+b)`), so
any additive structure V has in F_p is destroyed on the way to E. Measured
(`geom` stage, control = same-size negation-closed random base):

| p | tower | t | &#124;V&#124; | &#124;V+V&#124;ₓ / random | &#124;V·V&#124;ₓ / random | AP3 z | &#124;F+F&#124; / control | E₂ / control |
|---:|:--|---:|---:|---:|---:|---:|---:|---:|
| 1009 | interval | 6 | 64 | **0.126** | 0.83 | **+109 σ** | 1.02 | 0.93 |
| 12289 | interval | 6 | 64 | **0.062** | 0.62 | **+432 σ** | 0.99 | 1.01 |
| 12289 | kummer | 6 | 64 | 1.000 | **0.031** | −4.6 σ | 1.02 | 0.97 |
| 12289 | isogeny | 6 | 64 | 0.939 | 0.94 | +2.7 σ | 0.99 | 1.02 |
| 1009 | isogeny | 6 | 64 | 0.898 | 0.88 | −1.4 σ | 1.12 | 0.78 |

Three readings, all measured:

1. **The interval base has maximal additive structure at the V level** —
   `|V+V|ₓ = 2|V|−1` (ratio 0.06–0.24 of the random value), AP count 100+ σ above
   random — and **none of it survives**: at the E level its sumset size and
   additive energy are within a few percent of the random control.
2. **The Kummer base has maximal multiplicative structure** and no additive
   structure: `|V·V| = |V|` exactly (ratio 0.03–0.13), AP z ≤ 0 (a multiplicative
   coset has *fewer* additive coincidences than a random set). The join needs
   addition, so it has the structure the join cannot use and lacks the structure
   it needs.
3. **The isogeny base is additively and multiplicatively random** at both levels;
   across all 16 configurations the E-level energy ratio is median 0.99
   [0.78, 1.14] and `|F+F|` is within 2 % of the control. It is also *not* a
   coset of a subgroup of E: `|F+F| / (3|F|)` has median 4.1 (a coset would give
   ≤ 3). The auxiliary-curve group ⟨Q⟩ does not transport to E — it is only a
   recipe for the numbers in V.

Consequence: `∑_c r(c)²` is at the random value, so no additive-combinatorial
(Fourier/Walsh/cyclotomic) aggregation can compress the sumset. The only
exploitable structure would be *in E*, and V is not a subgroup of E.

---

## 5. Lower bound, and exactly what would have to be added

**Model.** One join pass must test `nF` candidate pairs (that is what testing
"P1 + P2 = T for all P1 ∈ F" means), so a pass is Ω(nF) group operations. This
is the claw problem between the lists `F` and `T − F` in a group of size N; in
the generic-group model with memory m the van Oorschot–Wiener tradeoff gives
`time · m ≥ nF²`, so time ≥ nF at m = nF. The sumset table is the same bound
taken at the memory extreme (m = nF², time 1 per target) — and paying for the
table costs nF². There is no third option below both.

**Escape hatches, and why neither tower has one.** A sub-n join needs a *batch*
predicate: decide `∃ x ∈ V : φ(x) ∈ V` (φ = the degree-2 correspondence
`x ↦ x(Q−P)`) in o(n). The standard tools that do this for a *pure sum*
`x₁ + x₂ = c` are cyclotomic numbers / Gauss periods (for V = μ_n) and the
Walsh–Hadamard transform (for V an additive F₂-subspace). Both require the join
predicate to be the ambient group operation restricted to a subgroup:

* for V = g·μ_n the predicate is not `x₁+x₂ = c` but a degree-6 symmetric
  equation `G(x₁,x₂) = 0` (the two y-sign branches), so the cyclotomic algebra
  does not apply; the (s, q) = (x₁+x₂, x₁x₂) reduction just re-parameterises the
  same n-element set, and the resultant route costs Θ(n) with a *larger* constant;
* `F_p⁺` has no nontrivial additive subgroups, so the Walsh–Hadamard route that
  exists for binary Weil descent (where the x-line is F₂-linear and XOR *is* the
  group law on the x-line) has no prime-field analogue. This is the sharp reading
  of "PKM's structure helps an algebraic solver, not a table";
* for the interval arm, the natural attack is bivariate small roots on
  `G(x₁,x₂)=0`, `x₁,x₂ ≤ B`: the Howgrave-Graham heuristic bound for total
  degree δ = 6 is `x₁x₂ ≲ p^{2/(3δ)} = p^{1/9}`, far below the p^1 that a
  constant-probability relation needs. (Arithmetic, not a measurement.)

**Precise criterion.** Let `C` be the cost of producing one relation. The
sumset table achieves `C = B` (§3c); the enumerative per-target pass achieves
`C = N/B ≈ p/B`, i.e. the Θ(p) total of §2. Writing `C = B^θ` (θ = 1 for the
table), the m = 2 total is

```
total = B·C + C·N/B² + B  =  B^{1+θ} + N·B^{θ−2} + B ,
```

balanced at B = N^{1/3}, giving `total = N^{(1+θ)/3}` (for θ ≥ 0). Hence

* θ = 1 ⟹ p^(2/3) — measured, 0.75 at toy sizes;
* θ = 1/2 ⟹ p^(1/2) — merely *matches* rho;
* θ = 0 ⟹ p^(1/3).

So Route 2 needs a join with **θ < 1/2**, i.e. a per-relation cost below √B,
and it needs it *in E*. Nothing measured or known supplies that over F_p.

---

## 6. Consequences for the programme

1. **Route 2 is closed for m = 2.** The binding quantity is not |V| and not the
   per-pass join; it is the product (relations needed) × (pairs per relation),
   which is Θ(p) for the enumerative join and Θ(p^(2/3)) for the sumset table.
   Neither is below rho, and no V can change that because the join obligation
   scales with B exactly like the relation supply.
2. **The 32× / 7.4× F4 lead does not interact with this.** The tower *is* a
   membership oracle for V, and measured it is 14× (Kummer) to 188–1320×
   (isogeny, per membership) *slower* than a hash of the same V. Where the tower
   wins (F4 per-system multiply-adds) it is competing with another algebraic
   solver, not with the enumerative join.
3. **One cheap, real improvement exists**: replacing the enumerative m = 2 join
   with a sumset/pair table moves the relation phase from ~2N to ~N^(2/3)
   (measured 7.1×/20×/30× at p = 1009/12289/40961; exponent 0.77 measured,
   2/3 asymptotic). It costs N^(2/3) memory and does not reach rho.
4. **Where the join question still pays**: at m ≥ 3, where a *free* join lowers
   the base exponent (B = p^(1/m)) and the join cost enters as B^(m−1) rather
   than as N/B. That is the only regime in which "the join is the bottleneck"
   is a true statement. It belongs to Route 3.

## 7. Next concrete step

Hand the m = 2 verdict to the synthesis and re-aim this route at the m = 3 join.
`(1+θ)/3` is an m = 2 result; the transferable instrument is the measurement
itself — hits per pass, ops per relation, total for `B` relations, and the
E-level energy against a matched control. Run it on the m = 3 join (two-sum table
plus a scan, and the `pkm_semaev` per-system route) at p ≈ 2^13 and B = p^(1/3),
and report which term dominates: the join (B² = p^(2/3) per target) or the F4
per-system cost. That number, not this one, decides whether a structured join is
worth pursuing at m ≥ 3.

## 8. Claim boundary

Toy instances only, p ≤ 40961, one prime-order curve per prime (first hit of a
deterministic seed), all timing in pure Python (the *op counts* are exact; the µs
figures are Python-level and only their ratios are meaningful). No NIST-prime
run is made; §1's NIST rows are 2-adic arithmetic on the published primes. The
Θ(p) identity in §2.1 assumes a square system with B columns and m = 2 relations
— the standard accounting. Energy/sumset diagnostics are computed for |F| ≤ 140
with 3 matched controls each. No claim about a deployed curve, and no T2/T3 claim.

---

# Part B — the m = 3 version of the same instrument (2026-11-25, same day)

Asked for: run the m = 3 join (Q = P1+P2+P3) at p ≈ 2^13 with B ≈ p^(1/3), report
hits/pass, ops/relation, the total to collect B relations, the E-level structure
against a matched control, **both** cost terms (join vs per-system), the rank
defect, and which term dominates. Code: `route2_m3.py` (imports the Part A
instrument), store `route2_m3_results.json`, log `run_m3.log`, relation records
`route2_m3_relations.jsonl`. Wall 4.6 s.

## B.0 Normalisation

The columns of the relation matrix are the **points of F**, so the base size in
the random model |F|^m/p is |F|, not |V| (about half of V lifts). At p = 12289,
#E = 12421, **p^(1/3) = 23.16**; the interval arm gives |V| = 28, **|F| = 22 =
0.95·p^(1/3)**. Tower arms give |V| = 2^t: |F| = 12 (0.52·) and 20 (0.86·) at
t = 4, and |F| = 30 (1.30·) at t = 5. `B` below means |F|.

`route2_m3_relations.jsonl` holds, in addition to the rank series, **14 real
3-sum relations found on the curve with p, a, b, the three points and the target**
— each carries `verified: true` from the in-run group check, and each can be
re-checked from the record alone (done once independently).

Relation counting matters: a triple of distinct points has 6 ordered triples, 3
(unordered-pair, third) table hits, and **1** relation. `threesum_census`
enumerates all |F|^3 ordered triples and all C(|F|+2,3) multisets, so
relations/target = **C(|F|+2,3)/N exactly** (0.163 measured at |F| = 22, i.e.
2024/12421 — verified against the census, not a model).

## B.1 The join, measured at B = p^(1/3) (`m3_join`, `m3_terms`)

| p | tower | t | \|V\| | \|F\| | \|F+F\| | \|F+F+F\| | rel/target | ops/rel | total(j2) | total(j1) | E₃/ctl |
|---:|:--|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 12289 | interval | — | 28 | 22 | 243 | 1702 | 0.163 | 135.0 | **3223** | 34158 | 0.97 |
| 12289 | kummer | 4 | 16 | 12 | 73 | 286 | 0.029 | 409.5 | 4992 | 31940 | 1.03 |
| 12289 | isogeny | 4 | 16 | 20 | 201 | 1336 | 0.124 | 161.3 | 3436 | 33875 | 0.93 |
| 12289 | kummer | 5 | 32 | 30 | 451 | 4043 | 0.399 | 75.1 | 2719 | 34934 | 0.81 |
| 12289 | isogeny | 5 | 32 | 30 | 445 | 3864 | 0.399 | 75.1 | 2719 | 34934 | 0.99 |
| 16381 | interval | — | 22 | 24 | 289 | 2144 | 0.157 | 152.6 | 3962 | 45773 | 1.01 |

* hits/pass: measured vs the exact value (½·|F|(|F|+1)·|F|/N) agree to
  0.89–1.14 across the six configs (column `meas×` in `run_m3.log`);
* **3-sum coverage is only |F+F+F|/N = 1702/12421 = 13.7 %** at |F| = 22 — at
  B = p^(1/3) a random target is decomposable with probability 0.14, at 0.86
  representations on average;
* E-level 3-sum energy / matched control = 0.81–1.03 (median 0.98): the base is
  additively generic for m = 3 exactly as it was for m = 2 (Part A §4);
* ops/relation and totals follow the closed forms `rel = C(|F|+2,3)/N`,
  `j1 = (|F|/rel)·|F|(|F|+1)/2`, `j2 = |F|(|F|+1)/2 + (|F|/rel)·|F|`.

## B.2 Both terms, and which dominates

At p = 12289, |F| = 22: relations/target 0.163 ⇒ **targets (= algebraic
systems) = 135**.

| term | operations | in p | vs rho (139) |
|:--|---:|---:|---:|
| join A: pair table (253 build) + scan (2970) | **3223** | 6.0 p^(2/3) | 23.1× |
| join B: algebraic, **Route 1 target** C = 2^T = 22 | **2970** | 5.5 p^(2/3) | 21.3× |
| join B: algebraic, **measured F4** 1.34e10 MA/call | **1.8e12** | p^(2.99) | 1.3e10× |
| join C: random triple + target list, |F| = 22 | **2159** | 4.0 p^(2/3) | 15.5× |
| best m=3 join overall (join C at |F| = 11) | **764** | 1.4 p^(2/3) | 5.5× |

* **The measured F4 term dominates by 2.4 × 10⁹** over join A. The F4 per-system
  figure is 127.4 M MA at N_var = 10, scaled by the measured 15.72× per +2 tower
  variables to N_var = 3·log2|F| = 13.4 → 1.34e10 MA per call. (Cross-check: the
  system size N_var = m·T ≈ log2 p for **every** m, which is exactly why the F4
  exponent is m-independent at p^{2.6} — Part A's synthesis table, now with the
  m = 3 point measured.)
* **If Route 1 delivers Θ(2^T) = 22 per system, the two terms are the same
  object**: 2970 vs 3223 vs 2159. The algebraic route's "scan" *is* the pair
  table's scan; there is no separate win. This is the quantitative form of
  "at m = 3 the solver and the join converge".
* Implied total exponent: **2/3 for both terms** (Route 1 realised), **2.99** with
  the measured F4. The single-point exponents ln(ops)/ln p (0.857 / 0.848 /
  2.994) mix constant and exponent; the law is c·p^(2/3) with c = 6.0 (A), 5.5
  (B), 4.0 (C at B = p^(1/3)), 1.4–2.8 (C at its own optimum).

## B.3 Does the m = 3 join improve the exponent? (`m3_strategies`)

Exact closed forms (validated against the census above), minima over |F| = 4…64:

| p | j1 (enumerative) | j2 (pair table) | j3 (random triple) |
|---:|---:|---:|---:|
| 1009 | 2018 @ \|F\|=4 | 453 @ 16 | **164 @ 6** |
| 4099 | 8182 @ 4 | 1195 @ 26 | **393 @ 8** |
| 12289 | 24842 @ 4 | 2551 @ 40 | **773 @ 12** |
| fitted exponent | 1.004 | 0.692 | **0.620** |
| law | 3p | 4.95 p^(2/3) | 2.8 p^(3/5) |

The model projection of the same closed forms (N = 10³…10¹², not measured) gives
**0.603** for j3, i.e. 3/5. So:

* the pair table gives **2/3 for every m** (m = 2: 2.4 p^(2/3); m = 3: 4.95 p^(2/3))
  — the extra summand costs a scan of |F| per target, which exactly cancels the
  extra dimension;
* the memory-free random-tuple join gives **m/(2m-1)**: 2/3, 3/5, 4/7, 5/9, …
  → 1/2 from above. m = 3 measured at 0.62;
* **neither reaches rho.** m = 3 buys a 0.15 reduction in fitted exponent over
  m = 2's table (0.62 vs 0.77 measured) and still sits 5.5–23× rho at p = 2^13.

`m2_m3` at equal base: the m = 3 table join is 2.03–2.39× *more* expensive than
the m = 2 table join at the same |F| (the scan). m = 3 only wins when it is
allowed to shrink its base and use join C.

## B.4 The rank defect — and a correction to Part A §2.3

Part A said the m = 2 relation matrix has `1ᵀ` in its left null space. **That is
wrong**, and the measurement says so: every m = 2 row is `e_i + e_j`, whose
entries sum to 2, so `1ᵀ·row = 2 ≠ 0` — `1ᵀ` is in no left kernel (checked
200/200). The truth is the graph statement:

```
m = 2 : row = e_i + e_j = the unsigned incidence matrix of a graph on the
        |F| columns.  rank = n − #(bipartite connected components),
        isolated vertices counting as trivial bipartite components.
        The kernel vector is the ±1 bipartition sign vector.
```

Measured (n = 24, 24 relations, 200 trials): `rank = n − #bip comps` in
**200/200**; 111/200 have a *non-trivial* bipartite component, and on each one
the ±1 sign vector is in the left kernel in 111/111. So the m = 2 defect is real
but it is a sign ambiguity on a bipartite component, not a global `1ᵀ`; it is
cured by one relation with an odd cycle (an m = 3 relation, or a base point with
known log).

**m = 3 removes it.** A row is a 3-multiset: three ±1 weights cannot sum to 0
(sums are ±3, ±1), and a row `2e_i + e_k` forces `w_i = w_k = 0`. The 3-uniform
hypergraph incidence matrix is therefore generically full rank — measured full
rank in **40/40 trials at every n ∈ {16, 24, 32, 48}**.

How many relations per column are needed (`rank_need`, 40 trials each):

| \|F\| | m=2: R_full | R/n | m=3: R_full | R/n | coupon (n/m)(ln n+γ) |
|---:|---:|---:|---:|---:|---:|
| 16 | 27.2 | 1.70 | 20.2 | **1.26** | 26.8 / 17.9 |
| 24 | 41.8 | 1.74 | 34.9 | **1.45** | 45.1 / 30.0 |
| 32 | 65.8 | 2.05 | 46.6 | **1.46** | 64.7 / 43.1 |
| 48 | 96.8 | 2.02 | 77.0 | **1.60** | 106.8 / 71.2 |

m = 3 needs **1.26–1.60 relations per column** against m = 2's **1.70–2.05** — a
25 % reduction, because each relation covers 3 columns instead of 2 (coupon
collector, (n/m)(ln n + γ)). This is a real but small constant win; it does not
change any exponent.

## B.5 Verdict on the m = 3 question

1. **At B = p^(1/3) the join is ~4–6 p^(2/3)** (2159–3223 ops, 15–23× rho at
   p = 2^13); the per-system F4 term is **1.8e12** (p^(2.99), 1.3e10× rho) and
   dominates by nine orders of magnitude.
2. **If Route 1 lands, the two terms coincide** (2970 vs 2159–3223): the
   algebraic solve and the pair-table scan are the same |F| per target. The
   bottleneck then moves to the p^(2/3) join, which m = 3 cannot remove.
3. **m = 3 does not reach rho by any measured join**: best 0.62 (random triple,
   law 3/5) vs rho's 0.50. m = 4 projects to 4/7 = 0.57, m = 5 to 5/9, → 0.50
   from above; the memory-free ladder approaches rho and never beats it.
4. The two things m = 3 genuinely improves are (i) the exponent over the m = 2
   table (0.62 vs 0.77 measured) and (ii) relations per column (1.3–1.6 n vs
   1.7–2.1 n), plus it removes the m = 2 sign defect.

## B.6 Claim boundary (Part B)

Toy instances, p = 12289 and 16381, one prime-order curve each, |F| = 12–30.
Join costs are **exact operation counts** from the closed forms validated by the
|F|^3 census; the "measured" hit-rate column is a 1500-hit sample (ratios
0.89–1.14). The F4 per-system figure is *extrapolated* from the recorded 127.4 M
MA at N_var = 10 with the measured 15.72× per +2 variables — it is a model
number, not a run at m = 3. The rank study uses random relation rows (uniform
over multisets), which is the correct model for uniformly random targets, and 40
trials per cell. No NIST-prime run; no T2/T3 claim.
