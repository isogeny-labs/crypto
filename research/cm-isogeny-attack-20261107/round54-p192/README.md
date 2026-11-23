# Mixed trace quotient coordinates

Read report.md or report.pdf for the derivation and exact scope. build.py evaluates the mixed trace over the explicit round53 kernels in a quadratic tower, keeping every fixed-panel outcome. verify.py uses independent packed field/curve arithmetic and recomputes all trace values. p192_shift.py constructs an actual pole-free P-192 quadratic shift; verify_shift.py verifies it through the base-field twist. No P-192 trace evaluation is included.

Replay in a disposable copy of the study layout: `python3 build.py`, `python3 verify.py`, `python3 p192_shift.py`, `python3 verify_shift.py`. These write records. Python3.9+ suffices. Dependencies.sha256 pins the exact earlier model, field, kernel certificate and P-192 point input. Hydrate earlier artifact-store manifests if needed. PDF rendering uses typst and pymupdf.

The eight shifts were fixed before evaluation; all collisions remain in certificate.json. Shift n=6 is a common separating specialization selected from this complete panel. It gives exact control evidence, not an out-of-sample or runtime claim. The P-192 shift is an independent chart construction. [pcc:unverified]

The function-field model is birational on an open set. A global target model, its group law, and efficient P-192 trace/normalization remain open. Scientific standing is an unsigned statement record pending a trusted signed verdict. [pcc:unverified]
