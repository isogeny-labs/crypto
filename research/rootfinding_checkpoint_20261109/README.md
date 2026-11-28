# Exact finite-field rootfinding research checkpoint

This additive research snapshot preserves independently checked five-prime certificate/recovery work and three fresh bounded experiments. It does not change production solver code or an existing frozen report. Historical source paths remain inside the archives; browse the exact copied source under [code](code/).

## Evidence

- [Expanded 320-chart report](v3/REPORT.md): three cold five-NIST certificate panels and15 planted positive controls. The v3 root-recovery build failure remains preserved.
- [New recovery and leads report](v4/REPORT.md): fixed conditional all-roots recovery; exact leaf-parabola optimization; whole-h elimination oracle and its failure to prune actual tiny blocks.
- [v3 complete source/input/raw packet](evidence-v3.zip), SHA256 da0fe57647bb24a6cc7cfdf7234e9f6a971ca90beb3a65bafa0bbc9a0f106978.
- [v4 source/input/raw packet](evidence-v4.zip), SHA2564043ed186caf247ebd0c203819e7193cabc48929949efc2fc175c0d9ff99dce5.
- [Publication source hashes](PUBLICATION_MANIFEST.json). Archive integrity, ledger journals, source/input identities and independent result reviews were checked locally.

Each archive is a separate immutable snapshot. Extract each into a separate empty directory; never extract over historical evidence. Before reproduction use a new output directory/manifest/ledger: workers deliberately refuse to overwrite raw artifacts. System GMP/FLINT libraries and compiler identities are pinned in receipts, not bundled. The earlier full NIST setup requires the separately identified FLINT proof library and source presets; no binary dependency is downloaded by a worker. Local checkout and commit receipts are provenance, not claims that remote CI ran.

## Findings and limits

Across P-192/P-224/P-256/P-384/P-521, the repaired enumerator and optimized variant agree with independent full conditional sets: three ordered residual tuples per planted chart. These are controls, not hard benchmarks or full original S3 solutions. Leaf optimization reduced native visits22,509→271 while the independent verifier still traversed the original box. Single native runs0.716s→0.585s and total workers3.542s→3.168s do not qualify a speed ratio.

Fresh outer-search lead: one certificate for an h block, with exact polynomial dependencies. The oracle passed917 identities and10,449 support checks; zero actual blocks were rejected. The next [seven-feature hypothesis](code/research/outer_leads_review_20261109_v4/STRUCTURED_SEVEN_DIMENSION_LEAD.md) addresses that limitation, with explicit feature-height risks. Recent [Dual Coppersmith prior art](https://eprint.iacr.org/2026/2740) is relevant: its official abstract was read, full PDF access was403, and no priority claim is made.

The requested8x target and immutable2x release gate remain open. No genuine completed matching full all-roots F4/F5/F6 comparison, global hard solve, comprehensive field/curve/scale result, or security conclusion is claimed. All3 new experiments have one-core/4GiB worker limits and preserve exact unknown-on-cap semantics. Full original auxiliary/order/exceptional coverage and GF(2^83), ordinary binary, Koblitz and odd-extension timing qualification remain active workstreams.
