#set page(paper: "a4", margin: (x: 15mm, y: 13mm))
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false, leading: 0.58em)
#set heading(numbering: none)
= From principal norms to a grounded graph
*Rust source and Runpod protocol checkpoint · 17 November 2026*

The new production entrypoint `run_extension_quadratic_pair_dlp_on` admits two quadratic cofactor classes and passes exact signed witnesses into the existing sparse-rank and scalar-recovery pipeline. This report records implementation and finite controls; no new natural-yield or performance run has occurred. PCC-20261117-92f18d is UNVERIFIED producer evidence. [pcc:unverified]

== Algebraic admission
For a genus-three cover over Fp², construct the degree-nine norm A² − C²h. Nine, seven or five distinct small points leave zero, one or two quadratic factors. The bounded quartic splitter checks squarefreeness, absence of Fp-linear factors and the degree-two Frobenius condition, then attempts at most eight recorded equal-degree splits. Each admitted quadratic is irreducible, has denominator coprime to C and a valid ordinate v = −A/C modulo U. The complete factorization and both signed canonical classes are checked before graph admission. [pcc:unverified]

#image("grounded-witness.svg", width: 100%)

A source with one known endpoint extends a grounded witness; two known endpoints give an exact cancellation. Ungrounded edges are explicitly discarded. Expanding source witnesses as integers preserves coefficient 2. Sparse kernel certificates establish actual scalar-field rank. The existing full-only Nagao descent, protected affine solve and elliptic scalar check are retained initially. [pcc:unverified]

== Controls and independent replay
#table(columns: (1fr,1fr,1fr), inset: 6pt, stroke: 0.4pt,
  [*Worker controls*],[*Qualifier controls*],[*Transport*],
  [36 pass, serialized],[143 pass, serialized],[Three-arm simulation passes])

Both optimized native builds and scoped strict Clippy pass. Independent coefficient arithmetic checks line constraints, norms, small support, class signs and ordinates; direct Cantor addition checks each original relation. Independent graph replay checks stored/emitted integer incidence, including −2R₀ + R₁ + R₂ and corruption refusals. Complete candidate/split-RNG/cohort replay, dense rank, sparse traces and full affine checks are wired into the separate qualifier. Original build/lint and parallel global-accounting test failures are retained. These finite checks do not supply a signed execution verdict. [pcc:unverified]

== Registered measurement
Three fixed-order cold processes: original IC, current quadratic sparse/table collector, grounded two-quadratic collector. Same exposed p1009/seed389 public G/Q, binary, host and one allowed CPU; zero warmups, retries or exclusions. Bounds: 262144 unique sources and quartic histories, 65536 grounded classes, witness support 256, eight split attempts; 1018081 B values in batches of 1024; 8192 descent residuals and 32 accepted descents. [pcc:unverified]

Runpod allocation: 4 CPU / 8 GiB, at most 0.40 USD/hour, absolute 6000-second owned-runner lifetime. Fixture: 60 seconds; each worker: 900; execution wrapper: 3000; separate qualifier: 1200. Publish/hash-freeze source, protocol and binaries before allocation. Verify a detached deletion guardian before the sole dispatch; collect, hash, scan and delete before interpretation. [pcc:unverified]

Native counts remain incomplete, so complete costs and ratios remain null. This exposed, single fixed-order trial cannot admit primary rho parity; fresh unknown-log/scaling comparisons, strongest matched rho and signed independent admission remain separate obligations. [pcc:unverified]

*Sources checked:* Gaudry–Thériault–Thomé, three-author February 2005 preprint §§2.3/3.1/3.3 (author copy); Cavallar filtering; Shoup, author text §20.2; retained46/47 implementations and43 zero-anchor descent result. KB unavailable; original AMS Cantor–Zassenhaus PDF returned403. No novelty, uniform cofactor-class distribution or transferred asymptotic bound is claimed. Full URLs and SHA-256 identifiers are in Amendment43. [pcc:unverified]
