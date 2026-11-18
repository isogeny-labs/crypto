#!/bin/bash
# J4: unit-p786433-a1-s10-k16, D = 6 (PROVISIONAL binary: the final pod binary is rebuilt after the optimizer merge and
# re-gated; replace BINARY.sha256 then). Instance file copied to /workspace/instances/.
exec "$(dirname "$0")/run_job.sh" J4-unit-p786433-a1-s10-k16-D6 /workspace/instances/unit-p786433-a1-s10-k16.json 6
