#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Six native representation attempts: terminal outcomes")

#text("variant-regression-v1 is terminal, with all six bounded attempts retained. Inputs and the frozen runner come from corrected exporter verification-v8. Every input is p37/seed0/query2. The two pure successes have independent signed-set/RREF audits; allocation-censored mixed attempts have no invented physical yield, rank or recovery result.")


#image("../../figures/joint-mechanism.png", width: 100%)

#text(size: 6.8pt)[#table(columns: 5, inset: 4pt, text("Arm"), text("Encoding"), text("Recorded exit"), text("Physical-set verdict"), text("Rank / columns"), text("Pure L0"), text("Full symmetry, finite"), text("0"), text("complete, independently matched"), text("1 / 3"), text("Pure L0"), text("Full symmetry, geometric"), text("0"), text("complete, independently matched"), text("1 / 3"), text("Phi"), text("Implicit inverse, finite"), text("250"), text("allocation failure; no complete set"), text("unknown"), text("Phi"), text("Implicit inverse, geometric"), text("250"), text("allocation failure; no complete set"), text("unknown"), text("Doubling"), text("Implicit inverse, finite"), text("250"), text("allocation failure; no complete set"), text("unknown"), text("Doubling"), text("Implicit inverse, geometric"), text("250"), text("allocation failure; no complete set"), text("unknown"))]

#text("All four failing stderr logs report allocation failure and the watchdog's core-dump message; core size was disabled. Exit 250 is preserved as emitted through the wrapper, not recoded into a successful empty result. There is no complete root summary for those processes. The unchanged limits are one thread, 8 GiB, 1,180 s cooperative and 1,200 s outer, under the busy guard.")


#text("Full symmetry repairs the pure baseline's weak representation on this fixture. In contrast, the source-block formulation completes for all three arms and matches their physical oracles, with ranks 1/3, 2/3 and 1/3 for pure, phi and doubling respectively. This does not establish which method is fastest on a population or complete collector. No admitted timing, end-to-end price, GAE band or matched rho claim follows from these runs.")


#text("The requested extension now attempts all 297 construction positions using the same frozen exporter, reusing the three completed regression exports instead of recomputing them. Construction success does not replace complete roots, exact physical sets, collection through rank or full cold costing.")
