# Frozen input consistency before collection

This component closes a driver input-binding gap. It performs metadata and
hash checks only on existing recorded data. It does not launch collection,
extract roots,generate curve points,validate admission or provide an
independent mathematical verdict. Old pinned sources/data remain unchanged.

Pin the exact fixed-main baseline reference by SHA256 and compare a prior
recorded local baseline to it. Require identical top-level key sets and
identical typed JSON for every field except standing,stages,cost_scope.
Those three metadata values may differ;they confer no verification authority.
The remaining17 fields bind pool/order/reference,images/signs,old base,rows,
public composition companions,scope and deterministic counts/rank/checks.
Bind cover,quartic model and backmap bytes by their fixed SHA256 values.

Use a separate typed copy of the repository's recorded-JSON reader. Preserve
numeric lexemes and type tags;reject unsupported encodings rather than
coercing data. No external JSON library or changes to old parser/source.

Freeze these controls before execution:
- Positive:fixed reference against recorded local baseline with pinned inputs.
- Positive:change only standing,stages and cost_scope;mathematical data stays.
- Twelve semantic rejects:point coordinate,point order,old base coordinate,
  projection sign,mapped image,reference support,row coefficient,public norm
  companion,rank value,missing required field,extra root field,value type.
- Exact byte rejects:reference whitespace drift,and separate cover,model,
  backmap whitespace drift. Semantically equal JSON is still different from
  an exact construction-input pin.
- Typed parser controls:number versus same-text string,and [] versus {}.
- CLI rejects:unknown mode,ambiguous mode and existing output. Existing result
  bytes must remain unchanged by the collision call.

Guard has --self-test and --check modes. Qualify only --self-test with recorded
inputs and declared mutations,then one --check on those same recorded inputs
to exercise driver-mode transport. No new mathematical replay or collection.
Build cap300seconds/1GiB;each guard call30seconds/1GiB. Max input32MiB.
Fresh write-once source/input/binary manifests precede calls;retain failures,
exact exits,diagnostics and process clocks. No cap/outcome fitting.

Prepare a successor driver that runs the guard after the fixed baseline
replay and before collector --collect. Its actual baseline/collection paths
remain unexecuted. The driver/envelope still requires external admission;
this input guard neither reads remaining budget nor configures signing.

Sources:split_tangent_20261123 run-admitted.sh and protocols;native json.hpp;
fixed replay/reference artifacts and validate_maps_metadata.py. KB unavailable
as recorded in the journal. This is a consistency guard,not a new crypto
algorithm,performance result or signed geometric verification. [pcc:unverified]
