# Frozen full-quartic workload admission check

Both unmodified full-spec measurement objects retain their86400-second
allowance, one round, zero warmup, eight targets and all three arms. The
maintained local EKS request validator rejects the requested86400-second
request deadline with exit2; its ordinary contract caps this at3600seconds.
Its ecbench measurement path also requires at least five measured rounds and
warmup. No launcher, workflow dispatch or paid resource was invoked, and no
full panel was run. These are admission checks, not failed experiments.

The request envelopes intentionally contain no invented per-run image/binary
hash. They are validation probes only and must never be submitted as executable
requests; omission of build would otherwise select the deployed fixture. The
source-revision field records the historical caller/source label and does not
build that executable. Source image/build identity and a compatible original
long-job/control contract remain required before launch. A bounded pilot may
be separately registered, but cannot replace these requested full panels.

The same skill caps each pool at128GiB; a requested256GBclosure job cannot fit.
No implicit pool/quota change or smaller substitute is authorized by this
admission report. Standalone closure commands also need the maintained custom
command/image path, not this ecbench envelope. EKS MCP is absent in this tool
catalog, and the current GitHub connector exposes no workflow-dispatch method.
The configured CLI/browser fallback has not yet been used to dispatch anything.
The read-only `gh auth status` check reports that its configured GH_TOKEN is
invalid. The connected GitHub API remains usable for normal feature publication,
but exposes no workflow-dispatch method. No credential value was read/printed,
no identity/trust change was attempted, and no browser dispatch occurred.
The total extra-cloud budget remainsUSD100, no new paid worker was launched,
and the denied SSM/ephemeral GetParameter operation was not retried.

The separate maintained `resume/request.schema.json` admits an absolute original
deadline180..86400seconds in the future. It therefore removes the ordinary
one-hour deadline obstruction in principle, but its `spec` still explicitly
requires L2, warmup>=1, rounds>=5, a reference and matching A/A. That does not
admit the unmodified one-round/zero-warmup full spec. It also requires real
tested build/image/binary and recovery-runtime digests; none was fabricated.
The native resume client is an additional build/access dependency, not a custom
closure-command launcher. No resumable request or cloud submission was made.

Sources checked: isolab-eks/SKILL.md and its submission/recovery references,
AGENTS.md section9a, HARNESS.md ordinary and resumable contracts, ACCESS.md,
the ordinary and resume request schemas, and the actual main workflow read
from Git. The skill states: "Do not request a quota increase or raise pool caps
implicitly." These are implementation/resource/admission gaps, not a proof
that the research workloads cannot succeed on a compatible backend.
