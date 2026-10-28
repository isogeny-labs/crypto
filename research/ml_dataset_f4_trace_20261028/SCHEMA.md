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

`claims` contains exactly these Boolean fields, all false in producer v1: `input_encoding_checked`, `matrix_rank_independently_verified`, `execution_stream_complete`, `total_operation_count_verified`, `unsat_verified`, `groebner_basis_complete`. It also contains `first_fall_degree:null` and `last_fall_degree:null`. Native rank is only an observation until a separate checker accepts the entire matrix trace.

Row versions are versions of **slots**. All start at0. A swap increments both slots by1; a self-swap increments that slot once, so both after-version fields are equal. A row XOR increments only its target slot. Source versions must be current. The checker reconstructs the actual words from matrix rows, validates swaps/XORs and rejects an omitted-word XOR unless the omitted pivot words are zero. `words_after` and all `output_row` values must match the independently reconstructed state. All padding bits beyond the declared columns are zero.

The checker independently enumerates generator-then-multiplier candidates: each generator fi has degree<=D, all subsets of q variables of degree<=D-degree(fi), generated in the native increasing-degree combination order. Product masks are OR and duplicate products cancel modulo2. Every candidate, including a zero product, must appear exactly once before `columns`. Nonzero products determine the complete support columns and matrix-row sequence. No omitted/high-degree original generator is allowed. An empty input yields no candidates, no columns or matrix rows and rank0.

For each column the suffix algorithm scans remaining rows from pivot_row upward, selects the first set bit or emits null, swaps it into pivot_row and clears that column from all other rows in ascending slot order. It stops columns early only when rank equals row count. `output_row` covers every original kept matrix slot, including zero rows. A successful `end` requires complete input/candidate/matrix/pivot/output phases. A `stopped` record or missing end is censored, regardless of producer rank-like fields elsewhere.

`reason` is a tagged object with `kind` and optional bounded `detail`: invalid_input, unsupported_profile, candidate_limit, event_limit, stream_byte_limit, event_byte_limit, workspace_limit, memory_limit, admission_limit, native_size_limit, deadline, cancelled, sink_failure or integrity. Invalid-input/unsupported errors can return before begin; a sink failure cannot safely append stopped to the failed sink and returns the last acknowledged prefix instead. All reasons preserve unknown/incomplete claims.

The independent checker must reject duplicate/unknown fields, wrong schema/profile/ring/order/bindings, noncanonical numbers/word strings, hash/sequence gaps, missing or duplicate inputs/candidates/columns, row versions, padding, input degree>D or q>8/D>4, excessive resource descriptions and false completion claims. Public S3/selector equivalence is outside this native-only schema; `input_encoding_checked` remains false even if the supplied matrix arithmetic is correct.
