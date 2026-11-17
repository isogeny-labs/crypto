# Closed quadratic fibers, distinct opt-in candidate

The NTT1 calibration exceeded the local word objective, but rational full cold
remains construction dominated. A separate fiber variant reuses the literal
NTT1 facade/adapter/worker and adds an independent root flag. Existing arms keep
the generic root finder; explicitly named fiber arms enable NTT and closed
linear/quadratic roots. Half-GCD remains the unchanged owner algorithm.

For quadratic a*x^2+b*x+c, discriminant b^2-4ac and the counted owner sqrt/inverse
produce all field roots. Sort/dedup handles repeated roots; every emitted root
is evaluated in its polynomial. Zero polynomials retain their original error.
Certification still checks exact fiber size, every sibling, denominator poles,
map coprimality and total leaves. No full-splitting assumption is substituted
for a check. Exhaustive small-field polynomials include nonsplit/repeated cases.
The general last-fall-degree argument remains unverified and unchanged.

NTT1 files/data and all old evidence remain immutable. Per-stage table reset,
setup counters/cold clocks and exact double-query control remain as before.
The primary experiment will include fresh seed and an explicit indexed-target
offset. Its frozen prior-target manifest includes the old9c662 speed study;
a collision retains the input and stops without resampling. New larger word
m32768 is compared separately, alongside word16384/P2242048/rational512.
Schoolbook remains the test/calibration reference; complete cold primary
compares existing Euclid/Newton, shared-field Newton, fixed fiber-half candidate,
its AA/double controls and native lookup. No fastest or L2 claim.
