# Odd-characteristic norm coordinates validated

Hypothesis: the prepared odd norm compiler is an exact coordinate change preserving complete (r,s) fibers for fixed h,k, including characteristic three and zero branches. Its first scientific execution passed all 7,683 registered cases.

The independent oracle constructs all seven norm coefficients by closed formulas using literal field convolution, then divides by X-z. The compiler instead multiplies F^2-fG^2 as polynomials. Every r,s,t and every quotient coefficient agreed. For each h,k in F3,F5,GF9, the complete (r,s) domain maps bijectively onto all (e1,e2) pairs:

| Field | Cases | Complete parameter domain | h=0 cases | k=0 cases |
| --- | ---: | --- | ---: | ---: |
| F3 | 81 | yes | 27 | 27 |
| F5 | 625 | yes | 125 | 125 |
| GF9 | 6,561 | yes | 729 | 729 |
| GF25 | 256 | seeded only | 75 | 76 |

The quintuple repeated-root example H=X^5 is retained in every small field. Invalid target points are rejected, and a characteristic-two backend is explicitly refused by this odd-specific compiler. Its signed formulas divide by two, never by h or k.

For the actual coefficients and generator target of each requested NIST curve, 32 full-width parameter cases passed independent target-equation and full norm-quotient checks, totaling 160 cases. These are nonlinear representation checks, not new curve/order proofs, supported-source searches or hard all-roots timings. Generic field arithmetic currently supplies odd primes and quadratic extensions; other extension degrees still need an arithmetic backend.

One frozen experiment ran through lab.runner resume/run-ready with 90 seconds, 4 GiB and one core. Worker time was 0.780608020 seconds; peak process RSS 33,676 KiB. These include controls and are not comparative production measurements. Raw SHA256: 731c9bec4fce47d83850e94993af6f615487aa67855a20b3f287d033e0db3802, 189,902 bytes. Previous source, inputs, manifests and raw records were preserved.

The portable standard-library verifier imports neither compiler nor solver. It reconstructs all small transcripts with polynomial multiplication, checks their hashes and complete coordinate bijections, verifies repeated-root examples, and reconstructs all 160 NIST quotient records. It passed. Sole-model source preparation is separate from independent algorithmic oracles and external source admission.

New next hypothesis: turn exact subspace support into complete root/multiplicity extraction without scanning the factor base. The prepared subspace_factor_20261115_v29 implementation stores linearized subspace polynomials and recursively splits disjoint Fp cosets with gcd(H,R^n). Raising to n=degree(H) retains all primary multiplicities, avoiding derivative/squarefree assumptions. Its proof bounds visited nodes by 1+n*d, rather than p^d-domain enumeration. Degree is capped at five, basis length at 83 and characteristic at 31. Large-characteristic primes require a different interval/factorization route. Negative support and unsupported inputs never become empty root sets.

That extractor is syntax-checked but unexecuted and unregistered. Next freeze a separately reviewed worker, compare complete tiny supported/unsupported polynomial cases and multiplicities, and test planted repeated/split quintics over actual GF(2^83) with an 18-dimensional factor base. Test odd extensions and dependent-basis refusal too. Begin with 90 seconds/4 GiB/one core and explicit case/node/raw caps. Compare paid costs against the strongest matching factorizer before claiming speed. Additive-polynomial gcd splitting is prior art; this is not novelty clearance.

Root extraction from a given supported H still leaves the outer norm search, all parameter charts, curve/source/ghost branches and original ordered polynomial roots unresolved. The requested 8x goal and unchanged genuine-F4/F5/F6 release gate remain open. No qualified end-to-end speed ratio is claimed. Internal management provenance remains separate from this public mathematical checkpoint.

Reproduce: restore exact raw JSON through artifact references if relocated, then run python research/autolab_cycle_20261115_v29/REPLAY_RECORDED.py. New worker execution requires a fresh frozen plan and unused raw paths; never overwrite evidence.
