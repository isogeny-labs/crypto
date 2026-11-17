from pathlib import Path
import subprocess,hashlib
p=Path('research/pencil_lifecycle_20261030/integration/followup-continuation-v2/signed-scheduler')
print(subprocess.check_output(['python3',str(p.parent/'signed-labels/bind.py')],text=True),end='')
for f in [p/'Cargo.toml',p/'Cargo.lock',p/'src/lib.rs',p/'tests/scheduler.rs',p/'check.sh',p/'bind.py']:
 print(hashlib.sha256(f.read_bytes()).hexdigest()+'  '+str(f))
