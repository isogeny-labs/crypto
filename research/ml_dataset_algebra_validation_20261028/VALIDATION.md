# Independent Boolean matrix trace validation

Final checker source `2ca93f74a00a5ff133aeaf5cab772f616a110791` passed ten
release unit tests (zero failures, one explicitly ignored native-fixture
integration helper), strict release all-target Clippy and the separately invoked
native-fixture integration helper. The CLI was compiled from that same source.

| Executable | SHA-256 |
| --- | --- |
| Independent checker CLI | `3e294a77fcef48ad39462a90c67a7da5bc55996fd70282363823a70183603f3d` |
| Independent checker test executable | `0adf3cc01b50615af9279b1e13c18cf443dc2699ba4cf71ee8fd76790ee8582e` |
| Native producer test executable | `e368482f8f8d56070d19e50c51aad1948d3238d1d579776f6c54e7f1243d4501` |

Actual native fixtures came from source
`29973950752cd0e2389a07608eee63781841afc2`, using externally supplied binding
identifiers and the executing producer binary's hash. These identifiers are
equality bindings, not execution attestation. The native producer's complete
library gate is separately owned and is not implied by this interop result.

## Executed cross-implementation cases

| Actual native fixture | Records | Independently reconstructed rank |
| --- | ---: | ---: |
| Empty | 4 | 0 |
| Constant | 10 | 1 |
| Boolean quotient collision | 11 | 1 |
| Row swap | 22 | 2 |
| Multiword suffix boundary | 1288 | 92 |
| Native row cap zero, stopped | 4 | null |

The multiword case contains 296 rows, 92 columns and 68 actual XOR events with
a nonzero start-word offset. The checker reconstructed the Boolean products,
columns, packed rows, complete elimination schedule and scalar Boolean rank.
It checked the native word-XOR total and the named admission formula, which
are explicitly narrower than complete execution or total machine cost.

Four tampered multiword streams changed an omitted word, a suffix word, the
start-word offset and a pivot version. Every stream had its hash chain
recomputed; all four were rejected by both the test harness and CLI. Censored
prefixes before/after the selected XOR retained no final rank. The capped native
case retained its final candidate and explicit stop, also without final rank.
CLI review JSON was byte-equal to the test-harness review for all six fixtures.
Terminal integration time was 2026-10-28 17:39:38 UTC.

The row-cap boundary defect and its correction are described in
`ROW-CAP-CORRECTION.md`. The original source review missed that compatibility
edge. Its unchanged historical review, the subsequent finding and initial
warning/lint logs remain retained. The corrected code is supported by the new
focused regression and actual cap-zero native fixture; the historical review
is not relabeled as a review of the correction.

## Scope and reuse of unchanged gates

This standalone Rust crate imports no producer arithmetic or event types. It
operates only within the frozen small Boolean matrix profile and explicit
memory/work limits in `DESIGN.md`. It rejects unsupported or malformed complete
records; an unfinished stream can retain a validated arithmetic prefix.

Root runtime/build/tests are unchanged from base
`c14d4552e513f6e28bda0e0e045f5e64beb492db`. Its inherited full-library gate was
5183 passed, zero failed, 123 ignored; it was not rerun or represented as a new
checker test. The standalone crate's own tests and lint are the new executable
gates. Current-head hosted checks remain a separate observation.

The checker does not verify the original PDP encoding, source execution,
durability, a native stop reason's actual cause, whole-solver instruction cost,
UNSAT, a complete Gröbner basis, or first/last fall degree. Scientific corpus
counts remain unchanged. Existing cloud jobs, original deadlines and the
USD200 total maximum are preserved.

The initial `local-validation-archive.json` is historical evidence for the
earlier nine-test checker. Final corrected source, binaries, actual native
fixtures, test logs and independent replay reports require the separate final
archive receipt; neither archive rewrites the other's status.
