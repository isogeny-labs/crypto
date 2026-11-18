#!/usr/bin/env python3
"""Pod job driver: the control server starts only python3 jobs, so this runs one shell script
from /workspace/code and passes its exit code through."""
import subprocess, sys
sys.exit(subprocess.call(["bash", "/workspace/code/" + sys.argv[1]] + sys.argv[2:]))
