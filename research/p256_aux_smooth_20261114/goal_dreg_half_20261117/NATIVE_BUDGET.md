# Strict native algebra allowance within the frozen240 seconds

The four CoCoA cells each used a45-second bound; they are not rerun.
The single known-answer capability probe used a10-second bound.
Reserve the remaining50 seconds for retained-membership qualification:
one25-second native invocation, and only if it completes, one25-second
fresh-directory byte replay. This is stricter than RETAINED_PROTOCOL's
120-second upper bound and keeps total algebra bounds at240 seconds.
Build/serialization work is recorded as development setup, not a scientific
measurement or part of the solver allowance. Address-space cap2GiB.
No retry after a failed/timeout membership invocation. No additional CAS
call, public-service request or parameter search. Unfinished cells stay open.

The retained-point method's primary source read before implementation is
Abbott, Kreuzer, Robbiano, Computing Zero-Dimensional Schemes, section3,
Theorem3.1 (generalized BM algorithm):
https://www.symbcomp.fim.uni-passau.de/fileadmin/dokumente/fakultaeten/fim/lehrstuhl/kreuzer/Artikel/Computing_Zero-Dimensional_Schemes.pdf
The original Buchberger–Möller construction is classical (EUROCAM1982,
LNCS144,24–31). Novelty is not claimed for this algorithm or its use as an
exact retained-point representation.
