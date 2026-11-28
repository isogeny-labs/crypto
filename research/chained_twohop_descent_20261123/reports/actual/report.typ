#set page(paper:"a4",margin:16mm,numbering:"1")
#set text(font:"DejaVu Sans",size:9.5pt)
#set par(justify:false)
= Two-hop Richelot descent
Finite-kernel qualification · 23 November 2026 · PR 4992

#text("Native C enumerated all225 two-step routes and the retained mod4 lifts of four fixed Frobenius representatives. Lean-only D compiled all nine explicit finite-model statements and printed no axioms. Complete small raw streams are public; the first A/B/C startup failures remain retained. Unsigned claims: PCC-20261123-e04b98 and PCC-20261123-1add8a. [pcc:unverified]")

== Written geometric boundary
#text("Let char(k) differ from2; let A,C be abelian surfaces over k; and let f=psi composed with phi be k-defined with both geometric steps of type(2,2). Assume A[2](k)={0} and L=ker(phi) is not Galois stable. Then ker(f)=A[2] and f=u composed with[2] for a k-isomorphism u:A→C. This is a standard classification consequence, not a novelty claim. [pcc:unverified]")

#text("Proof. K=ker(f) has order16 and is killed by4: phi(x) is in the second2-torsion kernel, so2x is in L. Since2 is invertible, these are finite etale kernels. If K has type(4,4), its four-element subgroup K[2]=2K equals L and is characteristic, contradicting nonstable L. If K has type(4,2,2), its doubled subgroup has one nonzero point, fixed by Galois; that contradicts A[2](k)={0}. The remaining type is(2,2,2,2), hence K=A[2]. The k-defined quotient[2] factors f with degree-one isogeny u, giving the claimed k-isomorphism. [pcc:unverified]")

#image("flow.svg",width:100%)

#text("The classification and quotient/descent bridge above are written mathematics; the Lean module formalizes explicit finite models. A rational endpoint equation does not prove descent of the selected composite morphism. Nonrational individual quadratic factors can define a stable whole kernel. Rational first kernels, longer chains, odd-degree maps and trace/norm constructions are outside this conclusion. [pcc:unverified]")

#text("Inheritance: Magma Richelot handbook, double-Richelot paragraphs; Castryck–Decru–Smith2020, section6 Definition2, DOI10.1515/jmc-2019-0021. Prior CT13 excludes mixed kernels on its odd-order fixture; CT21/T20 and cryptanalysis CT50 establish the earlier six-cycle single-rational-step forced return. Exact CT50 source is retained at f9c7b49e. [pcc:unverified]")

#pagebreak()
== Complete native component C
#text("Source ee45e761; Rust blob4541fb295d255f68564437efa84e28772985d701; result fcc9079069993385b6d5b169f4d72208a46ae1b6. All720 root permutations are compared in symplectic F2^4 and even-subset representations, checking16 elements and256 ordered pairs per action. There are304 fixed-free actions:40(3,3),120(6),144(5,1). [pcc:unverified]")

#text("Every15 first Lagrangians has15 quotient Lagrangians:225 routes, with8 good/6 mixed/1 dual per first kernel. There are151 distinct composite kernels:120 good,30 mixed,1 dual. Full groups, closure, isotropy, quotient pairing, doubled images and two-torsion sets are checked. For each of four fixed root permutations, all65536 entrywise mod4 lifts are tested;1024 of each odd similitude multiplier are retained. Every retained lift is tested on every complete route kernel. No order-three assumption is imposed on a lift. [pcc:unverified]")

#image("stable-incidences.svg",width:100%)
#text(size:8.5pt,"Recorded counts from complete native stdout; each representative has2048 retained lifts and225 routes. Bars count route/lift incidences, not distinct maps or geometric realizations. The dual kernel repeats for each first route. No timings, sampling uncertainty or performance ratios. [pcc:unverified]")

#text("Controls: one copied mixed record with a corrupt doubled-image invariant is rejected; its restored record is accepted. Full positive witnesses retain stable good K with rational first L even in a fixed-free action, and stable mixed K with nonrational first L when rational two-torsion exists. They protect both hypotheses. Arbitrary retained mod4 lifts are not claimed to arise geometrically. [pcc:unverified]")

#text("Whole stdout:33494B, SHA256 c18c77fabe991d1845014f0cddc77f73336b2e1d5c69b7bbf8569c1344a52da7. Every225 complete kernel bitset is public in first-C-native-pass-Lean-startup-failure/native.stdout. Four consecutive u64 hexadecimal buckets encode the256 mod4 points; each coordinate uses two bits. No row is summarized away or split to evade artifact policy. [pcc:unverified]")

== Source and controls, rather than a benchmark
#text("One std-only Rust scientific execution; no package download, file/network input, clocks or worker threads. Strict compile, Clippy and format phases exit0. This is the exhaustive frozen structural development unit, not the original EKS-only full Jacobian workload. Source-operation/runtime or solver advantage is not measured here. [pcc:unverified]")

#pagebreak()
== Separately completed Lean component D
#text("Unchanged Lean blob459bbe1414a812cde2eaf38289db832819a2c567; source1d51fef4f3396948d7ea3a4fad681c906702c478. Actual Lean4.35.0 (eb6cafb7) compilation succeeds with warnings as errors and default logical proof limits. Every named statement prints no axioms. Full declarations and proofs are bound in COMPLETE_FORMAL_DECLARATION_BINDINGS.json. [pcc:unverified]")

#table(columns:(1fr,1.3fr),inset:5pt,
  [Written component],[Exact Lean coverage],
  [Mixed doubled subgroup],[doubled_range; doubled_generator; doubled_point_nonzero. Explicit C4×C2×C2 model.],
  [Mixed fixed point],[additive_injection_fixes_doubled_point; no_fixed_free_mixed_automorphism. Zero-preserving additive injective maps.],
  [Good doubled image],[doubling_covariance; doubled_image_preserved. Additive maps on explicit C4×C4; image inclusion.],
  [Cubic action control],[cubic_fixed_only_zero; cubic_order_three. One explicit F2^4 action.],
  [General geometric result],[No Lean declaration. Kernel classification, arbitrary Galois/abelian-variety realization and quotient factorization remain written dependencies.])

#text("Init only; no Mathlib/crypto-lean instance. No sorry, custom axioms or native evaluation is used. lean4checker --fresh is unavailable. Polynomial-only IDC1 over2^61−1 does not certify these finite-group/cardinality assertions. All producer claims remain unsigned; independent signed acceptance is pending. [pcc:unverified]")

== Flow, first failures and custody
#text("A: Lean version abort134 before science. B: smaller runtime-stack setting still abort134 before science, refuting the first repair prediction. C: strict Rust/Clippy/format/native complete0, then Lean version abort134; overall C is incomplete. D: only Lean version/proof resume, both0; no repeated native work. Source/tool/qualifier postguards match. Each run has a separate report and retained first streams. [pcc:unverified]")

#text("C trace records a successful1GiB anonymous reservation followed by a1073745920B MAP_STACK request rejected ENOMEM. D sets MIMALLOC_ARENA_RESERVE=65536 before initialization plus the recorded smaller runtime stacks. All calls retain2GiB virtual-memory caps,120s phase deadlines,one worker and default logical proof limits. Four actual Lean shared-library hashes are pinned; installed stdlib/OS are ambient rather than hermetic inputs. [pcc:unverified]")

#text("Whole D stdout624B, SHA256 cd634f65f94799e59cf2e3e65d8d1d79bbad43705a548a667535b53f840d3522. Full ELFs and377104B olean remain whole in temporary RAM/store-pending. Every small raw stream is checkpointed in Git. Combined E is prospective main-CI-only, requesting whole Actions artifact upload; no E execution or remote artifact existence is asserted. [pcc:unverified]")

#text("Remaining work: full geometric Lean bridge, signed independent validation, whole remote custody and a distinct next chain mechanism. Existing canonical scoreboard/timeline graphs receive no eligible cost row from this study. Required CI failures and49-check excess for parent4990 remain explicit. Primary runtime sources: Lean4.35 runtime/thread.cpp; mimalloc2.2.7 options.c/arena.c. [pcc:unverified]")

#link("https://github.com/isogeny-labs/crypto/pull/4992")[PR4992] · #link("https://github.com/isogeny-labs/crypto/issues/3904")[Persistent queue3904] · Full source/custody/proof map: RESULT.md
