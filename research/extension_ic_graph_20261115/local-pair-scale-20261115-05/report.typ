#import "../local-report-template.typ": render
#render(summary: json("summary.json"), plan: json("runtime-plan.json"),
  figure: image("ratios.svg", width: 100%), title: "Fresh scale holdouts for pair graph IC",
  run: "local-pair-scale-20261115-05", duration: "16.335",
  findings: [All nine targets recover. At p1039, orders1121589499,1121647183,
  1121564651 exceed2^30; graph/rho ratios are14.27,7.68,5.36. No interval reaches1.
  The p127/seed23 pivot A/A median0.881 fails the frozen10% tolerance;
  its4.01 ratio is retained as an inconclusive diagnostic, never accepted or
  retried. Larger-case sampling counts62284–175997 and increased algebraic
  work motivate controls for walk coalescence and a prepared S3 implementation.
  This bounded growth check does not establish a complexity exponent.
  [pcc:unverified]])
