#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("All 21 five-factor censuses pass public-point completeness and rank replay")

#text("All 21 cases and 63 arm comparisons pass, with no missing case. The exact signed sets, zero/projective rows, factor-log rank and complete factor-point checks match the earlier scalar-keyed census.")


#image("../../figures/five-zero-rank.png", width: 100%)

== #text("Independent computation")

#text("The replayer builds a sorted ordinary-triple table keyed by complete curve points. The first sign is positive; querying a point and its negative restores both global signs. Pure tuples use their two smallest IDs as the query pair. Mixed tuples use their smallest ordinary ID and image factor. Every physical tuple has one canonical partition, separately from the earlier scalar-keyed ten/six-partition join. Every returned five-point sum and distinct abscissa condition is checked.")


#text("The table has 28,560 entries at p37 and 3,823,440 at p337, each 32 bytes on this build. It uses no factor-log vector or BSGS input. Anchored rows are reconstructed directly from public action coefficients. A separate unnormalized cross-multiplication elimination and extended-Euclid back substitution reproduce the earlier Fermat-based RREF ranks and full-factor point verification. The public field/curve arithmetic and orbit builder are shared; this is not an independent arithmetic kernel or formal proof.")


== #text("Confirmed result and limits")

#text("Phi/doubling factor-log ranks are 2/2 versus 1/2 at p37, 2/8 versus 1/8 at p157, and 1/14 versus 3/14 at p337; other tested curves have rank zero. All three seeds retain their own matching records. Only p37 phi recovers all unanchored factor logs. Every target coefficient is zero; no target recovery occurs. Equal mixed aggregate counts of 504 tuples and 15 projective rows conceal these differing ranks.")


#text("This is an executable public-point meet-in-the-middle enumeration used as a verifier. Its throughput is not admitted as polynomial collection, precomputation or complete IC cost. Actual collection through rank, charged setup, target descent, generic controls and matched prices remain required. No benchmark-index or leaderboard row is promoted.")


== #text("Receipts")

#text("Source 3f8e6e8b8fa135022b7b90fa7f2c7d9ad062d8f1, actual source hashes, all 81 passing native tests, commands and immutable runner checksum are in runs/five-point-replay-v1/metadata/. The frozen runner SHA-256 is bd82a9160ff24db690b2b82ea5a4c7152dff2898db6c63174f69597b54c49734. The unchanged native summary is results/five-point-replay-v1-summary.json, SHA-256 8b92c608edbb446095ceffb539e995da3f8dbae3e6ee95dd980b87b308a0989d. One worker, 8 GiB and 1,200 seconds per case after separate compilation apply, with the shared busy reservation. No timing comparison is admitted.")
