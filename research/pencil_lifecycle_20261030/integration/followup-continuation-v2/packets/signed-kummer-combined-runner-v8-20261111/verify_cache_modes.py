#!/usr/bin/env python3
"""Read-only validation of the frozen donor cache mode inventory."""
from pathlib import Path
import hashlib
import json
import stat

P = Path(__file__).resolve().parent
inventory_path = P / "cache-mode-inventory.json"
inventory = json.loads(inventory_path.read_text())
manifest_path = Path(inventory["cache_manifest"]["path"])
manifest_bytes = manifest_path.read_bytes()
assert hashlib.sha256(manifest_bytes).hexdigest() == inventory["cache_manifest"]["sha256"]
manifest = json.loads(manifest_bytes)
assert inventory["schema"] == "pencil.cache-mode-inventory/v1"
assert inventory["donor_roots"] == manifest["roots"]
assert inventory["path_order"] == "lexicographic_unicode_codepoint_absolute_paths"
paths = sorted(manifest["files"])
assert len(paths) == inventory["file_count"] == 2165
assert inventory["mode_table"] == ["0600", "0644", "0700", "0755"]
indices = inventory["mode_index_nibbles"]
assert len(indices) == len(paths) and set(indices) <= set("0123")
counts = {name: 0 for name in inventory["mode_table"]}
executables = []
for index, (path, nibble) in enumerate(zip(paths, indices)):
    mode = int(inventory["mode_table"][int(nibble, 16)], 8)
    st = Path(path).lstat()
    assert stat.S_ISREG(st.st_mode), path
    assert stat.S_IMODE(st.st_mode) == mode, path
    assert mode & 0o7000 == 0, path
    counts[format(mode, "04o")] += 1
    if mode & 0o111:
        executables.append(index)
assert counts == inventory["mode_counts"]
assert executables == inventory["executable_file_indices"]
assert len(executables) == inventory["executable_file_count"] == 82
assert inventory["special_permission_bits_allowed"] is False
assert inventory["file_types_allowed"] == ["regular"]
print(json.dumps({
    "schema": "pencil.cache-mode-inventory-check/v1",
    "all_regular": True,
    "all_modes_match": True,
    "special_bits": 0,
    "files": len(paths),
    "executable_files": len(executables),
    "inventory_sha256": hashlib.sha256(inventory_path.read_bytes()).hexdigest(),
}, sort_keys=True))
