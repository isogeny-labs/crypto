# Runpod: rho native arithmetic price validation

[Report](report.pdf) · [Original Rust summary](summary.json)
· [Both exact raw workers](native.raw.json) · [Native raw export](raw-export.json)
· [Flow](flow.jsonl) · [Frozen protocol](../AMENDMENT_27_NATIVE_RHO_PRICES.md)
· [Original custody](local-evidence.json) · [Cleanup](cleanup.json)

Both public-input rho workers recover72701199046 on p101,order265379843621,
seed317. Their actual469686steps and full semantic operation traces match,
as do their additional elliptic replay primitive vectors. The original
copied Rust qualifier passes actual-scalar/held-out binding, calibration,
independent explicit phase reconstruction, inversion-once, phase sums and
existing-unit repricing. [pcc:unverified]

| Source algebra | Existing harness | Native tower |
| --- | ---: | ---: |
| Product and square: Fp M / A | 55 / 44 | 55 / 44 |
| Inverse: Fp M / A / I | 150 / 104 / 1 | 80 / 42 / 1 |
| Observed primitive subtotal, bitops | 117413099033 | 114521413145 |

This corrects arithmetic coverage. The walk is unchanged and this is not
an algorithm speed improvement, IC comparison or rho-parity result. The
native counted source uses the same deferred Barrett algebra; production
removes the additional hooks. The explicit rho tally is reconstructed from
its recorded operations, and only the separate outer replay is added.
Legacy TUNED credits remain separate. Native bookkeeping, RNG internals,
tuned-reference pricing and complete totals remain open. [pcc:unverified]

[The coverage note](coverage-note.json) makes diagnostic-calibration scope
explicit: the legacy harness_prices call and the new Harness calibration
are outside the reconstructed subtotal, while whole-process wall includes
their execution. This note changes no original raw, qualifier or summary.
[pcc:unverified]

Sourceafdfc612cc74eba2ce59d6ec059412a81fab557a was committed and pushed
before the sole scientific dispatch. The user's backend switch to Runpod
occurred before any32data. CPU-only pod s1ngkkcetckkq6 had4vCPU/8GiB,
quoted0.12USD/hour; CPU37was bound from observed affinity. The Docker image
was pinned by digest, and an authenticated native Rust capture service
received a credential-free immutable bundle. No Python scientific or
bootstrap code was used. [pcc:unverified]

One fixture and two workers complete, one validation/zero warmups,
60secondsfixture/900perworker/1900whole/no retries. Exit0in0.491seconds
is a practicality observation without A/A intervals. All264source hashes,
input bundle and portable pre-call/pre-worker/post-call/qualified-output
hashes pass. Actual scalar replay and exact trace criteria were unchanged.
[pcc:unverified]

The exact11175-byte harness raw,12361-byte native raw,8900-byte summary
and32309-byte native export all fit Git and are copied byte-exactly.
The complete sealed bundle/returned archive, copied executables/source and
host facts remain in the local custody directory. The native export makes
no canonical-schema conformance claim. [pcc:unverified]

Artifacts were downloaded and hash-checked before deletion returned204;
subsequent lookup returned404. The failed bootstrap pod was separately
terminated before the corrected deployment; its redacted startup error and
cleanup are retained in validation/runpod32-startup-*. No scientific call
ran on that failed bootstrap. Runpod API credentials stayed outside the
pod, repository and scientific bundle; the separate capture token was
removed from the measurement process environment. [pcc:unverified]

The single final PDF page was visually inspected. Exact PDF/SVG/150dpiPNG
benchmark copies and local originals pass their hashes. The presentation
copy uses the actual recorded CPU37; the original frozen template remains
locally hash-bound. Raw, qualifier, criteria and original summary are
unchanged. PCC-20261116-0e87ec is unverified; no independent signed verdict,
complete-cost ranking or canonical admission is asserted. [pcc:unverified]
