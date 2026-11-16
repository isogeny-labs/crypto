#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Doubled-triangle control passes all 21 saved zero sets")

#text("The separate public-point reconstruction passes every original case and has no missing case. It reads saved zero sets and repeats no BSGS oracle.")


#image("../../figures/zero-identity-outcome.png", width: 100%)

#text("For every ordinary source P, the checker reconstructs [2]P - P + psi(P) + psi²(P) = O and its global negative from actual points and sign encodings. The distinct allowed factors and doubling-image injectivity make 2L different physical tuples. Every predicted tuple occurs in the saved complete zero set, and every predicted augmented row is zero. The separately checked Lean coefficient identity is not substituted for these actual group, set-membership and row checks.")


#text(size: 6.8pt)[#table(columns: 5, inset: 4pt, text("p"), text("Predicted signed tuples"), text("Observed tuples"), text("Additional tuples"), text("Observed factor rank"), text("37"), text("72"), text("72"), text("0"), text("0/2"), text("73"), text("96"), text("96"), text("0"), text("0/3"), text("97"), text("72"), text("72"), text("0"), text("0/2"), text("157"), text("216"), text("216"), text("0"), text("0/8"), text("229"), text("192"), text("192"), text("0"), text("0/7"), text("241"), text("192"), text("192"), text("0"), text("0/7"), text("337"), text("360"), text("384"), text("24"), text("1/14"))]

#text("Each table row is observed separately on all three seeds. The control explains the whole physical zero set on six curves. At p337 it explains the 360 zero rows, while the 24 additional tuples contribute the separately observed rank one. Thus neither all physical zero relations nor all doubling zero relations can be dismissed as tautologies.")


#text("The checker reuses the immutable 79-test runner from five-zero-v1, SHA-256 5d38c7f75fb1ce23cf41e13daaf0e6900478fb8b5dded208980b2086d4b250fd. The predeclared protocol is SUPPLEMENT_ZERO_DOUBLING_IDENTITY.md; run-zero-identity-audit.sh waits for all 21 saved outputs before checking. Commands, input/runner hashes and exit zero remain in the raw run.")


#text("Unchanged native summary: results/zero-identity-audit-v1-summary.json, SHA-256 4cffa2b82a00b3a97eae945cf4a77d31b723c7fcbe0b25115afa2244586d5077. One worker, 8 GiB, 1,200 seconds and the busy reservation are retained. This is a structural subset control, not a new collector, target recovery or complete-price comparison. Every zero-only target coefficient is zero.")
