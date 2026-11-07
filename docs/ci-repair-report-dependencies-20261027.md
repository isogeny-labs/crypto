# Report dependency backlog integration

Integrates the exact Dependabot heads with normal merge commits:

- #2691: `602d103a67db1da0ddc74a53a7b355a1818c5112`, pypdf 6.10.0 -> 6.21.2.
- #2692: `f189bbbee264779cd4cc9a6760fe2d3454085933`, PyMuPDF 1.26.5 -> 1.26.7 in round 08.
- #2693: `6ae7681bc39b73a947734a0eaa0cf115d0747280`, Pillow 12.3.0 -> 13.0.0.

The executable/source delta is exactly those three dependency pins in the two
existing requirements files. No experiment, protocol, ledger, mathematical
statement, native source or canonical rendered asset changes. Package metadata
confirms the guide's documented Python >=3.12 environment meets the new pins;
the round report's PyMuPDF pin requires Python >=3.10.

Both existing report builders ran in isolated copies using their complete
updated requirements. The twist guide rebuilt with pypdf 6.21.2 and Pillow
13.0.0; its existing checker passed all 16 pages, 16 bookmarks, eight links,
vector-only diagrams and safe text boundaries. Its 16-page contact sheet was
visually reviewed. The native C fixture rebuilt with C11 warnings as errors and
matched the committed fixture byte-for-byte.

The round-08 builder rebuilt and rasterized all five pages using Typst 0.15.0
and PyMuPDF 1.26.7. Six source/fixture/protocol/ledger inputs in the isolated
copies match their committed originals exactly. These are rendering and fixture
checks, without solver execution, benchmark timing, regenerated research
measurements or new performance claims. Generated scratch reports are not
substituted for frozen canonical evidence.

Sources reviewed: all three complete submitted diffs, both requirements files,
the two report builders, the guide's existing validation script and native
fixture replay instructions, and the exact versions' PyPI metadata. Raw build
and validation logs, complete resolved package versions, and source hashes are
retained under `ci-repair-report-dependencies-20261027/`.

Fresh current-head merge checks and review remain required before merge.
