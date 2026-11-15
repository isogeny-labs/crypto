from pathlib import Path
import subprocess,hashlib
p=Path('research/pencil_lifecycle_20261030/integration/followup-continuation-v2/signed-native-restarts')
print(subprocess.check_output(['python3',str(p.parent/'signed-scheduler/bind.py')],text=True),end='')
for f in [p/'Cargo.toml',p/'Cargo.lock',p/'tests/native.rs',p/'check.sh',p/'bind.py',p/'PLAN.md',p.parent/'signed-stream/src/lib.rs',p.parent/'signed-stream/Cargo.toml']:
 print(hashlib.sha256(f.read_bytes()).hexdigest()+'  '+str(f))
