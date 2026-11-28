#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Independent v3 source review
Presentation of the root-authored sealed review; this report does not add a second independent review. Status: source correction checked; execution not released.
#image("flow.svg",width:100%)
== Checked source correspondence
The Rust trace serializes Record with sequence and event. Event carries kind and data. The v3 acceptance parser requires exactly those envelope fields and integer sequence zero before reading event.kind and event.data. The identity-rejection fixtures now use the actual Record envelope. The deliberate first header fault remains an empty prefix; subsequent traces retain identity checks.
The review matched all 34 combined-runner and 17 lock-gate manifest members before publication packaging. It checked preserved worker, staged inputs, mathematical source and shared 45-second resolver / 60-second lock-gate cap. No actual lock was generated.
== Remaining bindings
Actual generated locks, independent runtime qualification, publication, live lease, prior-debit and final release bindings remain required. No execution or cost ratio is recorded. The reviewed source keeps the exact complete panels and attempt limits; this source review is not a measured panel result.
== Evidence custody
Source review SHA256:
#text(size:8pt)[bd68f2c7be8f62c22e8c13ead082b672b47d48aa1741bdb2cce27c2461a7e979]
The original review remains unchanged. Separate qualification-publication-custody-20261111 preserves original packaging metadata and records its artifact-pointer supersession. All source and reviewed semantic bytes are retained. This presentation is accompanied by editable SVG, CSV and JSON.
