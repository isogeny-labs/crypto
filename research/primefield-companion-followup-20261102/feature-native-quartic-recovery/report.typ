#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Quartic conditional recovery
2026-11-05 · T-1676

A pair-specific separating character completes the previous width14 mean polynomial. The runtime source can advance geometry and seeds while omitting an unneeded residual character. [pcc:unverified]

#image("pipeline.svg", width: 100%)

== Construction
For each two-point signature fiber with endpoints a,b, let mu=(a+b)/2. Choose an offset e where their characters have opposite nonzero signs. Set beta=(a-mu)/chi(a+e). Then mu+beta chi(x+e)=x for both endpoints. Cached signatures select the correction; singleton inputs retain the mean polynomial output. Runtime stores coefficients and sign patterns, not endpoint scalar labels. [pcc:unverified]

Compilation enumerates the public coefficient field and performs dense synthesis. This remains charged setup. The correction layer adds signature comparisons and rule storage; there is no compressed-setup claim.

== Source work and exact checks
The F479 policy uses14 signs for463 inputs and15 signs for16 inputs:6722 total sign values. Twelve exceptions request offset14; four request offset15. The latter skip the character at14 while retaining both intervening geometry and seed updates. [pcc:unverified]

Twenty public-point fixtures on E/F509, y squared = x cubed +2x+10, P=(3,68), r479, reuse one prepared source. They cover all16 exceptions and0,1,2,478. Their extra work is20 geometry steps,480 converted points and16 character evaluations. The direct-phase field counts extensions only; initial-window and preparation costs remain separate. [pcc:unverified]

The native suite records60 passing tests and22 source/data hashes. Tests compare skipped-step seeds with a complete window and count actual character-consumer calls. A separate checker imports no producer modules, rebuilds all479 signatures with a square bitmap, verifies the polynomial corrections, and checks20 public points by repeated addition. It rejects both a changed correction coefficient and a changed recovered scalar. Seven compiler warnings are retained. [pcc:unverified]

== Scope and next step
The full-field identity check and20 public-point integrations have distinct scope. No timing panel or cloud job ran. The original4563-case cohort is preserved. Next instrument complete costs for compilation, reusable preparation, initial signs, routing, geometry and residual evaluation before a comparative panel.

== Evidence and sources
Claims PCC-20261105-8014e0 and PCC-20261105-9938d9 have local verdict not_covered and no signed standing. This separate checker was authored by the producer. Source locks, raw logs and receipts accompany DERIVATION.md. Prior quartic and adaptive-source evidence supplied the construction; #link("https://dlmf.nist.gov/27.9")[NIST DLMF27.9] supplies the character convention. KB unavailable: repository wrapper could not find its executable. No novelty-priority claim.
