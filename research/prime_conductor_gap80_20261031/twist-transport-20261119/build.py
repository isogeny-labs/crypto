#!/usr/bin/env python3
"""Build orchestration only; preserve external library artifacts."""
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
OUT = Path('/tmp/conductor-twist-20261119/complete_twisted_pipeline')
LIB = '/workspace/tools/conductor-native/usr/lib/x86_64-linux-gnu'
SOURCE = 'research/prime_conductor_gap80_20261031/native/complete_twisted_pipeline.cpp'
argv = ['g++', '-std=c++17', '-O3', '-Wall', '-Wextra', '-Werror',
        '-isystem', '/workspace/tools/conductor-native/usr/include',
        '-isystem', '/workspace/tools/conductor-native/usr/include/x86_64-linux-gnu',
        SOURCE, '/tmp/conductor-native-build/optimized.o', '-L' + LIB,
        '-Wl,-rpath,' + LIB, '-lpari', '-lflint', '-lgmpxx', '-lgmp', '-o', str(OUT)]
attempt = HERE / 'build-b'
attempt.mkdir()
(attempt/'command.json').write_text(json.dumps({'argv': argv}, indent=2)+'\n')
paths = [ROOT/SOURCE, ROOT/SOURCE.replace('complete_twisted_pipeline.cpp', 'projective_walk.hpp'),
         ROOT/SOURCE.replace('complete_twisted_pipeline.cpp', 'prepared_norm.hpp'),
         ROOT/SOURCE.replace('complete_twisted_pipeline.cpp', 'optimized.c'),
         Path('/tmp/conductor-native-build/optimized.o')]
(attempt/'inputs.sha256').write_text(''.join(hashlib.sha256(p.read_bytes()).hexdigest()+'  '+str(p)+'\n' for p in paths))
with (attempt/'stdout').open('w') as out, (attempt/'stderr').open('w') as err:
    result = subprocess.run(argv, cwd=ROOT, stdout=out, stderr=err)
(attempt/'exit').write_text(str(result.returncode)+'\n')
if result.returncode == 0:
    (attempt/'binary.sha256').write_text(hashlib.sha256(OUT.read_bytes()).hexdigest()+'  '+str(OUT)+'\n')
raise SystemExit(result.returncode)
