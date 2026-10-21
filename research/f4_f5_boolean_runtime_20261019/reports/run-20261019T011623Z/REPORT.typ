#set page(paper: "a4", margin: 15mm, numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false)
#heading(level: 1)[Native comparison: source-preparation failure]
#text("Run 20261019-011623-boolean-native-compariso-9fc034. Source preparation only; c7i.2xlarge on-demand. The instance terminated automatically.")


#text("The working-tree patch included PR files outside the sparse checkout. git apply rejected the patch before compilation or timing. No native cost or speed ratio exists for this attempt.")


#text("Raw receipts: preparation-failures/initial-job.json, initial-status.json and initial-job.log. Corrected runs use complete checkouts; no measurement gate was weakened.")


#image("../../preparation-failure.svg", width: 100%)