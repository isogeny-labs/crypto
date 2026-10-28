#set page(paper:"a4",margin:18mm)

#set text(font:"DejaVu Sans",size:10pt)

#heading(level:1)[Retired-instruction A/A control]

#text("ECBS1h825e5ad6666e; same actual-40-bit subgroup model, workload and binary. Five measured identical-rho pairs and one warmup pair; all answers verify, all ten measured records replay identically. Primitive and logical native counters agree exactly.")

#image("plot.svg",width:100%)

#text("Frozen ratio range: 0.9999703014–1.0000120823. Median: 0.9999992766. These are user-space process instruction counts, not qualified wall times. L0; no isolation speed claim.")

#text("Source: ../results/instruction-AA.json; sealed spec, host, plan and records in ../instruction-AA-session/; audit ../correctness/instruction-AA-replay.json.")
