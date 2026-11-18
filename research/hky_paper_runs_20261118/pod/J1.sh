#!/bin/bash
# J1: unit-p65537-a1-s10-k14, D = 6 (PROVISIONAL binary: the final pod binary is rebuilt after the optimizer merge and
# re-gated; replace BINARY.sha256 then). Instance file copied to /workspace/instances/.
exec "$(dirname "$0")/run_job.sh" J1-unit-p65537-a1-s10-k14-D6 /workspace/instances/unit-p65537-a1-s10-k14.json 6
