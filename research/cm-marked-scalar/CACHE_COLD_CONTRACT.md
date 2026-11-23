# Prefix reuse and full cold evaluation, version 2

2026-11-08, T-1888, PR #3674. This amendment precedes all affected timing data. It retains P-192 as primary and includes P-224/P-256/P-384/P-521, rather than silently extending the earlier two-curve timing draft. The earlier frozen counted producers, reports and raw archives stay unchanged.

## Completed implementation

cache_core.hpp computes the original fixed mu action using mu0=m*k+c. It retains positive integer-labelled multiples of P from a fixed prefix chain and the Q-odd table. In a shared digit column, it replaces [d]Q+[e]P by an already retained point only if the integer coefficient m*d+e is present; a zero coefficient cancels the column. It never infers a point action from a curve-identity collision. Prefix chains are explicit dependency programs for radices 7,11,13,19 and powers of two through32. No optimal-chain claim is made. The relation radix remains the saved certified ell.

The point qualifier checks each retained point against independent OpenSSL scalar multiplication and each schedule against independent GMP digit/count logic. The GMP checker consumes the declared chain program as common data but does not call the producer's point-bank operations or BIGNUM recoder. cache-build-v1 passes6300 boundary point/schedule cases and71570 retained-point checks, plus five cache-hit positive and five wrong-residual controls. Full cold v1 passes575 untimed cases; final v2 adds binary A/A pairs and passes625, with five corrupted expected-output controls. No comparative count search or clock is collected in these qualifications.

Logical table entries and retained bank objects are distinct counters. Neither counts all temporary allocations or underlying field work. Odd-table normalization executes inside arithmetic; prefix intermediates reused outside that table may remain projective. All of that work is inside the cold timer when timing is authorized. Unknown internal counters for OpenSSL reference APIs remain null.

## Primary first-use algorithm

The primary candidate is cache-default. Its rule sets m=ell, offset=0 and window4 for subgroup sizes at most200bits, window5 through300bits, otherwise window6. It performs the centered quotient/remainder preparation inside each call. It has no installed-plan lookup, per-point calibration, count-census search, known input logarithm or cached point result. Runtime m,mu are supplied public algorithm parameters, as are the curve and scalar for the reference. In the fused deployment no map-constant generation is required: the supplied closure proof justifies [ell], and its explicit point chain is executed every time.

The secondary cache-installed and binary-installed modes load/parse the existing count-selected plan artifact inside their calls. Their historical search/derivation remains in the preceding experiment record. That old search cost is not measured in this new per-call interval, so an installed-plan result alone cannot support a full from-scratch first-use claim. The primary deterministic method avoids that dependency rather than excluding required runtime work.

## Cold boundary and matched outputs

cache_eval.cpp accepts a serialized request containing the common curve/scalar parameters and affine input point. The primary interval begins before parsing it. Five adjacent, exclusive phases cover:

1. Request deserialization, fresh curve/field/group construction, input decoding and curve check.
2. Deterministic quotient/remainder/chain preparation, or installed-plan I/O/parsing where applicable.
3. Prefix, tables/normalization, shared scalar arithmetic and point-bank destruction.
4. Symmetric output curve check, compressed encoding and equality to the independent expected point.
5. Remaining point/group/JSON destruction and bookkeeping before the function returns its encoded output.

The phases must sum exactly to total_ns. No curve or point table is reused across calls. The backend shared library is loaded at process start; process startup, compilation, protocol admission and known-answer workload generation are excluded symmetrically and identified separately. These exclusions are not hidden plan/point preparation. The scalar stays the original mu for every arm. The expected answer is available only to the common verifier, not to the point-action function.

## Proposed timing matrix and decision

There are25 configurations: window widths2..7 and their A/A twins; selected direct and twin; OpenSSL public multiscalar and twin; OpenSSL single API and twin; binary-default/binary-installed and their twins; cache-default, cache-disabled and cache-installed. cache-disabled uses the same prefix bank and deterministic parameters but disables merged-column reuse. The OpenSSL single API is typed separately; it is not the only regular reference.

The input manifest has four calibration and eight held-out nonzero points per curve. SHA256 expansion supplies at least twice the subgroup bit length before reduction, with a panel/curve/index/block domain; no 256-bit-only sampler is used for P-384/P-521. The candidate receives only coordinates and public parameters. One warmup plus six paired rounds, with fixed seeded shuffling, gives3500 calibration and7000 held-out rows. Every row must match its frozen expected point and full phase sum; duplicates, missing rows and failures invalidate the panel.

The planned native analysis reports the calibrated reference choice, but its decisive gate is stronger: at each held-out point, cache-default must beat the fastest median of every reproduced regular/binary reference. For each curve the minimum such speedup must exceed max(1.01,the maximum bidirectional paired A/A median ratio across those references and points). Report every curve independently, including ties/losses; no candidate or point selection from held-out results. Installed-plan or arithmetic-only gains cannot replace the primary result. No population significance or universal speed claim follows from this finite panel.

## Admission, freeze and next required work

The native receipt/analysis must bind the exact worker, source header, analysis, isolation binary, nine input files, environment switches, CPU/SMT reservation and both raw hashes. Accept only an admitted, uncontended receipt with exit0; verify the legacy preflight thresholds (at most0.2 other CPU seconds over two seconds, CPU/memory PSI at most5), measured contention at most0.1*wall, and post-run PSI at most5. The inherited reservation FD and single-CPU affinity are checked before the worker can read any benchmark clock.

The protocol is deliberately still a draft. Complete and qualify the native row/receipt analyzer, bind all sources/binaries/inputs, validate/freeze/publish the record, and only then attempt isolated timing. Per panel: one timed worker attempt, no unchanged retry,120-second deadline, one process/thread and2GiB address-space ceiling. No new cloud job or spending authorization is created. The standing EKS budget/quota constraints and native isolation policy stay unchanged. Do not restart Diem or the quiesced old followup.

Sources checked (KB): no loaded research-kb tools; migrated wrapper status fails127. Prior CONSTRUCTION.md, BENCHMARK_CONTRACT.md and residual-v1 records were read. Primary sources: Grabner–Heuberger–Prodinger, https://www.helmut-prodinger.at/pdffiles/joint.pdf, introduction/section2; Bernstein–Lange, https://www.hyperelliptic.org/EFD/precomp.pdf, introduction and precomputation/normalization discussion. These establish standard simultaneous-scalar/precomputation background, not this implementation's speed. No novelty or optimality claim. No new curve-fact query is inferred from an unavailable ecdb tool.
