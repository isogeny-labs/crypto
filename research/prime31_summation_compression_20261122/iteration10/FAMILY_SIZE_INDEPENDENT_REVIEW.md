# Independent review of the smooth-modulus family correction

Date: 22 November 2026 America/Los_Angeles / 23 November 2026 UTC.

**Disposition.** The correction is accepted as a bounded, source-backed
mathematical derivation. Its power-of-two selection, divisibility, growth,
fixed-source density application and conditional four-summand balance all
check. No correction to the reviewed bytes is requested. This review does
not establish a relation-solving algorithm, a growing prime-order source
family, or a machine-checked theorem. **[pcc:unverified]**

## 1. Exact reviewed inputs and method

| Input | SHA-256 |
| --- | --- |
| `FAMILY_SIZE_CORRECTION.md` | `841704b12548a1ab4c2e35062cff9b48a8a9229dd83d2e7b3426a629501c996d` |
| `iteration7/FIXED_CURVE_FIBRE_DENSITY.md` | `c1725df2f00f61a972edbb5cfee803fe97bd2c27d8f761fbb08b73385c70b7a1` |

The first input was read in full. The second input's hypotheses, exact
regular-fibre partition, sampling multiplicity and charged construction
recurrence were re-read; its earlier independent mathematical review is
inherited rather than promoted to a signed or machine-checked proof here.

The primary analytic input was independently read in Mei-Chu Chang,
*Short character sums for composite moduli*, Corollary 11, printed pages
4 and 39 of the [author's PDF](https://math.ucr.edu/~mcc/paper/143%20Char060114.pdf).
Both the opening statement and the deduction following equations
(8.4)–(8.5) were inspected. The latter explicitly handles the exceptional
real-zero case, so this application does not silently assume its absence.
No secondary least-prime estimate is substituted for that source.

This is source reading and written algebra only. No prime search, point
count, finite-field experiment, benchmark, cryptographic execution or
remote mutation was performed.

## 2. The prime family satisfies the primary hypothesis

Put \(Q_k=2^{k+2}\). Its largest prime factor is 2, so

\[
\frac{\max_{\ell\mid Q_k,\ \ell\text{ prime}}\log\ell}{\log Q_k}
=\frac{1}{k+2}\longrightarrow0.
\]

The residue class \(-1\) is coprime to \(Q_k\). Chang's corollary therefore
supplies a prime in that class below
\(Q_k^{12/5+\varepsilon_k}\), with \(\varepsilon_k\to0\). It is enough to
choose the least prime in the class; the subsequent argument does not
require an estimate for prime density inside the class. [pcc:unverified]

Every positive integer congruent to \(-1\pmod {Q_k}\) is at least
\(Q_k-1\). Consequently the selected primes tend to infinity as \(k\)
increases. Repetition for finitely many adjacent moduli causes no issue:
there is an infinite distinct-prime subsequence. In particular, this does
not assume infinitely many Mersenne primes.

## 3. Divisibility survives the requested exponent selection

Fix a constant \(0<\beta<5/12\), and set

\[
e_k=\lfloor\beta\log_2p_k\rfloor,
\qquad N_k=2^{e_k}.
\]

Choose a constant \(\eta>0\) such that
\(\rho=\beta(12/5+\eta)<1\). Once
\(\varepsilon_k<\eta\),

\[
e_k\le\beta\log_2p_k
<\rho(k+2)<k
\]

for every sufficiently large \(k\); the final inequality follows from
\(k(1-\rho)>2\rho\). Since \(e_k\) is an integer, this even gives
\(e_k\le k-1\) eventually. The weaker conclusion used by the correction,
\(N_k\mid 2^k\), is therefore valid. Hence

\[
4N_k\mid Q_k\mid p_k+1.
\]

This checks the factor-of-four margin explicitly; merely saying that
\(N_k\le p_k^\beta\) would not have established the required divisibility.
[pcc:unverified]

The floor supplies

\[
\frac12p_k^\beta<N_k\le p_k^\beta.
\]

Since \(\beta<1/2\), eventually
\(p_k^{1/2-\beta}\ge16\), and therefore
\(N_k\le\sqrt{p_k}/16\). Also \(N_k\ge4\) and \(p_k\ge4096\) eventually.
The endpoint \(\beta=5/12\) is correctly excluded: the slack needed for
\(\rho<1\) is not provided by an unspecified \(o(1)\) exponent there.
The statement concerns each fixed interior exponent, not a uniform
algorithmic estimate for exponents approaching the endpoint.

For a fixed rational \(\beta\), choosing this cutoff need not use
floating-point logarithms. Integer power comparisons can determine the
largest power of two satisfying the threshold. Their bit costs remain
part of ordinary input/setup arithmetic; this observation is not a new
cryptographic computation performed in the review.

## 4. The fixed-source density lemma applies with its original scope

The reviewed iteration7 lemma requires a supplied nonsingular
short-Weierstrass curve over a prime field with odd rational group order,
and a power of two satisfying

\[
p\ge4096,\qquad4\le N\le\sqrt p/16,
\qquad4N\mid p+1.
\]

Section 3 verifies every numerical and divisibility hypothesis for the
chosen family. Its curve hypothesis remains an input hypothesis. The
lemma's exact torus sampling and lift tests select a full regular Dickson
fibre with

\[
N_k/4\le B_k\le N_k.
\]

Thus \(B_k=\Theta(p_k^\beta)\). Failed fibre trials are already included
in the inherited expected \(O(N_k\log p_k)\) basic-field-operation
recurrence; field inversion and bit-arithmetic costs retain the original
qualification. Storage is \(O(N_k)\). No unspecified proportion of a
fixed previously selected fibre is assumed. [pcc:unverified]

The conclusion applies to a supplied whole prime-order curve. For an odd
composite rational group it supplies rational base points, but does not
bound how many lie in a prescribed proper prime subgroup. Cofactor
clearing would change the abscissa set and cannot be inserted without a
new argument. Nor does choosing a new prime field transfer the discrete
logarithm problem of an arbitrary existing input curve into that field.

## 5. The conditional exponent correction is algebraically consistent

Under the explicitly stated conventional four-summand model, with
\(B=p^{\beta+o(1)}\), reusable setup exponent \(\sigma\le2\), linear
algebra exponent 2 and per-target query exponent \(\tau\), the relevant
budget is

\[
\widetilde O(B^\sigma+B^2+pB^{\tau-3}).
\]

For \(0\le\tau<1\), the last term decreases as \(\beta\) increases,
whereas \(B^2\) increases. Equating the two exponents gives

\[
2\beta=1+(\tau-3)\beta,
\qquad\beta_*=(5-\tau)^{-1},
\qquad\gamma_* = 2/(5-\tau).
\]

The setup term does not exceed \(B^2\) under the stipulated
\(\sigma\le2\). The range

\[
1/5\le\beta_*<1/4<5/12
\]

lies entirely inside the newly justified parameter range. Also
\(\gamma_*<1/2\) is equivalent to \(\tau<1\). Thus the extra
\(\tau<1/2\) restriction produced by fixing the older guaranteed
exponent at \(\beta=1/5\) is not needed in this chosen-field model.
[pcc:unverified]

This is a correction to a conditional optimization, not a proof of its
premises. The sublinear query bound, sufficiently many usable relations,
rank or independence, complete misses, recovery and verification still
need their own guarantees. In the pencil formulation with
\(D=\Theta(B^2)\) and \(\tau=2\theta\), it leaves exactly the same missing
query threshold \(\theta<1/2\) under setup linear in \(D\).

## 6. Input construction and remaining exclusions

Prime existence must not be treated as free cold setup. If the chosen
implementation enumerates integers \(tQ_k-1\) until the first prime,
the number of candidates is

\[
\frac{p_k+1}{Q_k}
\le Q_k^{7/5+o(1)}.
\]

The equality uses the exact residue class; the inequality follows from
the primary least-prime bound, with its additive \(1/Q_k\) absorbed into
the asymptotic upper bound. Each primality test, any requested proof,
and source-curve/group-order construction or certification must still be
charged when not supplied. This is an available enumeration upper bound,
not a necessary cost or a recommendation to use that search.

No growing prime-order curve family with a priced construction is proved
here. No arbitrary-prime guarantee, proper-subgroup density guarantee,
summation-polynomial root solver, signed-relation rank theorem or
end-to-end asymptotic speedup follows. Historical iteration8 and pinned
iteration7 files remain unchanged; this review binds only the exact
correction bytes listed in Section 1. **[pcc:unverified]**
