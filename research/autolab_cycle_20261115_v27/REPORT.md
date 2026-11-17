# Full-system displacement preflight and prepared odd-field backend

Hypothesis: the complete coefficient-span incidence matrix in natural r/w coordinates has displacement rank at most eight, allowing structured elimination beyond the prefix-only shared-kernel compression. Two operators were frozen before measurement: ordinary coefficient shifts and shifts chosen from the exact Frobenius intertwining relations. The full system retains all 93 baseline rows, four independent Gold constraints where applicable, and 45 incidence rows. No favorable submatrix replaces the complete matrix.

The hypothesis was refuted on all 32 frozen directions per family:

| Family | Ordinary displacement rank | Frobenius-aware rank | Rank outside the 48-row AS block |
| --- | --- | ---: | ---: |
| pre-Gold73 | 94 or 95 | 57 | 9 |
| Gold69 | 98 or 99 | 61 | 13 |

AS denotes the Artin-Schreier block combining r-square and linear r terms. Its mixture prevents the simple low-displacement model. After isolating it, the remainder exactly reaches the derived boundary bounds (nine without Gold, thirteen with Gold). This is a concrete mixed structure to study; it is not an implemented fast solver or proof of superiority.

The natural-coordinate matrix rank matched the baseline codimension plus the independently field-constructed restricted incidence rank in every case. Tiny controls tested all 256 two-by-four binary matrices on all 16 vectors, yielding 4,096 literal operator equalities. An independent standard-library verifier reconstructs all 64 complete matrices using polynomial long division, checks ordinary and mixed displacement ranks and the outside-AS bounds, and passed.

One experiment ran through lab.runner resume/run-ready after source/input hashes and budgets were frozen: 90 seconds, 4 GiB, one core. Worker time was 1.024899416 seconds, peak process RSS 156,708 KiB. This includes parsing the full actual fixture; it is not a comparative production benchmark. Raw SHA256: dbefdcd0f029e8aae8a945e106556a2d9e237f58908f206b9260ddfe4f8b337b, 21,942 bytes. Rank-only diagnostics do not reconstruct incidence RHS or claim affine solving, support or original roots.

A syntax error was caught by AST parsing before registration. The unexecuted draft is retained; two missing spaces were corrected before freezing. The shell's subsequent coordinator call had an empty ready queue and executed no experiment. Its records were preserved under different filenames from the actual run. No frozen algorithm or raw evidence was edited, and no failed worker was retried.

The field stream now has prepared generic_block_20261115_v27/backend.py and a signed lifting proof. It uses explicit subtraction, exact inversion, canonical coordinates and complete affine kernels; supplied arithmetic covers odd prime fields and quadratic extensions with a verified nonsquare. Other extension degrees require another arithmetic backend. It was syntax-checked and statically reviewed, but not executed or registered. The scientific cutoff left insufficient reserve for its independent small-field oracle. No odd-field measurement or validation is invented.

Next experiment: freeze a reviewed worker for complete tiny original fibers over F3,F5,GF9 and GF25, including nonzero lifting corrections, rank drops, empty and inconsistent systems. Add separately labeled exact backend checks over the five source-identified NIST primes. Initial budget 90 seconds/4 GiB/one core and at most 100,000 literal inner assignments. Treat errors as new counterexamples and variants. Those backend checks cannot establish hard all-roots performance coverage.

Structural successor: derive and test an exact Artin-Schreier-block plus bounded-displacement residual solver, retaining all pivots and singular branches. Compare it with the existing AS/Frobenius compiler and strongest packed direct method, charging setup, extraction, reconstruction and serialization. The rank-eight route is retired; no unchanged displacement retry is queued. Gohberg-Kailath-Olshevsky and bounded-degree linearization remain primary prior art, not novelty evidence for this program.

The requested 8x goal and unchanged genuine-F4/F5/F6 release gate remain open. There is no complete original 339-variable root solve, source/ghost/chart closure, new field-wide timing qualification or end-to-end speed ratio. Sole-model preparation is not independent external source admission. Internal management provenance remains separate from the public mathematical checkpoint.

Reproduction: restore the exact raw JSON if artifact maintenance relocated it, then run python research/autolab_cycle_20261115_v27/REPLAY_RECORDED.py. Use a fresh frozen plan and new raw output paths for new worker executions; never overwrite prior evidence.
