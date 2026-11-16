# Boolean F4 suffix trace wire schema v1

Stable first-slice handoff to the separate checker. Native code is being implemented; no emitted fixture or test pass exists yet. Schema/profile: `crypto.boolean-f4-suffix-trace/v1` / `boolean-macaulay-f4-suffix/v1`.

Each JSONL record has these fields in order: `schema`, `sequence` (1-based u64), `previous_sha256` (64 lowercase hex), `event`. Initial previous hash is 64 zeros. The record digest is SHA256 of its canonical UTF-8 JSON bytes **excluding** the terminating newline. The next record includes that digest. The sink receives those bytes plus one newline. Sequence/hash advance only after the sink returns success; a failed or partial write is outside the acknowledged prefix.

`event` is an internally tagged object with first field `kind`, then fields in the order listed below. All counts/IDs/masks are unsigned integer JSON literals; no signed, floating or string aliases. Boolean masks fit in a u8 and must use only the low q bits. Packed u64 words are strings of exactly 16 lowercase hexadecimal digits. Input terms are unique descending degree/ascending mask. Candidate term lists are ascending mask, matching the native product visitor. Column masks are unique descending degree/ascending mask. Empty terms denote zero; mask0 denotes the constant monomial.

| kind | Fields after kind, in order |
|---|---|
| `begin` | `profile`, `binding`, `n_vars`, `degree`, `input_count`, `input_sha256`, `limits`, `native_row_cap`, `native_column_cap`, `kernel`, `ring`, `order` |
| `input` | `index`, `terms` |
| `candidate` | `index`, `generator`, `multiplier`, `terms`, `matrix_row` (null for zero product, otherwise its contiguous 0-based kept row index) |
| `columns` | `monomials` |
| `matrix_row` | `row`, `version` (zero), `words` |
| `matrix_ready` | `rows`, `columns`, `candidates` |
| `pivot` | `column`, `pivot_row`, `selected_row` (null when no remaining row has that bit) |
| `swap` | `left`, `right`, `left_version`, `right_version`, `left_after_version`, `right_after_version` |
| `row_xor` | `target`, `pivot`, `target_version`, `pivot_version`, `after_version`, `start_word`, `words_after` |
| `output_row` | `row`, `version`, `words` |
| `end` | `rank_native`, `word_xors_native`, `rows`, `columns`, `candidates`, `admission_units`, `claims` |
| `stopped` | `reason`, `acknowledged_before_stop` |

`binding` has `source_commit` (40 lowercase hex), `producer_sha256`, `configuration_sha256`, `context_sha256`, `call_id` (all 64 lowercase hex). These are declared caller bindings, not proof of actual execution source. `input_sha256` is SHA256 of canonical JSON `[q,D,ordered_input_term_arrays]`; arrays have the same u8-mask values as `input` records. `ring` is `F2[v]/(vi^2+vi)`, `order` is `degrevlex:v0>v1>...`, and `kernel` is `native-suffix-rref`.

`limits` has `max_candidates`, `max_events`, `max_stream_bytes`, `max_event_bytes`, `max_workspace_bytes`, `max_memory_bytes`, `max_admission_units`. The absolute deadline and external cancellation callback are runtime controls and cannot be reconstructed as arithmetic proof. Hard ceilings are candidates4096, events32768, stream4194304 bytes, event65536 bytes, workspace2097152 bytes, accounted component33554432 bytes and admission_units16777216. Lower limits may censor. One event and1024 stream bytes are reserved for a terminal stop record; completion also consumes the declared totals. Admission units are a named conservative work-admission model, not total native operations or instructions.

Admission minima are max_events>=2, max_stream_bytes>=2048 and max_event_bytes>=1024. Other limit fields may be zero within their hard ceilings, subject to the component-memory admission inequality in ADMISSION.md; such settings can censor before begin. Values outside the admitted ranges fail before native work.

`claims` contains exactly these Boolean fields, all false in producer v1: `input_encoding_checked`, `matrix_rank_independently_verified`, `execution_stream_complete`, `total_operation_count_verified`, `unsat_verified`, `groebner_basis_complete`. It also contains `first_fall_degree:null` and `last_fall_degree:null`. Native rank is only an observation until a separate checker accepts the entire matrix trace.

Row versions are versions of **slots**. All start at0. Every selected pivot emits `swap`, including a self-swap when `selected_row == pivot_row`. A swap increments both slots by1; a self-swap increments that slot once, so both after-version fields are equal. A row XOR increments only its target slot. Source versions must be current. Column c is bit c%64 of word floor(c/64): column0 is the low bit of word0. The checker reconstructs the actual words from matrix rows, validates swaps/XORs and rejects an omitted-word XOR unless the omitted pivot words are zero. `words_after` and all `output_row` values must match the independently reconstructed state. All padding bits beyond the declared columns are zero. Each actual row XOR at column c adds W-floor(c/64) to `word_xors_native`, including suffix words with zero operands or results; W=ceil(columns/64). Candidate generation, lookup, serialization and hashing do not contribute to that narrowly named native counter.

The checker independently enumerates generator-then-multiplier candidates: each generator fi has degree<=D, all subsets of q variables of degree<=D-degree(fi), generated in the native increasing-degree combination order. Product masks are OR and duplicate products cancel modulo2. Every candidate, including a zero product, must appear exactly once before `columns`. Nonzero products determine the complete support columns and matrix-row sequence. No omitted/high-degree original generator is allowed. An empty input yields no candidates, no columns or matrix rows and rank0.

For each column the suffix algorithm scans remaining rows from pivot_row upward, selects the first set bit or emits null, swaps it into pivot_row and clears that column from all other rows in ascending slot order. It stops columns early only when rank equals row count. `output_row` covers every original kept matrix slot, including zero rows. A successful `end` requires complete input/candidate/matrix/pivot/output phases. A `stopped` record or missing end is censored, regardless of producer rank-like fields elsewhere.

`reason` is a tagged object. The kinds invalid_input, unsupported_profile, sink_failure and integrity require exactly `kind` then `detail`, a string of at most256 UTF-8 bytes. All other kinds require exactly `kind` and forbid detail: candidate_limit, event_limit, stream_byte_limit, event_byte_limit, workspace_limit, memory_limit, admission_limit, native_size_limit, deadline and cancelled. Sink-failure detail is truncated to64 Unicode scalar values before serialization; the other detailed reasons use bounded static producer messages. Invalid-input/unsupported errors can return before begin; a sink failure cannot safely append stopped to the failed sink and returns the last acknowledged prefix instead. All reasons preserve unknown/incomplete claims.

`native_row_cap` and `native_column_cap` are captured once from the native environment-resolved limits before begin (defaults20000 and40000). They are unsigned native-size observations, represented as canonical integers in 0..u64::MAX on the supported64-bit target, and never authorize allocations. Completion requires R<=native_row_cap and C<=native_column_cap in addition to the independent trace bounds R<=4096,C<=163. A zero cap admits only a zero size in the corresponding dimension.

The native legacy row-cap check occurs after emitting/keeping the candidate that raises the nonzero-row count to native_row_cap+1. That single bounded candidate is therefore valid in a censored prefix; no later nonterminal event is allowed, and completion is forbidden. In particular, cap0 with a constant input emits its candidate with matrix_row0 and then stops. Independent trace candidate/workspace/event caps still take precedence if they refuse earlier.

The independent checker must reject duplicate/unknown fields, wrong schema/profile/ring/order/bindings, noncanonical numbers/word strings, hash/sequence gaps, missing or duplicate inputs/candidates/columns, row versions, padding, input degree>D or q>8/D>4, excessive resource descriptions and false completion claims. Public S3/selector equivalence is outside this native-only schema; `input_encoding_checked` remains false even if the supplied matrix arithmetic is correct.

A final non-newline-terminated byte tail is outside the acknowledged JSONL prefix and may only be treated as censored. A malformed newline-terminated record fails verification. The exact named admission model is frozen in `ADMISSION.md`; it is separate from the native word-XOR counter and never certifies total execution cost.
