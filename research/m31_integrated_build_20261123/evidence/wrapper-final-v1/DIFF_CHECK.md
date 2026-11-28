# Whitespace observation

The source and prose diff passes its scoped whitespace check. The full staged
diff returns 2 for trailing spaces in the shell-escaped raw command captures
and final blank lines in Cargo's raw stdout/stderr. Those receipt-bound bytes
are retained unchanged. See diff-check.log for the exact paths; no full-diff
whitespace pass is claimed.
