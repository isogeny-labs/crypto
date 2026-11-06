# Frozen diagnostic: native timeout formatting

Question: can the pinned YAFU timeout-print statement itself explain an
exit-139 failure? Preserve the original R05 run and its native executable.
Inspect pinned source c7d90283972820f99b3b4920bfc1eff05c1f6342 and retain its
file hashes. Its nfs_obj.timeout is uint32_t. The sieving timeout message
uses `%1.4sec`, which parses as a string conversion `%1.4s` followed by `ec`.
The other NFS timeout message passes the same integer to `%1.4f`.

One local diagnostic, no factorization: compile a C extraction of each
statement with format warnings promoted to errors; compile the original
sieving statement without that promotion and execute it once under a
five-second limit with core dumps disabled; compile and execute corrected
integer-format versions. Retain every exit code, command and output.
Stop after these controls, not after a new sieve allocation. A compiler
diagnostic or extracted-statement crash establishes the formatting defect;
it does not identify the original production instruction pointer without
a production backtrace. A patch remains unvalidated in the full NFS
pipeline until a separately bounded genuine native control passes.
