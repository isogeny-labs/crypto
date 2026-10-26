#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= Equal-advice n = 83 (P4-min): stopped, BL precompute cannot finish in the cap
Pod `mhjde0s1ysa5aq`, 32 vCPU, 22:08Z–01:22Z, about \$4.15. No result: the protocol never left the BL table precompute.

#image("bl-progress.png", width: 100%)

*Recorded.* 240.4 M of 934.7 M distinguished points after 3.10 h; 3.74e9 steps per 590 s epoch; new DPs per epoch 14.47 M → 11.22 M (×0.9862 per epoch).

*Derived.* Finishing needs 13.3 h (constant gain) to 26.8 h (declining gain) at 32 threads, \$17–\$34 on this host, far past the 6 h cap. The card's 1.4–3.5 h estimate was low by 4–8×.

*Not claimed.* No BL-vs-IC reading, no claim, no decision.
