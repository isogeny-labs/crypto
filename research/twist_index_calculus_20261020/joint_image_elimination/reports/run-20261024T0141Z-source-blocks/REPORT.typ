#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Source-block regression: complete physical sets, local rank comparison")

#text("regression-v2 resumes only the three source-block unions whose original launchers lost their process owners during recovery. It uses the frozen 950e15f38c4a2646a14518ee89cc672687ac2d6a runner copied from v1. The six terminal global attempts were not repeated. All three resumed solves and their independent audits have exit zero; complete.txt is retained.")


#image("../../figures/source-block-rank.png", width: 100%)

#text("Each union covers all allowed source values in three disjoint blocks U^12=T, with T=1,10,26, on the predeclared positive p37/seed0/query2 fixture. Each block reports complete roots within the unchanged per-query budget. The verifier compares the exact signed physical sets with the earlier independent group/BSGS oracle, checks actual group sums and independently reduces the folded matrix.")


#text(size: 6.8pt)[#table(columns: 4, inset: 4pt, text("Arm"), text("Distinct signed physical relations"), text("Independent rank / columns"), text("Signed-set audit"), text("Pure L0, partial symmetry"), text("1"), text("1 / 3"), text("pass, no missing or extra tuple"), text("Phi image"), text("2"), text("2 / 3"), text("pass, no missing or extra tuple"), text("Doubling image"), text("1"), text("1 / 3"), text("pass, no missing or extra tuple"))]

#text("All numbers above are copied from the native retained audits (../../results/source-block-regression/). A distinct row count is not substituted for independent rank. The phi relations occur in the T=10 block; the other two blocks are still fully solved and charged in the actual execution. No arm has full column rank from this single residual, so none constitutes a complete DLP run.")


#text("This establishes a local rank advantage for phi on one selected positive query. It does not establish a uniform-residual yield advantage, a larger prime result, a free twelvefold symmetry quotient, or an end-to-end speedup. All 21 source partitions and map coordinates were checked natively; full group-action equivalence of the partition remains open.")


#text("The run preserves one thread, 8 GiB address space, 1,180 s cooperative and 1,200 s outer limits, and the busy reservation. Its component durations are unadmitted diagnostics. They are not compared as a speed ranking. The stronger pure four-factor symmetry control and mixed inverse-variable controls have separate runs and must remain in any subsequent pricing.")
