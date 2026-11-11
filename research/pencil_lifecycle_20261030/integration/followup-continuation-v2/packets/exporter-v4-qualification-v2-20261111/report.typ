#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Exporter qualification v2
Source successor · 2026-11-11

The monitor now calls lstat once. Only FileNotFoundError permits an absent target directory, and only during materialization. Existing paths must be directories by their lstat mode. Nonabsence errors propagate.

#image("flow.svg",width:100%)

The exact source archive,4,500-file expected inventory,106 overlays, ten exporter controls and one full-file6,300triplet synthetic control are unchanged. The proposed bound remains1,800build plus600execution seconds,2CPU and16GiB. Workspace,unit and output identities are fresh; no admission is released.

Original publication metadata is preserved separately. The original ignore-file bytes were reconstructed only after matching the parent manifest hash and length. Mutable artifact/ignore files are excluded from the successor semanticmanifest.

No compiler, nativepayload, tests, indexoperation or publication occurred. Independent review and actualserialboundary bindings precede execution.
