# Conductor-gap size expansion and precision target

Follow-up to [PR #2560](https://github.com/isogeny-labs/crypto/pull/2560),
which merged into the still-open parent #2430 on 2026-10-26. This study
keeps parameter feasibility, explicit curve/map construction and completed
ECDLP comparisons separate.

See [the frozen protocol](PROTOCOL.md) for the size ladder, certificate
requirements, 99% simultaneous-confidence objective and pilot rule.
The [results report](REPORT.md) and [inspected PDF](REPORT.pdf) record what actually executed; the
underlying larger-target measurement requirement stays open in
[the problem record](../../problems/conductor-gap-crater-sampling-and-size.md).

No large-group solver campaign is launched by this parameter search.
The earlier crater interval and frozen measured archives remain unchanged.

The independent native verifier accepts 128 native-size and six upper
parameter classes, including 254-bit prime subgroup-order parameters.
Exact models, floors and maps remain unconstructed. The proposed million-
solve confirmation is blocked pending these constructions, a disjoint
pilot and a resource estimate; it has not started.
