#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Tian quartic: bounded published-class DLP control

*Receipt written:* 2026-10-20T07:21Z UTC. *Command:* `tian-quartic-km --dlp-linear` from `native/Cargo.toml`. *Host:* shared Intel Xeon 6975P-C; L0 operation counts only.

The solver received Tian's published quartic classes S,T and a cap of 10,000, without the published scalar. It tested consecutive multiples of S, recovered 1987 at the 1,987th candidate, and recomputed 1987P=Q on the source curve. This is one unusually small published scalar. The raw source-verified receipt is `correctness/tian-km-linear-dlp-v1.json`; the native 12-price analysis is `results/tian-km-linear-dlp-price-grid.csv`.

At M+0.5S+10I the complete cold candidate costs 49298368688 base-field multiplication equivalents, or 51979.77× the mean of five complete optimized source-rho solves. The ratio spans 3710.83–60937.36× over the 12 arithmetic prices. Native logical work is unpriced in this comparison and wall time is unqualified.

#figure(image("cost-ratio.svg", width: 100%), caption: [Recorded cold cost of the one published-class destination DLP relative to mean source rho; dashed line is equal cost.])

The source-to-Jacobian map was not executed; the published S,T are direct inputs. The proposed genus-3 index-calculus solver was not run. This control does not establish a general cubic-transfer attack cost or an external-system speed comparison.
