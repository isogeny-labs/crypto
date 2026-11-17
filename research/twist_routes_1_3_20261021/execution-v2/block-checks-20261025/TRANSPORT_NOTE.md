# Child exits and the yielding command transport

Two yielded shell sessions returned transport exit 1 even though the actual
scientific child completed successfully: the initial Callgrind preflight
(session 16723) and the separate-worker instruction probe (session 88983).
These session identifiers refer to the interactive execution transport.

The first preflight's raw profiler output is retained; the separately named
v2 preflight captures its actual exit 0 explicitly. The instruction probe's
shell captured and printed `worker-cost-exit=0`, and its transport JSON records
the captured child exit. All 31 recorded outputs and raw instruction traces
were subsequently checked by the independently frozen Rust auditor. The
batch probe has its separately saved actual child exit in the run tree.

The transport status and child exit are different observations. A transport
anomaly alone is not acceptance of a scientific result. The hashes, matched
outputs, complete-basis certificates and independent audit determine which
results are accepted. No automatic approval rejection was observed.
