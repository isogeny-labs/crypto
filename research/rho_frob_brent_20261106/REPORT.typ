#set page(margin: 2cm)
#set text(size: 10pt)
= Brent cycle detection in the fully priced Frobenius rho
2026-11-06 · protocol frozen before data (`PROTOCOL.md`, commit f26d48c0af) · eight Koblitz curves, 32 targets × 3 rounds, paired by target

*Question.* Is `rho.signed_frobenius_v2_full` (Brent detection) cheaper than `rho.signed_frobenius_full` (the legacy walk, the `rho-frob` reference) in fully priced GAE?

*Answer: mixed.* The v2 walk is cheaper on three curves: n23 1.095× [1.029, 1.164], n31 1.47× [1.20, 1.80] and n37 1.065× [1.014, 1.119]. It is dearer at n13: 0.718× [0.58, 0.89]. The rest show no difference, n41 included at 1.0008×. Every record replays, and the A/A control reads 1.000. All of these are unverified assertions.

#figure(image("fig_brent.svg", width: 100%), caption: [Left: S(legacy)/S(v2) with 95 % intervals. Right: mean S per arm.])

*Mechanism.* The legacy walk loses walks to the step cap when a cycle is longer than its window; Brent removes those losses, which matters at n23–n37. On the tiny n13 graph, Brent's escape by doubling keeps landing in new cycles, while the legacy walk abandons and restarts, which is cheaper there.

*Meaning.* Switching the reference would make it stronger by up to 47 % at n31, and every IC row's distance to it would grow by the same factor. No reference is changed here.

*Limits.* The interval is reported at full prices; the band's point estimates agree at weights 0, 0.1 and 1. The run is local, and counted GAE does not depend on the host.
