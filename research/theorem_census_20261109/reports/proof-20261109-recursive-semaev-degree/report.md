---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Canonical Semaev family: last-input degree at every arity

Lean defines the actual canonical resultant family over independent
polynomial variables and proves the following degree theorem over every
field, in every characteristic. [pcc:unverified]

$$\deg_{\text{last input}} S_m=2^{m-2}\qquad(m\ge3).$$

For stage $n$, the arity is $n+3$. Its coefficient ring is defined by
$R_0=K[x_1][x_2]$, $R_{n+1}=R_n[Z]$; one further polynomial layer supplies
the last input $T$. The base is the actual invariant-form S3 quadratic.
Each successor is the actual fixed-size Sylvester resultant with S3,
using the constant coefficient embeddings of $K$ and two fresh coordinates.
The declared sizes are $2^{n+1}$ and 2. [pcc:unverified]

The induction proves the degree of these defined polynomials. Its base
leading coefficient is $(x_1-x_2)^2\ne0$, and its successor has leading
coefficient $P_n(Z)^2\ne0$. Each coefficient ring is a domain. The degree
recurrence is therefore a proved consequence of actual determinants, not
an assumed numerical sequence. [pcc:unverified]

![Actual definitions, general step, induction and remaining obligations.](proof.pdf){width=100%}

\newpage

# Connections, controls and evidence

An explicit polynomial equality identifies the recursive arity-four member
with the earlier canonical S4. The base evaluation formula agrees with
the actual invariant-form S3. The checked successor identity is
$[T^{2^{n+2}}]P_{n+1}=P_n(Z)^2$. No finite-field interpolation supplies these
equalities. [pcc:unverified]

| Arity | Last-input degree, derived from the checked formula |
|---|---|
| 3 | 2 |
| 4 | 4 |
| 5 | 8 |
| 6 | 16 |

The theorem concerns the last input at every arity. Symmetry and the other
input degrees beyond S4, irreducibility and the geometric zero-sum
equivalence beyond S4 remain open. The prior smooth F5 control
$S_4(0,1,4,T)=1$ still disproves unrestricted specialization-degree
preservation. The preceding general-step counterexample separately refutes
resetting the declared Sylvester size after specialization. Neither refutes
the qualified generic degree theorem. [pcc:unverified]

Source: ../../lean/RecursiveSemaevDegree.lean. Key bindings are
`recursivePolynomial`, `basePolynomial_eval`,
`recursivePolynomial_one_eq_canonicalS4`, `recursivePolynomial_succ_coeff`,
`all_arities_last_input_degree` and the explicit combined audit certificate.
The whole tracked census gate passes 308 declarations, 0 failing and
0 trusted-native exceptions; counts include generated declarations.
Only propext, Classical.choice and Quot.sound are accepted. [pcc:unverified]

Source commit 196d2fac precedes issue. Packet PCC-20261109-3bc113 pins the
combined theorem; its full check receipt is
../../evidence/pcc-recursive-semaev-degree-check.json. Standing remains
unsigned until a separate signed verdict. [pcc:unverified]

Supplement IDC1h0872017c01480c3d checks only the scalar monic-square
resultant for fixed $D=8$, with the actual ten-by-ten determinant AST:
32 derived and 11 extra points over $2^{61}-1$, 10 variables, total degree
bound 18. The unchanged Rust checker accepts, with a matching proof-file
digest. These finite one-host checks do not establish the all-arity
coefficient/degree theorem or other-characteristic conclusions; the
universal mathematics rests on Lean. [pcc:unverified]

Primary read scopes: Semaev2015 §2/pp2--3 and Kosters--Yeo2015 §2/p3;
exact versions, URLs and hashes are in ../../recursive-semaev-degree-sources.json.
Original Semaev2004 remains unread after 403/404 responses. Related sections
do not replace original-source reading or a full-paper audit. [pcc:unverified]

All five standalone attempts are retained: four failed intermediate builds,
then one pass before the final combined tracked gate. No hypothesis was
silently weakened or supplied as an axiom. No new curve population,
statistical study, benchmark, frozen-data change, paid compute or separate-host
run occurred. The available canonical scoreboard remains unchanged; this
proof introduces no performance/curve metric. The original fourteen-item
request remains partial. [pcc:unverified]
