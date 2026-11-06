#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Full p37 residual census and all 21 four-factor zero fibers")

#text("uniform-zero-v2 completes all 21 zero-fiber cases and all three frozen p37 population cases, with no missing case. This is the registered scalar/group-oracle diagnostic, not a polynomial collector or IC price.")


#image("../../figures/uniform-population.png", width: 100%)

== #text("Uniform population")

#text("For every p37 seed the subgroup has 1,873,633 residuals and L0 has 36 abscissae. All signed candidate tuples are assigned to their actual residual. Every measured mass equals its combinatorial domain size.")


#text(size: 6.8pt)[#table(columns: 4, inset: 4pt, text("Arm"), text("Signed-tuple mass"), text("Nonempty residuals"), text("Sum of squared yields"), text("Original L2"), text("1,370,880"), text("949,248"), text("2,476,032"), text("Phi"), text("4,112,640"), text("1,551,768"), text("16,283,616"), text("Doubling"), text("4,112,640"), text("1,584,745"), text("14,842,680"), text("Pure L0"), text("942,480"), text("675,000"), text("1,749,648"))]

#text("The native records retain exact rational means and variances, the full histograms and their frequencies. Phi and doubling have identical first moments. Phi's second moment is larger and its support smaller here. Neither difference establishes independent rank yield or solving cost. The three seeds are presentations of the same curve, not three independent curve samples. The earlier conditioned residual tape is a different scope.")


== #text("Four-factor zero fibers")

#text("Original, phi and pure have no signed physical zero relation in any of the 21 cases. Doubling has genuine physical relations on every case. For p=37,73,97,157,229,241 its factor-log rank is zero. At p337, each seed has 384 physical relations, 360 zero augmented rows and factor-log rank 1/14. Thus the doubled-triangle identities cannot be assumed to explain every observed relation. Their separately registered point/row reconstruction checks the predicted 2L subset and reports the additional relations, without repeating this oracle.")


#text("Every zero-only row has target coefficient zero. No target recovery or complete factor-log recovery is claimed. These bounded findings do not exclude higher arities or other components.")


== #text("Execution and evidence")

#text("Driver run-uniform-zero.sh, source label f2ab4b4636c07f87929b5852ab9ae8967ba1d5fe plus actual source/runner hashes. Original one-thread, 8 GiB and 1,200-second per-case limits and the shared busy reservation are retained. The earlier prebuild failure stays separate. No admitted wall-time comparison or benchmark promotion follows.")


#text("results/uniform-zero-v2-summary.json is the unchanged native summary, SHA-256 fe6383a480bf594e8393d3e85619f9123501a12c0b5b431148ca2aad12bf23be. The raw run retains all 84 arm records, population histograms, public hashes, commands, failures and exits. Source and target models are the original fixed-model fixtures; selection among different twists is not tested.")
