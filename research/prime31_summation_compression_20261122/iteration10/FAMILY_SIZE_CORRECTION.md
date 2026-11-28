# Family-size correction from smooth-modulus prime bounds

**Disposition.** The previously used general Linnik exponent 5 is valid, but unnecessarily restrictive for the power-of-two moduli in this construction. Chang's existing smooth-modulus result permits every fixed factor-base exponent `0 < beta < 5/12` along the constructed prime-field family. In particular, it permits every optimal exponent in the stated conventional four-summand model when `0 <= tau < 1`. This is an application of prior art, not a new prime-distribution theorem or an achieved relation-solving speedup. Historical iteration8 notes are left unchanged. [pcc:unverified]

## 1. Primary source and exact hypothesis

Mei-Chu Chang, *Short character sums for composite moduli*, Corollary 11, states that if a family of moduli `Q` satisfies

\[
\max_{\ell\mid Q,\ \ell\text{ prime}}\log\ell=o(\log Q),
\]

then every coprime residue class `a mod Q` contains a prime

\[
p<Q^{12/5+o(1)}.
\]

The statement is on printed pages 4 and 39 of the [author's PDF](https://math.ucr.edu/~mcc/paper/143%20Char060114.pdf); the proof following equations (8.4)–(8.5) treats the possible exceptional real zero. The [author-submitted version](https://arxiv.org/abs/1201.0299v2) is dated 7 May 2014. No recent claimed improvement to the general Linnik constant is needed.

For comparison, the general exponent 5 is established in Xylouris's [2011 thesis, Theorem 2.1, printed page 12](https://bib.math.uni-bonn.de/downloads/bms/BMS-404.pdf), and [2018 paper, Theorem 1](https://www.mathnet.ru/eng/cheb681). The correction concerns use of a stronger applicable hypothesis, not an error in that general theorem.

## 2. Exact prime and fibre-size construction

Put `Q_k = 2^(k+2)` and choose a prime

\[
p_k\equiv-1\pmod{Q_k},\qquad
Q_k-1\le p_k<Q_k^{12/5+\varepsilon_k},
\qquad \varepsilon_k\longrightarrow0.
\]

Such primes exist by Chang's corollary: the only prime divisor of `Q_k` is 2, so `log 2 / log Q_k -> 0`, and `-1` is coprime to `Q_k`. The lower bound implies `p_k -> infinity`, with an infinite distinct-prime subsequence if necessary.

Fix `0 < beta < 5/12` and define

\[
N_k=2^{\lfloor\beta\log_2 p_k\rfloor}.
\]

Choose `eta > 0` such that `beta(12/5+eta) < 1`. For sufficiently large `k`,

\[
\log_2 N_k\le\beta\log_2p_k
<\beta(12/5+\eta)(k+2)<k.
\]

Thus `N_k` is a power of two dividing `2^k`, and consequently `4 N_k | p_k+1`. Moreover,

\[
\tfrac12p_k^\beta<N_k\le p_k^\beta,
\qquad N_k\le\frac{\sqrt{p_k}}{16}
\]

eventually, because `beta < 5/12 < 1/2`. Also `p_k >= 4096` and `N_k >= 4` eventually. These are exactly the size and divisibility hypotheses required by the already-reviewed fixed-source-curve fibre-density lemma. The endpoint `beta = 5/12` is not asserted.

Therefore, for an **already supplied nonsingular odd-order curve** over one of these fields, that lemma supplies a verified full Dickson fibre with liftable base size

\[
N_k/4\le B_k\le N_k,
\qquad B_k=\Theta(p_k^\beta).
\]

Its expected base-construction cost remains `O(N_k log p_k)` basic field operations and `O(N_k)` storage, under the same sampling model as the fixed-source lemma. For an exact implementation of the exponent cutoff, a fixed rational `beta` can be used and the power threshold compared with integer arithmetic. [pcc:unverified]

## 3. Consequence for the conditional four-summand budget

Under the previously stated conventional model—four summands, sufficient usable relations, a per-target query exponent `tau`, linear-algebra exponent 2, and reusable setup exponent `sigma <= 2`—the budget is

\[
\widetilde O\!\left(B^\sigma+B^2+pB^{\tau-3}\right).
\]

Balancing the last two terms gives

\[
\beta_* =\frac1{5-\tau},\qquad
\widetilde O\!\left(p^{2/(5-\tau)}\right).
\]

For `0 <= tau < 1`, one has `1/5 <= beta_* < 1/4 < 5/12`. Every such exponent is now supported by the smooth-modulus family argument. The additional `tau < 1/2` restriction obtained by fixing the guaranteed base exponent at `beta=1/5` is therefore unnecessary. The conventional requirement `tau < 1` for beating the square-root exponent remains.

This is only a correction to parameter availability. Neither the required sublinear query exponent nor the assumed relation yield, independence/rank, and complete end-to-end solver costs have been proved by this argument. [pcc:unverified]

## 4. Input regime and construction costs

This does **not** establish the necessary two-power divisibility for an arbitrary input prime. It describes a chosen infinite prime-field family; for a given source curve, its own prime and available two-power divisor must satisfy the explicit conditions. It neither changes the curve underlying an input discrete-log problem nor proves the existence of a growing prime-order curve family over these selected primes. An already supplied whole prime-order curve is a permitted input; concentration of a proper prime subgroup inside a fibre is not proved.

Prime existence also does not supply a cheap prime-finding algorithm. If primes are constructed by enumerating `t Q_k - 1`, the number of candidates examined up to `p_k` is `(p_k+1)/Q_k`, bounded by `Q_k^(7/5+o(1))`; each primality test and any required certificate must be charged. That is an available search bound, not a necessary lower bound or a claimed fast implementation. Prime construction, source-curve construction, and any required group-order certification belong in cold setup whenever those objects are not supplied as inputs. They cannot be omitted when comparing a generated-instance workflow with its baseline.

No program, cryptographic experiment, benchmark, source patch, or remote mutation was performed for this correction. The work is a source-backed mathematical derivation, pending the project's normal review and claim gates.
