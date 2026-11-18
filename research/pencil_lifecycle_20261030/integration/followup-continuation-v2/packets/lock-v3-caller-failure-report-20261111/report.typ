#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Lock-gate v3 caller failure
Presentation of the coordinator's saved attempt observation. No additional attempt was launched.
#image("flow.svg",width:100%)
The caller failed with ValueError while parsing the UTC deadline string 2026-11-11T12:00:00Z using datetime.fromisoformat. The observation records exit code 1 before workspace or service creation. Workspace existence was false; unit readback reported MainPID zero, LoadState not-found and ActiveState inactive.
Tool elapsed time was 0.194582531 seconds. Native payload elapsed time is unknown/not applicable because no payload was submitted; it is left null. The complete 60-second reservation remains charged, bringing coordinator cumulative charge to 3660 seconds and remaining allowance to 125940 seconds. No automatic retry occurred.
The original observation remains unchanged. Source fixes and any subsequent gate are separate attempts requiring prospective bindings. Observation SHA256:
#text(size:8pt)[dd0a3bf66a209336aac0593aa01b3bf218ce72778068ecaf85c3dfb4ee66b661]
