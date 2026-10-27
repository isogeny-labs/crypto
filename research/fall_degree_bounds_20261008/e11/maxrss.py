#!/usr/bin/env python3
"""Run a command and write its peak RSS (kB, ru_maxrss of the child) to a file.
Usage: maxrss.py RSS_FILE CMD [ARGS...]   (stdout/stderr pass through)"""
import resource
import subprocess
import sys

rc = subprocess.call(sys.argv[2:])
with open(sys.argv[1], "w") as f:
    f.write(str(resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss))
sys.exit(rc)
