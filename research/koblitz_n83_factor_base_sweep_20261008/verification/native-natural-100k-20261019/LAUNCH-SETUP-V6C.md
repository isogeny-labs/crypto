# Setup-only correction after two refused launches

Original v6 shell redirection failed because exec/ did not exist; neither
card runner nor solver executed and no receipt/raw stdout was produced.
Director retained the actual mechanical failure report and absent-output
assessment. v6b created only the parent and captured an actual native
stopped receipt: command1 refused with `checkout not clean` before creating
any worker directory, because new exec JSON streams were untracked. The
initial ignore check used a .log path and did not establish that JSON relay
and receipt files were ignored. No solver ran in either attempt.

Add a local tracked .gitignore for the entire new exec/ subtree, and verify
JSON relay/receipt paths are ignored. Do not change the original protocol.
All conflict, wall, input, worker, runner, resource, readiness and replay
limits stay unchanged. Commit this setup correction and freeze clean HEAD.
Compile new separate K2048 and K4096 v6c cards/specs against that HEAD, each
with a fresh relay/receipt/worker-output id. Assess K2048 before dispatching
the independent K4096 card. Preserve all old plans, failure streams and
assessments; no timed solve is retried and no failed output is overwritten.
Only metadata/setup changes; the previously tested native code is unchanged.
