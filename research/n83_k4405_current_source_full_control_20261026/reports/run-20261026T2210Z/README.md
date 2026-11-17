# K4405 bounded resource control visual record

The [PDF](report.pdf) embeds the [recorded resource trace](resource_trace.svg)
for the [K4405 evidence bundle](../../evidence-01/). The figure has 1,613
samples and cites the original `samples.json` digest in `receipt.json`.
[`plot.py`](plot.py) reads only those retained samples and renders both the SVG
and a 150 dpi [PNG](resource_trace.png). [`report.typ`](report.typ) is the PDF
source. Build from this directory with:

```sh
python3 plot.py
typst compile report.typ report.pdf
```

The record is a local host resource control with terminal status
`BOUNDED_UNKNOWN_HOST_SWAP_ABORT`; it supplies no rank or target timing row.
