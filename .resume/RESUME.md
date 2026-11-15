# Resume: native graph index calculus toward rho parity

## Goal
Continue Rust graph/algebraic index-calculus implementation, matched experiments and hypothesis iterations until rho parity or an evidence-backed stopping decision. User explicitly requires Rust and has now authorized the local or Runpod backend. Do not treat missing EKS/S3 access as blocking this local route.

## Lead / hypothesis
The cubic-extension graph/pivot adapter reuses pencil_symbolic_graph. New symmetric S3 pair finding eliminates the product coordinate through a cubic in the sum coordinate, intersects the constant-component GCD, and retains denominator/dependent-equation recovery. Both pair-only and hybrid-triple variants are integrated. The first local pilot measures which costs remain. KB wrapper unavailable; repository S3/S4/graph sources and residual-walk sections 11.1/11.4/11.6/11.7/11.9 reviewed. No novelty or performance result asserted before data.

## Spec
research/extension_ic_graph_20261115/{protocol.json,AMENDMENT_1.md,AMENDMENT_2_LOCAL.md,local-pilot-plan.json}. User replied "Use local or runpod" to the missing-runner question. Local checkpoint plus published source/spec replaces the unavailable lab route; do not claim standard EKS/L2/S3 admission. Frozen first local pilot: p=67, seed=3, all five oracles, pivot_a/graph/pivot_b/rho_a/rho_b, one retained warmup, five measured rounds, 10000 residuals, one pinned allowed CPU, 8 GiB, whole-process 900 seconds, zero retries. Predeclare median AA tolerance 10%, round bootstrap and minimum n=2^30 for the large-order local-reference-parity flag. This small pilot cannot satisfy that flag. Cloud spend remains zero; no Runpod instance authorized beyond existing bounded scope was created.

## State
Regular PR https://github.com/isogeny-labs/crypto/pull/4288 contains native core and controls. Current continuation fixes the hosted Clippy slice-pattern finding and adds oracle selection and standalone Rust/Plotters analysis. Post-fix library tests report 30 passed; analysis tests report 5 passed; both production-library and analysis Clippy checks with warnings denied pass. Actual benchmark binary built. Source hashes and logs are under research/extension_ic_graph_20261115/validation/. Whole-parent library validation remains incomplete, and inherited Koblitz lint/evidence/benchmark-record CI failures remain documented. No parity claim. The next source commit is pinned by the runtime plan and binary hash before execution.

## Next steps
1. Commit and push current source/protocol/analysis. Create immutable local run /workspace/ic-local-runs/local-p67-all5-20261115-01 with copied binaries, runtime plan containing source SHA and binary hash, explicit argv, pre-call hashes, native source checkpoint, CPU/cgroup/OS/toolchain provenance.
2. Once no compiler/build processes remain, launch the frozen driver via timeout/time/taskset on the pinned CPU, retaining raw JSONL, stderr, process exit and resource use. Record PID/session and do not duplicate a launch if disconnected; inspect the existing run first.
3. Run the built native_analysis executable on the raw JSONL plus runtime plan into a new output directory; preserve all capped/failed/partial records. Inspect JSON and SVG, render a native Typst report, capture hashes and export flow metadata. Do not rerun unchanged data merely to improve controls.
4. Select the next hypothesis from measured wall ratios and explicitly incomplete legacy component shares, freeze successor inputs/budgets before execution, test/publish native changes and continue. Larger fresh subgroup sizes and a stronger matched reference remain necessary before a broad parity conclusion. Do not substitute rho fallback recovery for a graph-IC result.
5. Keep the existing PR current, fix source-specific CI failures, and preserve unrelated studies' records and evidence. Source custody is local+GitHub for the authorized local route; upload large raw artifacts later when a legitimate transport is available.

## Runs in flight
None immediately before the first local pilot. Planned directory /workspace/ic-local-runs/local-p67-all5-20261115-01 must be create-new. No EKS/CodeBuild/Runpod job exists. Earlier larger full-library output /tmp/extension-ic-full-library-tests.log is incomplete/interrupted with its hash retained in validation/. Exact sparse include fixtures were restored and patterns added; avoid broad sparse reapplication/archive hydration. Use /usr/bin/git.

## Stop and success conditions
Every scalar must replay; graph/pivot relation fingerprints and same-instance rho controls must agree. Warmups/failures/censors remain in raw data. Scientific acceptance thresholds stay frozen; local times and incomplete legacy counters remain separate. Keep iterating from actual observations and preserve the broader rho-parity objective. Any stopping decision must name tested coverage, actual limitations and the next lead; do not infer impossibility from a failed pilot.

---

# Resume: crypto @ claude/hky-progress-growth-law-20261115 (growth-law cells rerun)

## Goal
Owner (2026-11-15): "Yes go ahead with the engine fix and rerun"; budget "All three cells, 24h each, $80".
Decide d_F at the three open Conjecture G cells of the PKM/Kummer tower (prime_ic_leads_20261110/growth_law).

## Lead / hypothesis
Conjecture G: d_F(W,t) = ffd(W) + max(0, floor((t - s_W)/4)). Registered predictions:
(a) W=1,t=12: d_F=6; (b) W=3,t=13: d_F=7; (c) W=2,t=14: d_F=7.

## Spec
research/prime_ic_leads_20261110/growth_law/rerun_20261115/PLAN.md (frozen before data) and cells_job.sh.
Binary pkm_tower_pilot from commit in source_commit.txt, sha256 in binary.sha256 (checked on pod).

## State
Engine fix committed (fall_degrees_with + --hky-progress). Pod about to launch.

## Runs in flight
RunPod pod bdr-growthlaw (cpu5m, 16 vCPU), job /work/in/cells_job.sh, outputs /work/out.
Kit: research/runpod_kit_20261112/rp.sh (get/exec/delete). Cleanup: rp.sh delete bdr-growthlaw.

## Stop and success conditions
Pod deleted at ALL_DONE, on BINARY HASH MISMATCH, or before spend exceeds USD 80 (~72 h at USD 1.06/h).
Reading rules in PLAN.md (holds / fails / censored). Partial: decided degrees from out/prog_*.jsonl.

## Next steps
Poll at most every 4 h: rp.sh get out/ into rerun_20261115/out/. At ALL_DONE: copy back, delete pod,
close benchrec record, write REPORT + figure + PDF, update PR.
