# Reference-shifted six-symbol necessary bound

Frozen before implementation or execution. The current actual closed rank is
six on the unchanged original 225 elliptic columns. [pcc:unverified]

Use the parent KM reference divisor E_ref consisting of original quartic
points 223, 224, 225. Verify its coordinates, actual map images and signed
column labels freshly; set rho=f_*(E_ref) and u=2 times its coefficient word.
A function with pole divisor bounded by 2E_ref has an effective degree-six
zero divisor after padding cancelled poles, and its image sum is 2rho.
This is a necessary image condition; it does not prove a word comes from
a function or a principal divisor. [pcc:unverified]

Exhaust the alphabet O and both signs of each original column. Enumerate
every unordered triple i<=j<=k, storing its sum minus rho. Evaluate each
triple separately with named OpenSSL and GMP arithmetic and compare exact
compressed points and ordered hashes. Store 32-byte packed records; sort
in place by canonical x, labels and orientation. Nonidentity opposite
orientations pair; identity records pair with all identity records,
including themselves. Every six-symbol multiset can be split into two
unordered triples. Subtract u and normalize row sign. Keep every distinct
row, including zero, with its lexicographically first witness. A cap breach
rejects completeness rather than truncating. [pcc:unverified]

Check a second join using separately indexed positive, negative and identity
streams and a sorted merge. Compare exact row/witness sets. Verify every
retained word's target sum, coefficient reconstruction and zero row sum in
both group implementations. Require recovery of the doubled-reference zero
word. If 2rho=O, reuse the completed zero-target six result instead of
repeating it. Record target comparisons with O, kappa and 2kappa; coincidence
with a four-symbol target does not duplicate a six-symbol search.

Positive control uses the public named generator G and 2G, shift rho=3G,
and coefficient reference 6G. Compare both joins with exhaustive unordered
six-word group evaluation on five symbols. Require both zero and nonzero
rows and both identity and opposite-orientation joins. Test altered row,
missing triple, missing candidate and changed orientation. Fixtures never
enter the actual matrix.

Bind the original six rows and 108x429 tangent matrix. Check all row sums
in both arithmetic engines and ranks 6 and 108/102/6. Add only independent
candidate rows, supported in the original 225 columns, to capped exact
bases. Compare original-column rank with full rank minus auxiliary rank.
Image independence alone is not success: the goal additionally requires
actual degree-six RR/KM realization and separate elliptic/elimination replay.

Limits: one allowed CPU; 1 GiB virtual address space; strict C++17 build
300 seconds; each worker 600 seconds; dense helpers at most 256 rows and
1024 columns; at most 4096 retained rows. Native C++ does all mathematical
work. Python may perform file, hash, JSON and report bookkeeping only.
Do not commit the roughly 493 MB triple table. Freeze source/input/binary
hashes, argv, exits, cost receipts and ordered stream hashes. Preserve failed
attempts. Run locally; Conductor is ignored per the explicit user instruction.

Sources checked: parent `km_pushforward_20261126/REPORT.md` and frozen
pushforward/verify/extract sources; `correspondence_audit_20261124/REPORT.md:71`
and native reference binding at `correspondence-source.cpp:308`; current
`SIX_PROTOCOL.md`, original `six-v1` source/result and `REPORT.md` references
to Diem and Gaudry–Thériault–Thomé. These motivate a necessary shifted trace
bound using an established reference convention, not a novelty claim.
KB tools are unavailable in this runtime; wrapper results are recorded in
the dated journal. [pcc:unverified]
