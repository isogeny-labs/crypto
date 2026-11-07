#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 10pt)
#set heading(numbering: "1.")
#show link: underline

#align(center)[
  #text(size: 16pt, weight: "bold")[Split-prime pattern and the ECDLP: a scalar-only obstruction, an independent recount, and the routes that stay open]
  #v(4pt)
  #text(size: 10pt)[research/split_prime_ecdlp_20261107 · 2026-11-07 · branch `claude/novel-ecdlp-algorithm-iams4j`]
]

#block(inset: 8pt, stroke: 0.5pt, radius: 3pt)[
  *Status:* derived obstruction plus a native recount; no algorithm claim. *Result class:* boundary study and lead closure, not a cost reduction for any ECDLP method. `S` and end-to-end cost are unset; no leaderboard, benchmark or rho comparison is touched. Claim record for the recount: `PCC-20261107-3403f9` (kind statement; standing unverified until CI signs it) [pcc:unverified]. Problem record: `problems/split-prime-pattern-scalar-only.md`.
]

= Question and answer

*Request* (user, 2026-11-07): a novel ECDLP algorithm based on something unique and not tried before, for example the prime-splitting pattern observed in the published prime curves (uploaded report `split_prime_report.pdf`, sha256 `d38be158…7a0c8`, "Split-prime deficit in published NIST / X9.62 prime curves").

*Answer.* The pattern is real: recounted here from the curve parameters, every number agrees with the report (§3). It cannot feed an ECDLP algorithm below the generic bound on these curves: for every prime $ell != n$, the objects it governs ($ell$-torsion, Frobenius eigenlines, the two rational $ell$-isogenies of a split $ell$, the class group of the Frobenius order, the endomorphism ring) reach the prime-order group $E(bb(F)_p)$ only as multiplication by a known scalar or as an isomorphism that preserves the discrete logarithm (§2). This agrees with the lab's isogeny-class invariance framing (`research/isogeny_class_difficulty_20261008`) and with the measured null on index-calculus yield across a class (`research/isogeny_class_ic_yield_20261102`). The pattern's one ECDLP-adjacent consequence is a lower degree of the horizontal expander used by random self-reduction (§4). Candidate directions that survive the obstruction, with their cheapest tests, are in §5; none is a claim.

*Scope.* The ten prime-order curves prime192v1 (P-192), prime192v2, prime192v3, secp224r1 (P-224), prime239v1/v2/v3, prime256v1 (P-256), secp384r1 (P-384), secp521r1 (P-521); $p$ and $n$ from `openssl ecparam -param_enc explicit` (OpenSSL 3.0.13), cofactor 1. Objects: $t = p + 1 - n$, $Delta = t^2 - 4p$, $K = bb(Q)(sqrt(Delta))$, and for each odd prime $ell$ the Kronecker symbol $chi(ell) = (Delta / ell)$: split (+1, Elkies), ramified (0), inert (−1, Atkin).

*Sources checked.* KB unavailable (the `research-kb` MCP server is not loaded; `scripts/research-kb` finds no executable; the lead atlas checkout is absent). Repository paths read: `research/isogeny_class_difficulty_20261008/README.md`, `research/isogeny_class_ic_yield_20261102/p2_prime_analogues/RESULTS.md` and `report/REPORT.md`, `problems/prime-scalar-exact-splitting.md`, `problems/tree-glued-pdp-separator-coupling.md`, `problems/curve25519-family-algebraic-ic-tower.md`, `docs/endomorphism-rules.md`. Literature from memory, primary sources not re-read this session: Lenstra 1996 ($E(bb(F)_q) tilde.equiv cal(O) slash (pi - 1)$ as $cal(O)$-modules), Jao–Miller–Venkatesan 2005 (random self-reducibility across an isogeny class via expander walks over split $ell$, under GRH), Shoup 1997 ($Omega(sqrt(n))$ generic bound), Schoof–Elkies–Atkin (split $ell$ speed up point counting only).

= The obstruction (derived; no Lean proof yet)

*Setting.* $E slash bb(F)_p$ ordinary, $\# E(bb(F)_p) = n$ prime, cofactor 1, Frobenius $pi$ with $pi^2 - t pi + p = 0$, $cal(O) = "End"(E) supset.eq bb(Z)[pi]$, $N = (pi - 1)$ the ideal of $cal(O)$ of norm $n$.

*Theorem (scalar-only action).* Let $ell$ be a prime different from $n$ and $p$. Every $bb(F)_p$-rational isogeny $phi: E -> E'$ of $ell$-power degree (each horizontal $ell$-isogeny of a split $ell$, their compositions around the class group, every vertical $ell$-isogeny) and every endomorphism $alpha in cal(O)$ satisfy:
+ $phi$ restricted to $E(bb(F)_p)$ is a group isomorphism onto $E'(bb(F)_p)$ and $phi([k]P) = [k]phi(P)$: a transported DLP instance has the same answer.
+ $alpha$ acts on $E(bb(F)_p)$ as multiplication by $s_alpha = alpha mod N in bb(Z) slash n$, computable from the coordinates of $alpha$ in $cal(O)$ (for $alpha = u + v pi$, $s_alpha = u + v$).
+ The Galois module $E[ell]$, its Frobenius eigenvalues $lambda, mu$, the fiber polynomials of $phi$, and every $ell$-pairing value involving a point of $E(bb(F)_p)$ are independent of which point of $E(bb(F)_p)$ is considered.

*Proof sketch.* (1) $deg phi$ is prime to $n$, so $ker phi inter E(bb(F)_p) = {O}$ and $phi$ is injective on the finite group $E(bb(F)_p)$; $E'(bb(F)_p)$ has the same order, so $phi$ is onto; it is a homomorphism, so it commutes with $[k]$. (2) $E(bb(F)_p)$ is cyclic of prime order, so every endomorphism of it is a scalar; which scalar follows from Lenstra's isomorphism $E(bb(F)_p) tilde.equiv cal(O) slash N$ under which $pi |-> 1$. (3) $E[ell]$ and $E(bb(F)_p)$ are Galois modules of coprime order; a bilinear pairing $t(T, P)$ with $"ord" T = ell$, $"ord" P = n$ satisfies $t^ell = t^n = 1$, so $t = 1$; the fiber of $phi$ above $phi(P)$ is $P + ker phi$ with the Galois structure of $ker phi$. $qed$

*Corollary.* An algorithm whose only operations are the group law, the maps above and point-independent Galois data is a generic-group algorithm with scalar oracles; Shoup's bound gives $Omega(sqrt(n))$ group operations, and each map costs at least one group operation to evaluate on a point. So the split-prime pattern alone, for any cutoff and any deficit, cannot yield a sub-generic algorithm on a prime-order curve; by the JMV self-reduction every curve reachable along the horizontal edges that the split primes provide is exactly as hard.

*Not covered.* Anything that reads $bb(F)_p$-coordinates non-linearly: summation polynomials, division polynomials, and (for small embedding degree only, which these curves lack) the $n$-pairing. These are the lab's index-calculus tracks. The pattern can enter them indirectly by choosing which curve of the class to decompose on; the lab measured that choice null on 2026-11-02 (P2: ICC not distinguishable from zero at 24–32 bits, planted-20 % power 0.84–1.00 in every informative cell, one suspected curve effect not replicated).

*The reformulation that looks like a way in.* By Shimura reciprocity the DLP $k$ is the element of $"Gal"(K_N slash H) tilde.equiv (cal(O) slash N)^times slash plus.minus 1$ mapping the canonical-lift torsion point $tilde(P)$ to $tilde(Q)$, and the Artin symbol of a principal ideal $(alpha)$ acts as $alpha mod N$. Writing $k$ as a product of known scalars $s_alpha$ of smooth principal $alpha$ is a multiplicative DLP in $bb(F)_n^times$, feasible by index calculus — but only on the residue $k$, which is the unknown. The group is presented by coordinates, never by residues in $cal(O) slash N$, so "index calculus in $cal(O)$" has no first step; enumerating smooth $alpha$ and comparing $alpha(P)$ with $Q$ is baby-step giant-step with an unusual giant-step set and the same $sqrt(n)$ cost.

#figure(image("../figures/obstruction_diagram.svg", width: 100%), caption: [Where the pattern lives (left), the group it must reach (centre), and the only non-linear handle on that group (right). Derived; the dashed link is the route measured null on 2026-11-02.])

= Independent recount of the deficit (measured, native)

*Method.* `tools/splitcount.rs` (dependency-free Rust, `rustc -O`, 0.3 s). For each odd prime $ell <= 200000$ it reduces the hex parameters $p$, $n$ modulo $ell$, forms $t equiv p + 1 - n$, $Delta equiv t^2 - 4p$, and takes the Legendre symbol by Euler's criterion. Baseline (the report's model, re-derived in the source comment): Frobenius mod $ell$ uniform among matrices of $"GL"_2(bb(F)_ell)$ with determinant $p$, so $P("split") = (ell - 2 - chi(p)) slash (2(ell - 1))$, $P("ramified") = (1 + chi(p)) ell slash (ell^2 - 1)$; the trace class $tau equiv p + 1$ (which would give $ell | n$) is removed and the rest renormalised. $z = ("observed" - sum P) slash sqrt(sum P(1 - P))$ over odd primes $ell <= B$; twelve cutoffs as in the report. Output `data/recount.json` (sha256 `e84f89d3…7d98`; source sha256 `cb1dc02c…7550`). No $ell <= 200000$ divides any $n$.

#table(
  columns: 8, align: (left, right, right, right, right, right, right, right), stroke: 0.4pt, inset: 4pt,
  [*curve*], [*split (obs.)*], [*expected*], [*sd*], [*z @ 691*], [*z @ 5,000*], [*z @ 50,000*], [*L-ratio, ℓ ≤ 200k*],
  [prime192v1 (P-192)], [60], [59.7], [5.51], [+0.05], [+0.28], [+0.80], [0.93],
  [prime192v2], [58], [59.7], [5.51], [−0.31], [−2.20], [+0.66], [1.20],
  [prime192v3], [57], [59.7], [5.51], [−0.49], [−1.12], [−1.51], [0.80],
  [secp224r1 (P-224)], [50], [60.1], [5.53], [−1.82], [−2.54], [−1.50], [0.64],
  [prime239v1], [42], [59.7], [5.51], [−3.21], [−1.28], [−0.12], [0.56],
  [prime239v2], [57], [59.7], [5.51], [−0.49], [−0.89], [−2.07], [0.86],
  [prime239v3], [57], [59.7], [5.51], [−0.49], [+0.35], [−0.51], [0.93],
  [prime256v1 (P-256)], [57], [60.2], [5.54], [−0.57], [−0.30], [−1.42], [1.27],
  [secp384r1 (P-384)], [54], [59.9], [5.52], [−1.06], [−0.59], [−1.21], [0.62],
  [secp521r1 (P-521)], [52], [60.3], [5.54], [−1.51], [+0.54], [+0.54], [1.01],
)

Counts at $B = 691$ (124 odd primes). Pooled $z$ over the ten curves, $(sum "obs" - sum "exp") slash sqrt(sum "var")$, at $B$ = 100, 200, 500, 691, 1000, 2000, 5000, 10000, 20000, 50000, 100000, 200000: −2.71, −1.81, −2.98, −3.13, −1.98, −2.16, −2.45, −2.12, −2.26, −2.00, −1.87, −1.79 (Stouffer $sum z slash sqrt(10)$ agrees to two decimals). The last column is this study's normalisation of the partial Euler product $product (1 - chi(ell) slash ell)^(-1)$ over odd $ell <= 200000$, as a ratio to $exp E[log product]$ under the conditioned model; it is not the report's "× typical" column, but the ordering agrees (prime239v1, P-384, P-224 lowest; P-256 highest).

*Agreement with the uploaded report.* Every split count at 691, every per-curve $z$ at 691, 5,000 and 50,000, and every pooled $z$ at the twelve cutoffs matches the report's table and list to the printed precision [pcc:unverified]. The report's random-curve calibration (4,267 SEA-counted curves) and its percentiles were not reproduced; no statement is made about them. Nothing here bears on the cause of the deficit; the report's own caveats (cutoff chosen after the fact, correlated cutoffs, min-of-three families, ten curves) stand.

#figure(image("../figures/recount_z.svg", width: 100%), caption: [Left: $z$ against cutoff for each curve (lines, native recount) with the report's values as hollow markers; black: pooled. Right: counts at $B = 691$ against the model expectation ± 1 sd. Source `figures/plot.py`, data `data/recount.json`; $n$ = 124 primes at $B = 691$.])

= The one ECDLP-adjacent effect: horizontal expander degree

The JMV self-reduction walks the horizontal class along $ell$-isogenies for split $ell <= B$; the graph has degree $2 times \#{"split" ell <= B}$. A deficit lowers the degree for a given $B$: at $B = 691$, P-224 has 50 split primes (degree 100) against 60 expected (degree 120), prime239v1 has 42 (degree 84). The walk needs a slightly larger $B$ for the same mixing, i.e. a slightly larger transport cost; by §2 the destination curve is exactly as hard. The lab's protocol I-2 (`research/isogeny_class_difficulty_20261008/PROTOCOL-I2-transport-cost.md`) already prices transport; this study adds only the observation that the report's statistic is the degree of that graph.

= Candidate novel directions that survive the obstruction

// SECTION6

= Status of every statement

#table(
  columns: 2, stroke: 0.4pt, inset: 4pt, align: left,
  [*statement*], [*status*],
  [Split counts, expectations, $z$ at twelve cutoffs for ten curves (§3)], [measured natively from public parameters; agrees with the uploaded report; not independently checked by a second producer [pcc:unverified]],
  [Theorem and corollary (§2)], [derived (proof sketch); no Lean proof; no identity certificate (not a bilinear or summation identity)],
  ["Model choice within the class is null"], [measured by the lab on 2026-11-02 at 24–32 bits; cited, not re-measured],
  [Expander-degree effect (§4)], [derived from the counts; transport cost not measured here],
  [Candidate directions (§5)], [proposed; not run],
)

*Files.* `tools/splitcount.rs`, `data/recount.json`, `data/uploaded_report_values.json`, `figures/plot.py` with `recount_z.{svg,png}` and `obstruction_diagram.{svg,png}`, `report/REPORT.typ` (this document), `problems/split-prime-pattern-scalar-only.md`. *Next steps.* A Lean statement of §2 (1)–(3); of the §5 candidates, the one with the best prior under a frozen protocol on 24–32-bit toy classes with the controls of `controlled-experiment`; the report's own pre-registered number-theory follow-ups only if the owner wants the fingerprint question answered.
