#!/usr/bin/env python3
"""VALIDATOR instrumentation of examples/mt_fieldops.rs (mt_timed, frozen a992ed6c80).  Adds NO counted operation and
changes nothing before the output line: when MTFO_DUMP=<path> is set it writes {xs, qx, qy, found} (planted logs,
target points, recovered logs) as JSON to <path> after the shared verification pass.  Used only for the counter replay
and the independent EC check of recovered logs, never for timing.
usage: patch_dump.py <pristine main.rs> <out main.rs>"""
import sys
src = open(sys.argv[1]).read()
anchor = "    let phases: Vec<String> = snap"
assert src.count(anchor) == 1
dump = r'''    if let Ok(path) = std::env::var("MTFO_DUMP") {
        let js = serde_json::json!({
            "xs": xs.iter().map(|v| v.to_string()).collect::<Vec<_>>(),
            "qx": qs.iter().map(|q| q.map(|t| t.0.to_string()).unwrap_or_default()).collect::<Vec<_>>(),
            "qy": qs.iter().map(|q| q.map(|t| t.1.to_string()).unwrap_or_default()).collect::<Vec<_>>(),
            "found": found.iter().map(|v| v.to_string()).collect::<Vec<_>>(),
        });
        std::fs::write(path, js.to_string()).unwrap();
    }
'''
out = src.replace(anchor, dump + anchor)
open(sys.argv[2], "w").write(out)
print("patched", len(src), "->", len(out))
