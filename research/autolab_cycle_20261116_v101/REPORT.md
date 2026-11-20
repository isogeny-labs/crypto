# C101: guarded local search

Hypothesis: bind exact leading-coefficient guards to the same 76 parameters as fourteen local splitting predicates, then test a bounded necessary-constraint search. The proof and review are in the variant directory. All prior attempts remain intact.

The independent oracle passed 96 source-product controls, 256 parameter bindings and three full-prefix support controls. The guarded CNF has 51,165 variables and 187,211 clauses. Its SHA256 is 1d459da6ca814dc462d2030bb293398f6dadafe88e32538424b4997c1429f3f9. Repeated and zero roots are retained.

Pinned existing Z3 4.15.4 was copied locally, with native-file hashes recorded; no native binary is published. The preregistered single-core search used a 45-second deadline including formulation (2.770 seconds). It returned UNKNOWN/timeout after 45.089 seconds with zero models, 428,251 conflicts and solver-reported maximum memory 223.41 MB. This is internal engine memory, not process peak RSS; process evidence is archived separately. Two experiments ran through lab.runner resume/run-ready, each with a 90-second outer limit, 4 GiB and CPU 0. The oracle passed; search is inconclusive.

No original root count, UNSAT certificate, all-roots completion or qualified speed ratio follows. This tests one affine necessary family, not the full 339-variable chain. The release gate remains unchanged and unpassed; 50x is the user stretch target.

Decision: do not repeat the identical search. Next test native XOR elimination with exact back substitution before nonlinear branching, preserving shared parameters and all exceptional branches. See NEXT_QUEUE.json. This is an implementation/reduction hypothesis, not a novelty claim. Binary XOR structure motivates a different backend than the existing odd-characteristic quadratic/twist backend; neither is universally faster by field type alone.

Portable replay verifies the archived raw records and repeats the independent guard/support oracle. Large base graph and CNF are reconstructed from the already published C100 graph plus the small guard delta; their frozen hashes remain authoritative. Native search reexecution requires the exact library files in NATIVE_RECEIPT.json.
