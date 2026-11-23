# Development evidence

The native receipt is from Debian g++ 14.2.0, C++20, on the local Linux x86_64 workspace. An optimized build and an independent invocation of a build with undefined-behavior instrumentation and `-DNDEBUG` produced identical stdout. The digest file was recorded after the runs.

The JSONL files are projections of the original standalone development runs and omit wall time fields. The temporary host was shared and no timing qualifies for the leaderboard. Solver msolve 0.7.5 ran single-threaded, with a three-second cap per request. The panel used the first four seeded targets at selector size 16 and three at selector size 8, with suffix length two steps. The mixed-target seed is 781815 and its request budget is 30 draws. The original logs and Python harness remain in the Codex workspace. The native differential and group checks are independently reproducible from the C++ source in this study.

The full Rust suite and Lean checks were unavailable in this workspace. The C++ verifier and the historical solver diagnostics are the only executed checks represented here.
