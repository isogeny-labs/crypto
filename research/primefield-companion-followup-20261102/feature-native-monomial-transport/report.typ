#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Combined monomial transport
2026-11-07 · T-1735 · Native development construction

The residual consumes its four normalized inputs through a fixed monomial W. Multiplying the inherited transport laws gives a recurrence for W directly. The implementation stores one homogeneous pair per row, with the original geometry lanes and polynomial residual unchanged. [pcc:unverified]

#image("flow.svg", width: 100%)

== Identity and anchors
For selected indices n and fixed powers e, let D(R) be the product of (x(nP)-x(nR)) raised to e. Then W(R+P)W(R-P)=D(R)W(P)²W(R)². The inherited pair update consumes the two preceding W pairs. Exponents reduce modulo p−1 on nonzero inputs. The −P anchor uses the parity of the sum of powers; it is not always negative. O, P and 2P remain explicit anchors. The first two nonanchor rows use direct phase and division values. [pcc:unverified]

== Exact check scope
Three native tests pass. The represented ratios and signs agree with the frozen four-column source on 499 complete-subgroup rows and 176 rows from eleven length16 shifted windows. An explicit even-parity anchor test checks −P and the one-pair representation. Sixty imported tests are filtered. Seventeen dependency hashes and the binary hash are retained with the full command and logs. These tests share geometry and residual code with the reference. [pcc:unverified]

== Costs and next construction
This moves input exponentiation from residual consumption to coordinate-difference powers during transport. Setup, direct initializations, geometry conversion and resultants remain. No measured complete-cost improvement is asserted. Integrate the single pair into the adaptive cursor, advancing it on skipped residuals, then perform matched accounting. The original 4,563-case cohort is unchanged; no cloud or timing panel was run.

== Assumptions and provenance
Prime p greater than3, odd prime subgroup r at least7, r distinct from p, gcd(r,p−1)=1, inherited word-size bounds and conditional seed-normalization law. All recurrence operands must be nonzero. Sources: predecessor multiseed-factor and normalized-transport derivations, transport-accounting report, and NIST DLMF23.10.3 (https://dlmf.nist.gov/23.10.E3), read November7. KB unavailable: no MCP tools or wrapper executable. No novelty-priority claim.

Claim PCC-20261107-0c701d is unsigned (local verdict not_covered). Evidence: DERIVATION.md, lib.rs, transport.rs, tests.rs, receipt.json, source-lock.json, build.log and test.log. No native failure; warnings retained.
