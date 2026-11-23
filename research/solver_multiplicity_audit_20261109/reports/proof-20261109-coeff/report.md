---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Genericity coefficient discrepancy

The retained msolve preflight announces coefficients k^344 but prints
−2147483648 for k=2. Lean proves that the announced coefficient is 1 in
F5, while the printed integer maps to 2. It also proves the advertised
integer exceeds int32_t's maximum. Closed arithmetic claim
PCC-20261109-c7c353, unsigned. [pcc:unverified]

![Exact raw/source/arithmetic scope. Only the arithmetic node is kernel formalization; the source/log binding is inspected evidence.](boundary.pdf){width=100%}

The pinned source computes floating pow, casts to int32_t, and only then
reduces modulo the characteristic (msolve.c:609–619). Its print routine
repeats the cast (:165). The c=2 stdout reaches exponent 344 at line16952;
its parameter output has coefficient vector [1,−2147483648,1]. Relevant
source/raw excerpts are in ../../evidence. [pcc:unverified]

C11 draft N1570 §6.3.1.4 paragraph1 makes real-to-integer conversion
undefined when the truncated value cannot be represented. This source
path crosses the representable range for its advertised powers. The
observed coefficient does not implement the stated power pattern, even
as an F5 element. Lean does not formalize C, libm rounding, IEEE execution,
or a sanitizer trace; the arithmetic result is scoped to the exact
recorded integers. No incorrect solution is asserted. [pcc:unverified]

Formal namespace: SolverCoefficientAudit, in SolverCoefficientBoundary.lean.
Declarations are listed below. The transitive project gate now passes
96 declarations, with no failing or trusted-native exception. [pcc:unverified]

```text
advertised_power_outside_int32
observed_coefficient_wrong_in_five
observed_coefficient_wrong_in_five_at_first_overflow
```

The first overflowing mathematical power already differs in F5 from
the recorded negative integer. No proof hole, custom axiom or native
decision exception is used. [pcc:unverified]

The separately frozen 16-call experiment passes its exact basis/count
checks. Those checks do not validate complete parameter coordinate maps
or every genericity coefficient. The successful counts and this source
finding have distinct scopes. No frozen protocol, native analysis,
raw output or immutable export is changed by this after-data source audit.
[pcc:unverified]

Sources: msolve0.6.5 commit 5e72b9d9; N1570 primary PDF (SHA256 prefix
208969fec021), §6.3.1.4. Full hashes, exact excerpts, artifact hydration,
and the proposed field-first coefficient repair are in
../../SOURCE_OBSERVATIONS.md. The repair has not been implemented or
benchmarked. No new solver execution or paid spend occurred.
[pcc:unverified]
