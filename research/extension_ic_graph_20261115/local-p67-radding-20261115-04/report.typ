#import "../local-report-template.typ": render
#render(summary: json("summary.json"), plan: json("runtime-plan.json"),
  figure: image("ratios.svg", width: 100%), title: "Incremental residual-sampling treatment",
  run: "local-p67-radding-20261115-04", duration: "1.028",
  findings: [Symmetric pair-only takes 1.294/2.901 times local rho on seeds 3/11,
  compared with 31.47/40.47 for fresh 03. Its sampled residual counts are 402/1204,
  versus 1884/1829 in 03; setup costs 1623/1629 group operations and sampling
  costs 452/1357, with 1/3 restarts. Different samplers have different streams;
  these counts do not isolate a per-residual speedup. The hybrid takes 17.56/28.84
  times rho. Neither pair stratum reaches parity. Fresh scale holdouts test
  whether the narrower gap persists. [pcc:unverified]])
