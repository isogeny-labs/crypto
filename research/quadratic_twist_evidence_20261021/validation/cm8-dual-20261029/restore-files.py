#!/usr/bin/env python3
"""File I/O only: restore exact duplicate bytes; not a scientific verifier."""
from pathlib import Path
import json
import shutil
import subprocess
import sys

repository, validation, destination = map(Path, sys.argv[1:])
mapping = json.loads((validation / "DEHYDRATED.json").read_text())
for relative, source in mapping.items():
    target = destination / relative
    target.parent.mkdir(parents=True, exist_ok=True)
    if source.startswith('git:'):
        target.write_bytes(subprocess.check_output(['git','show',source[4:]],cwd=repository))
    else:
        shutil.copyfile(repository / source, target)
