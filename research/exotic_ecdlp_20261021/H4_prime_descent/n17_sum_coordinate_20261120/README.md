# H4 n17 pair-sum and total-sum coordinates raise equation support

Two exact, invertible three-block XOR substitutions increased source-equation monomial counts on both frozen `n=17` H4 systems. Pair-sum changed the combined count from 158,920 to 166,170 (+4.56%); total-sum changed it to 163,870 (+3.11%). Maximum Boolean degree stayed six. Neither met the [predeclared ten-percent reduction gate](PROTOCOL.md), so no additional F4 timing arm was launched from this lead. The [previous capped pair](../n17_msolve_20261120/README.md) remains the solver-stage measurement for these inputs.

| Frozen input | Control terms | Pair-sum terms | Total-sum terms | Maximum Boolean degree |
| --- | ---: | ---: | ---: | ---: |
| Planted | 79,285 | 82,885 | 81,737 | 6 in all three |
| Ordinary query | 79,635 | 83,285 | 82,133 | 6 in all three |

For six-bit subspace vectors `(u,v,w)`, pair-sum uses `s=u+w`, and total-sum uses `s=u+v+w`. In characteristic two each map is its own inverse; the selected subspace is closed under these sums. [screen.py](screen.py) expands every source monomial in the Boolean ring, cancels duplicate terms modulo two, applies each transformation twice to every polynomial, evaluates all mapped known roots, and checks 256 deterministic assignments per arm. [screen-result.json](screen-result.json) retains exact term counts, degree counts, hashes, and the null selection decision. The same eighteen Boolean field equations are used in each serialized input.

## Proof map

| Result | Lean declaration | Exact hypotheses | Status |
| --- | --- | --- | --- |
| Pair-sum is involutive | `H4TripleSum.pairChange_involutive` | Every dimension `n` and Boolean triple | Checked |
| Total-sum is involutive | `H4TripleSum.totalChange_involutive` | Every dimension `n` and Boolean triple | Checked |
| Root bijection for pair-sum | `H4TripleSum.pairSolutionEquiv` | Any predicate on Boolean triples, transformed by `pairChange` | Checked |
| Root bijection for total-sum | `H4TripleSum.totalSolutionEquiv` | Any predicate on Boolean triples, transformed by `totalChange` | Checked |
| Serialized polynomial expansion matches the predicate transform | — | Exact H4 equations and Boolean-ring arithmetic | Checked by symbolic double transform and assignment controls in `screen.py`; a polynomial-ring Lean bridge is open |

[TripleSum.lean](TripleSum.lean) contains the readable proofs. [build.sh](build.sh) checks the source hash, pinned `v4.35.0-rc4` toolchain and Lake manifest, and Mathlib commit `021c46bdd042f67b538b9f9871d556a93713a9a9`; the module built successfully. [build-output.txt](build-output.txt) retains the `#print axioms` audit, which lists only `Quot.sound` for the four declarations. The module has no placeholders or custom axioms. `lean4checker --fresh` is unavailable on this host.

The generated `.ms` files are retained losslessly as four gzip/base64 text artifacts. [packaging.json](packaging.json) gives the raw and encoded SHA-256 digests and sizes; [package.py](package.py) checks round-trip decompression. [PACKAGING_AMENDMENT.md](PACKAGING_AMENDMENT.md) records this representation step after the frozen screen. Re-run `screen.py` to regenerate the raw `.ms` files, then `package.py` to verify the compact copies. This screen rules out the two simple sum-coordinate substitutions as an input-sparsity route at this exact scale; it does not measure their F4 wall times.
