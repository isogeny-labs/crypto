#set page(paper: "a4", margin: 15mm, numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false)
#heading(level: 1)[Native comparison: launch capacity refusal]
#text("Run 20261019-013933-boolean-native-batch-fin-aed805: the on-demand c7i.2xlarge request was rejected by the shared 32-vCPU Standard On-Demand quota before any instance booted. No compilation, timing or solver result exists for this attempt.")


#text("Raw launch/status/command receipts: preparation-failures/quota-final-*. Dedicated retries use the same c7i.2xlarge type in the separate Spot quota. The frozen inputs, one-worker/core reservation, output verification and admission gates remain unchanged. No other job was stopped and no account setting changed.")


#image("../../quota-failure.svg", width: 100%)