#set page(paper: "a4", margin: 19mm)
#set text(size: 11pt)
= Division-free normalized sign source
Prime-field source construction · 2026-11-03

The sign-only API now classifies the homogeneous ratio directly. For nonzero canonical `N,D,c_inv`, the normalized value `(N/D)*c_inv` is +1 or -1 exactly when `N*c_inv` equals D or -D. The raw-value API still performs division and returns unchanged metadata.

#image("pipeline.svg", width: 100%)

== Checked construction
One optimized build and one bounded invocation pass all 106 native methods, none filtered. The new predicate checks cover 1944 nonzero triples over F7 and F13, 3888 rescalings and six invalid tuples. Saved raw, seed and window checks pass; all eight complete recovery receipts match the previous low-product implementation. All 24 prebuild source/data locks pass.

== Arithmetic consequence
The finite sign path omits the final denominator inversion and its accompanying product. Calibration multiplication, explicit domain checks and denominator negation remain. Other source inversions are unchanged. This is a derived local statement; no complete-cost or timing comparison ran. The identity is independent of subgroup order and feature width, within the supported prime-field domain.

== Next dependency
Instrument the changed source path separately before a frozen matched comparison. The source partition and decoder program sizes are unchanged; the original 4563-case cohort remains intact. No cloud job was submitted.

== Sources
See DERIVATION.md for the proof and limits; receipt.json, source-lock.json and raw logs for checks. #link("https://www.hyperelliptic.org/EFD/g1p/index.html")[EFD homogeneous representations] supply established context; cross multiplication itself is not a novelty claim.
