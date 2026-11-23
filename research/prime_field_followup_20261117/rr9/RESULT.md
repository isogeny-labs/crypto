# RR9 B8 certificate follow-up

Both frozen negative residuals have independently verified 8,713-byte branch certificates and 86,124-byte sign-independent polynomial identities. The frozen positive recovered its chart and replayed on the original curve. This work uses only the three frozen B8 residuals from the previous pilot. It measures certificate correctness, size, and single-host timings. It does not establish an index-calculus speedup. [pcc:unverified]

## Sign-independent certificate generation

Three methods ran on both equivalent presentations of all three residuals, three repeats each. All 54 runs replayed in external C++. The negative identities each use 20,672 terms, with maximum multiplier total degree seven. Median and range below pool the 12 negative runs for each method, in milliseconds.

| Method | Basis decision or joint basis and lift | Separate lift | Full process median | Full process range |
| --- | ---: | ---: | ---: | ---: |
| Singular liftstd | 343 | — | 376 | 332–448 |
| Singular std then lift | 28 | 336 | 405.5 | 343–686 |
| Singular slimgb then lift | 62 | 388 | 525 | 411–789 |

Basis-only decisions are much cheaper than recovering exact multipliers. The separate lift does not reduce term count or full cost on these examples. The final cohort uses the configured optional Singular module path. Earlier fallback-module and timer trials are retained in the workspace. These are single-host measurements with process startup and text output included in the full time.

A support bitmap over the 3,432 monomials of total degree at most seven plus canonical field coefficients reduces the four previous exact text identities from 488,328–491,334 bytes to 86,124 bytes, about 5.7×. The C++ decoder reconstructs the equations from the public residual and checks exact expanded equality to one. It rejected a coefficient mutation and a truncated file. This is an encoding result. The algebraic lift and the identity itself stay the same.

## Signed branch certificate

For each prescribed abscissa x and canonical curve square root y, let d=x-r and use L±=dA(x)+(s±y)V(x). On these fixtures d is nonzero. The evaluation equation q obeys L+L−=-dq. The verifier checks the polynomial product identity against the independently reconstructed RR9 equations. A contradiction for each sign assignment therefore rules out a common zero, including over the algebraic closure. Each certificate stores the coefficients of an exact affine combination equal to one and proves collective branch coverage using a 256-entry bitmap. [pcc:unverified]

Both negatives needed all 256 leaves and 510 incremental row insertions. The proof is 8,713 bytes, about 56× smaller than the earlier text identity. The proof forms differ. Replay rejected a modified affine multiplier and a missing leaf. The positive used the first sign branch, generated a 34-byte chart witness, and independently replayed the seven chart coefficients and the original-curve point sum.

| B8 case | Branch generator median, lifting included | Group-law MITM median, lifting included |
| --- | ---: | ---: |
| Negative 1 | 162.017 μs | 20.767 μs |
| Negative 2 | 162.694 μs | 20.764 μs |
| Positive | 2.459 μs | 10.071 μs |

Each median uses 301 in-process iterations. The comparison measures generation and the group decision, with neither certificate I/O nor certificate replay included. The positive's first-branch position is favorable. The signed search is enumerative and about 7.8× slower than MITM on the two negatives. Its result is a compact affine proof.

No B31 experiment followed because the original sign-independent route remains much slower than the generic check. The branch proof is useful as a small-instance correctness fixture. Further speed work on the sign-independent route needs to reduce the lift itself, for example by changing the linear solve used to recover multipliers.

## Replay

The source, raw files, and frozen protocols are in the same directory. The following scripts write regenerated evidence into the current evidence checkout. Run them in a fresh extracted copy to keep the original outputs and timings.

- Run `./replay.sh` with a C++20 compiler for the branch and generic group fixtures.
- With Singular 4.4.1 on PATH, run `./run_signfree_module.sh` and then `./summarize_signfree_module.sh`.
- Run `./run_codec.sh` after the Singular cohort to pack and verify its four first-repetition `liftstd` identities. The retained codec log records the original text inputs. Its byte lengths may differ from the current runner's text exports, which include timing fields.
