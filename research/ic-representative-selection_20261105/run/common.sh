# Shared paths for the run scripts (sourced). Run every script from the repository root.
set -euo pipefail
D=research/ic-representative-selection_20261105
REPSEL=$D/native/target/release/repsel
VOLCANO_RHO=./target/release/volcano-rho
ECBENCH=./target/release/ecbench
DESIGN=$D/design.json
