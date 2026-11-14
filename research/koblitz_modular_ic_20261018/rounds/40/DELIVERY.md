# R40 delivery reconciliation

The first normal push was rejected because the remote branch had gained
github-actions commit 0bed74d146a1d84a0148eaae96bbe7f3f14f5f04,
"Move PR data files to the artifact store". Fetch inspection showed only the
R39 source-data/local-input-template.sing migration and its manifest/ignore.
The manifest SHA256 0309454e93c46e7cb899c9b441d8246b18b161bcd39822a796dd79527b24492c
matched the original 3027578-byte file exactly before the merge. The commit
was merged without rewriting history. R39's sealed raw bundle separately
retains the complete input; no mathematical producer was repeated.
The configured fetch wrote FETCH_HEAD without creating the guessed remote
tracking ref, so inspection used the fetched commit explicitly.

Before the R40 seal, the generated SVG's trailing whitespace was normalized.
The regenerated one-page PDF was checked pixel-identical to the visually
inspected page. The cleanup is in scientific revision b9df19a55. This was
formatting validation, not a mathematical rerun.

The R40 archive was uploaded to setec-astronomy-artifacts/store and freshly
downloaded. All324 member hashes,48 source pins and deterministic repack pass.
Source pins were checked against the historical scientific commit or the
explicitly identified sealed R34 archived dependency-path record.
This does not constitute independent mathematical replay or new root-library
suite validation. Full saturation, target-model binding and larger requested
support obligations remain open.
