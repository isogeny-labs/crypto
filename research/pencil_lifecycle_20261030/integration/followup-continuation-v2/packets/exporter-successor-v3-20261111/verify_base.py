#!/usr/bin/env python3
"""Read-only prerequisite checker; no checkout, mutation, build or tests."""
import argparse,json,hashlib
from pathlib import Path
p=Path(__file__).resolve().parent
ap=argparse.ArgumentParser();ap.add_argument('source_root');a=ap.parse_args();root=Path(a.source_root)
bind=json.loads((p/'source-bindings.json').read_text())
for group in ['existing_cost_mapper','existing_native_schemas']:
 for name,digest in bind[group].items():
  actual=name.removesuffix('_after_predecessor');f=root/actual
  if not f.is_file() or f.is_symlink() or hashlib.sha256(f.read_bytes()).hexdigest()!=digest:
   raise SystemExit('unreviewed integration base: '+actual)
print('exact donor/schema base bindings match; compilation still unverified')
