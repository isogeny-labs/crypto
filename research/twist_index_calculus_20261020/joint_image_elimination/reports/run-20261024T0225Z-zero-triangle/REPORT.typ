#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Folded zero-triangle coefficient proof")

#text("The bounded one-worker Lean gate on ZeroDoubling.lean (../../proof/uniform-v1/ZeroDoubling.lean) passes its one theorem, ZeroDoubling.folded_triangle. It assumes w^2+w+1=0 in a universal commutative ring and proves 2-1+w+w^2=0. The axiom audit records no failing or trusted-native declaration. Pinned Lean 4.34.1, the source snapshot, command and timestamps remain in runs/zero-triangle-lean-v1/.")


#image("../../figures/joint-mechanism.png", width: 100%)

#text("The predeclared structural control (../../SUPPLEMENT_ZERO_DOUBLING_IDENTITY.md) uses this identity to predict a family of physical relations whose folded matrix rows vanish. This theorem alone does not prove group identities, base membership, distinctness of the four physical factors, inclusion in a complete signed set, or the predicted count. Those are explicit obligations of the separate native post-run checker. The first saved p37/seed0 doubling oracle reports 72 signed relations, 72 zero augmented rows and rank zero; the 21-case structural checker has no completed verdict yet.")


#text("This is algebraic correctness evidence under an 8 GiB/900 s gate, not a runtime comparison or a useful-relation yield claim. See the proof map (../../proof/PROOF_MAP.md) for the remaining bridges.")
