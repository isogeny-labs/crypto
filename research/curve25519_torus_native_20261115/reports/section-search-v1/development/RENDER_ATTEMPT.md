# First render command directory failure

The first render wrapper ran from the report directory while using repository-
relative paths. Graphviz reported:

    Error: dot: can't open research/curve25519_torus_native_20261115/reports/section-search-v1/flow.dot

It stopped before LaTeX. Its accidentally nested problems/README.md copy was
moved to the private development checkpoint, then the real index was restored
from exact tracked HEAD bytes before the intended one-row addition. Corrected
rendering used the actual report directory and report.tex basename. The final
three-page PDF was inspected on all pages. No scientific input was changed.
