# Frozen implementation admission, 2026-10-28

Parent approved the attached design and finite software validation on 2026-10-28.
This document supersedes only the design packet's pending-authorization wording.
Conductor T-763 owns the standalone consumer native directory and this new research
directory. Base is 88707e16b99f54976cfbcbe78c0f49379ab69da7. No producer arithmetic
imports, scientific jobs, extra cloud capacity, retries, seed searches or budget
increase are authorized. Parent owns delivery and the shared session log.

The cumulative checker ceiling per cell is 128,000,000 charged logical term/rank
units, 1,048,576 reserved affine add/doubling units, and 4,194,304 dense coefficient
cells. These are software admission counters, not timing, RSS or total cost.

Decoded-memory policy: cap each v2 artifact including newline at 33,554,432 bytes.
Before typed decoding, run the existing duplicate-key parser on bounded bytes and
account its decoded JSON recursively: each Value slot 64 bytes, each map entry
128 bytes plus key capacity, strings by capacity, and arrays by capacity*64 plus
nested payloads. Account retained raw bytes, the decoded transcript, one comparison
batch payload at a time, typed representation using a conservative extra copy of
the same JSON accounting, and native state/dense scratch separately. The tracked
admission total is at most 536,870,912 bytes; refuse before each large allocation.
The map/row scratch charge is 128 bytes per retained sparse term, plus 256 bytes
per retained row; source clones and pivot snapshots are counted as separate rows.
Dense scratch reserves 16 bytes per coefficient (matrix and comparison) and row
metadata before allocation. The checker drops each comparison payload before
loading another. Its JSON input-size and nesting limits remain in force.
This conservative decoded-storage model excludes allocator metadata and temporary
serializer/JSON parser implementation allocations and makes no whole-process RSS
claim. The finite validation supervisor separately enforces 1024 MiB per cell.

The 32 primary and eight optional supplement configs in frozen/ are fixed inputs.
Supplement execution is allowed only if primary semantic coverage is absent;
maximum 40 cells total, each 30 seconds, 1024 MiB and 128 MiB capture. Every error
and incomplete result is retained; no cell is replaced. Frozen supervisor/plan
wrappers may be added before any fixture is generated, without changing configs.
A mismatch with the production supervisor admission must be reported, not evaded.

Literature and implementation contract: the previously read Cavallar CWI
MAS-R0012 (2000), sections 2.1-2.2, explains singleton peeling/merging;
https://ir.cwi.nl/pub/4453/04453D.pdf. SEC1 v2 sections 2.2.1-2.2.2
https://www.secg.org/sec1-v2.pdf underlies the existing independent group arithmetic.
This is established finite algebraic certificate replay, not a new algorithm,
complexity result, search-completeness proof or measurement of attack cost.
