# Semaev vs hashed relations as constraint graphs: a treewidth census on toy curves

Date 2026-11-11. Slug `s3tw`. Continues `research/hashed_ecdl_tensor_audit_20261107/AUDIT.md` (desk audit
of the Bochum hashed-factor-base tensor-contraction ECDL claim). Every number below is produced by
`tensor-treewidth` (`crates/hashed-ecdl-tensor/src/bin/tensor-treewidth.rs`) from the rows in `rows/`,
laid out by `render.py` (which computes nothing about any network). Labels: **[measured]** read from a
row; **[derived]** arithmetic on measured values, shown; **[exact]** a quantity the code recomputes
from the object itself (a width from its order, an acceptance from the network); **[heuristic]** an
upper bound from a greedy order; **[bound]** a lower bound with the argument that makes it one.
No claim here is independently validated: `[pcc:unverified]` throughout.

## Results (one line each, then the tables)

All rows: `rows/*.json`, laid out in `TABLE.md`; graphs `treewidth_vs_bits.svg`, `exponents_vs_bits.svg`;
report `REPORT.pdf`. Schoolbook networks were measured at b = 8–36 (40 in a reduced run if present in
`rows/`), Fourier networks at b = 8–24 (plus S3 at 28 and 36 if present): the 2-hour runtime budget of the
sweeps cut the rest, said so here rather than extrapolated.

1. **The encoding is the bottleneck, not the solver.** The treewidth *lower* bound — what no elimination
   order can beat — exceeds the enumeration/rho exponent b/2 by 4–5× at every size and grows about
   2 per bit against 0.5 per bit for enumeration **[bound, measured]**: schoolbook S3 lb = 22, 39, 53, 69,
   74 at b = 8, 16, 24, 32, 36 (b/2 = 4, 8, 12, 16, 18); hashed group law lb = 27, 48, 67, 84, 95. So
   even a treewidth-optimal exact contraction costs at least 2^{~2b} on these graphs, against 2^{b/2}
   for enumerating one slot. No solver that treats the bit-level network as a Boolean tensor network
   can reach enumeration, let alone rho.
2. **The generic bound is loose by two orders of magnitude, and it does not matter.** The D.1
   exponent s/√log₂ s is 30–150× the *best measured upper bound* **[derived]** (S3, b = 36: 23,928 vs
   616; hashed group law: 119,731 vs 900). Structure a tree decomposition can see is real and large, but
   the structure that remains (lb ≈ 2b) is still a 2^{1.5 b} factor above enumeration.
3. **Semaev buys constants, not a regime.** Against the hashed group law at the same b (schoolbook),
   S3 has 3.7–5.3× fewer edges (b = 36: 97,408 vs 521,803), a 15–25% lower width lower bound
   (74 vs 95) and a 20–30% lower best upper bound (616 vs 900) **[measured]**; its lower-bound slope
   is 1.86 per bit against 2.43 **[derived]**. The whole gain comes from dropping the ordinates and the
   curve-equation checks; the hash gadget itself costs about 2× in s and ≤ 10% in width (hashed vs
   small-x group law: lb 95 vs 86, ub 900 vs 752 at b = 36).
4. **Chained S3 (m = 3, sparse S4)** has the same width as the group law at the same b (lb 84, ub 670
   at b = 36) with a larger enumeration exponent 2b/3; the ratio lb/(enumeration) is 3.5 instead of 4.1,
   still far from 1 **[measured, derived]**.
5. **The Fourier gadget is the worst choice for a structure-exploiting solver.** Its transform vertex is a
   clique of 132 (b ≤ 12), 260 (b = 16–32) or 520 variables, so the width is set by the multiplier, not
   the relation: lb 139–405 and ub 181–870 at b = 8–24 for every relation **[bound, heuristic]**, i.e.
   35–60× b/2. The O(b)-edge encoding minimises s (what the generic bound charges) by maximising
   width (what a decomposition pays). s grows with slope 1.2 in log–log for Fourier against 1.8 for
   schoolbook **[derived]**, as the audit's gadget count predicts.
6. **Contraction cost tracks the widest bag**: log₂ of the exact table work along the best order is the
   best width + 2 on every row **[exact]** (S3 b = 36: 618 bits; enumeration 18; rho 18).

What this says about the owner's question: encoding S3/S4 as a constraint graph gives the solver
structure it can exploit — the graphs are 20–150× narrower than the generic bound assumes — but the
structure is not of the kind that closes the gap. Any bit-level encoding of a b-bit modular
multiplication has width Ω(b) (communication argument; measured here as ≥ 2b in the lower bound
and ≈ 20b in the greedy upper bound), and a relation needs several of them, so the exponent is at
least a constant multiple of b where enumeration pays b/2. The lever that remains is algebraic, not
graphical: eliminating at the field level (Gröbner/F4 on the S3 system, the known index-calculus route)
rather than at the bit level. The closed verdict of the audit stands for the hashed encoding; the
Semaev encoding does not reopen it by this route.

Validation: every schoolbook row at b = 8 and 10 passed (0 mismatches; positives 1–13 of 82–4,104
candidates) **[exact]**. The Fourier rows reuse the crate's own tested language and the
multiplier-independence test (same accepted set for both gadgets on the 17-curve).

Limits are in the report: one curve per b; greedy upper bounds (eager min-fill only to b = 16, lazy
beyond, noisy under tie-breaking); MMD+ lower bounds are valid but weak; the model is exact Boolean
contraction, so it does not credit algebraic structure inside a tensor.


## Question

The audit closed the hashed-relation lead on admission and constants: the generic contraction theorem
(openai/math Theorem D.1) never engages below 2^131071 edges, and the networks carry no structure a
solver could use beyond "s edges". The owner's follow-up: Semaev summation polynomials give genuine
algebraic relations; encoded as constraint graphs they might give a solver structure to exploit, where
hashed relations carry none. The cheap first experiment: build the O(b)-edge point-addition graph,
measure its treewidth and contraction cost on toy curves (b about 20–40), and compare against the
bound, to see whether the encoding or the solver is the bottleneck.

Treewidth is the right instrument because a Boolean tensor network with primal-graph treewidth `w` is
contracted exactly by variable elimination in `O(V · 2^(w+1))` table operations, whatever the
network's size `s`; this is the best any structure-exploiting exact contraction can do up to the choice
of order, and it is the quantity the D.1 bound `2^(s/sqrt(log s))` ignores.

## What was built (all native, in the crate the audit covered)

| piece | where | what |
| --- | --- | --- |
| multiplier switch | `encoding.rs` `Multiplier::{Fourier, Schoolbook}` | `modular_multiply` uses Theorem 3's Fourier gadget (default, unchanged) or the schoolbook AND/ripple gadget; constant bits fold in the schoolbook rows |
| small-x factor base | `encoding.rs` `input_point_small_x`, `assert_small_x`, `input_small_x` | `x < 2^l`: the high bits are constant zero, membership costs no gadget |
| Semaev S3 gadget | `encoding.rs` `semaev_s3_zero` | `(x1-x2)^2 x3^2 - 2((x1+x2)(x1x2+a)+2b)x3 + (x1x2-a)^2 - 4b(x1+x2) = 0` through `modular_multiply`: four signal×signal products, three by constants when `x3` is constant |
| four languages | `relations.rs` | `grouplaw-hash` (Bochum Lemma 11, m = 2, derived last slot, the crate's own language), `grouplaw-smallx` (same group law, small-x base), `semaev-s3-smallx` (m = 2, abscissae only), `chained-s3-smallx` (`S3(x1,x2,t) = 0`, `S3(t,x3,x_R) = 0`, m = 3: the sparse form of S4) |
| width instrument | `treewidth.rs` | primal graph (one clique per vertex over its edges); Bodlaender–Koster simplicial / almost-simplicial safe reductions; MMD+ lower bounds (min-d and least-c contraction) on the core; min-degree, eager min-fill and lazy min-fill orders with seeded restarts, each width recomputed exactly from the executed order; `log2` of the table work `sum 2^arity + sum 2^(|N(v)|+1)`; PACE `.gr` export and `.td` verifier |
| census driver | `bin/tensor-treewidth.rs` | toy prime-order curves (random `p ≡ 3 mod 4`, BSGS point count, certified by `Curve::new`), planted targets, build, measure, validate, JSON rows |

Tests: `cargo test --release -p hashed-ecdl-tensor` passes (96 lib tests + integration tests, including
13 new: S3 vanishes exactly on `x(P±Q)`; each gadget accepts exactly the direct-arithmetic solutions on
the 17-curve for both multipliers; the hashed language's accepted set is multiplier-independent; paths,
cycles, cliques, a grid and the PACE round trip for the width code). `cargo clippy --all-targets -D warnings` clean.

## Protocol

- Curves: one prime-order short-Weierstrass curve per `b ∈ {8,10,12,14,16,20,24,28,32,36,40}`, seed
  `20261111 ^ (b << 32)`, `p ≡ 3 mod 4` (cheap square roots; irrelevant to the graph). Targets are
  planted sums of factor-base points (so every validated row has at least one positive). The curve
  parameters are in each row.
- Slots: `m = 2` with `l = ceil(b/2)` free abscissa bits per slot; the chained S3 has `m = 3`,
  `l = ceil(b/3)`.
- Hash rows: `s` = first prime ≥ 2p, seeds `(3,1)` and `(5,2)`, `tau = ceil(s·(isqrt(r)+1)/r)` (L09's
  convention, expected base size about `sqrt(r)`).
- Validation (`b ≤ 10`, schoolbook): the network is evaluated through its witness program on **every**
  candidate the language ranges over (every small-x point; every `(x1,x2)` box; every positive tuple of
  the chained S3 found by scanning `t` over `F_p`, plus 200 random tuples; every affine point for the
  hashed rows) and compared with direct arithmetic. 0 mismatches is required; the count of positives is
  recorded.
- Width: `analyse` = clique bound, MMD+ on the raw graph, safe reductions, MMD+ on the core (lower
  bound = max of all three), then min-degree (3 restarts), eager min-fill (`b ≤ 16`, 3 restarts), lazy
  min-fill (3 restarts) on the core; `best upper` = minimum width. Fourier graphs: min-degree only
  (vertex arity 132–520 makes fill scoring hopeless and the clique bound already dominates).
- Reference exponents per row: D.1 at constant 1, `s/sqrt(log2 s)` (the audit's convention); brute
  force `s`; naive enumeration `(m-1)·b/m`; rho `b/2`.

## Sources checked

- research-kb: unavailable in this session (no executable, no MCP server). Lead atlas: unavailable
  (crypto-autoresearcher not checked out). Stated per the parallel rules.
- Read: `research/hashed_ecdl_tensor_audit_20261107/{AUDIT,README}.md`;
  `research/ic_prime_eth_leads_20261031/leads/L09/REPORT.md` (sizes `s`, `Delta` and D.1 admission for
  the Trimoska ANF, the prime PDP CNFs and the crate's networks; no width was measured there);
  `crates/hashed-ecdl-tensor/src/{encoding,reduction,width,census,network}.rs`.
- Not on disk and not re-derived: the Bochum manuscript text (pinned in the audit's `sources/`).

## Files

- `rows/*.json`: the recorded rows (one JSON object per network).
- `TABLE.md`, `slopes.json`, `tables.typ`: layouts of the rows (`render.py`).
- `treewidth_vs_bits.{svg,png}`, `exponents_vs_bits.{svg,png}`: graphs of recorded values.
- `REPORT.typ`, `REPORT.pdf`: the report with both graphs.
- `reproduce.sh`: rebuilds the binary and regenerates every row and the layouts.
