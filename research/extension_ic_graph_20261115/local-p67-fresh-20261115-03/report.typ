#import "../local-report-template.typ": render
#render(summary: json("summary.json"), plan: json("runtime-plan.json"),
  figure: image("ratios.svg", width: 100%), title: "Matched fresh-sampling control",
  run: "local-p67-fresh-20261115-03", duration: "2.385",
  findings: [This is the predeclared fresh treatment for the sampler comparison,
  on the same source/binary and target fixtures as treatment 04. Symmetric
  pair-only takes 31.47/40.47 times local rho on seeds 3/11; symmetric hybrid
  takes 23.30/32.45 times rho. Both targets and every control pass locally.
  This is one member of the frozen paired stage. The initial pilot is 02.
  [pcc:unverified]])
