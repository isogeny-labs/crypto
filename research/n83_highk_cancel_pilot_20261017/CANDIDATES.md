# Exact high-K worker-policy candidates

The four immutable IC1 manifests cross two enumerated root11 factor bases
with two target-worker policies under source SHA-256
`8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829`.
All four use two target workers; join waits for both, while cancellation
cooperatively stops after the first verified relation. The base construction,
packed root index, rank criterion, and source implementation are fixed for
each K. `generate_candidates.py` checks the source manifests and reproduces
the canonical IDs.

| Folded columns | Usable base points | Policy | Exact candidate ID |
| ---: | ---: | --- | --- |
| 2,816 | 467,456 | Join | `IC1N83Ckb1a1fb467456PDP4rootRCguidedLAgaussTDdirectISO0h974bf560d5c0` |
| 2,816 | 467,456 | Cancel | `IC1N83Ckb1a1fb467456PDP4rootRCguidedLAgaussTDdirectISO0h7a3cca2a9993` |
| 3,072 | 509,952 | Join | `IC1N83Ckb1a1fb509952PDP4rootRCguidedLAgaussTDdirectISO0hbf1f87cc02bf` |
| 3,072 | 509,952 | Cancel | `IC1N83Ckb1a1fb509952PDP4rootRCguidedLAgaussTDdirectISO0hde7207572a27` |

The currently running exposed-point pilot executes the cancellation arms
only. Its measurements do not select a factor base or worker policy. A fresh
Tier 1 factorial study must preregister a new target fixture, matched
same-point rho, alternating arm order, calibration, independent replay, and
an exact selection rule before dispatch.
