#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Mixed-fiber F4-backed DLP: counted lower bounds

*Commands:* `five_leads_mixed_f4 --dlp-cohort`, `mixed-fiber-polynomial-dlp --f4-replay`, and `mixed-fiber-f4-analyze`. *Host:* shared Intel Xeon 6975P-C; L0 operation counts and unqualified internal F4 wall times.

The native F4 solver built the Semaev-S4 resultant plus three fiber equations and returned complete rational roots for 2,290 nonidentity relation queries across the 540 original one-target arms. Another 26 identity right-hand sides needed no polynomial system. Every F4 root set matched exhaustive polynomial enumeration; there were no timeouts or mismatches. The collector consumed these roots, sign-lifted them, recovered all 540 scalars and source-verified every answer. The analyzer matched each target, query list, relation count and answer to the pair-table and exhaustive-polynomial controls.

F4 reported 467,482,105,353 prime-field row-reduction multiplications. At M+0.5S+10I and 0.1M per declared native logical operation, the resulting *lower bounds* against matched rho are 160899× homogeneous, 232110× mixed and 239616× matched random. All 36 pooled pricing cells lose to rho and the pair-table control. Mixed/homogeneous is 1.44258× in the counted lower bound, with 36/36 losses.

#figure(image("cost-ratios.svg", width: 100%), caption: [F4-backed DLP counted lower bounds versus matched source rho; 180 targets per arm.])

*Raw F4 roots:* `correctness/mixed-fiber-f4-dlp-oracle-v1.jsonl`; *verified DLPs:* `correctness/mixed-fiber-f4-dlp-v1.jsonl`; *full grid:* `results/mixed-fiber-f4-dlp-lower-bound-price-grid.csv`. Symbolic preprocessing, substitution field operations, allocation and control flow are uncounted. The summed F4 internal wall time was 356.1 s on a busy host, not a qualified rho wall-time ratio. No external msolve comparison ran.
