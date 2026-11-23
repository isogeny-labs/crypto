#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Serif",size:10pt)
= Direct small filtered-closure controls
Observations only. Native development tests computed the degree-bounded closure for six synthetic m3, ell3 prime-chain systems. These are not measurements on the n83 fixed factor bases and do not establish a full-size LFD or regularity result.

#image("dimensions.svg",width:100%)
Recorded dimensions are in the selector quotient, not the unreduced polynomial ring. Development ID: filtered-small-20261030. Each bar is one exact finite-field computation; no statistical uncertainty or timing comparison is claimed.

*Inputs and calculation.* The fields are101 and103. Each system has three selector blocks of width3 with weights1,2,4 and one unrestricted intermediate U, curve coefficients a=2,b=3, and targets1,7 or half the weight sum. No U field equation is introduced. The monic selector equations are reduced exactly. Graded row elimination repeatedly adds allowed variable multiples until the queue empties; final low-degree basis rows are checked for closure again.

*Witness membership.* In all six cases the saved terminal affine witness has degree4, is outside V4 and inside V5. V4 has quotient dimension2 in every case. V5 has dimension185 at targets1 and7 and163 at the exceptional targets54 (field101) and55 (field103). The final V5 closure check covers440 or370 row-variable products respectively. All three unit tests pass, including a separate example requiring multiplication of a newly obtained degree fall and selector/U normalization checks.

*Exceptional targets matter.* At each exceptional target the earlier scalar pairing is zero, but direct membership still finds the witness outside V4. The scalar functional is a sufficient test in these positive cases, not a necessary one. Zero pairing must remain inconclusive. This does not establish the corresponding behavior at ell27 or for arbitrary target distributions.

*Preserved data.* Each of the six case files contains exact generators, the witness, every monomial column and every sparse normalized echelon row for both degree bounds. SMALL_FILTERED_FORMAT.json defines the masks, U exponents, pivot indices and coefficient encoding. The raw archive also contains exact source, Cargo lockfile, test executable, logs and controller status. The hard development limits were120seconds,2GiB,4096columns and50000processed rows; no limit was hit. Diagnostic compiler/test times remain in raw logs only.

*Verification and remaining work.* Both development invocations are preserved. Version2 adds explicit rejection of the mutated witness g+1 in all six fixtures. Its S3 replay custody is recorded in validation-terminal-filtered-small-v2/custody.json; initial-version custody is retained separately. These controls test the filtered-space implementation and can expose mistakes in the proposed general bridge; they do not prove its hypotheses. Next work remains exact n83 source/factor binding and independent verification or formalization of the syzygy-to-filtered argument before any new production LFD certificate. Original Koblitz/general-binary/prime scope and elite-base search remain open.

*References.* Caminata–Gorla, arXiv:2112.05579, Section1; SMALL_FILTERED_CONTROL_PLAN.md; PRIME_FILTERED_FUNCTIONAL_DESIGN.md.
