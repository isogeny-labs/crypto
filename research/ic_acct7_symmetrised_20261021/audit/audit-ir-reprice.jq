# Join each audit-ir line (Callgrind solve Ir, unchanged) with its record's
# bitop/v1 total as repriced by `ecbench work --counted` (the fixed binary),
# by seq -> record_id from the session's records.jsonl.  Run:
#   jq -n -c --slurpfile ir audit/n17-audit-ir.jsonl \
#     --slurpfile rec sessions/n17-symmetrised/records.jsonl \
#     --slurpfile work audit/n17-work-counted.jsonl -f audit/audit-ir-reprice.jq
($rec | map({key: (.seq|tostring), value: .record_id}) | from_entries) as $id
| ($work | map({key: .record_id, value: .work.Ok.bitops}) | from_entries) as $bits
| $ir[]
| ($bits[$id[.seq|tostring]]) as $b
| {schema: "ic-acct7.audit-ir-repriced/v1", seq, method, solve_ir, total_gae,
   bitops_recorded: .bitops, ir_per_kilo_bitop_recorded: .ir_per_kilo_bitop,
   bitops_repriced: $b,
   ir_per_kilo_bitop_repriced: (.solve_ir / ($b|tonumber) * 1000)}
