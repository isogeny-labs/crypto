#!/bin/bash
# J3: unit-p65537-a1-s10-k15, D = 6 (binary: pod/BINARY.sha256, the merged engine 9fb60e9c13, gated in README section 3).
# Instance file copied to /workspace/instances/.
exec "$(dirname "$0")/run_job.sh" J3-unit-p65537-a1-s10-k15-D6 /workspace/instances/unit-p65537-a1-s10-k15.json 6
