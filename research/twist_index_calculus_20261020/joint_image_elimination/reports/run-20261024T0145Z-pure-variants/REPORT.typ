#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Fully symmetric pure control: both encodings audited")

#text("The pure portion of variant-regression-v1 is terminal. Its finite and geometric encodings come from corrected exporter verification-v8; the same frozen v8 runner solves them. Mixed phi and doubling attempts in the same run remain pending or running and have no verdict in this report.")


#image("../../figures/pure-symmetry.png", width: 100%)

#text("On p37/seed0/query2, each pure encoding gives the exact same one signed physical relation as the source-block control:")


#text("    [(5,-1), (21,-1), (23,+1), (24,+1)]")


#text("Both native root enumerations report complete. Both independent signed-set/RREF audits pass, with one distinct projective augmented row and rank 1 of 3 columns. The geometric output contains two field roots; one fails the requirement of four distinct allowed Fp factors. Every surviving sign assignment is checked in the actual group. Discarded nonphysical field roots are not counted as useful relations.")


#text("The native coefficient check lowers pure total degree from 16 to 8 and the extension-term count from 1,485 to 495 by using all four elementary symmetric coordinates. This is generic permutation symmetry and is a necessary baseline. It cannot be credited to a twist-derived map.")


#text("The native summaries and independent audits are retained loose for inspection (../../results/pure-variant-regression/), with the original commands, hashes and logs under runs/variant-regression-v1/. One thread, 8 GiB and the frozen 1,180/1,200 s limits are unchanged. The host does not admit these component durations as performance evidence. No full-rank collection, recovery, matched rho or full GAE price is supplied.")
