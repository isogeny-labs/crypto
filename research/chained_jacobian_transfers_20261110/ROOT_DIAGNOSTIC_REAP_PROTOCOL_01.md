# Original diagnostic-reap failure isolation

2026-11-13,prospective before this exact-test replay. The unfiltered original
root retry03 terminated124 at1800s;two visible failure labels and unknown
suite totals are preserved in62aefb/9b8180. This follow-up does not replace
that required whole-suite result. [pcc:unverified]

Sources checked: unchanged src/cryptanalysis/ecbench/runner.rs lines2659–2845
(both exact tests,Root/tracking/journal,DiagnosticSessionClock::begin),
ROOT_LIBRARY_REPLAY_PROTOCOL_03.md and the whole original timeout log;
unchanged .claude/hooks/checkpoint-hook.sh ordinary-unit exception. KB
unavailable: no MCP tools,repository wrapper status and two exact/negative
queries report no executable;no novelty or absence inference. Conductor
unavailable127;scope is additive protocol/custody/journal/PCC only.
[pcc:unverified]

Question: where do the two existing diagnostic-reap tests panic on this
container? Competing hypotheses are an unsupported clock/environment read
and a resource-publication invariant failure. The first original panic and
backtrace identify the next discriminating source read;either result remains
bounded evidence for this binary/environment,not a full-suite diagnosis.
[pcc:unverified]

Invoke the SAME original binary with --exact and --nocapture separately for
ok_without_observed_reap_cannot_mark_or_read_sidecars and
publication_collision_preserves_previous_bytes_and_pending_failure.
Each invocation has120s timeout/one test thread/RUST_BACKTRACE=1,retains its
whole output and literal exit. No recompilation,test/source/ignore/namespace
or privilege change. Freeze all2137 original root inputs including binary
and authentic fixture before and after;hold07 also stays active. Outcomes
0/101/124 and all failure messages remain distinct. No expected pass is
assumed;only unchanged inputs and faithfully captured outcomes are required
for a diagnostic record. [pcc:unverified]

Ordinary unit diagnostics use the original documented nonexperiment opt-out:
append precise reason with original PCC and attempt original checkpoint push
before launch. Performance admission remains closed. Raw output lives at
/workspace/scratch/ct69-original-root-reap-diagnostics-01;the thin shell
launcher executes only the unchanged binary and hashes custody files.
After terminal results,read the actual failing source and choose a focused
remedy or independent environment route;never repeat an unchanged global
suite or weaken its requirements. [pcc:unverified]
