#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.55em)
#let v = json("verification.json")
= CRIU + EBS recovery and a fresh L2 epoch
Iso Lab · 21 October 2026 · operator validation

*Observed:* a native SHA-256 process resumed from RAM and an encrypted EBS
snapshot on a replacement EKS Spot worker. Native replay verified its complete
output. A separately submitted fresh benchmark epoch passed the existing L2,
replay, A/A and S3 checks. *Automatic recovery of ordinary harness jobs remains
unimplemented.*

#image("flow.svg", width: 100%)

#table(columns: (42%, 58%), inset: 5pt,
  [Check], [Observed result],
  [Process checkpoint], [Step #v.checkpoint_sequence; CRIU 4.2.1; UID/GID 1000],
  [Replacement], [c8a.2xlarge; us-east-2a → us-east-2c],
  [Original writer fence], [EC2 terminated at 05:17:19.212 UTC; replacement pod created at 05:17:20 UTC],
  [Checkpoint consistency], [#v.checkpoint_file_hashes_verified file hashes; encrypted complete EBS snapshot],
  [Native process replay], [#v.replay.sequence final steps; #v.replay.steps recorded values; #v.replay.starts process start],
  [Fresh epoch], [#v.benchmark_epoch.l2_executions L2 executions; #v.benchmark_epoch.reproduced_replays reproduced measured replays],
  [A/A controls], [#v.benchmark_epoch.aa_pairs pairs admitted; recorded 95% ratio interval 0.989244–1.012734],
  [S3 verification], [#v.benchmark_epoch.artifact_hashes_verified payload hashes; complete manifest],
  [Fresh worker], [Eight logical CPUs; eight physical cores; no SMT],
)

The SHA-256 chain is a correctness fixture, not a speed benchmark. Its pauses
allow checkpointing. The fresh ecbench job uses the registered 20-bit fixture,
two targets, nine arms, five measured rounds and a warmup. No timing crosses the
restore boundary. The A/A interval is a recorded control result, not a speedup.
Canonical algorithm graphs and leaderboards are unchanged.

== Identity and evidence
The running source is retained as #raw("fixture-source-executed.rs"). Its hash
is #raw(v.executed_source_sha256). The running binary hash is
#raw(v.generation.fixture_sha256). The native verifier recomputes every recorded
hash from the public seed and checks the exact target and original output prefix.

Process outcome: #raw(v.replay.sha256).
Old instance: #raw(v.original.instance_id); replacement: #raw(v.restored.instance_id).
Fresh job: #raw(v.benchmark_epoch.receipt.job_name).
Fresh pod UID: #raw(v.benchmark_epoch.receipt.pod_uid).
Fresh source: #raw(v.benchmark_epoch.receipt.build.source_revision).

Full details, exact commands, receipt identities, source hashes and private S3
locations are in #raw("REPORT.md"), #raw("protocol.json"),
#raw("fresh-epoch-protocol.json") and #raw("verification.json"). Raw process
memory stays in encrypted private storage and is excluded from this report/Git.

== Failures and limits
The initial process start lacked #raw("setpriv") and its dump found no process.
The first archive lacked #raw("tar") and produced an empty S3 object, rejected
before generation commit. Both failures are retained. Corrected tool setup and
download verification produced the committed generation. A transient replacement
Spot quota/capacity error cleared without raising the 96-vCPU quota or 64-vCPU
lab cap.

Containerd CRI restore-via-create stayed disabled. The privileged operator
supervisor restored a trusted child; ordinary experiment pods retain Restricted
admission. This one test does not qualify arbitrary commands, perf/network/GPU
state, other instance families or unattended recovery. Its fresh epoch was an
operator submission and is not the pending PR 2342 main-branch Action proof.

The remaining controller/API and benchmark epoch integration is tracked in
#raw("problems/eks-resumable-l2-experiments.md"). Disposable pods, workers and
PVCs were removed; the encrypted snapshot and private S3 evidence are retained.
