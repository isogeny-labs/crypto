#!/usr/bin/env python3
"""Assemble exact recorder function/tests in an isolated Rust software-test crate.
No cryptographic or mathematical computation is performed by this assembler.
"""
from pathlib import Path
import hashlib,sys
source=Path(__file__).resolve().parents[2]/'native/src/round22_record.rs'
text=source.read_text()
function=text[text.index('fn test_counts('):text.index('\npub fn record(')]
tests=text[text.index('#[cfg(test)]\nmod seal_tests {'):]
out=Path(sys.argv[1]);(out/'src').mkdir(parents=True,exist_ok=True)
(out/'Cargo.toml').write_text('[package]\nname="koblitz-recorder-regression"\nversion="0.1.0"\nedition="2021"\n[workspace]\n[dependencies]\nserde_json="1"\n')
(out/'src/lib.rs').write_text('use serde_json::json;\n'+function+'\n'+tests)
print('source_sha256='+hashlib.sha256(source.read_bytes()).hexdigest())
print('Exact test_counts function and seal_tests module extracted; all other study/root code excluded.')
