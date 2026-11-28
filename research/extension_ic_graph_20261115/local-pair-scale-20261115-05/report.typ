#import "../local-report-template.typ": render
#render(summary: json("summary.json"), plan: json("runtime-plan.json"),
  figure: image("ratios.svg", width: 100%), title: "Fresh scale holdouts for pair graph IC",
  run: "local-pair-scale-20261115-05", duration: "16.335",
  findings: [All nine targets recover. At p = 1039, orders 1121589499, 1121647183,
  1121564651 exceed $2^30$; graph/rho ratios are 14.27, 7.68, 5.36. No interval reaches 1.
  The p127/seed23 pivot A/A median 0.881 fails the frozen 10% tolerance;
  its 4.01 ratio is retained as an inconclusive diagnostic, never accepted or
  retried. Larger-case sampling counts 62284–175997 and increased algebraic
  work motivate controls for walk coalescence and a prepared S3 implementation.
  This bounded growth check does not establish a complexity exponent.
  [pcc:unverified]])
