# Four-module nonbranch norm replay

The fresh native run compiled Nonbranch, IndependentControls and Readback with exit 0, then obtained the intended exit 1 from ExpectedFailure. Independent archival parsing found exactly 30 distinct named axiom records, all restricted to propext, Classical.choice and Quot.sound. The expected failure contains one unsolved False goal and empty stderr. No compiler was rerun during this review.

## Mathematical scope

The source connects the polynomial identity n^2 - u1*u2*U = b^2*F to the actual weighted-Proj point stalk on y^2=F(x). At a nonbranch point selected by n(x0)=b*y0, it proves the additive valuation of n-b*y equals the sum of valuations of u1, u2 and U. The hypotheses remain explicit: a field, on-curve point, separable quintic, nonzero 2, nonzero y0 and b, the two support divisibilities, and the selected-sheet equation. The source derives domain and DVR instances from imported geometric results; it does not postulate an arbitrary valuation satisfying the desired equality.

Independent source inspection followed AffineRing to the polynomial quotient by Y^2-F(X), ProjPointRing to the actual Proj presheaf stalk, and pointRingEquiv to localization at the point ideal. This supports the stated conditional geometric scope. Source types were inspected; this run did not emit a separate full elaborated-type dump.

## Concrete controls and nonvacuity

The rational fixture uses F=X^5-5X^3+4X+1, u=X^2-1, v=X^2-4, n=-9, U=-81X and b=-9. It proves the coefficient tuple (-9,0,0,-81,0), both divisibilities and a Bezout identity with nonzero constant 38569 establishing separability. The identity reduces to 81+81X(X^2-1)(X^2-4)=81F.

At (1,1), n-b*y=0 and the selected norm valuation equality is instantiated with proved hypotheses. At (1,-1), n-b*y=-18 is a unit, while the product of support/residual germs is a nonunit; the source proves the analogous valuation equality does not hold there. ExpectedFailure tries the false sheet-selection proposition and produces the intended unsolved False goal. The 20 control declarations are listed alongside the 10 producer declarations in declarations.csv.

## Runtime, costs and custody

The three outer phase durations were pre 164.83639216498705 seconds, compiler 217.6436788199935 seconds and post 32.08078653505072 seconds. Total launcher duration was 414.67755899898475 seconds; caller duration 414.78407207201235 seconds. These are different accounting boundaries. The campaign conservatively charges 2,400 active worker-seconds. The graph shows recorded phase durations, not a performance comparison.

Each phase retained one CPU, 8 GiB memory, zero swap and 128 tasks. Phase limits were 900/600/900 seconds, serial with no retry. Exact argv is archived. The four Lean commands were reaped with empty remaining process groups. Three manager-journal receipts match exact PID 1, UID 0, boot, unit and captured invocation, with start-success followed by successful terminal records. Outer receipts report terminal ownership confirmed.

Pre and post dictionaries are exactly equal: 16,736 dependency files and 3,111,892,318 bytes. Module hashes match the frozen four sources; the current freeze matches the coordinator release. Readback used raw files rather than trusting summary booleans. Nonbranch emitted two retained letI style warnings. They are not proof holes or compilation failures.

## Review boundary and archival record

Recorded machine status is passed; independently inspected source fidelity is faithful within the conditional nonbranch theorem and explicit rational controls. This reviewer did not author the frozen mathematical sources, but had already seen the reported outcome and is not a blind re-deriver. No independent binary/kernel rechecker or additional reviewer compiler run occurred. Therefore this packet does not claim a new formal validated_scoped promotion under the full formal-review workflow.

Pinned shared caches remain the execution trust boundary: equal pre/post hashes do not exclude transient concurrent changes. Later branch-point, finite valuation, branch transport and root-multiplicity packets are outside these 30 names. The report makes no cohort-cost or runtime-speed claim.

The 59-file native/control archive has SHA256 8dfaaba7ccf538485ed0d2ab4585bfc06140cc793a94c44a75172fae1d8e425e and 2,462,556 bytes. Artifact-store push, remote verification and separate full GET/hash verification preserve it. .artifacts.json gives the content-addressed reference; raw-manifest.json lists every file. readback.py repeats the archival consistency checks without invoking Lean. Rehydrate using ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull with this packet directory.

Sources: exact frozen Nonbranch and IndependentControls, scope.json, native readbacks, dependency dictionaries and owner receipts; Common22SeparateChartMap and WeightedProjNonbranchZeroOrder definitions. KB was unavailable, so exact repository sources were used. The technical review changes no hypothesis or official ledger status.
