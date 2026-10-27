#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("external: recorded execution")

#text("Job 20261023-192006-twist-image-msolve-6e5855. Outcome: done, recorded exit 0. Host class: c7i.xlarge, lifecycle spot. Started 2026-10-23T19:21:31Z; last status update 2026-10-23T19:26:55Z.")


#text("Official msolve 0.11.0 versus native complete GB on 378 fixed inputs, eight passes, all bases equal; outer isolation admitted.")


#image("graph.svg", width: 100%)

== #text("Command, controls and evidence")

#text("Exact command SHA-256: 7083e649e5625fcfaa3a9d1bc690b778394dd5061c71cee5796f000807814865. The command, native raw output, exit receipts, setup/build logs and host metadata are in runs/jobs/20261023-192006-twist-image-msolve-6e5855/ after extracting the content-addressed archive. Job snapshots record the source revision; frozen cold source is bundled separately. The registry specifies all arms, fields and seeds through these commands; the panel protocol and its supplements define the controls and caps.")


#text("This job has 0 originating resource-admitted cold-pipeline timed records. Copied checkpoints are not additional samples. A cold graph displays every originating admitted cold record, identified by the native JSON. A diagnostic count graph displays the recorded summary's signed relations and rank/columns, with each population labeled in the JSON; it establishes no wall-time advantage. Replayed summaries are deterministic checks rather than independent targets. Medians and paired ranges used for the final comparison are independently recomputed across complete whole rounds in results/summary.json; a per-record resource pass alone does not admit an incomplete round. The full-column phi-L0 arm is censored consumed cost, not a verified solution. External GB timers are a separate population described by the external report, not included in the cold-record count.")


#text("Recorded exit-count map: {\"0\": 1}. Recorded summary-status map: {}. Recorded independent audit-status map: {}.")


#text("Raw observations: results/job-observations/20261023-192006-twist-image-msolve-6e5855.json. Partial JSON records, failed exits and refused isolation are retained in the raw archive. Missing isolation is missing, never an admitted time. No performance conclusion follows from a job's done/0 state alone. Build and test gate durations are not algorithm timings.")


#text("See protocol (../../PROTOCOL.md), cost boundary (../../BOUNDARY.md), final report (../../README.md) and raw reproduction (../../ARTIFACTS.md). The msolve job alone measures internal GB timers on 378 fixed systems; it does not run a complete external DLP solver. Other jobs with fewer than two comparable admitted timings use an execution diagram rather than an invented numerical plot.")
