# N83 K5500 full control on the existing high-memory EKS runner

This workflow runs the pinned current-source K5500 point-0 correctness control
under a four-hour child cap, then independently replays its result. It is a
single-base component experiment. The full N83 factor-base selection also
requires the paired Tier 1 one-target IC/rho comparison and the remaining
factor-base arms.

The workflow has no manual trigger. It starts only when the separate approval
marker `.github/n83-k5500-highmem-approved.json` is pushed to
`codex/n83-k5500-highmem-full-20261021`. That marker is absent from this
preparation PR. Fill it from `approval_template.json` only after:

1. The capacity-only Actions job on PR #2394 has completed with a passing
   receipt for the pinned source and runner script. Record its run ID, exact
   head SHA, artifact ID and receipt SHA-256.
2. The existing `crypto-eks-highmem` runner's chargeable rate and quote
   source are verified. Record the five-hour allocation upper bound and a
   separate reserve. `rate * 5 + reserve` must fit the owner-approved upper
   dollar amount. Obtain explicit approval for this EKS allocation; approval
   for another provider or experiment does not apply.
3. The marker names the approver and a short expiry. Preserve the quote and
   approval record outside the workflow. Commit the marker only after these
   checks. The marker contains no credential.

The workflow downloads that one capacity artifact, verifies its byte digest,
status, required checks, pinned source and runner hashes, exact Actions run
and head, approval window, quote arithmetic and live host envelope before
building. The preflight writes a rejected receipt for local validation
failures; an absent or invalid GitHub artifact stops during the preceding
download step and retains the Actions log and any fetched metadata.
The full runner retains the public input, launch, stdout, stderr, base dump,
result and bounded-stop receipt. The independent replay writes a separate
status. Every outcome is uploaded as an Actions artifact. Any online timing
from this control needs Tier 1 noise and same-point rho evidence before it
can enter the factor-base selection.
