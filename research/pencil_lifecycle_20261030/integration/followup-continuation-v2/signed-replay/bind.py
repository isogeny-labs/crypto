from pathlib import Path
import hashlib,subprocess
p=Path('research/pencil_lifecycle_20261030/integration/followup-continuation-v2/signed-replay')
print(subprocess.check_output(['python3',str(p.parent/'signed-stream/bind.py')],text=True),end='')
for f in [p/'Cargo.toml',p/'Cargo.lock',p/'src/lib.rs',p/'src/main.rs',p/'tests/native.rs']:
 print(hashlib.sha256(f.read_bytes()).hexdigest()+'  '+str(f))
