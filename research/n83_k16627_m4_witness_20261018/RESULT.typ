#set page(paper: "a4", margin: 18mm)
#set text(font: "New Computer Modern", size: 11pt)
= N83 complete-witness result

The exact K = 16,627 signed-Frobenius base imported and passed group
preflight. Its full four-summand orbit-domain model reached 201,598 variables,
5,915,238 clauses and 3,901 XOR rows. All seven x blocks were checked or
pinned before SAT entry at 40.267 s. The one-CPU, hard-4-GiB positive arm
reached its 1,800 s outer cap without a solver return (`UNKNOWN_wall_cap`),
exit 124, no OOM, peak 2,661,351,424 bytes. The dependent unpinned arm was
not attempted.

#image("witness-phases.svg", width: 100%)

The synced phase file, exact input and source hashes, container state and
commands are in `research/n83_k16627_m4_witness_20261018/`. The solver call
is the localized remaining stage; propagation versus conflict search is
unresolved. No natural relation panel, useful-rank comparison or complete
index-calculus total was produced.
