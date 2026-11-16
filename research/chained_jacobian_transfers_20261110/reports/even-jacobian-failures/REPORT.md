# Retained protocol and proof failures

ProtocolV1/73b7dc was frozen after its prose gate reported one missing
standing tag. V2/6f1e93 adds that annotation only,before adapter code or
execution;all mathematical predictions and acceptance criteria stay fixed.
Both versions and actual lint outputs remain preserved. [pcc:unverified]

EvenJacobian formal01/044c8c fails its build at the variable-scalar Frobenius
rebasing identity:grind linarith reports unexpected d-smul instances.
The compiler's recovery prints sorryAx for that failed declaration;the
module has zero accepted audit declarations and is not promoted. Full source,
build/gate/status and hashes remain in evidence/even-lean-failed-01.
[pcc:unverified]

Corrected gate02 explicitly distributes with IntModule.zsmul_sub,then proves
the additive cancellation. No theorem statement or hypothesis changes.
All six exact statement bindings locally pass,zero trusted native;actual
AV/Picard bridges remain written/native. Full37/3c1cb7 and its complete
624 afterhashes are separate accepted native evidence,not a signature.
[pcc:unverified]

The diagram records actual failures/remedies,not timing. Editable DOT/SVG,
renderer and inspected PDF accompany it. The main source-linked report is
[whole rational maps](../even-whole-maps/REPORT.md). GitHub transport422/502
failures and recovery remain separately archived in512551;no native
mathematical prediction failed in this component. [pcc:unverified]
