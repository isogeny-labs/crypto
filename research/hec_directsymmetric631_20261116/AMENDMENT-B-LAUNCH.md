# Case-insensitive output collision before solver spawn

B full-algebra control passes. First B collector stops before Command::spawn: outer SOLVE-B.stderr and child solve-B.stderr alias on this case-insensitive filesystem. Empty child stdout was created, then create_new(stderr) failed. No B solver process was launched, no result exists. Preserve outer status101and all files. Change child prefix only from solve-B to job-B; use outer RUN-SOLVER-B logs. Rebuild/hash/commit/checkpoint/gate before one new launcher invocation. No solver input/options/verifier/acceptance changes.
