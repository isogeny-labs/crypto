# Native hyperelliptic IC follow-ups, 2026-11-14

[Report](REPORT.pdf), [ratio graph](ratios.svg), [large-prime trials](large_primes.svg).
Source and protocol: [study](../../research/hec_ic_speedup_20261113/FOLLOWUPS.md).
Record: [20261114T2210Z-hec-followups](../records/20261114T2210Z-hec-followups.json).

Hybrid median solve time / Fast is 0.50–0.78 at genus2 p>=503, over twenty seeds per instance.
The direct oracle's median stage ratio is 0.84–0.87 at genus3 and 0.83 at genus4 p61, but 0.96
at p31 misses the 0.90 bar. Reference oracle and Fast remain defaults.

The genus4 0/1/2-large-prime baseline completes all 36 arms with checked signed recombinations;
full catalogue projection/validation dominates its charged wall time. Both full and half bases
are reported. No cap is reached.

Unvalidated: one L0 arm64 macOS host, no independent replay certificate. External comparison
not run. Equation plus subgroup identifies these hyperelliptic instances; ICV1 is elliptic-only.
