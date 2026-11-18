# C72: field/domain crossover and sparse-chain counterexample

Hypothesis: full-field degree-12 factorization has nearly interval-independent cost, while exact finite-difference scanning grows with interval size. A field/domain dispatcher frozen from C71 training may avoid both small-domain factor loss and large-domain scan cost. Scope is every original finite conditional root for fixed ordered source offsets, not independent-source search.

The frozen ladder covered all five standard NIST fields and 12,16,20-bit source intervals. Both exact methods matched on all 15 cases. Independent literal enumeration passed 96 changed interval systems and 4,848 shift/auxiliary checks. A factor crossover appeared by 16 bits in P192/P224/P256, whereas P384/P521 still favored scanning there. No field or loss was removed.

The cold panel completed all 90 preregistered runs: five fields, two sizes, three methods, three fresh-process repetitions with rotated order. All methods returned the same complete conditional root set on each case. Interpreter startup, parsing, polynomial setup, shifts, all finite auxiliaries, literal original verification and final stdout are charged in supervised child lifetime. External parent comparison bookkeeping is outside each child timing and inside full experiment lifetime.

| Field | Source bits | Factor median s | Difference median s | Dispatcher median s | Scan/factor |
| --- | ---: | ---: | ---: | ---: | ---: |
| P192 | 12 | 0.083094 | 0.044696 | 0.040029 | 0.538 |
| P192 | 20 | 0.082753 | 1.697525 | 0.087607 | 20.513 |
| P224 | 12 | 0.101234 | 0.036192 | 0.038437 | 0.358 |
| P224 | 20 | 0.098045 | 1.661209 | 0.096919 | 16.943 |
| P256 | 12 | 0.108623 | 0.038668 | 0.041685 | 0.356 |
| P256 | 20 | 0.108703 | 1.790304 | 0.097764 | 16.470 |
| P384 | 12 | 0.195935 | 0.038965 | 0.039018 | 0.199 |
| P384 | 20 | 0.184577 | 1.815935 | 0.196887 | 9.838 |
| P521 | 12 | 0.403991 | 0.041654 | 0.040130 | 0.103 |
| P521 | 20 | 0.395038 | 2.007915 | 0.395893 | 5.083 |

At large domains factoring beats scanning by 5.08x to 20.51x in these development medians; scanning wins on every small domain. The predeclared dispatcher selects the faster backend class on all ten cases. Jitter and overhead remain: several repeat groups exceed the 10% spread gate. No qualified host isolation, genuine F4/F5/F6 comparison, hard fastest-baseline floor, twenty-instance holdout or 50x gain is established. These are exposed training fixtures; true independent-source search is unresolved.

Memory was recorded for every cold child: wait4 lifetime and child peaks ranged 12,544–12,800 KiB; post-exec /proc sampled peaks ranged 9,660–9,836 KiB. The marker precedes solving. Samples are lower bounds, and lifetime peaks include inherited/pre-exec effects. All raw outputs and sampling counts are archived; small conditional measurements do not predict hard-target memory.

A third experiment tested sparse structure: x_(i+1)=x_i^2, x_k=1 has a path graph of treewidth one. Over F257, k=1..8 gives 2^k distinct original tuples; over F256 it gives one because Frobenius is bijective. All 16 cases matched independent complete forward enumeration. Thus graph sparsity alone cannot predict all-roots cost. Test genuine linearized/Frobenius blocks in binary systems and separate odd-field image/factor machinery, tracking eliminant degree, remaining parameters and output size. No novelty/general speedup claim follows.

Three frozen <=90-second, 4-GiB, one-core experiments ran through resume/run-ready. The scientific goal remains active. Next: profile actual independent-source summation chains for guarded linearized blocks, using monomial dependencies rather than derivative-only graphs, and freeze exact small block oracles before performance work. Global GF(2^83) search, odd extensions and genuine full-width F6 remain active streams.
