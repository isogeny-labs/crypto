# Startup gate work in progress

Two focused budget-helper tests passed (exit 0), covering complete grant binding
and live activation-phase recovery without repeating requests. The receipt pins
the modified budget.rs hash over base d30b0ade734. Their logs and receipt were
uploaded and freshly downloaded/hash-checked; see preintegration-readback.json.

This validation predates module registration. It does not validate the whole
startup gate, CLI integration, real activation, deployment or cloud generation.
The separate startup module is under implementation and independent review.
The artifact helper initially encountered a Git index lock after successful
uploads; its retry reused the objects and completed manifest staging. No test
was rerun or source changed to resolve that archival issue.
