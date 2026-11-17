#!/usr/bin/env python3
"""File I/O only: restore exact duplicate bytes; not a scientific verifier."""
from pathlib import Path
import json
import shutil
import sys

repository, validation, destination = map(Path, sys.argv[1:])
mapping = json.loads((validation / "DEHYDRATED.json").read_text())
for relative, source in mapping.items():
    target = destination / relative
    target.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(repository / source, target)
