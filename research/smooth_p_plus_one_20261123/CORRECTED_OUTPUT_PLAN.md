# Corrected-input output panel: design and admission

2026-11-25 · `corrected_output_v1` · Planned; no solver run submitted

The next panel checks the qualified canonical-form implementation on the
same public toy targets as the retained liftable-fiber panel. It compares
complete signed relation sets for LiftableCutoff and CofactorMsolve against
the native Direct reference. **Thirteen targets, twenty-six planned answers,
zero executions in this component.** This is an output qualification plan;
all performance measurements and full-cost recovery remain separate work.

The [machine-readable plan](corrected-output-plan.json) binds the scientific
design. It is a local planning record, not a validated
`experiment.protocol/v1`, a launcher request, a reservation or an admission
receipt. Source and build identity reuse the completed
[semantic guard qualification](SEMANTIC_GUARD_RESULT.md).

## Inputs and unchanged controls

```text
driver source: 55f8d14f3b5fb1c997740ce8952360c2a31535e6
driver SHA-256:
5d25b56793ff93865eeb130fd30466d651911e95bdbb8b0f49aff42faa5a9ffe
msolve SHA-256:
74088d6775c82845faedcbdc24a82d3709e39ee7ebf3e3e4aabde080f9ab3c28
```

These identify local qualified bytes. A future container must bind its actual
driver, solver and shared libraries, OCI digest and source provenance before
submission; this plan does not invent those missing image identities.

| Panel | Ordered targets | Planned answers |
|:--|:--|--:|
| F127, depth 4 | (20,91), (116,69), (41,95), (41,32) | 8 |
| F8191, depth 4 | 17G, 43G, 89G, 152G | 8 |
| F8191, depth 5 | 17G, 43G, 89G, 152G, planted (4375,6159) | 10 |

The four F8191 multiples have coordinates (792,883), (1806,2202),
(2071,2384), (2228,5734), as recorded in the historical panel. This is an
explicit regression cohort, not a new random sample or a blinded comparison.
The generator and base construction remain at the pinned source. Depth-5
planted-target selection remains its deterministic four-pool construction.

Use the pinned binary with the following argument suffixes, in table order:

```text
127  4  /absolute/msolve  liftable-cutoff cofactor-msolve
8191 4  /absolute/msolve  liftable-cutoff cofactor-msolve
8191 5  /absolute/msolve  liftable-cutoff cofactor-msolve
```

Candidate precedes comparator for each target, as in the prior panel. The
Direct reference builds the full admissible signed relation set. Existing
base construction, pair-index setup and JSON emission lie outside each
query deadline; they must remain visible in whole-process accounting.
The unchanged finite construction audits are retained rather than repeated
solely to generate new output.

\newpage

## Acceptance and failure policy

Keep three seconds per query, all 16 fixed forms, the 20000-term and
262144-byte capture limits, and every semantic-check cap from
[SEMANTIC_GUARD.md](SEMANTIC_GUARD.md). Require the recorded
`canonical_semantic_v1` policy and a semantic receipt for every positive or
empty output admitted to extraction. Positive RURs must pass all equations and the exact form before
signed extraction. The inherited degree-separation retry is unchanged;
semantic rejection adds no retry. Construction and validation consume the
same absolute query deadline.

An answer qualifies only when extraction completes and its entire signed
relation set equals Direct. Empty markers retain the explicitly unchecked
solver contract; equality to the finite Direct reference is the panel's
separate completeness control. A positive semantic receipt alone cannot
establish omitted components or solver dimension truth. Preserve each
system, canonical input, form, raw RUR, receipt/error and returned relation set
under the unchanged capture caps, plus panel stdout, stderr and exit status.
A capture omission at a cap remains incomplete. Never replace a rejected output with another
attempt outside the existing policy.

A mismatch stops that panel immediately; its remaining targets are reported
unattempted. Attempt the other two frozen panels once, provided overall
admission and budget remain valid. Timeouts, malformed output, guard failures,
capture limits and process failures are incomplete. Prefix rows are not
complete answers. No timeout expansion, automatic job retry, replacement
target, hidden field scan or new form family is part of this panel.

Panel-level success means all 26 planned answers complete and equal Direct.
Any mismatch refutes full-panel qualification; an incomplete answer leaves it
unestablished. Report all outcomes by target and arm, including missing rows.
Historical outputs remain byte-equal and retain their original fixed-form
policies. In particular, the earlier F127 comparator failure and omitted
(41,32) suffix remain in the prior record even if a future panel completes.

## Resources and admission

The intended backend is one ordinary Iso Lab Spot command job, one panel
process at a time, one worker, 1 CPU and 2 GiB memory. Preserve the prior
2 GiB virtual-memory limit, 60 seconds per panel and 5 seconds termination
grace. The proposed enclosing job deadline is 900 seconds, including queue
and provisioning, with zero automatic retries. Reserve a maximum USD 5 for
the whole component, including build, provisioning/idle time, all failed
attempts, storage and transfer. This is a proposed ceiling, not a reservation
or evidence of available daily/monthly headroom.

No timing interval from this ordinary command job qualifies as L2 evidence.
Do not infer a speed ratio from completion counts or diagnostic durations.
A later cost experiment must price setup, every relation attempt, validation,
deduplication, rank/linear algebra and scalar replay, and use the admitted
matched reference and A/A policy. The present driver rejects recovery mode
for LiftableCutoff, so recovery integration remains a separate deliverable.

![Admission and output-qualification path. The missing access, protocol and image bindings precede all three planned solver panels; no run or timing datum is represented.](corrected-output-flow.pdf){width=95% height=2.3in}

\newpage

## Current access evidence and next actions

The [admission inspection](evidence/corrected_output_plan_v1/admission.json)
records the accessible interfaces and reviewed source identities. No AWS or
Kubernetes CLI, configured AWS profile/credentials, kubeconfig or EKS MCP is
available in this executor. The authenticated GitHub route is a distinct
alternative, but its inspected workflow builds ecbench/fresh-owner and the
registered norm-branch payload; it has no input for this native driver plus
dynamic msolve dependencies. Substituting a deployed integration fixture
would not execute this plan. No workflow was dispatched.

The canonical protocol skill and its separate rules document could not be
read through the GitHub connector.
The retained earlier copy contains only a `VLT1` marker; it is not usable
protocol guidance. Canonical schema/checker access and a valid freeze/export
therefore remain unresolved. The checkpoint script is tracked and readable,
but no current S3 checkpoint or supported upload identity/token is present.
The [scoped resume note](CORRECTED_OUTPUT_RESUME.md) records this plan; it
is not an S3 receipt. The pre-existing tracked root resume aggregate remains
byte-equal; a current study checkpoint must be established before execution.
Live pool capacity, budget balance and reserved spending have not been read.

Before any build or run, obtain the already authorized execution route and
readable canonical protocol tooling; qualify and publish the exact container
payload and launcher binding; validate/freeze the canonical record; then
verify Git custody, live capacity, remaining daily/monthly balance and the
USD 5 reservation, and publish a current S3 resume checkpoint. Submit only
after all receipts bind the same workload and image. Reconcile any ambiguous
submission against its original request/job identity before another action.

The [Iso Lab skill](https://github.com/isogeny-labs/workspace/blob/main/.agents/skills/isolab-eks/SKILL.md)
requires: “Stop new launches when the budget is exhausted or its remaining
balance is unknown.” Its instruction and repository
`taskq/deploy/eks-karpenter/COST_CONTROLS.md` apply here because current
headroom is unknown. No provider, quota, identity or trust change is made.

Required artifact upload/byte replay for eleven prior raw files is still
unfinished. PR #5142 stays draft. The private Cloudflare hook retains 713
records, with no receiver URL or active sender. Those publication/delivery
dependencies provide no scientific validation and do not change this panel.

## Sources checked

Sources checked (KB): wrapper status and the two exact queries retained in
[kb.json](evidence/corrected_output_plan_v1/kb.json) exit 127 because no KB
executable is installed; no KB MCP was available. The fallback review reads
the actual fixed-panel driver, prior liftable result/protocol, parser
diagnosis, semantic guard qualification, and maintained execution guidance.

The primary [PKC 2016 prime-field paper](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
Algorithm 1 and Sections 3.2/3.4, already describes chains of low-degree maps,
auxiliary-curve isogeny factor bases and the full cost expression
\(P(p,m)+m!p\,T(E,m,L)/(\deg L)^{m-1}+(\deg L)^\omega\).
It motivates retaining setup, relation collection and linear algebra in any
later comparison. This component changes output acceptance; it asserts no
new map mechanism. The implemented family remains the norm-one torus/Dickson
construction; an actual auxiliary elliptic-curve isogeny chain is not added.
