# Principal-open scheme overlap: checked formal result

The separately presented affine and infinity-chart coordinate rings have localized rings `AffineOpen` and `InfinityOpen`. `Common22PrincipalOpenSpec.lean` applies the previously checked inverse ring maps to their spectra. Pinned Lean checks an isomorphism `principalOpenSpecIso` between these two affine schemes. It also checks that each localization morphism into its own chart spectrum is an open immersion with range the corresponding basic open (`D(X)` or `D(z)`).

The source replay exits zero. Axiom audits for all five new declarations list only `propext`, `Classical.choice`, and `Quot.sound`.

| Gate | Status | Checked declaration or remaining construction |
| --- | --- | --- |
| Affine localization open immersion | Checked | `affinePrincipalOpenEmbedding_isOpenImmersion` |
| Infinity localization open immersion | Checked | `infinityPrincipalOpenEmbedding_isOpenImmersion` |
| Ranges are `D(X)` and `D(z)` | Checked | Two `*_opensRange` theorems |
| Isomorphism of localized affine schemes | Checked | `principalOpenSpecIso` |
| Two-chart gluing into a chosen smooth projective curve | Open | Construct the glued scheme and identify it with the projective completion |
| Infinity stalk and divisor/Pic⁰ transport | Open | Stalk comparison and principal-divisor theorem |
| Rust complete-law semantics and rho cost | Open / unmeasured | Independent correspondence and native experiment |

The isomorphism is between spectra of the two localized chart rings. It does not yet identify the glued two-chart object with a specific smooth projective genus-2 curve, establish the infinity stalk inside that object, or validate the Rust formulas. No operation or rho ratio follows from this proof.

`principal-open-spec.csv` is the machine-readable gate list; `principal-open-spec.svg` draws the same checked/open boundary. `principal-open-spec-build.log` is the pinned Lean replay. For review, first compile `Common22SeparateChartMap.lean` to an olean in `/dev/shm`, then run `TMPDIR=/dev/shm LEAN_PATH=/dev/shm lake env lean Common22PrincipalOpenSpec.lean` from this directory.
