#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Serif",size:9pt)
= Cubic coefficient-map development
*Development controls only. No n83 cubic production or new FFD/LFD result.*
The implementation and raw controls prepare the degree-seven coefficient map.
Current source-bound conditional FFD intervals7–17 remain unchanged.
#image("controls.svg",width:100%)
*Recorded checks.* Six small-prime maps have matching sparse and independent dense ranks:2/16,34/84,181/260 for ell1,2,3 at each of primes11 and101. The independent terminal-square control at ell8 has rank56/56 at both primes. A seven-selector terminal cubic annihilator is retained explicitly; it demonstrates why the previous projection size cannot simply be reused. It is not an original27-selector kernel.

*Coverage and correctness.* All ordered triples of local indices0–26 are covered by nine-selector subsets. A pair-complete but triple-incomplete family is rejected. Cubic monomials exclude repeated selectors but retain intermediate cubes. Independent quartic expansion, four deterministic cap reasons, replay and corrupted-journal controls pass.28 native invocations comprise12 new and16 duplicated donor invocations. The S3-restored test executable passes all28 again; its raw control artifacts are retained separately. The development CLI rejects an83-bit prime without creating output.

*Prospective domain sizes: exact combinatorial counts, not measured ranks.*
#table(columns:3,inset:3pt,[m],[Columns],[Production rank],[3],[6608],[unmeasured],
[4],[25536],[unmeasured],
[5],[69760],[unmeasured],
[6],[155440],[unmeasured],
[7],[302736],[unmeasured],
[8],[535808],[unmeasured],
[9],[882816],[unmeasured],)
*Interpretation.* Full injectivity, triple covering and the source-normalization argument could provide conditional FFD≥8. Deficiency,caps and interruption cannot provide that bound. Production requires its own frozen resource envelope and all42 source identities. No exactFFD,newLFD,regularity,solving-degree,performance or elite-base improvement is established. The full three-family m2–9 objective remains open.

*Retention.* PRIME_CUBIC_FORMAT.json describes sparse monomials,rows,pivots,journal events and control files. The archive contains source,executables,logs and fixtures; it was restored from S3 and hash-verified before and after replay. Source:2475a71af6518d7a6e222434ea6c8486105481b5. Archive SHA256:68697a529713a52d6ad657defc5ccc7bcef836292d05029f5f4d59d9287450e7.

Definitions:#link("https://arxiv.org/html/2112.05579")[Caminata–Gorla,Definitions1.3–1.5 and Section4]. Study-specific argument:PRIME_CUBIC_DESIGN.md. Independent scientific review pending.
