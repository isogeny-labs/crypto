# Admission accounting for Boolean F4 suffix trace v1

This freezes the producer's finite admission model, not a count of all instructions, allocations, wall-clock work or native bit operations. Source/binary/configuration bindings are caller-declared metadata. Arithmetic correctness and complete schedule replay belong to the separate checker.

Let ti be the canonical term count of generator i, R the number of nonzero candidate products kept as matrix rows, C the complete support-column count, and W=ceil(C/64). The completed call's `admission_units` is the sum of:

| Phase | Units charged before the associated work |
|---|---|
| Generator degree scan | ti once for every generator, including zero generators |
| Candidate product | 1+2ti+ti² for each multiplier candidate, including zero products |
| Column collection | When R>1, the sum of normalized term counts of all R rows; zero when R<=1 |
| Packing | Normalized term count plus W for each nonzero row |
| Pivot search | One for each remaining row actually tested, through the first set bit or all remaining rows if no pivot exists |
| Swap | One per selected pivot, including self-swaps |
| Clearing tests | One per nonpivot row tested for each selected pivot; R-1 per selected pivot in this full-RREF profile |
| Row XOR | W-floor(c/64) for each actual suffix XOR at column c |
| Output | W for every output slot, including zero rows |

All other cancellation polls have zero units. Input validation, multiplier scheduling, sorting, input serialization, event encoding/hashing, callback work and allocator work are bounded separately or excluded from this named model; the ti² allowance conservatively admits candidate work without claiming an exact native sort count. No total-operation claim may be inferred.

Candidate units are checked before forming the candidate. Search/test/XOR units are checked before that operation. A failed sink can leave actual work after the last acknowledged record; partial prefixes therefore do not establish an exact total even for this model. Deadline/cancellation stops are producer observations. A terminal end is permitted only after the full output schedule and a final cancellation/deadline check.

Independent cumulative controls cap candidates4096, events32768, complete stream4194304 bytes, each complete record65536 bytes, workspace2097152 bytes, accounted components33554432 bytes and admission units16777216. Reduced caller limits are allowed within the profile; one event and1024 stream bytes are reserved for a terminal record. Sink failures are sticky and never retried. A failed write is outside the acknowledged prefix, even if the external sink physically retained some or all bytes.

Workspace admission reserves128KiB before beginning for bounded canonical input encoding, multiplier schedules, <=163 support columns, indexes, sorting and metadata. Before keeping each nonzero candidate with s terms it reserves s*sizeof(Mask)+4*sizeof(Vec<u64>)+64 bytes, covering source terms, outer-vector growth, packed words and slot versions under q<=8,D<=4. Before allocating the reduced-polynomial result vector it reserves rank*sizeof(F2BoolPoly); before each row readback it separately reserves popcount(row)*sizeof(F2BoolMono). This separate readback admission allows reduced rows to be denser than their source rows. The component admission check is 2*max_stream_bytes + max_workspace_bytes + 2*max_event_bytes + 1MiB <= max_memory_bytes. This accounts for bounded components and a capped external sink allowance; it is not a process-RSS or allocator-overhead measurement. The producer does not buffer the stream; the caller must independently cap its storage/sink and must make each callback bounded.

Zero polynomials have degree0 and participate in the complete multiplier schedule. Empty input at q=0,D=0 produces zero candidates/rows/columns/rank and zero admission units. A constant input at q=0,D=0 produces one candidate and one support column. Every original term must have degree<=D and only q low mask bits; original polynomials are never truncated or silently omitted.
