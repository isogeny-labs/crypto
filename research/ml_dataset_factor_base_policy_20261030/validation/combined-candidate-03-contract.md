# Candidate03 dependency closure correction

Freeze before assembly: preserve candidate02 and all failed evidence. Add only
src/cryptanalysis/prime_root_bounded.rs from the already pinned donor commit
0fd2d838db414f45c227bfb6aa7f4f796022aba2, Git blob
1ae331c0422069f7ed3ff17276539df4be5109c6. The donor root-trace module imports
this sibling through an explicit path. Check all explicit path attributes in
overlay Rust files against overlay plus base snapshot. Do not import donor HEAD
changes or rewrite mathematical behavior. Run dataset then fixture unit phases
sequentially on a new guarded snapshot, stop on first failure. Full original
acceptance and cloud gates remain in force.
