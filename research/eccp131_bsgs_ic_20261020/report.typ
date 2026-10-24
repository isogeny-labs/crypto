#set page(paper: "a4", margin: (x: 16mm, y: 16mm))
#set text(font: "Libertinus Serif", size: 9.2pt)
#set par(justify: true)

= Certicom 130-bit challenge workflow integration

#text(size: 8pt)[Recorded 2026-10-20; exact-curve probe added 2026-10-20 · Darwin arm64 · Rust/Cargo 1.93.1 · PARI/GP 2.17.3]

== Exact identities and command paths

The published #link("https://www.certicom.com/en/curves-list")[Certicom curve list] supplies the prime-field ECCp-131 group and the distinct binary Koblitz ECC2K-130 group. The ECCp-131 field, curve, generator, subgroup order, cofactor, and published point are bound in the #link("../../src/ecc/curve.rs")[curve constructor] and #link("../../src/bin/ic/params.rs")[IC profile]. ECC2K-130 already has a fixed-parameter file in `aburan28/cryptanalysis`.

#figure(image("workflow.svg", width: 100%), caption: [The exact prime two-sum probe reaches zero natural rank under its fixed budget; the published-point scalar and full IC cost remain open. Source: `workflow.dot`.])

== Observed validation

- `cryptanalysis` commit `3ac3f59363` passed two ECCp-131 release unit tests; `ca-ic inspect` checked both published points, `ca-icx` identified its scaled regime, and `crax --analyze-only` accepted the target. ECC2K-130's fixed-parameter status was stopped with zero saved relations; #link("ecc2k130-status.json")[raw status] is retained.
- `crypto` commit `07dd1d97fa` passed exact-group bounded BSGS checks; the release CLI verified planted scalar 12345 on both groups and refused an oversized challenge interval. The earlier full library run had 4,445 passed, 11 failed, and 119 ignored; fixture-related and timed-case reruns are detailed in #link("REPORT.md")[the source report]. `ic inspect` checked both prime points; PARI/GP later checked the full order.

== Exact prime IC admission

The #link("EXACT_IC_PROBE_PROTOCOL.md")[fixed protocol] used the published ECCp-131 generator and target, 512 first on-curve abscissae, 128 seeded setup candidates, 128 target candidates beginning with the published `Q`, one Rayon thread, and a 600-second watchdog. The release unit control recovered a planted scalar from natural rows on a separate generated prime-order curve.

- Independent #link("eccp131-pari-check.out")[PARI/GP output] checked `ellcard(E)=n`, both points on the curve, and `[n]G=[n]Q=O`; a #link("eccp131-primes.out")[separate check] returned `isprime(p)=isprime(n)=1`. Its #link("eccp131-pari-pair-sums.out")[complete two-sum enumeration] found 262,144 distinct abscissae from `B=512`, covering 524,288 nonzero points; `Q` itself is outside that set.
- The #link("ic-prime-eccp131-b512-seed131.json")[raw `ic prime` result] matched those counts and detected and reconstructed 8/8 planted sums. It recorded zero natural setup rows, rank 0, zero target-descent rows, and an undetermined target column. The command exited 0 as a completed diagnostic. Process wall was 1.12 seconds and maximum RSS was 24,477,696 bytes; these are probe costs, not full-solution costs.
- For this two-sum base, the exact uniform hit probability is `524288/(n−1) ≈ 3.3824×10⁻³⁴` (`2⁻¹¹¹·¹⁸⁷⁵`). Even with independent rows on every hit, collecting `B+1` rows has an optimistic expectation near `2¹²⁰·¹⁹⁰³` candidates. At the maximum admitted `B=8192`, the two-sum support bound is `2⁻¹⁰³·¹⁸⁷⁵` per candidate. These are cardinality calculations for this method, not a lower bound on every IC strategy.

== Boundary and next check

The exact two-sum code and bounded probe now run on ECCp-131, while `ca-ic prime` in `cryptanalysis` still uses a generated smaller curve for its relation pipeline. The published scalar, a natural full-rank corpus, target descent, and complete cold solution cost remain required; #link("../../problems/eccp131-full-parameter-index-calculus.md")[the problem record] tracks them. PARI/GP found `p−1=2·5·r` with a 127-bit prime `r`, limiting the direct smooth-`p−1` construction; auxiliary-curve and other oracle routes remain untested. Bounded BSGS still requires a justified interval. No speed or full-parameter IC cost conclusion is made here.

The existing index-calculus scoreboard and progress timeline were checked. Neither has a new measured row from this integration, so their canonical data and rendered copies remain current. Full commands, revisions, initial sparse-checkout setup failures, raw artifact digests, and status distinctions are in #link("REPORT.md")[REPORT.md].
