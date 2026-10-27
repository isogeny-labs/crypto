# Interpretation corrections to the 2026-10-22 study

Date: 2026-10-25. Classification: accounting and scope corrections. The
parent's frozen measurements, reports and replay records are unchanged.
These corrections affect interpretation, not its pooled rho ratio 0.9958.

1. The parent's `log_p(N)=0.370557...` describes one parameter tuple.
   It is not an observed or proved asymptotic exponent. Direct evaluation
   costs O(N). A claim in p requires a specified growing parameter family.
   With fixed depth d and fixed D0, the construction uses
   p=t^2/4+abs(D0)*4^(d-1)*N^2; the displayed logarithm alone does not
   justify the README's O(p^0.37) asymptotic statement.
2. The approximately 0.04-second transfer figure counts two evaluations
   after the map exists. The original receipt separately records
   0.145098486 seconds of construction and 0.018640696 seconds per point.
   Their additive descriptive total is 0.182379878 seconds, before other
   parameter-search/certification costs. Neither the warm evaluation figure
   nor this shared-VM sum is an isolated cold-speed result. Later optimized
   construction has its own receipts and must not replace these values.
3. The transfer theorem supplies an algorithm on the source at destination
   solve cost plus map discovery/setup and two point evaluations. It does
   not assert identical optimal costs, identical constants, or identical
   exponents when transfer dominates the destination solver. In general,
   O(q^alpha)+O(q^beta) has the larger exponent. Galbraith's reported
   small-crater O-tilde(q^(1/4)) transfer bound and general heuristic
   O-tilde(q^(2/5)) bound therefore have different implications: the latter
   does not preserve every solver exponent above 1/4. A hypothetical
   q^(1/3) destination solver, for example, transfers at the q^(2/5)
   bound under the general guarantee alone.
4. Pooled equivalence is not equivalence at every level. The original
   primary average gives 75.8% of its weight to levels five and six.
   Its crater run-level 95% interval is approximately [0.580,1.528].
   The new protocol allocates equal target counts to each level and
   reports nominal and simultaneous per-level intervals explicitly.
5. Lean proves the abstract injective-homomorphism transfer and conditional
   arithmetic lemmas. The parent proof map explicitly leaves elliptic-curve
   isogeny, endomorphism-order and geometry bridges outside that formalization.
   This replication adds no claim that those bridges are formally proved.

Sources: parent README lines 15-21 and 166-173; THEOREMS.md Theorems 2 and 5;
`research/deep_volcano_prime_gap_20261022/run/pair.json`; original analysis
and validation receipts. New measured results remain in this study only.
