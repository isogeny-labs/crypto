# Exact label-allocation audit of Frobenius splitting scores

Anonymous research addendum,30October2026. No submission has been made.
The new exact calculation concerns label assignments over previously verified
scores, not newly sampled curves. It preserves all earlier papers/results.

Native summary tables are copied byte for byte from the separately verified
audit in research/nist_skip_criterion_20261025/exact_pair_audit_20261030.
The paper distinguishes exact allocation arithmetic, external procedural
limitations, public generator base changes and the unresolved historical goal.

Build with make check and bundle with make eprint using shared papers/paper.mk.
The standalone package includes paper.mk, both native-generated table files, the exact allocation distribution TSV, its checked C++17 SVG generator, and the rendered SVG/PDF figure. The source TSV is copied byte for byte from the separately verified audit; SHA-256: `384315ac60305ebca5b74d75dbbd7dba15f76f61b8076540e44c87bcfd8779a2`.

Rebuild the visual from this package with `g++ -std=c++17 -O2 -Wall -Wextra -Werror figures/allocation_distribution.cpp -o /tmp/allocation_distribution && /tmp/allocation_distribution data/distribution.tsv figures/allocation-distribution.svg && rsvg-convert -f pdf -o figures/allocation-distribution.pdf figures/allocation-distribution.svg`. The generator checks the 205 contiguous bins, total allocations, every inclusive suffix count, and the observed 2U=117 tail before drawing.
The package format does not imply ePrint submission or cleared authorship.
