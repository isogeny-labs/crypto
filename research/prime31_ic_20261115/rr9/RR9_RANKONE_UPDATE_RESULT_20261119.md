# Prime31 RR9 one-leaf rank-one chart update: exact original-curve B8 gate

**Recorded November 19, 2026 UTC.** [Draft PR #4783](https://github.com/isogeny-labs/crypto/pull/4783), stacked on parent [#4780](https://github.com/isogeny-labs/crypto/pull/4780). Protocol frozen **before** source/benchmark: [protocol](RR9_RANKONE_UPDATE_PROTOCOL_20261119.md). **Source-bound GitHub Actions PASS:** [run #44458247199](https://github.com/isogeny-labs/crypto/actions/runs/44458247199), candidate commit \`af1cc2108e9ae3b213e73536619647fa93ef2d8e\`; original native arithmetic unchanged from \`1df714645aa683cf975bc76fcacb9842cb4780fd\`. [pcc:unverified]: no independently **signed** PCC verdict has been obtained.

The data establishes an exact faster *four-row chart preprocessing kernel*, not a public-target unknown-index extractor or a cheaper complete prime-field ECDLP solver.

## Exact mathematical identity

On original \`p=2147483647\`, \`E: y²=x³+847514846x+2087444724\`, order \`n=2147446831\` (cofactor 1), use the same target-conditioned eight-point RR9 row of #4776/#4780,

\[
h(x,y;R)=[(x-r)x^i]_{i=0}^3\ \vert\ [(y+s)x^i]_{i=0}^3,\qquad R=(r,s).
\]

For a half matrix \`H=[A|B]\` with \`det A != 0\`, let \`M=A^{-1}B\`. Replace row \`j\` of \`A\` with \`a'\` and of \`B\` with \`b'\`. Set

\[
u=A^{-1}e_j,\qquad d=1+(a'-a_j)u,\qquad v=b'-a'M.
\]
Then the determinant lemma gives \(\det A'=\det A\cdot d\), and whenever \(d\ne0\),

\[
\boxed{\quad M'=M+\frac{u}{d}v.\quad}
\]

**Proof:** \(A'=A+e_j(a'-a_j)\) and \(B'=B+e_j(b'-b_j)\). Since \(B=AM\), \(B'-A'M=e_j(b'-a'M)\). Thus \(M'-M=A'^{-1}e_jv\). Sherman–Morrison yields \(A'^{-1}e_j=u/d\), proving the identity. If \(d=0\), \(\det A'=0\) and the chart must fail closed, not produce a negative relation assertion.

For RR9 rows specifically, \(A=\operatorname{diag}(x_i-r)\operatorname{Vandermonde}(x_i)\), and \(M\) encodes interpolation of the rational ordinates \((y_i+s)/(x_i-r)\). This explains the rank-one structure. The determinant lemma and rank-one update are **classical identities**; no originality is claimed.

## Exact original-curve and synthetic verification

The Rust example reuses the pinned original \`prime31_algebraic_ic::Arithmetic\`, original RR9 point rows, original exact 8x8 determinant and existing graph-chart Schur predicate. No generic EC group-sum tables enter the candidate kernel.

- Two public residuals: ordinary generator \(G=(625037715,2100627251)\), plus the sum of the eight first positive signed slot leaves, \((1991076510,718644930)\).
- Canonical B8 abscissae: \`[0,1,2,3,5,6,8,9]\`. Original weighted supports \`W=[1,3,5,7,11,13,17,19]\`, with both signs. Each residual has \(4\times16=64\) one-row substitutions: **128/128** matched full chart recomputation; each paired Schur determinant residue equals a separately computed 8x8 determinant.
- Of these original-curve cases, **4** give zero determinant and **124** nonzero; no failed chart. 128 deterministic synthetic 4x8 random-matrix identities and a singular repeated-row negative gate passed.
- The independent Python BigInt verifier *reconstructed the original curve, scalar multiplications, signed slot support, full determinant, rational chart inverse and update for all 128 entries*, without calling the candidate's Rust arithmetic. Source-specific CI passed 40/40 original library tests and the added example's exact tests. These are producer/CI tests; independently signed PCC remains [pcc:unverified].

The immutable [raw JSON artifact #14498379058](https://github.com/isogeny-labs/crypto/actions/runs/44458247199/artifacts/14498379058) has ZIP SHA256 \`2cec081b37a82568a0057b1ad93eba86f15b9220bcf0163fc14f2c95f0ed5d05\` and uncompressed JSON SHA256 \`1981520fbed78958095c701e9a4e9e6e00964ab5f785f94bfab74472c028a7ce\`. [Machine-readable frozen summary](RR9_RANKONE_UPDATE_SUMMARY_20261119.json).

## Same-host chart-only A-B-B-A diagnostic

The preregistered schedule repeats all 128 same-source swaps **500 times = 64,000 chart calls per arm**. The update arm charges the old chart and \(A^{-1}\) construction once per 64 same-target sibling substitutions in *each* repeat. Both arms produce the **same arithmetic digest**, \`1619931901\`. These are chart-preprocessing times; setup of public weighted EC supports is outside both arms, and no public B8 extraction pair scan was performed.

| 64,000-chart diagnostic (GitHub runner, release Rust 1.100.0) | Fresh four-row graph chart | Rank-one update |
|---|---:|---:|
| A-B-B-A first | 36.730208 ms | 11.862426 ms |
| A-B-B-A second | 36.520456 ms | 11.895897 ms |
| **Mean** | **36.625332 ms** | **11.8791615 ms** |
| **Local preprocessing ratio** | 1.000x | **3.083x faster** |

The rank-one kernel uses one field inversion per swap plus the reusable old inverse; fresh construction uses four pivot inversions. The time includes shared-base chart/inverse setup **amortized across 64 siblings**. It would be misleading to extrapolate the 3.083x directly to a whole candidate or ECDLP attack.

## Actual stopping point and competitive frontier

Both algorithms still require exactly \(16^8=4,294,967,296\) right/left chart-pair predicates for one fully exhausted negative **B8** target under the #4780 outer search. With the current 30-product difference determinant, that is \(128,849,018,880\) pairwise field products **per negative target**, *unchanged by this update*. Even completely free chart generation would leave this work. This experiment does not use B8 natural 46-positive/18-negative blinded targets or recover any factor-base unknown indices.

The separate matched original-Rust 1,024-log benchmark [#4774](https://github.com/isogeny-labs/crypto/pull/4774) had signed shared BSGS **3,342.793 ms** versus ternary generic-table W8 IC **7,834.933 ms**. Those figures are a distinct job and cannot be compared as same-runner ratios with this 11.9 ms preprocessing component. [#4728](https://github.com/isogeny-labs/crypto/pull/4728) demonstrates 31/31 rank and 1,024/1,024 verified logs *using generic 4+4 group-sum tables*, not algebraic unknown-index extraction.

## Prior lead and literature audit

1. Existing RR9 algebra: [#4466](https://github.com/isogeny-labs/crypto/pull/4466) Hermite; [#4547](https://github.com/isogeny-labs/crypto/pull/4547) seven-anchor generic completion; [#4728](https://github.com/isogeny-labs/crypto/pull/4728) four-window/weighted algebra and generic training; [#4776](https://github.com/isogeny-labs/crypto/pull/4776) exterior 70 terms; [#4779](https://github.com/isogeny-labs/crypto/pull/4779) separate fixed Plücker linear-span audit; [#4780](https://github.com/isogeny-labs/crypto/pull/4780) 30-product four-plane graph determinant; and [#4692](https://github.com/isogeny-labs/crypto/pull/4692), already owning nonenumerative shared-coefficient remainder/F4 extraction. This PR tests only exact rank-one *chart preprocessing*; it does not duplicate #4692's unknown-index scope.
2. Additional pipelines: [#4773](https://github.com/isogeny-labs/crypto/pull/4773) 256-branch two-prefix, [#4774](https://github.com/isogeny-labs/crypto/pull/4774) complete matched BSGS; [#4738](https://github.com/isogeny-labs/crypto/pull/4738) Table-B Semaev/sibling/wwalk controls approaching counted rho at some small-prime cells but still generic \(\sqrt r\) complexity, and [#4747](https://github.com/isogeny-labs/crypto/pull/4747) larger 2^36 amortized table-build tuning (a different curve/workload). They are not evidence of a cheaper non-generic *Prime31* complete solver.
3. Negative autoresearcher lanes: [SA-04 and SA-18 determinant/norm outcomes](https://github.com/isogeny-labs/crypto-autoresearcher/blob/d54c01d325b9e4508d51f56dc113833be27ddc9a/experiments/semaev-alternatives-20261105/SUMMARY.md); separate native prime-field IC implementation [cryptanalysis reference](https://github.com/isogeny-labs/cryptanalysis/blob/6b64ee6ca07f92db014cf24d277439730ac31f4b/suite/src/cryptanalysis/ec_index_calculus.rs).
4. Published prime-field algebraic IC: [Petit–Kosters–Messeng PKC 2016](https://doi.org/10.1007/978-3-662-49387-8_1); [Amadori–Pintore–Sala FFA 2018](https://doi.org/10.1016/j.ffa.2018.01.009); [Kudo–Yokota–Takahashi–Yasuda 2018 hybrid/symmetry](https://kyushu-u.elsevierpure.com/en/publications/acceleration-of-index-calculus-for-solving-ecdlp-over-prime-field/). These are prior non-generic avenues; their claims do not eliminate the need for costed complete solver measurements.

The repository's research-KB tool/CLI was unavailable in this connector runtime, so this is a bounded cross-PR/repository/literature audit, **not** an exhaustive novelty proof.

## Priority experiment queue (strict pass/fail)

**P0 — unknown-index extraction, not faster chart arithmetic.** Coordinate with #4692 to construct a truly public-only B8 selector: e.g., an F4/remainder-tower or structured incidence solver exploiting low-degree/interpolation relations. Input only the public residual, curve and unsigned/signed factor-base supports. Do not reveal planted words, use generic 4+4 group sums, or count exact determinant comparisons as nonenumerative unless the outer \(16^8\) search is actually removed. Test the registered **46 positive / 18 negative natural B8 targets**, hold out target identities, report memory, roots, all failures and exact Fp work. Require independent group replay of every recovered witness.

**P1 — compare to the fastest competing IC frontier.** If P0 passes, feed its *real measured* relation cost into #4728's existing Rust four-window train/recover implementation and run 31/31 rank and 1,024 blinded verified logs, including rejected queries and linear algebra. Compare the actual cold-one, 64, 1,024 full-cost wall times and field/EC counts on one host against optimized x/y-variants, APS/Semaev and strongest BSGS/rho. No weak/special curve substitution.

**P2 — scalability and falsification.** Repeat non-generic B8 selector at factor-base sizes 4,8,12,16,24,31 and on a ladder of ordinary cofactor-one prime curves; report failure modes and distinguish component improvements from exponent changes. If any gate fails, record the negative and stop promoting that lane.

**Conclusion:** This run completed the pre-registered rank-one correctness and exact-cost gate, with a measured **3.083x chart-only preprocessing reduction**, while leaving B8 exhaustive pairwork and full ECDLP cost untouched. [pcc:unverified].
