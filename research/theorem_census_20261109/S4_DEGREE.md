# S4 degree: symbolic inputs and specialization

Lean proves symbolic individual degree 4 in every input position of the
actual S4 resultant, over any field and in every characteristic. Each
chosen position is the outer variable of K[x1][x2][x3][T], with the other
three inputs independent formal variables. [pcc:unverified]

For any choice of position, after fixing the other three coordinates x,y,z,
the canonical polynomial has degree at most 4, fourth coefficient
S3(x,y,z)², and exact degree 4 if and only if S3(x,y,z)≠0.
The proof computes the actual Sylvester determinant over its polynomial
coefficient ring; it includes explicit map/evaluation compatibility with
the previously checked scalar S4. It never infers polynomial equality
from finite-field pointwise agreement. [pcc:unverified]

On the actual smooth F5 curve y²=x³−x, the specialization S4(0,1,4,T)
is the nonzero constant 1 and has degree 0. Lean explicitly disproves
universal preservation of degree after fixing other coordinates. This
refutes that strengthening, not the qualified generic degree theorem.
[pcc:unverified]

The [source](lean/S4Degree.lean) includes:

- `quadratic_resultant_ring`: actual determinant over any commutative ring.
- `quartic_coeff_four`: universal leading-term coefficient identity.
- `positionPolynomial_curve_eval`: actual scalar S4 evaluation in every position.
- `positionPolynomial_curve_degree_iff`: the sharp specialized degree criterion.
- `all_four_symbolic_degrees`: degree 4 with the remaining three inputs formal.
- `actual_specialization_constant`, `specialized_degree_preservation_false`: faithful polynomial control.
- `s4_degree_audit_certificate`: explicit combined declaration binding these scopes.

The entire census gate passes 259 declarations, 0 failing and 0 trusted-native
exceptions. Counts include generated declarations and are not 259 new
mathematical results. Packet PCC-20261109-b93977 binds the combined
statement and remains unsigned. No custom axiom, proof hole or
native_decide exception is accepted. [pcc:unverified]

The full local packet check passes at17:28:09Z. Exact source, gate, packet
and native supplementary identity digests are bound in
[s4-degree-verification.json](s4-degree-verification.json).
[pcc:unverified]

Supplement IDC1h9d53585cac843775 checks only the rewrite
Res2,2(aX²+bX+c,(X−z)²)=(az²+bz+c)², with the unchanged repository checker:
4 variables, degree bound 6,32 derived plus11 extra points over 2^61−1.
Local source commit 3d1dd96b preceded issue; its proof-file digest matches.
These one-host finite checks do not establish generic degree, actual
coefficient extraction, other characteristics or independent/signed
standing. The universal mathematical conclusions rest on Lean.
[pcc:unverified]

Primary review: [Semaev2015](https://arxiv.org/pdf/1504.01175), Section 2,
pp2–3; page 3 visually checked for exponent m−2 and algebraic-closure
coordinates. [Kosters–Yeo2015](https://arxiv.org/pdf/1503.08001), Section 2/p3,
supplies the general invariant formula/recursion. Exact hashes/read
scopes and planned coefficient-check source are in
[s4-degree-sources.json](s4-degree-sources.json).
Original Semaev 2004remains unread after the documented 403/404 responses.
Related papers do not replace reading the original. [pcc:unverified]

Scope is m=4. General all-m recursion/degree, m5 degree and irreducibility
proofs remain open. No solving-degree, regularity, algorithm-complexity
or all-fourteen completion is inferred. Earlier frozen native populations,
protocols and raw data are unchanged; no new curve population, benchmark
or paid compute was run. [pcc:unverified]

Reproduce the formal check with
`tools/lean/proof-gate.sh research/theorem_census_20261109/lean`.
See [gate](evidence/s4-degree-gate.log) and
[proof diagram/report](reports/proof-20261109-s4-degree/report.pdf).
All eleven standalone build logs are retained, including the failed attempts.
[pcc:unverified]
