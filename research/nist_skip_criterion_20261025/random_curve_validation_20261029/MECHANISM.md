# Meaning of the statistic and the missing security mechanism

For an odd prime ell not dividing the field characteristic, strict character
+1 means the Frobenius characteristic polynomial has two distinct roots
modulo ell. There are two Frobenius-stable lines in E[ell], giving rational
cyclic ell-isogeny kernels. This does not say that their individual points are
rational. Character -1 has no such line. Character zero is a separate case:
the action can be scalar or non-scalar, so zero must not be described as
absence of rational isogenies. The reported statistic counts split primes,
not all rational isogenies, paths, endpoints, graph expansion or map costs.

There is no demonstrated inference from a smaller strict count at 691 to an
easier ECDLP on these curves. The count is a property of p and n, unchanged
by a different generator or an isomorphic coordinate representation. Curves
with different full orders over the same field are not F_p-isogenous to each
other, so this table cannot itself provide an isogeny reduction between a
published and an omitted curve.

Known small-isogeny reductions explain how to transport a DLP to another curve
in an appropriate isogeny class when an efficient map is available and its
kernel does not destroy the target subgroup. An efficient destination DLP
solver is still needed. Jao--Miller--Venkatesan's GRH-based random-reducibility
results concern curves with the same field/order and controlled endomorphism
levels. They neither make DLP easy nor turn the local count at 691 into an
ECDLP difficulty ordering. Do not interpret these conditional results as a
security proof for the supplied curves.

A plausible weakening hypothesis must supply: a concrete exploitable target
property; a computable transfer or attack; preservation of the prime-order
subgroup; a destination solver advantage; and matched total costs including
map discovery, preprocessing, evaluation, solving and recovery. A hidden-map
hypothesis additionally needs a defined retained witness and a measured or
proved separation from public reconstruction or cheaper alternative routes.
None of these is established by the splitting count or by its percentile.

SEA point counting uses Elkies primes, and their distribution affects its
work. This supplies a concrete generation-related confounder to investigate,
not a discovered historical explanation. More useful Elkies information can
help point counting; an observed preference for fewer primes does not by
itself establish a faster-SEA motive. Counted algorithmic work, full runtimes,
censoring, early aborts and actual historical implementation would have to
be compared. The random baseline must never preferentially retain curves
that finish within a per-candidate timeout.

A convincing statistical deviation could support investigation of selection.
It would not identify the designer, a security motive or a deliberate backdoor.
Absence of a known attack does not prove security or rule out deliberate
selection for a non-security reason. The present report supplies neither an
attack nor an authenticated decision trace.

Sources: Shparlinski--Sutherland, arXiv:1112.3390v2, pp.1--3;
Jao--Miller--Venkatesan, arXiv:0811.0647v2, Theorems 1.5--1.6 and sections
4--6; PARI's official ellsea documentation. See PROTOCOL.md for direct links.
