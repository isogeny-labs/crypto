#!/usr/bin/env bash
# Execute the Lean source auditor. Input export is separate metadata processing.
set -eu
audit_project=$(realpath "$(dirname "$0")/lean")
audit_input=$(realpath "${1:?path to exported manifest.json}")
cd "$audit_project"
STATEMENT_BINDING_INPUT="$audit_input" lake env lean BindingProbe.lean
