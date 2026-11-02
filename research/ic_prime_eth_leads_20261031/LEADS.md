# Prime-field ECDLP index calculus after the "ETH is false" proof: review and twelve measurable leads

Date: 2026-10-31. Author: Claude (session on SSD990-2). Scope: the five repositories the owner named
(`crypto`, `cryptanalysis`, `ml-cryptanalysis`, `crypto-autoresearcher`; `cryptanalysis-rl` is not on
this disk and is named only as a sibling in `crypto-autoresearcher/docs/experiment-protocol.md`), the
OpenAI preprint *The Exponential Time Hypothesis is False* (17 Oct 2026, revised 25 Oct; text read in
full), the Bochum/MPI preprint *A Subexponential Time Algorithm for Elliptic Curve Discrete Logarithm*
v0.2 (29 Oct 2026; read in full from `/Volumes/SSD990/downloads/subexponential-ecdl.pdf`), the public
discussion as of today, and the lab's own records (atlas, leaderboards, the 2026-10-24 regime review,
the 2026-10-31 `prime_sat_*` review, the 2026-10-31 prime-PDP SAT sweep). Four delegated Sonnet audits
(ETH proof structure, Bochum cost chain, Lean toolchain plus atlas collisions, literature) fed this note;
their reports are summarised, and every number below names its source. Nothing here was measured; the
leads are written to be measured. Lean files in `lean/` compile (axioms: `propext`, `Classical.choice`,
`Quot.sound` only; no `sorry`, no `native_decide`).

Label convention, as in the lab: **measured** (quoted from a committed file), **derived** (arithmetic
from quoted values, or a Lean theorem here), **plan**.

## 0. One-paragraph verdict

The "ETH false proof" is OpenAI's deterministic #k-SAT algorithm; its sharper bound rests on a Boolean
tensor-contraction theorem (Appendix D, Theorem D.1) whose only exponential term is 2^{O(s/√log s)} in the
number of bit-edges s, with vertices treated as black boxes. The Bochum paper uses that theorem as an oracle
for relation finding with hashed factor bases and obtains 2^{O(b/(log b)^{1/4})} for prime-field ECDLP,
conditional on D.1. No bug is public in either proof (Impagliazzo–Paturi–Williams are examining; the Lean
statements are weaker than the papers). Both are galactic: D.1's recursion is inert below s ≈ 2^{2^85}
edges, and even if D.1's constant were 1, the Bochum route is below Pollard rho only for groups of more
than 2^256 **bits** (Lean theorem `no_crossover`). The repos already implement the Bochum algorithm, have
read D.1/D.4 twice without finding a gap, and have measured every SAT, Gröbner, lattice and lifting route
on prime fields as enumeration-rate or worse. What is new and measurable is narrower than "subexponential
in practice": (i) a characteristic-2 Fourier gadget that gives the same linear-size network for binary
curves and, as a SAT encoding, turns field multiplication into XOR layers plus small products (exactly
what WDSat's Gaussian elimination exploits); (ii) sparse-parity (expander-code) hashed factor bases whose
membership is pure XOR; (iii) a strong-backdoor census that decides, cheaply and for good, whether any
propagation-based solver can see structure in prime-field PDP; (iv) Lean-certified exponent rules that
make every ladder's go/no-go decision mechanical. Twelve leads follow, each with instances, metric,
baseline, decision rule and cost.

## 1. The ETH-false proof: what it is, what it depends on, where it is thin

Sources: `scratchpad/dl/eth.txt` (pdftotext of the 25 Oct PDF, 3,815 lines; line numbers below refer to
it), the Sonnet proof-structure audit, and the literature pull.

**Claim (Theorem 1.1, l. 56–67).** For fixed k ≥ 2, exact #k-CNF on n ≥ 20 variables in
(n+m)^{O_k(1)} · 2^{O_k(n log log n / √log n)} bit operations and polynomial space; for m ≤ Dn the exponent is
O_{k,D}(n/√log n). The main text's rational-interpolation route gives only n(log log n)²/√log n; both
sharper bounds come from Appendix D by tensor contraction (l. 67).

**Architecture (main text).** Counting-preserving sparsification to bounded-occurrence 3-CNF (each variable
in ≤ 3 clauses, so the clause-product polynomial has degree ≤ 3n); each variable occurrence replaced by a
linear form ⟨P_{e,v}(T), X⟩ of degree ≤ R in an interpolation parameter T; variables split into r groups
so that, once O(τn) shared integer "measurements" y_g = C_g x_g are fixed, the cube sum factorises into a
product of per-group sums; Taylor data of H(t+z) at Q points by inclusion–exclusion; Hermite interpolation
recovers H(0) = #F; derandomised by conditional expectations. Parameters: λ = ⌈log₂ n⌉,
r = ⌊√λ/ν²⌋, Q = ⌊λ/(128ν)⌋, R = 4⌈√λ⌉, D = r^{2Q} ≤ (2n)^{1/128} (l. 642–649, 947–957). The
binding "sufficiently large n" condition 6R ≤ Q needs λ ≳ 10^{10} (Sonnet audit, derived).

**Appendix D, the part that matters here.** A Boolean tensor network is a bipartite multigraph; each edge
carries one bit; each vertex v has a tensor T_v : {0,1}^{δ(v)} → ring; the contraction is
Z = Σ_x Π_v T_v(x|δ(v)) (l. 3008–3017). **Theorem D.1 (l. 3018–3029):** with at most Cs edges, maximum
degree ≤ ⌈s^{9/10}⌉, polynomially many vertices, entries in (Z/2^{P₀})[V]/(V^ℓ) with P₀, ℓ ≤ Cs, and each
entry computable from its description in time (s+L)^b · 2^{A s^{9/10}}, the contraction is computed
deterministically in (s+L)^{O(1)} · 2^{O(s/√log s)} time and polynomial space. The proof (D.4) recurses on
wires: pack edges into packets (Lemma D.3, needs the degree cap for the variance bound), deform wire
identifications by a Cauchy kernel W(T) of degree 2L−1 (D.4.3), recover F(0) by Hermite interpolation
with multiplicities tapered by the 2-adic valuation of the node (Lemma D.5), expand cross-group
interactions into "modes" of 2-adic weight (D.4.5), and recurse on child networks with ≤ 3M/32 edges
(D.4.6). Parameters (l. 3295–3311): H = s/⌊√λ⌋ (leaf size, summed directly), r = 64, ε = 2^{−17},
u_d = ελ/2^d, L = 2^a, Δ = ⌈s^{9/10}⌉, and the standing requirement u ≥ λ^{4/5} (D.5).

**Four facts the leads below rely on (all read from the text).**

1. The exponential cost depends on the edge count s only (l. 3612–3632). Vertex count, description length,
   degree and entry cost enter polynomially or through the dominated 2^{O(s^{9/10})} entry term.
2. Vertices are black boxes. Retained vertices are evaluated from immutable descriptions
   (l. 3595–3600); the only new tensors are generic packet tensors. **A linear or low-rank vertex gets no
   discount inside D.1.** (Lead 2 is therefore a SAT-side idea, not a D.1 improvement.)
3. Edges are bits. The ring is the value ring, not the edge alphabet; q-ary edges are not treated and the
   machinery is 2-adic (valuations, F_{2^e} residue field, l₀ = 1−z, l₁ = z). A p-ary digit network would
   need a p-adic analogue that the paper does not supply.
4. The thresholds are not practical. u₀ = λ/2^{17} with u₀ ≥ λ^{4/5} needs λ ≥ 2^{85}, i.e. s ≥ 2^{2^{85}}
   edges, at depth 0 alone (Sonnet audit, derived; the Bochum paper quotes the weaker 2^{2^{17}}). Below the
   admission threshold the algorithm is brute force 2^s. Even inside the recursion the leaf alone costs
   2^{s/⌊√λ⌋}: a factor ⌊√λ⌋ ≤ 5 in the exponent for any s below 2^{36}.

**Where a referee would push (Sonnet audit; I re-read the cited lines).** (a) Precision bookkeeping across
the recursion: canonical lifting "need not preserve addition or multiplication" (l. 3281–3282), so
correctness rests on creation-precision tagging and p_t = O(M D_t/u) staying polynomial; (b) the mode
weight divisibility and truncation that make identity (D.16) exact (l. 3509–3536); (c) the packet/degree
bookkeeping that keeps children at ≤ M/8 edges and degree ≤ Δ; (d) the tapered Hermite budget, which is
the only source of the 1/√u factor; (e) in D.3 the wire encoding of monomials. None of these is an error
that anyone has exhibited; the repos' two readings (`crypto` `research/prime_one_shot_20261029/continuation/tensor-review/{contraction-cost,contraction-precision}.md`, main) re-derived packet shrink, degree
invariants, interpolation counts and valuations and "found no concrete gap". The Lean development in
`openai/math` proves a weaker rate (n(log log n)²/√log n) and does not contain Theorem D.1
(complexitylib issue #1538, 2026-10-30); third-party Comparator replays accept both OpenAI's and
Anthropic's Lean files with standard axioms but nobody has machine-checked that the formal statements
match the papers (ethdiscussion.blog, 28 and 31 Oct). Kintali's simplification (26 Oct) proves only the
2^{εn} family without tensor contraction; O'Donnell's exposition (30 Oct) recasts the main route as a
sparse-permanent interpolation. Fortnow: "safe to assume ETH is toast", unverified. Status: **claimed,
under examination, no reported bug**.

## 2. The Bochum reduction, audited

Sources: `scratchpad/dl/bochum.txt`, the Sonnet cost audit, my own reading of pp. 1–10.

- **Encodings.** Ripple adders, comparisons, zero tests, gated equality, copying: O(d) edges, constant
  degree, parsimonious (Lemma 2). Integer multiplication (Theorem 3): one split of a Schönhage–Strassen
  transform of length L = L₁L₂ ≈ √J over Z/(2^J+1) with ω = 2^{2J/L}; fewer than 1100(d+1) edges, degree
  below 136(d+1)^{3/4}; parsimonious because every word is kept in canonical form. Modular arithmetic by
  quotient guessing (Lemma 8); curve membership (six constraints); complete point addition with five
  mutually exclusive selectors (Corollary 2); hashed membership: enc(A) = 2x + (y mod 2), prime s with
  2p ≤ s ≤ 4p, H_i(z) = (α_i z + β_i) mod s, F_i = {A : H_i(enc(A)) < τ}. The relation network
  "A_i ∈ F_i, A_1+…+A_m = R" has O(mb) edges and degree O(b^{3/4}) (Lemma 11).
- **Coverage (Lemma 10).** With L = (r−1)τ/s ≥ max{r^{1/m}, 10m}, for every target R,
  Pr_seed[R decomposes] ≥ 2/5, by a second moment that uses only pairwise independence of the affine hash.
  Lemma 13 converts fixed-target/random-seed into fixed-seed/random-target. Audit: moments and arithmetic
  re-derived, no error.
- **Extraction (Lemma 12).** Targets R = [u]P + [v]Q with fresh (u,v); the row dependency gives
  [U]P + [V]Q = O with V uniform, so ξ = −U/V unless V = 0. Sound.
- **Cost (Theorem 4).** N = Σ|F_i| = O(m r^{1/m} + m²) relations, each costing one prefix search of
  O(mb) contractions at 2^{O(mb/√log b)}; dense Gaussian elimination O(m³ r^{3/m} b²); total
  2^{O(b/m + mb/√log b)}; m = ⌈(log b)^{1/4}⌉ gives 2^{O(b/(log b)^{1/4})}. Remark 2: a contractor at
  2^{O(ℓ/(log ℓ)^α)} would give 2^{O(b/(log b)^{α/2})}.
- **What is odd-characteristic-specific** (needed for Lead 1): the Fermat ring Z/(2^J+1) and ω = 2^{2J/L}
  (L a power of two is a unit only because 2^J+1 is odd), the integer carry chain, canonical residues,
  the parity compression enc, Tonelli–Shanks in enumeration, p > 3 short Weierstrass formulas.
- **Concrete sizes (derived).** At b = 256, m = ⌈(log b)^{1/4}⌉ = 2, so the linear algebra alone is
  r^{3/2} ≈ 2^{384} > rho's 2^{128}; beating rho on linear algebra needs m > 6, which Theorem 4's own balance
  reaches only for b above about 2^{1296}. By the paper's own gadget counts the network is at least
  ~10^5 b edges per point addition.

**Lean (this note, `lean/IcTensorRoute.lean`, derived).** `blackbox_balance`: for every arity m the total
exponent satisfies E(m)² ≥ 4cb²/g with g = √log₂ b, so the Bochum m is optimal up to the constant and no
black-box use of D.1 does better than 2b√(c/g). `no_crossover`: if E(m) < b/2 (below rho) with m ≥ 3 then
g > 16c; with c ≥ 1 this is log₂ b > 256 (`crossover_bits`): the group order would need more than 2^{256}
bits. `arity_two_blackbox`: m = 2 is never below rho. Conclusion: **the Bochum route can never beat rho on
any curve anyone will deploy, whatever D.1's constant is, unless the contractor's exponent improves from
s/√log s to something like s/(log s)^{α} with α much larger** (Remark 2's lever), or the relation network
needs o(mb) edges (Section 4).

## 3. What the five repositories already did (so nothing below repeats it)

Sources: lead atlas (`crypto-autoresearcher/knowledge/leads/ecdlp/`, 154 leads, reviewed 2026-10-30,
79 paths changed since), the three Explore sweeps, `ic-regime-review-20261024/REVIEW.md`,
`prime-sat-tensor-review-20261031/REVIEW.md`, memory notes.

**The tensor route itself.** Native Rust implementation of the complete Bochum algorithm with the D.4
recursive backend (`cryptanalysis/bindings/rust/hashed-ecdl-tensor`, mirrored in `crypto`
`crates/hashed-ecdl-tensor` on main): 34 release tests, certificate parity, exact forced-recursion controls;
the literal constants force the finite-size fallback on every practical input; ECCp-131 (1,564,506 edges)
and ECC2K-130 (2,694,680 edges) networks exceed the exact-leaf budget without a relation; no S/rho figure
(`cryptanalysis/docs/hashed-ecdl.md`). Python prototype with bucket elimination
(`/Volumes/SSD990-2/btn_ecdl_20261029_backup`, also `crypto/research/btn_ecdl_20261029`): min-fill width
23 at n = 5 and 34 at n = 7 on the binary m = 2 network, i.e. 2^{5n}, five times the enumeration exponent
(measured). Two independent readings of D.1/D.4 (no gap), a ternary (3-adic) packet variant with 28 Lean
theorems and a counterfamily that kills a generic data-only shortcut, and the "common kernel" study
(gauge counterexample in Lean: the Boolean kernel is not multiplicative, K(W)K(V) ≠ K(WV) at the (11,11)
entry, 4 vs 6; coded-operator identity; packing bound labels·dim C ≤ dim V) — all on `crypto` main under
`research/prime_tensor_variants_20261030` and `research/prime_common_kernel_20261030`. Open problems filed:
`problems/hashed-ecdl-tensor-backend.md`, `problems/correlated-packet-common-kernel.md`,
`problems/prime-scalar-exact-splitting.md`. Atlas: `LEAD-EXO-hashed-factor-base-tensor-ecdl`
(open_next_step: audit, and the frozen 84-case panel in `research/hashed_ecdl_speed_20261029`).

**SAT on point decomposition.** Binary: closed with a measured obstruction, per-attempt UNSAT cost
2^{αl} with α ≈ m (WDSat 2.9985 bits per unit l at m = 3; 4.09–4.33 at m = 4), against α < m/2 − 1 needed
(`LEAD-ICB-decomposition-encoding-and-sat`; `cryptanalysis/experiments/pdp-scaling`). The one positive
constant: the trace XOR clause, 1.6–2.0× throughput. LLM-synthesised WDSat branching policies
(`ml-cryptanalysis-worktrees/pdp-sat-synth`): four to 26 winners at l = 6, −2.3 to −2.7 % in instructions
vs the hand reference, lex symmetry breaking is the dominant lever (+174 % without it); at l = 7 the policy
wins UNSAT by 1.9 % but loses planted SAT by 3.3× mean, so it is not scale-robust for relation collection
(memory `pdp-sat-synth-state`). Prime: the `prime_sat_*` thread (Boolean-digit S3, tensor-product Hilbert
series) proved a leading-form degree of regularity s + ⌈(s+2)/2⌉ for two summands, lowered by one with
membership predicates and to s + 1 with one-hot pair grouping; the 2026-10-31 review shows the grouped
invariant is G + 1 for any Boolean system (a regrading, dimension 4^s in both presentations) and that the
only SAT measurement (p = 17, 97) went the wrong way (membership augmentation raised work 2.7×). A
prime-field PDP SAT representation sweep is running today (`crypto-worktrees/prime-pdp-sat-sweep-20261031`;
Lean gate PASS with `rho_floor`; pilot at 16 bits, l = 6: 2·10^5 to 1.6·10^6 kissat conflicts per target
against an enumeration reference of 48–68; memory `prime-pdp-sat-sweep-state`). CDCL(T) over F_p with a
polynomial-calculus-certified lemma learner exists (`prime_sat_prefix_search_20261030`).

**Everything else on prime fields** (atlas dispositions): generic table/MITM/k-sum decomposition closed
by Shoup and Corrigan-Gibbs–Kogan (Table B rows 5.47–13.5× rho at 9.7–23.4 bits; k-sum k = 5 at 40 bits
9.7× rho); naive Semaev Gröbner closed (β_work 0.797 vs 0.30 gate); PKM towers open_blocked on memory
(degree grows at every measured cell; m = 3, N = 21 died at 99.6 GiB); digit presentation
exhausted_at_scale (curve-free null reproduces d_ff); Coppersmith small roots closed (reach 0.3293 per S3
block vs demand 8/9); xedni, function-field sections (α = 3 exactly), p-adic sections, extension-field
embedding, same-field covers, self-pairings, representation technique, residual partial relations (slope
−0.025 like rho), coincidence harvest (κ flat), retained-setup online (BL rho 53–317× faster at equal
advice): all closed with named obstructions. Owner rulings 2026-10-29 (`research/prime_ic_program_20261029/DECISIONS.md`, main): two counted tracks (generic, bounded by √r; algebraic, baseline
empty until the msolve reference arm runs; ~49 CPU-hours approved on EKS).

**ML / RL.** `cryptanalysis-rl` is not on disk; the RL question `RQ-RELN-61638a` (GNN/bandit/RL search
policies on the relation hypergraph) has no results; `ml-cryptanalysis` hardness and decomposability
predictors are null beyond size (memory `ic-dataset-state` for the IC solve dataset).

## 4. The structural picture (why "tensor Boolean + SAT" has not moved an exponent, in three lines)

Let b = log₂ p, arity m, window s = b/m bits, |F| = 2^s. Per-attempt decomposition cost 2^{es}.

1. **Ladder rule** (`ladder_rule`, `ladder_rule_real`, derived from the lab's cost model): the relation
   phase m!·p^{(1+e)/m} is below rho's p^{1/2} iff e < m/2 − 1. Arity 2 never (e < 0), arity 3 needs
   e < 1/2, arity 4 needs e < 1. Enumeration gives e = m − 1 (naive) or e ≈ (m−1)/2 with p^{(m−1)/(2m)}
   memory (MITM). The sweep's `rho_floor` is the same statement in the lab's units.
2. **Solver-saving floor** (`solver_saving_floor`): any encoding of the attempt has at least b interface
   bits (the m window indices, `interface_lower_bound`); a Boolean solver costing 2^{n_eff(1−δ)} with
   n_eff ≥ b meets the per-attempt budget 2^{b(1/2−1/m)} only if δ ≥ 1/2 + 1/m. SAT (δ ≈ 1 − α/m per the
   measured α ≈ m, i.e. δ ≈ 0), the polynomial method (δ = 1/(2d), `polymethod_vs_enumeration`), and
   tensor contraction (δ = 1/√log s) are all far below. **No general-purpose Boolean solver can make
   fixed-arity index calculus beat rho; a solver must see curve structure worth a constant fraction of the
   exponent.** That is what Lead 4 measures directly.
3. **Black-box balance** (`blackbox_balance`, `no_crossover`): letting m grow (Bochum) trades the 1/m in
   the budget against the m in the edge count; the optimum is m ≈ (log b)^{1/4} and it is below rho only for
   log₂ b > 256c² with c the constant in D.1's O(s/√log s). The only levers left on this route are (i) the
   contractor's exponent on *these* networks (the lab's common-kernel problem; Remark 2 says α ↦ α/2 in the
   final exponent), (ii) relation networks with o(mb) edges (each of the m−1 additions needs at least one
   b-bit multiplication, each needing ≥ b edges; so o(mb) needs a relation type that is not a sum of m
   points), (iii) structure that lets a solver skip a constant fraction of the interface.

Leads 1–3 and 8–9 work lever (ii)/(i) honestly at the level of constants and transfer to binary curves;
Leads 4–7 and 10–12 are the measurements that decide whether lever (iii) exists and, if not, record it so
that it is never proposed again.

## 5. Leads (12), each with a measurement that can start now

Decision rules use the §5a pricing band (0, 0.1, 1 GAE per native operation) and the ladder rule; a
"slope" is the fitted exponent e of log₂(work) against s at fixed m with fresh primes per s, bootstrap CI,
curve-free null (same-shape random polynomial), and the enumeration/MITM reference arm on the same
instances. Atlas collision status is from `tools/lead_atlas.py --match` run today.

### L1. Characteristic-2 Fourier gadget: linear-size relation networks for binary curves, and the same gadget as an XOR-native SAT encoding

- **Idea.** Replace the Bochum multiplication gadget (Z/(2^J+1), ω = 2^{2J/L}, integer carries) by
  Schönhage's triadic construction: multiply in B_N = F_2[x]/(x^{2N}+x^N+1), N = 3^ν, where x is a
  principal 3N-th root of unity and 3N is odd (hence a unit); split N = N₁N₂ ≈ √N once, re-encode inputs as
  polynomials over B_{N₁}, do length-3N₂ transforms with ψ = y^{N₁/N₂}, 2N₂ pointwise products of 2N₁-bit
  elements, carry-less recombination (no carries, no canonical-form problem, reduction mod the field
  polynomial by quotient guessing). Expected: O(n) edges, degree O(n^{3/4}), parsimonious. With it, the
  Bochum conditional theorem transfers to E(F_{2^n}) (and Lead 8 adds Frobenius). The same gadget read as
  a constraint system is **F_2-linear transform layers (XOR constraints) + ~2√n small products +
  hash/curve predicates**: precisely the form WDSat's XORGAUSS and CryptoMiniSat's Gauss–Jordan treat as
  free propagation, with the nonlinear budget shrunk from n² AND-gates (schoolbook) to about 2√n·(2√n)²
  = 8n^{1.5} bit-products, or recursively less.
- **Connects.** Schönhage 1977 multiplication ↔ WDSat parity reasoning ↔ the D.1 degree cap (the big
  transform vertices are exactly why the network is linear-size).
- **Exists.** Integer gadget proof and code (Bochum §2.3; Rust crate); the btn prototype notes the integer
  construction "does not carry over to GF(2)[x] without a characteristic-2 variant"; Pospelov's survey
  (arXiv:1010.1101 §3.2.2) gives the ring and split; WDSat pinned baseline and 60-instance corpus
  (`crypto/research/index_calculus_baseline_20260914`), `csd/pdp_anf.py` reproduces it.
- **Measure.** (a) Edge/degree census of the char-2 gadget for n = 13…131 against Theorem 3's
  1100(d+1)/136(d+1)^{3/4} (plan: derive constants, Lean-check the ring facts; `lean/Char2Fourier.lean`
  already verifies x is a principal 9th and 27th root in B_3, B_9 by kernel evaluation). (b) SAT arm: encode
  the m = 3 binary PDP at n = 15, 17, 19 (l = 5, 6, 7; the frozen corpus) as [one-hot/bit window vars] +
  [gadget circuit with XOR layers] and run WDSat `-x`/CryptoMiniSat with Gauss–Jordan versus the standard
  descended ANF; metrics: conflicts, instructions retired (the pdp-sat-synth all-in counter), UNSAT and
  planted-SAT cells separately; slope of log₂ work vs l.
- **Decision.** Slope ≥ 3 bits per unit l at m = 3 (the enumeration floor the atlas records) → the
  encoding is another constant and the lead closes into `LEAD-ICB-decomposition-encoding-and-sat` with the
  gadget numbers recorded. Slope with CI below 2.5 → the first binary PDP encoding to grow slower than
  enumeration; freeze a protocol and extend to n = 23, 31. Independently, the census decides whether the
  char-2 gadget meets O(n) edges with a constant ≤ the integer one (if not, say so; the transfer theorem
  still holds with a worse constant).
- **Cost.** Gadget derivation + Lean: 2–3 days; SAT arm on existing corpus: 2–3 days, one box.
- **Lean obligations.** principal root (done for N = 3, 9); four-step factorisation identity; carry-less
  no-wraparound bound 2N ≥ 2n−1; parsimony of the quotient-guess reduction.
- **Atlas.** Collides with the SAT/XOR leads and the HFB-tensor lead on keywords; no lead has the gadget
  or the XOR-layer encoding. Prior art on XOR-native PDP: Trimoska–Ionica–Dequen (ePrint 2019/313, CP 2020).

### L2. Ring-linear theory propagation for the prime-field Fourier gadget (CDCL(T) over Z/(2^J+1))

- **Idea.** The integer gadget is linear–diagonal–linear over A = Z/(2^J+1): the two transform stages
  are A-linear maps, only the L pointwise products and the carry chain are nonlinear. A CDCL(T) solver with
  a theory that does exact Gaussian elimination over A on the transform words (the analogue of XORGAUSS
  over a ring) propagates both transforms for free once enough words are fixed; branching is confined to
  the product words and the carries. Within D.1 this buys nothing (fact 2 in §1); as a solver it reduces
  the nonlinear port count from ~1100(d+1) to ~L(J+1)·3 ≈ 96(d+1) per multiplication (derived from
  Lemma 7's accounting).
- **Measure.** Build the m = 2, 3 relation network at b = 16…40 with the integer gadget; three solver
  arms: (i) bit-blasted schoolbook CNF (kissat/CaDiCaL; the sweep's quotient-witness mulmod is the
  reference), (ii) gadget CNF without theory, (iii) gadget + ring-linear propagator (CaDiCaL external
  propagator, as in `prime_sat_prefix_search_20261030`, with the PC-certificate contract). Metrics:
  decisions, propagations, instructions, theory calls; fraction of variables fixed by the theory at the
  first conflict; slope e vs s at m = 3.
- **Decision.** e ≥ 1 (enumeration-like, ladder rule needs e < 1/2 at m = 3) in all arms → record the
  ring-linear propagator as a constant and close; the number itself (ratio arm iii / arm i) is the
  deliverable either way. Any arm with e below 1 across s = 6…10 reopens.
- **Cost.** 1 week (propagator ~300 lines on the existing adapter; gadget emitter ~200).
- **Atlas.** No hit for ring-linear propagation (nearest: `LEAD-ICE-sat-xor-pdp-oracle`, open).

### L3. Sparse-parity (expander-code) hashed factor bases: membership as pure XOR

- **Idea.** Bochum's hash H_i(z) = α_i z + β_i mod s costs a b-bit multiplication per point (O(b) edges of
  degree b^{3/4}) and exists only to give pairwise independence for Lemma 10. Replace it by an F_2-linear
  hash on the coordinate bits: F_i = {A : M_i · bits(enc A) + β_i ∈ T} with M_i a sparse parity-check
  matrix (row weight O(1), Tanner/expander construction, small-bias Cayley shifts, exactly the family the
  ETH proof uses in §6 for its parity tests). Membership becomes O(b) XOR constraints of constant degree
  with zero multiplications; for an XOR-native solver (L1/L2) the whole membership layer is absorbed by
  Gaussian elimination. Random dense F_2-affine maps are 2-universal, so Lemma 10 goes through verbatim;
  sparse ones are only small-bias, so the second moment needs the explicit expander bound (the ETH proof's
  Lemma 6.1 machinery, or measurement).
- **Measure.** (a) Variance of the decomposition count X over seeds for affine-mod-s, dense F_2-affine,
  and sparse (row weight 3, 5, 7) hashes at p ≈ 2^{16…24}, m = 2, 3, 1,000 seeds × 256 targets, exact
  counts by enumeration; report Pr[X > 0] against Lemma 10's 2/5 and the ratio E[X²]/E[X]². (b) SAT arm:
  same instances as L1(b)/L2 with the three membership encodings; nonlinear port count and solver work.
- **Decision.** Sparse hashes with Pr[X>0] within 5 % of the dense ones at row weight ≤ 7 → adopt as the
  default base for every SAT/tensor arm (cost reduction is then measured, not asserted). Pr[X>0] falls
  with b → record the required weight growth; the lead then only trades constants.
- **Cost.** 2 days for (a) in Python/Rust; (b) rides on L1/L2.
- **Lean.** Mean yield is base-independent: `yield_conservation` (done, any subsets of any finite abelian
  group). Obligation: 2-universality of random F_2-affine maps (short Mathlib proof), small-bias bound for
  the sparse family (literature; cite SS96 / NN93 as the ETH proof does).
- **Atlas.** No lead compares hashed against structured bases under a solver (phrase 7 today);
  `LEAD-ICE-decomposition-probability-audit` and `LEAD-ICE-semaev-yield-model-coset-typing` (both open)
  are the yield-model neighbours; advance them with the variance numbers rather than file a duplicate.

### L4. Strong-backdoor census of chained-S3 point decomposition under field-linear + root-finding propagation

- **Idea.** The ETH proof's engine is "fix a few shared measurements, then the rest factorises". Its SAT
  counterpart is the strong backdoor: the smallest set B of variables such that every assignment to B makes
  the residual instance polynomial for a fixed propagator. For PDP the natural propagator is the lab's
  CDCL(T) theory: Gaussian elimination over F_p (or F_{2^n}) on linear constraints plus univariate
  root-finding (one unknown left in an S3 or in λ(x₂−x₁) = y₂−y₁). Enumeration corresponds to the backdoor
  "all bits of m−1 windows", size (m−1)s. The question nobody has measured: is the minimal strong backdoor
  of the chained-S3 system (variables: window bits, intermediate t = x(P₁+P₂), slopes λ, hash/membership
  auxiliaries, transform words if the gadget is used) smaller than (m−1)s, and does the gap grow with s?
- **Measure.** Exhaustive minimal-backdoor search (branch and bound over variable subsets with the
  propagator as the oracle) at m = 3, 4 on prime fields of 12–20 bits with windows s = 4…7, three curves
  per prime, 32 targets each, UNSAT and planted-SAT instances; also the binary analogue at n = 13…19 on the
  WDSat corpus (the pdp-sat-synth memory records "exact MVC = 5l−2 > the 3l x-bit propagation backdoor",
  which is this quantity for one propagator). Arms: which variable class forms the smallest backdoor
  (window bits, slopes, transform words).
- **Decision.** B*(m,s) = (m−1)s − O(1) on every cell → file a known negative: no propagation-based
  solver sees structure in PDP at these arities (closes L2's and L5's exponent hopes in one stroke, which
  is why this runs first). B*/s with slope below m−1 and a named variable class → that class is the
  branching order for L5 and the "measurement group" for any tensor-style hybrid.
- **Cost.** 3–4 days (the propagator and instance generators exist in `prime_sat_followup_20261029` and
  the sweep crate; the subset search is new, feasible to s = 7 by symmetry pruning).
- **Atlas.** Uncovered (phrase 3 today); neighbours `LEAD-ICE-hybrid-guess-determine-l7` (binary k*),
  `LEAD-ICE-chained-semaev-dreg-vs-semiregular`. Prior art: Williams–Gomes–Selman 2003; Semenov–Zaikin
  AAAI 2018 (backdoors for guess-and-determine on ciphers). No backdoor result for algebraic PDP found.

### L5. Transplant the PDP-aware SAT-policy synthesis loop to the prime-field hybrid encoding

- **Idea.** `ml-cryptanalysis` already runs an LLM proposer (Qwen3.8-27B, Modal) that writes WDSat
  branching policies, with a positive control, soundness gate, witness feedback and a frozen decision rule;
  it found robust 2.7 % wins at l = 6 and discovered lex symmetry breaking. Point the same loop at the
  prime-field hybrid encoding (the sweep's quotient-witness mulmod + one-hot pair windows, or L1/L2's
  gadget encodings) with the CaDiCaL external-propagator engine, policies over {window bits, λ, t,
  transform words}, and the Gauss gating decision exposed. Arms by factor base: window, affine-hash, sparse
  parity (L3).
- **Measure.** Same harness and rules 1–3 as `experiments/pdp-sat-synth-20261027/PROTOCOL.md`
  (promotion on 999 fresh instances, instructions as the ratchet count, CPU time guard), at p ≈ 2^{18…30},
  m = 3, s = 6…10; report the slope e of the best policy and of the reference on the same ladder, not only
  the constant at one size (the l = 7 regression in the binary program is the cautionary case).
- **Decision.** Constant-factor wins are engineering successes under the owner's criterion and ratchet
  the (empty) algebraic-track baseline; the lead advances the exponent question only if the best policy's e
  is below the reference's with CI excluding 0 on ≥ 3 sizes. Nothing below e = 1 at m = 3 across the
  ladder → close the prime SAT lane at this encoding family, in line with the digit-presentation
  foreclosures.
- **Cost.** 1 week to wire (engine kinds exist), multi-day search budget (owner decision, as before).
- **Atlas.** Partial: `LEAD-ICE-sat-xor-pdp-oracle` (open) and `LEAD-EXO-ml-learned-models` (open);
  prime-field policy synthesis is not covered.

### L6. The finite-size face of the tensor route: contraction width and model counters on the real networks

- **Idea.** Below the D.1 admission threshold the algorithm is brute force, but the finite-size analogue of
  "tensor contraction" is a planned contraction (cotengra/TensorOrder: carving/tree decompositions) or a
  component-caching model counter (sharpSAT-TD, d4). Nobody has measured the width of the actual
  Bochum/gadget networks beyond the prototype's n ≤ 7 binary cells (width ≈ 5n). The structured-cut arm:
  condition on a few transform words instead of input bits (the ETH proof's "measurements"), since cutting
  the DFT layers splits a multiplication into independent column/row transforms.
- **Measure.** For the integer gadget and the char-2 gadget networks at m = 2, 3, b = 16…64: min-fill and
  hypergraph-partition contraction width (cotengra), treewidth lower bounds (e.g. via minor/degree
  arguments), sharpSAT-TD and d4 time on the equivalent CNF; width after conditioning on k input bits vs k
  transform words; all against the enumeration exponent (m−1)b/m.
- **Decision.** Width ≥ (m−1)b/m at every size with no cut class beating bit cuts → record the known
  negative "exact contraction of arithmetic relation networks never beats enumeration below the D.1
  threshold" with the measured width constants (this closes "just run a model counter" re-proposals, which
  the atlas does not yet foreclose). A cut class whose width grows slower than bit cuts is the only
  positive outcome and would feed L4's variable classes.
- **Cost.** 3 days (the Rust crate exposes the graph; cotengra and sharpSAT-TD are off the shelf; the
  owner must approve downloads).
- **Atlas.** Covered at the top by the HFB-tensor lead; widths are not recorded anywhere except the
  prototype README and `prime_tensor_variants_20261030/reviews/fourier-width.md` (which argues Ω(b) live
  indices for the dense contraction, representation-specific).

### L7. Second moment of hashed versus window bases, and the m! constant

- **Idea.** The mean relation yield is base-independent (`yield_conservation`, and the lab's KN-FIND-007);
  only the variance differs. Bochum's Lemma 10 gives Pr[X>0] ≥ 2/5 from pairwise independence; window
  bases have additive structure that could make X more or less concentrated. For any practical arm the
  attempt count is 1/Pr[X>0] ≈ m!, and the constant matters at the toy sizes where everything is measured.
- **Measure.** Exact decomposition counts X(R) for all targets at p ≈ 2^{14…22}, m = 2, 3, 4, for window,
  affine-hash, sparse-parity and random bases of equal size; report Var X, Pr[X>0], and the fraction of
  targets with X ≥ 2 (which is what a parsimonious counter exploits). Planted positive control, random-label
  null (the lab's standard).
- **Decision.** Ratios within noise → the base choice is free for every solver arm (adopt the cheapest
  membership, L3). A base with Pr[X>0] higher by a factor that grows with b would be the first
  base-dependent effect beyond conservation and would be filed as a result; the prior is flat.
- **Cost.** 1 day.
- **Atlas.** Advance `LEAD-ICE-decomposition-probability-audit` / `LEAD-ICE-semaev-yield-model-coset-typing`.

### L8. Koblitz/binary transfer: Frobenius-invariant hashed bases and the ECC2K-130 edge census

- **Idea.** With L1's gadget the Bochum theorem applies to F_{2^n}; on a Koblitz curve the hash should be
  applied to a canonical representative of the Frobenius orbit (least cyclic rotation in a normal basis,
  the canonicalisation the lab's rho already uses) so that F_i is τ-stable and |F_i|, hence N and the
  linear algebra, shrink by n; the canonicalisation is one vertex of degree n with a poly-time predicate,
  inside the degree cap. Squaring is a wire permutation in a normal basis, so the gadget's Frobenius
  conjugates are free.
- **Measure.** Write the conditional theorem for E(F_{2^n}) with and without Frobenius folding (edge
  count, degree, N); recount the ECC2K-130 relation network with the char-2 gadget against the existing
  2,694,680-edge binary prototype network and the 1,564,506-edge ECCp-131 integer network (both measured in
  `cryptanalysis/docs/hashed-ecdl.md`); state the D.1 exponent s/√log s for each in bits next to the rho
  floor 2^{60.81} (measured constant) so the distance is explicit.
- **Decision.** Desk result; it either lands as the binary conditional theorem (a paper section) or shows
  the char-2 gadget's constant is worse than the integer one. No exponent claim beyond Bochum's.
- **Cost.** 2 days after L1(a).
- **Atlas.** `LEAD-ICB-frobenius-orbit-factor-base` is closed_obstructed for the generic saving; this lead
  only composes the known factor-n saving with the tensor route and must say so.

### L9. One yardstick: a BTN census tool and the D.1 cost row for every encoding the lab owns

- **Idea.** Five encodings of the same problem exist (descended ANF, chained-S3 circuit, digit
  presentation, hashed Bochum, gadget circuits) and are compared in incompatible units. A small tool that
  takes any encoding (CNF, ANF, or the Rust crate's graph), builds the bipartite BTN, and reports
  (s, Δ, #vertices, nonlinear ports, s/√log₂ s, the D.1 admission status, the enumeration exponent) puts
  them in one cost model, with the subdivision rule of Bochum Remark 1 applied uniformly.
- **Measure.** Run it on every encoding at the sizes already in the corpora; publish the table in the
  prime IC program directory. Also report the interface count k per encoding against
  `interface_lower_bound`.
- **Decision.** Tooling; its output is the input to L1–L6's decisions.
- **Cost.** 2 days.

### L10. Lean-certified exponent rules as the program's gate (done; file as known results)

- **Done (this note).** `lean/IcTensorRoute.lean`: `ladder_rule`, `ladder_rule_real`, `arity_two_never`,
  `solver_saving_floor`, `amgm`, `blackbox_balance`, `no_crossover`, `arity_two_blackbox`,
  `crossover_bits`, `polymethod_vs_enumeration`, `interface_lower_bound`, `yield_conservation`; all
  compile under the formal workspace (`lake env lean`, Mathlib pinned, 37 s) with standard axioms only.
  `lean/Char2Fourier.lean` (core Lean): principal-root checks for L1.
- **To do.** Put the two statements the 2026-10-31 review drafted and the three from here into
  `knowledge/frontiers/ecdlp/index-calculus/` as KR rows through `/curate-knowledge`: (a) black-box D.1
  index calculus cannot cross rho below 2^{256}-bit groups and its m-balance is optimal; (b) D.1 gives no
  discount for linear vertices and has no q-ary form; (c) polynomial-method solvers lose to guessing m−1
  windows for m ≤ 2d; (d) target-family amortisation costs b edges per target bit versus b/m per base bit,
  so it is dominated in the BTN cost model (derived in §6); (e) the two from the sibling review (d_reg is
  a regrading; Fermat-indicator counting has degree ~p). Cite the Lean file as the receipt. Add the
  sweep's `rho_floor` beside `ladder_rule` so the two conventions are reconciled once.
- **Cost.** Half a day plus review.

### L11. Propagation-complete and Karatsuba-structured arithmetic encodings for pure CDCL

- **Idea.** Independently of theories, the CNF encoding of b-bit modular multiplication decides CDCL
  behaviour; the sweep uses one quotient-witness encoding. Biere et al. (POS26) study
  propagation-complete and Karatsuba-recursive multiplier encodings for bit-vector and carry-less
  multiplication. Measure whether encoding choice changes the slope, not only the constant, on PDP.
- **Measure.** Four encodings (schoolbook, Karatsuba, propagation-complete per POS26, L2's gadget without
  theory) of the m = 3 relation at b = 16…32 under kissat/CaDiCaL, same instance set as L2; metrics as
  in L2.
- **Decision.** Constant-only differences → record and fix the best as the sweep default; a slope change
  is the only reopening outcome (prior: none, by `solver_saving_floor`).
- **Cost.** 2 days on the sweep crate.

### L12. Incremental prefix search: amortising learned clauses across the N attempts of one seed

- **Idea.** Bochum's relation finding is a prefix search of ~mb oracle calls per target and N targets per
  seed on the *same* network with only the target bits changed. A SAT instantiation should use one
  incremental solver per seed with target bits as assumptions, keeping learned clauses that do not depend
  on the assumptions (the curve, hash and arithmetic lemmas). This is the only amortisation the BTN cost
  model admits (target bits as free variables cost b edges each, see §6).
- **Measure.** Per-target work with and without clause retention across 256 targets at b = 16…28, m = 3;
  plus the planted-SAT/UNSAT split. Metric: instructions per relation.
- **Decision.** Constant (expected); it becomes the standard harness setting if ≥ 1.2× and feeds the
  relation-throughput objective the pdp-sat-synth memory asks for at l ≥ 7.
- **Cost.** 1 day.

## 6. Closed by arithmetic in this note (file, do not re-propose)

- **Direct D.1 on PDP** (any arity, any encoding): per-attempt exponent ≥ c·k/√log k with k ≥ b
  interface bits; never meets b(1/2 − 1/m) (`solver_saving_floor`); the only escape is m → ∞ (Bochum),
  which needs log₂ b > 256c² (`no_crossover`).
- **Target-family amortisation in the BTN model.** Making the target R = R₀ + [T]G free costs one point
  addition per bit of T, i.e. ~b edges per bit, while a base point costs ~b edges per b/m bits; so free
  target bits are m times less edge-efficient than enlarging the base, and letting (u,v) float costs 2b
  additions. Amortisation is therefore dominated inside D.1; the SAT-side residue is L12.
- **Descent trees with the tensor oracle.** Decomposing a level-k point into k+1 points costs a network of
  (k+1)b edges; the deepest level has (m+1)b > mb edges, so descent never beats the direct m-sum when the
  contractor's exponent is linear in s.
- **Polynomial-method solvers** (LPTWY 2^{(1−1/(5d))n}, BKW 2^{(1−1/(2.7d))n}, Dinur 2^{(1−1/(2d))n}) on
  Weil-descended or digit Semaev systems: lose to guessing m−1 windows for m ≤ 2d
  (`polymethod_vs_enumeration`); no application to summation polynomials exists in the literature.
- **p-ary digit networks** under D.1 as stated: not covered (Boolean edges only, 2-adic proof); the
  ternary packet variant already built by the lab is an internal adaptation and did not improve the
  exponent (`prime_tensor_variants_20261030`).
- **Linear-vertex discounts inside D.1:** none (black-box vertices).

## 7. Recommended order

1. L10 filings (half a day) and L9 census tool (2 days): they make every later number comparable.
2. L4 backdoor census (3–4 days): the cheapest experiment that can kill or redirect L2, L5, L6 and L11
   at once.
3. L1(a) char-2 gadget derivation + Lean, then L1(b) WDSat arm on the frozen corpus (1 week): the one
   lead with a theorem-level deliverable (binary conditional transfer) and a solver arm on existing
   instruments.
4. L3 + L7 variance study (2–3 days): decides the default base for all SAT arms.
5. L2 and L11 (1–2 weeks): measured constants for the prime arithmetic layer.
6. L5 (owner budget): the loop that can find what humans did not, now on prime fields.
7. L6, L8, L12 as desk/side studies.

Every lead above is a constant-factor or closure measurement unless its decision rule says otherwise;
none promises a subexponential prime-field algorithm, and §2 and §4 explain why none can at fixed arity
with any known Boolean solver. If the owner wants the exponent question pushed, the honest statement is
that it lives in two places only: the PKM m ≥ 4 degree cells (memory-blocked; outside this note) and the
contractor-exponent problem the lab already opened (`problems/correlated-packet-common-kernel.md`).

## 8. Files

- `LEADS.md` (this note).
- `lean/IcTensorRoute.lean` (Mathlib; build: `cd /Volumes/SSD990-2/crypto-autoresearcher/formal && lake env lean <path>`).
- `lean/Char2Fourier.lean`, `lean/lean-toolchain` (core Lean v4.35.0; build: `lean Char2Fourier.lean`).
- Source texts used: `scratchpad/dl/eth.txt`, `eth_paper.pdf` (openai/math, revision 7ef5c0cc), `bochum.txt`
  (pdftotext of `/Volumes/SSD990/downloads/subexponential-ecdl.pdf`); not copied here.
- Public sources consulted (today): github.com/openai/math history and preprint; hexagonmath.org
  2610.00577v1 and github.com/anthropics/formal-math/eth README; ethdiscussion.blog posts of 26, 29, 30 Oct
  and the Lean-replication comments of 28 and 31 Oct; blog.computationalcomplexity.org (Fortnow, Oct 2026);
  cs.cmu.edu/~odonnell/some_eth_false_exposition.pdf (30 Oct); shivakintali.github.io/papers/ETH.pdf
  (26 Oct); arXiv:1010.1101 (Pospelov, char-2 Schönhage); ePrint 2019/313 and arXiv:2001.11229 (WDSat);
  ePrint 2026/384 (structured generic-group model; the elliptic-curve theorem is in the ePrint revision,
  which was not fetchable); SODA 2017 LPTWY, ICALP 2019 BKW, arXiv:2005.04800 (Dinur).
