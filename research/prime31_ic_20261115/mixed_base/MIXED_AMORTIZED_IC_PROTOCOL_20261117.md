# Prime31 index calculus: fully charged reusable mixed-base logarithms (frozen 2026-11-17)

Status at freeze: **proposal only; no new Rust compilation or measurements**. This is a complete *index-calculus-derived* algorithm with a reusable relation base. The mixed 4+4 search uses generic point-table meet-in-the-middle; it is **not** a novel nongeneric S9 factor extractor. No fastest-rho claim is authorized.

## Mathematical plan

Reuse the actual ordinary cofactor-one (p=2^{31}-1) curve and native Rust `PRIME31`, `Arithmetic`, `Base::build`, mixed signed D4 tables, original-curve replay, and modular RREF from merged #4503 / open #4506. Take 31 first-rational-x points A and 31 Dickson-depth-six points D; deduplicate their one common point, leaving 61 unknown basis logarithms.

**Once per curve** choose public `a` and find signed `4A+4D=aG` relations; solve 61 independent modular rows for the 61 basis logarithms, independently checking each `[log]G=P`. Charge both table construction, all failed and dependent relations, and full-base scalar verification to **setup**. Do not use point labels, private scalar shortcuts, planted witnesses, or caller-side target scalars as solver input.

**Per target** form a fresh public `R=aG+bQ` with nonzero `b` and find a verified signed 4+4 representation. From its actual modular signed row `Σc_i log_G(P_i) - b log_G(Q)=a`, recover `log_G(Q)=b^{-1}(Σc_i log_G(P_i)-a)`. Check `[log]G=Q` inside the timed solver, then check against the separately held caller-known scalar after the timed solve.

## Immutable parameters and performance accounting

- Actual `PRIME31` native Rust project pinned to merged commit `1df714645aa683cf975bc76fcacb9842cb4780fd` in source-specific CI (root main may be sparse).
- Public inputs: **1024 deterministic fresh known-answer Q**, generated exclusively by the caller, distinct scalars not promised; test for and reject repeats before including target in the denominator. Target seed `0x414d4f5254313032`. Setup relation seed `0x503331424153454c`. Per-target public descent seed derived from `0x44455343454e5431` and target index. Independent `a,b in [1,n-1]` from SplitMix64.
- Setup cap **256** relation attempts, target cap **8** residual attempts; never silently skip cap failures. Preserve full per-target failures, attempts, probes, signatures, row and verified scalar in Rust-emitted JSON. Each failed attempt and partial computation is charged to the relevant timer.
- Report `setup_ns`, `descent_ns` per target, `setup+Σdescent` for M=1,64,256,512,1024, and completed/attempted counts; file is producer `[pcc:unverified]` until independently validated.
- **Correctness gate:** full rank 61, 61/61 basis point replays, 1024/1024 unknown logarithms with public curve replay, 1024/1024 caller-known-answer checks, all signed witness rows verified on the original curve. A failure is a failure, never a dropped measurement.
- **Performance gate:** compare fully charged `setup+Σdescents` against a **same-runner, CPU-isolated, complete** strongest optimized rho and a mature published/non-generic index-calculus implementation on **the same 1024 Q**. Also report single-target cold cost and break-even M, not merely post-setup per-target latency. Existing 24-Q rho32 (85.208 ms total in #4486) is a historical **unmatched** reference, never a certified win or 2026-11-17 measurement.
- No paid EKS, leaderboard change or merge without additional authorization/gates.

## Prior work and risk register

- Primary papers: Petit--Kosters--Messeng (PKC 2016, DOI 10.1007/978-3-662-49387-8_1), Amadori--Pintore--Sala (FFA 2018, DOI 10.1016/j.ffa.2018.01.009). Multi-target amortization and generic baby-step/giant-step precomputation are not new.
- Repo #4503 provided generic mixed 4+4 source; #4506 measured 2/2 complete but ~2.12 s **cold per target**, with ~2.05 s table construction charged separately each time. #4505 established exact mixed formal support, not speed. #4454/#4486 supplied complete APS/rho32 engineering controls; #4468 WZ benchmark remains blocked by unrelated syntax in a pinned root checkout (latest workflow #43994897230: src/cryptanalysis/sat.rs).
- #4466 RR9 Hermite/norm is a **given-leaf certificate**, not nonenumerative extraction. Autoresearcher #3301/#3305 and `experiments/semaev-alternatives-20261105/SUMMARY.md` / `ERRATA.md`: Vieta-conic, norm-presentation and tower-F4 negative controls; do not rerun unchanged. SA-19 twisted-Edwards height lead needs rational 2-torsion and is inapplicable to the specific odd-prime-order cofactor-one prime31 curve by rational-point group order. No novelty claim from current method.
- Research KB and Conductor MCP are not available among this runtime's connector tools; use these exact GitHub files and publisher sources as bounded fallback and do not equate absence with no prior art.

**Stop/go:** run the existing Rust code only; next algebraic research rung remains nonenumerative public-R RR9/S9 factor polynomial extraction with original-ideal certificates, Dickson negative control and no group-table lookup. An amortized speed win, if established, is an index-calculus implementation/cost result, **not** a subgeneric algorithmic breakthrough.
