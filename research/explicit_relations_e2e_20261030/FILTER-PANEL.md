# V15: filter controls and 63-bit probe

The v15 plan was frozen before any affected data. V14 was prepared but never
executed; preserve its protocol and UNEXECUTED-v14.md. V13 continues untouched.

Run `run-filter.sh` on one full RTX PRO 6000 only after the previous pod's
archive and deletion are verified. Source must include the frozen v15 record,
the optional negative filter and the separate 63-bit arithmetic extension.
The launcher first runs the host/Rust suite, CUDA arithmetic/hash/witness
checks, all eight query variants under memcheck/racecheck/synccheck, service
checks and cap headers. No local root crypto/plugin build is permitted by the
session's disk constraint.

At52bits, seed101 runs b/u/x and seed202 runs x/u/b. Baseline b uses dense CUB,
persistent queries and the full exact right cache; candidate u adds only the
negative filter, and x repeats u. Every successful52-bit trial must report a
full right cache, positive filter storage for u/x, and no filter storage for b.
Certificates must agree byte-for-byte across arms. Both63-bit targets use u,
with the existing explicit cache fallback if memory requires it. Record that
fallback and charged filter bytes. Per63-bit solve cap7200s; worker18000s and
controller18300s. Same row/work caps,25%reserve and fixed public inputs.

The native analyzer and strict post-run checker preserve all attempted cells,
timeouts and limits. Only exact recovery is primary; L0 clocks cannot establish
certified speedups, the fastest external implementation, a matched-rho advantage
or an absolute GPU maximum. A filter's possible match is never a certificate:
exact lookup and fresh native scalar/relation replay remain required.

Preparation validation: native launcher/analyzer/checker compile with warnings
denied; shell syntax and protocol validation pass. GPU compilation, sanitizer
results and all new recovery outcomes remain pending.
