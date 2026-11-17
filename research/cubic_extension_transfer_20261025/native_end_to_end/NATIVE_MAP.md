# Native map implementation

The actual native elliptic-to-Jacobian pipeline now solves both original public
instances using C++17/FLINT, native Singular and the pinned Rust quartic backend.
It does not execute Sage or Python crypto code. This is a correctness result;
the requested qualified cold comparison remains unfinished.

An evidence-linked diagram, sixteen-target attempt-count graph, and inspected
PDF summarizing these existing correctness receipts are in
[reports/transfer-results-20261030/](reports/transfer-results-20261030/).
They were derived from the committed receipts without rerunning the solver.

| Public source | Source/target order r | 3-isogenies | Recovered k | Jacobian base-point attempts |
| --- | ---: | ---: | ---: | ---: |
| p109 | 1,293,169 | 1 | 1,729 | 1 |
| p157 | 3,868,159 | 2 | 271,828 | 1 |

Final-build records: `results/p109-transfer-ic-final-20261030.json` and
`results/p157-transfer-ic-final-20261030.json`. Both verify the scalar on the
original source; full-transfer target-rho controls pass on both too. Earlier
passed records needing three and seven base-point choices are retained. Every
target divisor comes from inversion of an actual source image.

An additional eight separately generated targets per curve pass in
`results/panel-development-attempt2/`: sixteen full native IC solves and sixteen
independent Fp3 verifications, against separate known-answer receipts. Together
with the originals this is eighteen distinct instances. The first panel's failed
verification is retained. Bounded verification via `(k+t)D-tD` repairs an
unavailable binary chain without changing the candidate or accepting a mismatch.

`native_construction.hpp` reuses the unchanged prior native normalization and
obstruction evaluator, substitutes the explicit rational three-torsion lift,
walks rational degree-three isogenies, constructs the Aronhold/Riemann quartic,
descends it through the Weil cotangent basis, and chooses an ordinary rational
tangent chart. All 28 contact divisors are recovered by exact square identities.
Smoothness is tested over the algebraic closure by unit-ideal certificates in
all three projective charts using native Singular.

The input reader consumes only the public first nine fixture lines. It ignores
the saved target logarithm and the earlier fixed target-quartic equation. The
new target model is constructed from the actual source and selected kernel.
The elliptic source order is certified by a nonzero prime-order point and the
Hasse bound. Every preprocessing map checks the codomain, subgroup annihilation,
addition and several scalar identities. An independent sampled action matrix
checks the rational lift in the development test.

The rational odd bitangent is retained as the reference. The native forward
signature shifts by its labelled two-torsion point before polynomial inversion.
That shift permits recovery against the rational reference divisor.

`native_rr.hpp` constructs Riemann–Roch numerator spaces from exact cubic
contact-divisor jets and normalizes the seven Weil functions in a degree-three
residue algebra. `native_sections.hpp` imposes exact diagonal identities over
Fp: 220 rows, rank 112, eight sections. Its transformation passes twelve
independent holdouts on each curve. Image coordinates descend individually to Fp.

`native_inversion.hpp` uses saturated base-field polynomial equations, native
Singular slimgb/FGLM and checked triangular solutions in the degree-six splitting
field. Each of the six original-fixture inversions yields twelve ordered
solutions and two effective divisors. `native_group.hpp` recovers actual RURs
using a common rational projection and Hermite interpolation, including a triple
support control. It subtracts the rational odd reference and aligns signs using
P, Q and P+Q. The Rust bridge receives no planted scalar and exports the actual
target RURs, base point and order witness.

Independent Rust arithmetic certifies both original constructed target orders:
a nonzero r-torsion witness gives r dividing the group order, and the exact
genus-three Weil upper bound is below 2r. The checker also verifies (r+1)D=D.
Full-transfer records additionally export a witness for their projected model.
`source_adapter.rs` supplies a separate native Fp3 verifier, direct source rho
controls and known-answer generation; its solver reader accepts nine public
lines and rejects a planted-scalar line.

Sources reviewed: Tian, *Cover attacks for elliptic curves with prime order*,
[arXiv:2012.07173](https://arxiv.org/abs/2012.07173), Algorithms 1–2 and sections
3.1–3.2; the archived `quotient_theta.py`, `inverse_map.py`,
`polynomial_inverse.py` and `transfer_pipeline.py`; and the existing native
[torsion identity](../native_followup/THETA_ACTION.md). This is an implementation
of established constructions, not a novelty or performance claim.

Build with `bash build_jacobian.sh`, `bash build_source.sh` and
`bash build_map.sh construction_test rr_test sections_test transfer_test` in the
pinned native development image, with FLINT/GMP and Singular on PATH. No Sage
or Python crypto process is invoked. The mandatory checkpoint gate precedes
each build/check. Development limits are one CPU, 4 GiB and thirty minutes.
The transfer driver takes a public fixture, the archived result directory,
a fresh output path, `ic|rho|rho-neg|map`, and an optional solver seed (default
20261029). Run from this directory so `build/libcubic_jacobian.so` is resolved.

The square predicate uses the Fp Legendre symbol: nonzero Fp nonsquares become
squares in the even-degree ambient Fp18. A regression control demonstrates that
distinction. Initial eligibility results and all failed/interrupted attempts
are retained in `results/development-obstructions.json` and their raw logs.

Retries are bounded: two elliptic 3-isogenies, 99 common source multipliers and
sixteen Jacobian base-point choices, plus 32 bounded verification splits when a
binary scalar chain is unavailable. Earlier p109/p157 solves rejected two and
six base-point choices; final original controls use one choice each and record
their scalar splits. Setup still requires an admissible
primitive identity-divisor chart. Universal exceptional-input handling and the
universal extra-step coverage claim remain open.

No timing was taken or admitted. Earlier records use the reference solver-stage
counters. New records explicitly count the entire target-solver C ABI call,
including subgroup checks and verification retries; they do not price FLINT or
Singular. These counter scopes must not be compared as a performance gain.
A qualified cold comparison
still needs preregistration, independent nuisance replicates, all setup/retries
charged, matched source references, complete native-work pricing, independent
replay and observed L2 isolation. The EKS ecbench request does not accept this
standalone driver; an executable SHA alone does not supply that integration.
No paid job or replacement default validation fixture has been submitted.
No new exponent, universal coverage result or speed improvement is established.
