#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Serif",size:10pt)
= Source-bound terminal functional instrument
Observations only. This development stage adds native joins between an exact input/source, a retained terminal certificate and its factors, and a prior conditional top certificate. The positive test fixture is synthetic. The actual42saved n83cases have not run under this instrument.

#image("pipeline.svg",width:100%)

*What the checker verifies.* It checks the42key set, input/source/base identities and certificate hashes, regenerates the complete source template, and checks embedded input and source records. It parses and recomputes all coefficients of A0,A1,A2, checks their metadata and canonical encoding, and verifies the saved affine circuit. It then computes the coefficient C=A2−2tA1+t²A0, checks the direct pairing against its closed formula, and writes the complete C polynomial. Production requires a frozen protocol and the hash of the actual executing binary.

*Controls.* All22 native test invocations pass: six new tests and eight unique inherited donor tests through each of two module imports. The new tests cover a complete synthetic-source roundtrip, missing/duplicate case keys, changed input/source/base identities, certificate hash changes, top-identity and affine-circuit changes, and forged factor metadata. In the latter control, the factor and its digest are both changed; coefficient recomputation still rejects it. These are implementation controls, not22independent scientific results.

*Preserved fixture.* The archive retains the synthetic input, source, terminal and top records, all three binary factors, the resulting coefficient polynomial, binding record, exact Rust sources and Cargo lockfile, test executable and logs. The fixture output explicitly says it is not a saved production case. SOURCE_FUNCTIONAL_FORMAT.json specifies production paths, binary/JSON encodings, verification steps and replay instructions. Source hashes and S3 replay custody are recorded in validation-source-functional-development.

*Claim boundary.* The output is designed to record source-bound arithmetic. It does not compute V16, and it leaves LFD and regularity fields null. The filtered-space implication still depends on the separate conditional syzygy argument and independent mathematical verification. The earlier small closure dataset shows why a zero scalar pairing remains inconclusive. No elite-factor-base improvement or n83LFD result follows from these unit tests.

*Next execution.* Freeze a new protocol with the actual42case input/source/base identities, retained certificate/factor hashes, source revision and production binary. Stage all dependencies, qualify the execution backend, run and replay the complete panel, and preserve every failure. Do not overwrite prior results. The original Koblitz/general-binary/prime m2–9 and elite-base objective remains active.

*References.* SOURCE_FUNCTIONAL_PLAN.md; PRIME_FILTERED_FUNCTIONAL_DESIGN.md; Caminata–Gorla, arXiv:2112.05579, Sections1 and4. Development ID: source-functional-20261030.
