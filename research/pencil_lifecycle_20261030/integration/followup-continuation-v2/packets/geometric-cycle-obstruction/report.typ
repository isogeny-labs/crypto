#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
#text(size:16pt,weight:"bold")[Geometric cycle fixture boundary]
#image("coverage.svg",width:100%)

#text("Inspected the complete frozen set of 60 archived p=739 known-answer records, containing 420 case divisors. The deterministic selection extracted 114 distinct split-divisor edges. They form a forest: there is no cycle in this retained edge set.")

#text("The remaining cases were 126 nonsplit or repeated-root cases, 120 signed duplicates, and 60 degree skips. Generator equality was checked across all 60 archived records. Each accepted case carries its archived scalar relation s=(a+b*k) mod r. No new target, trajectory extension or scalar search was run.")

#text("The prospective construction would select an existing cycle, realize each selected divisor as an original-G transition through one of two algebraic predecessors (D-cG)/2, recover from the propagated pair, and pass the signed divisor to the existing graph adapter. Its labels would deliberately use the known answer to plant a target-bearing closing edge. This would be a known-answer geometric software fixture, not a collector search result. That construction did not start because the required cycle was absent.")

#text("Observed resource boundary: 420 of 420 permitted archived cases inspected; 0 of 840 permitted predecessor attempts; 0 native executions. No Q=kG terminal check or geometric stopping claim is reported. Existing synthetic equation controls and earlier recovery failures remain separate unchanged evidence.")

#text("The concrete missing input is an additional geometric relation set containing a signed cycle that supplies a target constraint. This finite forest observation does not exclude cycles in other relations or prove that the proposed original-G inverse-branch construction can reach a future cycle. A first unsigned cycle may also only anchor a component; terminal verification remains a separate check.")

#text("Reproduction: run select.py in a fresh private copy with the original archived path available; selection.json contains the input hash and all 420 dispositions. archive.jsonl preserves the exact input. Frozen PLAN.md preceded inspection. No source producer files were changed. Independent review pending.")

#text("Sources and interfaces: prior graph-integration packet and native SignedGraph; CCS recovery paper, sections 2, 6 and 7, https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf. The construction uses the established difference-pair recovery interface. No novelty claim.\n")
