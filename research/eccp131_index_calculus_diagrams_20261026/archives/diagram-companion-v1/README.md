# ECCp-131 index-calculus diagrams

Research diagrams compare elliptic and hyperelliptic index calculus, explicit
covers of Certicom ECCp-131, and extension-field approaches over F_(p²) and F_(p³).

Open [the ten-figure research viewer](diagrams/research/index.html) directly in a
browser. It is self-contained and works offline. Individual SVG and PNG figures,
a printable [PDF](diagrams/research/research-diagrams.pdf), editable Python
sources, and LaTeX formula sources are included. The
[four-figure introductory viewer](diagrams/index.html) is also preserved.

The research viewer includes comparison modes for equal field size, equal
generic group order, and the original subgroup embedded in a cover Jacobian;
a relation-rank toggle; and illustrative relation-yield controls.

## Mathematical scope

ECCp-131 is the prime-field Certicom challenge, distinct from ECC2K-130.
`parameters.json` records its published public parameters and source URL.
`evaluate_extensions.py` checks the points, subgroup order, extension arithmetic,
Frobenius projections, and small index-calculus controls. Its relation search uses
exhaustive tables, not a Gröbner solver. The small controls are different curves
with coefficients reduced modulo small primes.

Figures distinguish exact identities, heuristic assumptions, representative
asymptotic bounds, and illustrative cost models. Extending the field or embedding
the target into a larger Jacobian preserves the original subgroup order. Generic
full-Jacobian cost bounds do not automatically apply to these split covers or
their restricted target subgroups. The toy relations are constructed examples,
and the fiber census does not enumerate the full Jacobian. These artifacts do
not compute the challenge logarithm or establish a speedup over Pollard rho.

## Reproduce

Python 3.12 was used. From this directory:

```sh
python -m venv .venv
. .venv/bin/activate
python -m pip install -r requirements.txt
# Needed if a system chromium executable is unavailable:
python -m playwright install chromium

python evaluate_extensions.py --output /tmp/eccp131-results.json
python diagrams/research/worked_examples.py
python diagrams/research/build_research.py
python diagrams/research/render_research.py
```

The extension checks write to a separate file above because `results.json`
contains a recorded run with timing fields. The worked examples regenerate
`worked-example-results.json` and `fiber-census.csv`. The figure builder
regenerates SVGs, the offline viewer, and formula sources. The renderer produces
PNGs, a ten-page PDF and a ZIP bundle; it also checks SVG references and label
bounds, browser errors, all comparison modes, the rank toggle, sliders and SVG
downloads. ZIP bundles and obsolete archives are excluded from version control.

The render scripts use system `chromium` when available and otherwise use
Playwright's installed browser. Set `CHROMIUM_EXECUTABLE` to select another
Chromium executable. Rendering requires permission to launch a local browser.

To rebuild the introductory set:

```sh
python diagrams/build_diagrams.py
python diagrams/render_diagrams.py
```

Primary research references and figure-specific assumptions are in the research
viewer's expandable notes and `figures.json`.
