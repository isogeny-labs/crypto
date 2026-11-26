[.[] | {
  prime,depth,target,mode,planted,expected,charts_planned,charts_completed,
  forms_attempted: ([.systems[] | (.attempts // []) | length] | add),
  status: .result.status, reason: (.result.reason // null),
  returned: (.result.relations // null),
  completed_chart_rows: (if .mode=="LiftableCutoff" then
    [.systems[] | (.rows // []) | length] | add else null end),
  capture_omissions: ([.systems[] | (.input_prefix_capture // empty),
    (.attempts[]? | .rur_capture // empty)] | length)
}]
