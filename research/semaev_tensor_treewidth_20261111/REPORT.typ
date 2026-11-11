#set page(paper: "a4", margin: (x: 1.6cm, y: 1.6cm), numbering: "1")
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#show link: underline
#import "tables.typ": rows_table, slopes_table

#align(center)[
  #text(size: 15pt, weight: "bold")[Semaev vs hashed relations as constraint graphs:\ a treewidth census on toy curves]

  2026-11-11 · slug `s3tw` · `research/semaev_tensor_treewidth_20261111` · isogeny-labs/crypto PR \#3941 · all numbers `[pcc:unverified]`
]

#v(4pt)
*Labels.* *[measured]* read from a recorded row (`rows/*.json`); *[exact]* recomputed by the code from the object itself (a width from its executed order, an acceptance from the network); *[heuristic]* an upper bound from a greedy order; *[bound]* a lower bound with its argument; *[derived]* arithmetic on measured values, shown. The layout (`render.py`) computes nothing about any network.

= Question and instrument

The desk audit `research/hashed_ecdl_tensor_audit_20261107` closed the Bochum hashed-factor-base contraction lead on admission and constants: the generic contraction theorem (openai/math Theorem D.1) never engages below $2^(131071)$ edges, and a network of $s$ edges is charged $2^(s\/sqrt(log s))$ with no credit for structure. The owner's follow-up: Semaev summation polynomials are genuine algebraic relations; encoded as constraint graphs they might give a solver structure to exploit, where hashed relations carry none. The first experiment: build the $O(b)$-edge point-addition graph, measure treewidth and contraction cost on toy curves ($b$ about 20–40), and compare against the bound, to see whether the encoding or the solver is the bottleneck.

Treewidth is the right instrument. A Boolean tensor network whose primal graph (one variable per edge, one clique per vertex) has treewidth $w$ is contracted exactly by variable elimination in $O(V dot 2^(w+1))$ table operations whatever $s$ is; up to the choice of order this is the best any structure-exploiting exact contraction can do, and it is exactly what the D.1 bound ignores. So for each network we record a lower bound on $w$ (what no order can beat) and the best upper bound several orders achieve, and set both against the three reference exponents: the D.1 bound $s\/sqrt(log_2 s)$ at constant 1, naive enumeration of the free slots $(m-1)b\/m$, and rho $b\/2$.

Everything was built inside the crate the audit covered (`crates/hashed-ecdl-tensor`, Rust): a multiplier switch so `modular_multiply` uses either Theorem 3's Fourier gadget (the default, unchanged) or a schoolbook AND/ripple gadget; the small-x factor base $x < 2^l$ (high bits constant zero, no membership gadget); a Semaev $S_3$ gadget; four languages: *grouplaw-hash* (Bochum Lemma 11, $m = 2$, derived last slot, the crate's own language), *grouplaw-smallx* (same group law, small-x base), *semaev-s3-smallx* ($S_3(x_1, x_2, x_R) = 0$ on abscissae only, $m = 2$) and *chained-s3-smallx* ($S_3(x_1,x_2,t) = 0$, $S_3(t,x_3,x_R) = 0$, $m = 3$, the sparse form of $S_4$); and a width module (Bodlaender–Koster simplicial / almost-simplicial safe reductions, MMD+ lower bounds with min-d and least-c contraction, min-degree, eager and lazy min-fill orders with seeded restarts, every width recomputed exactly from the executed order, PACE `.gr` export and `.td` verifier). The census binary `tensor-treewidth` finds one prime-order curve per $b$ (random $p equiv 3 mod 4$, BSGS point count, certified by `Curve::new`), plants a target as a sum of factor-base points, builds every (relation, multiplier) network, measures it, and at $b <= 10$ evaluates the network on every candidate of its language against direct field arithmetic.

= Protocol

- Curves: one per $b in {8,10,12,14,16,20,24,28,32,36,40}$, seed $20261111 xor (b << 32)$; parameters recorded per row. Slots: $m = 2$, $l = ceil(b\/2)$; chained $S_3$: $m = 3$, $l = ceil(b\/3)$. Hash rows: $s_H$ = first prime $>= 2p$, seeds $(3,1)$, $(5,2)$, $tau = ceil(s_H (floor(sqrt(r))+1)\/r)$.
- Validation ($b <= 10$, schoolbook): every small-x point, every $(x_1, x_2)$ box, every positive tuple of the chained $S_3$ found by scanning $t$ over $FF_p$ plus 200 random tuples, every affine point for the hashed rows; 0 mismatches required, positives counted.
- Width per network: clique bound, MMD+ on the raw graph, safe reductions, MMD+ on the core (lower bound = max of the three); min-degree, eager min-fill ($b <= 16$, and $b = 20, 24$ for the two smallest relations with one restart), lazy min-fill, 3 restarts each where marked; best upper = minimum. Fourier graphs: min-degree only (vertex arity 132–520; the clique bound dominates).
- Cost: $log_2 (sum_"vertices" 2^"arity" + sum_v 2^(|N(v)|+1))$ along the best order *[exact]*.

= Recorded rows

#rows_table

= Slopes

Least squares of the recorded bounds against $b$ *[derived]*. The "best-upper" slope mixes eager min-fill ($b <= 16$) with the weaker lazy/min-degree orders beyond, so the eager-only slope is listed separately.

#slopes_table

#figure(image("treewidth_vs_bits.svg", width: 100%), caption: [Treewidth bounds of the primal graph against $b$; shaded bands are the bracket between the lower bound *[bound]* and the best upper bound *[heuristic, exact for its order]*. Dotted: the enumeration and rho exponent $b\/2$. Seed 20261111, one curve per $b$.])

#figure(image("exponents_vs_bits.svg", width: 78%), caption: [The three exponents on the schoolbook networks: the generic D.1 bound $s\/sqrt(log_2 s)$ (constant 1), the best treewidth upper bound, and naive enumeration $(m-1)b\/m$.])

= Reading

RESULTS_PLACEHOLDER

= Limits

- One curve per $b$; $s$ moves by a few percent with the Hamming weights of $p$ and the planted constants (the schoolbook rows fold constant bits), so $s$ is not monotone to the last digit.
- Upper bounds are greedy orders. The eager min-fill series is the trustworthy one; beyond $b = 16$ only lazy min-fill and min-degree ran, and their widths are noticeably worse (and noisier across tie-breaking seeds) than eager min-fill would give. The `.gr` export is there so a stronger external heuristic (FlowCutter, Tamaki) can tighten them; its output is checked by the crate's `.td` verifier, not trusted.
- Lower bounds are MMD+ on the reduced core: valid, weak. The true treewidth is somewhere in the shaded band.
- Treewidth of the primal graph is the exact-contraction cost model for *Boolean tables*; it does not credit algebraic structure inside a tensor (an F4-style Gröbner step on the $S_3$ system is a different algorithm and is not measured here).
- No pcc claim, no benchmark record (no timing claim), no leaderboard row.
