from pathlib import Path
import hashlib,subprocess
p=Path('research/pencil_lifecycle_20261030/integration/followup-continuation-v2/signed-exception-controls')
print(subprocess.check_output(['python3',str(p.parent/'signed-labels/bind.py')],text=True),end='')
for f in [p/'Cargo.toml',p/'Cargo.lock',p/'tests/controls.rs',p/'check.sh',p/'bind.py']:
 print(hashlib.sha256(f.read_bytes()).hexdigest()+'  '+str(f))
