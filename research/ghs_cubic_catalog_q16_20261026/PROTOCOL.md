# Cubic Jacobian catalog closure: validation protocol

26 October 2026. Conductor T-5382. This validation protocol was frozen after
the exploratory local LMFDB queries and before the independent Sage replay.
The discovery query is not a blinded experiment.

1. Use the prior immutable GF(16)/GF(4096) source census in
   `../ghs_binary_cubic_isogeny_q16_20261026/results.json` (SHA-256 recorded
   by `audit.py`). Its 8190 rows enumerate the two absolute-trace classes of
   `a` and every nonzero `b` in the normalized ordinary model
   `y^2+xy=x^3+a*x^2+b`. Require the independent Walsh recount in
   `../ghs_cubic_tate_bridge_20261026/audit.py` to pass.
2. Query the completed local LMFDB `av_fq_isog` index extracted on
   2026-10-19 from `public.av_fq_isog`, using the exact predicate
   `g=3 AND q=16 AND poly_str LIKE '1 0 0 % 0 0 4096 %'`.
   Freeze the returned label, polynomial, and `has_jacobian` rows in
   `catalog_snapshot.json`. Treat value 1 as a cataloged positive witness;
   value 0 is **not** a proof of nonexistence.
3. Join source rows by `t=4097-#E(K)` against polynomial
   `1 0 0 -t 0 0 4096`. Independently check class A/B/C via field trace,
   and the simple-root sufficient subgroup gate for each row's largest
   prime factor using `gcd(r,6-3t)=1`. Count all rows, orders, and the
   complement of the 36 direct-B orders. Assert exact totals.
4. Replay one previously unhighlighted class: source order 3974, subgroup
   1987, source normalized coefficients `a=32,b=306`; LMFDB label
   `3.16.a_a_aet`, first recorded hyperelliptic model. Recount the
   model over GF(16), GF(256), and GF(4096) and verify its genus and
   source order. Use Conway polynomial `x^4+x+1` for GF(16).
5. Report existence of an isogeny and of **some** prime-subgroup-surviving
   isogeny from the prior Tate/Hensel proposition. Do not infer a usable
   map, an attack cost, or publication novelty from the catalog join.

Primary prior work: [Gaudry--Hess--Smart](https://www.kantmath.de/publications/papers/weil_descent.pdf)
construct direct binary Weil descent; [Menezes--Teske, Section 7](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf)
identify the direct cubic genus-three family and model isogeny walks;
[Hess](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf)
proves subgroup survival for his generalized construction. The
[LMFDB finite-field abelian variety catalog](https://www.lmfdb.org/Variety/Abelian/Fq/)
already contains the Jacobian models used here. This audit is a finite
cross-index classification, not a new geometric construction.
