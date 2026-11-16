# Execution log

The first two ephemeral attempts, `20261019-011623-boolean-native-compariso-9fc034`
and `20261019-012707-boolean-native-optimizat-abf14c`, terminated during source
preparation. The working-tree patch included existing PR changes outside the
sparse checkout, so `git apply` failed before compilation or timing. Both
instances terminated automatically. Their job/status/log receipts remain in
`preparation-failures/`. Neither supplies native measurements. Corrected runs
use a complete source checkout. No isolation or scientific gate is changed.

Build/test receipts and accepted/rejected native samples are added only after
they actually finish. The frozen baseline files are copied verbatim from
`1340a6176`; the new study does not modify the prior instruction archive.

The first auxiliary local profile batch stopped at `generalized-native`:
the binary copied from the release example directory was an older driver
without that arm (exit101). Its earlier matrix records and the failed API
profile are preserved under `profiles/`; they are not pooled. The corrected
batch uses the previous study's exact final executable,
SHA256`199eb9cf47d10245e105bf9f6bed431b3ec3afc106ba7d891619c317876b3d0e`,
and separate `profiles-corrected/` records. Native cloud comparisons rebuild
the frozen baseline from source and are unaffected by this local artifact
selection error.

The three complete-checkout native jobs finished with status `DONE`, exit0:
initial `20261019-014036-boolean-native-baseline-8d3eb7`, vector/Gray
`20261019-013858-boolean-native-vector-gr-799a77`, and Semaev/final
`20261019-014050-boolean-native-batch-fin-43e52b`. They were fetched together
at the scheduled approximately50-minute check-in. Native phase receipts say
complete; each admitted cost has five samples. Initial native admission has
26 rejected attempts (including expected n19 refusals); phase3 has21. See
the individual JSON summaries for exact attempt names and reasons.

The first native candidate's extra SIMD table dispatch was neutral. Its
wide Gray ablation lost against scalar at16 Boolean variables and won at22.
The second optimization therefore freezes a smaller-size scalar policy and
a typed Boolean input path in `POLICY_PROTOCOL.md`. Fresh native confirmation
launched as `20261019-024352-boolean-native-policy-ty-fb16d2`, on-demand
c7i.2xlarge, TTL90m. Its source is the3029ec46 checkout plus the retained
patch, later committed as2f679c1ed. Results are admitted only after its phase
receipt and all requested paired controls complete.

After that launch, main was merged as c8eb6b098. Copilot was first requested
on the PR to resolve the one benchmark-index conflict; with no reply after
over20 minutes, the documented fallback retained every row from both sides.
No frozen evidence conflicted. Main's sparse checkpoint changes are retained;
the timed builds keep their original source identities. A serial post-merge
full library validation runs separately and does not share the remote timed
host. An initial shell-detached local launch was reaped before starting Cargo;
the persistent exec-session launch produces the actual validation receipt.

The next guarded candidate removes Rayon parallel-iterator plumbing only
when the current pool has one worker. `SERIAL_CLEAR_PROTOCOL.md` freezes
its same-binary matrix experiment and four-worker negative controls before
timing. Source commit baa12d1fc plus the retained topology-discovery script
patch launched as `20261019-030905-boolean-native-serial-cl-25ad47`, on-demand
c7i.2xlarge, TTL90m. Its source includes the merged main and prior local
report/implementation changes as a retained patch on3029ec46. No new
complete-root timing is inferred from this matrix-only experiment.

The cutoff/typed confirmation completed at03:26:10 UTC, exit0/DONE, and the
instance terminated automatically. It was fetched at the scheduled~50-minute
check. All72 final costs have five admitted samples; the policy and final-cost
analyzers each retain13 rejected attempts (the two summaries cover distinct namespaces and keep their rejections
separate). Every paired original output agrees. All16 typed confirmation
cells qualify faster than the frozen iteration1 wrapper. n17 complete JSON
roots qualify faster on all four cases; n23 JSON complete-root controls are
neutral. The n23 planted JSON decomposition control qualifies slower at0.9654
before/after (about3.5% regression); retain this result, do not call the entire
JSON refactor regression-free. The typed route qualifies6.0022x against that
frozen decomposition wrapper on the same fixture.

Fresh final n23 planted cold costs (ms): F426.369084, F526.272740,
F67.353925, generalized typed1.478109, batched Semaev46.398592. Generator:
F4370.494241, F5375.110200, F618.606717, typed1.493081,
Semaev45.641908. The typed path's fresh native ANF/lifting cost is close to
its prepared-input root cost. Together with the prepared-JSON/typed ablation,
this identifies conversion/normalization as the dominant avoidable wrapper
cost; the remaining native search kernel is the next generalized bottleneck.
No medians or ratios from other hosts are combined with these costs.

The first post-merge full-library exec session ended with signal143 before
completion; the cause was not established. Its partial log/exit receipt are
retained. The dependent waiting shell was terminated before starting another
suite, and the latest candidate was restarted directly in a persistent terminal
with all four optimization switches enabled and one test thread. Its outcome
is recorded only after completion.

Latest-candidate full serial run completed in1369.06s:4397passed,10failed,
118ignored, exit101. Nine failures are local sandbox-denied network binds;
one unchanged mq_fes wall-threshold test had equal outputs but failed its
single-shot timing ratio under load. The pollard_collab suite rerun with
network access passes38/38, and the exact mq_fes wall test passes serially
(equal outputs; its wall ratio is a test diagnostic, not a speed claim).
All4407nonignored tests are covered by passing executions. The failed full
invocation remains failed; no wholly green full-suite run is inferred.

The initial serial-clear run completed03:28:12UTC, exit0/DONE, and was fetched
at the scheduled~50-minute check. All12matrix cells have five eligible pairs;
8rejected attempts remain. All8one-worker cells qualify faster: primary n23
F41.0770 and F51.0652. The four-worker n17F4control qualifies slower at0.8436
CI[0.8284,0.8504]; the other three four-worker cells are neutral. This prevents
a regression-free claim for the unrestricted registry-query guard.

A narrower explicit-one-worker filter is frozen in SERIAL_GUARD_PROTOCOL.md
before the next native run. The registry-startup explanation is a hypothesis,
not a proven diagnosis. Source7a762d8df launched as
20261019-040338-boolean-native-serial-gu-6ec5e9, same c7i.2xlarge on-demand,
TTL90m, full checkout plus retained patch on3029ec46. The fresh run repeats
all12cells and adds one-worker direct legacy ablations; it never pools old
samples. A new full library run uses network access from the start, with four
kernel switch/worker correctness configurations and SDK/Clippy/example checks.

The refined guard validation completes with4407passed,0failed,118ignored,
exit0, in1075.15seconds. Network access is enabled from the start, one Rayon
worker/test thread, all four optimization switches1. All four kernel
worker/switch configurations pass10/10; standalone13tests, strict Clippy and
example build/tests pass. Earlier failures and signal143 remain preserved.

The explicit-one-worker confirmation completed04:23:21UTC, exit0/DONE,
and the instance terminated automatically. The scheduled final fetch finds
all12cells with five eligible pairs, including all eight direct legacy
ablations. Eleven rejected attempts remain. All eight one-worker incremental
cells qualify faster; all four four-worker controls are neutral. Primary n23
incremental ratios are1.080205 F4 and1.072972 F5. Direct same-host legacy to
candidate ratios are1.114066 F4, CI[1.097994,1.129672], and1.120856 F5,
CI[1.060791,1.140189]. These are paired matrix-call results. The eighteen-arm
solver table was measured earlier with serial clearing off and is not
repriced. The initial failed control remains part of the evidence.

Named-store publication of the final compressed raw tree was attempted via
`ecbench store push`, but the configured store bucket rejects HeadObject
with HTTP403. The complete deterministic archive is therefore retained in
Git with its SHA256; no IAM, bucket or account policy was changed.
