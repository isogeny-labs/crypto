# Joint source, Frobenius-primary and arithmetic admission

The frozen 96-case cohort uses six primes 499,631,883,1039,2011,4003 and constants 35..50 in the registered source family. All outcomes are retained: 20 certified large-prime subgroups and 76 bounded unknown cases. All 20 admitted sources pass exact Weil recovery and independent coefficient controls. Seven pass the necessary reciprocal-cubic kernel test, but only one has the coprime irreducible order-13 pair expected by the current builder and fits its extension-cardinality representation.

| Stage | Outcome |
|---|---|
| Fixed source cases |96|
| Certified prime subgroups |20|
| Bounded unknown under four-place rule |76|
| Independently checked Weil recoveries |20 / 20|
| Necessary reciprocal-cubic pass / fail |7 / 13|
| Current coprime order-13 primary pair |1|
| Primary pair plus p^13 fitting u128 |1|

The selected case is p631/c49: f=[49,89,73,48,91,41,123,1], N1=597, N=237185924, ell=59296481, L=[1,−35,−229,26730,−144499,−13935635,251239591]. Its monic F3 primaries are T³+T²+2 and T³+2T+2, with normalized reciprocal pairing and T13=1 modulo each. PUBLIC/CERT and full lift output are retained. No expected target scalar was supplied or computed.

The necessary pass alone is not a constructed kernel. Six other necessary-pass cases do not meet this builder's order-13 primary assumptions. The thirteen failures exclude the current rational (3,3,3) route on those sources, without claiming that other degrees/fields are unavailable. p^13 fits u128 for 499,631,883, not for the larger three fields; cardinality fit alone does not prove all intermediate arithmetic or dynamic-modulus correctness.

## Exact execution and custody

Source/complete-binary freeze 4acd0f46f2f0, checkpoint20261116T011953Z, preceded Stage1. All 96 outcomes and 20 certificates were committed599554daec20/checkpoint012054Z before Stage2. All 20 lift outputs were committed5bdc312b6490/checkpoint012127Z before Stage3. Each stage ran once, status0/stderr empty. An initial controller build had missing closing delimiters and an incomplete hash manifest; BUILD-PRODUCER and HASHES-PRODUCER-INCOMPLETE retain this pre-execution error. It was corrected and a complete successful build/hash/source freeze preceded every scientific run.

Source certification uses exact Fp counts, integer Weil intervals, point-order witnesses and unique-multiple cardinality certificates, plus proper-factor and ell² checks. Weil recovery compares the full 3p coefficient prefix with independent polynomial multiplication; no extension-field point count is claimed. Kernel admission enumerates all 18 invertible-constant F3 cubics, then checks irreducibility, coprimality and order13 exactly. These are known eligibility methods, not novelty or runtime improvements.

## Follow-through and claim boundary

The unique compatible F631 case is the input to the separately frozen actual-kernel construction in hec_kernel631_20261115. Its results must be read there; this admission stage alone supplies no torsion points, pairing values, normalization, quotient or map.

This cohort provides a bounded, reproducible target selection rule for a larger-field construction. It is not generic curve sampling or a success-rate estimate for arbitrary Jacobians. Source/kernel restrictions, construction/verification costs, full quotient/transfer, matched charged comparisons and genus2/4 remain. The mathematical tenfold objective is open; the old failed cold-cost result is unchanged. No timing benchmark was collected.
