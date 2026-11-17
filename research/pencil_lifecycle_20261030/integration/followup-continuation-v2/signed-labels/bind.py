from pathlib import Path
import hashlib,subprocess
p=Path('research/pencil_lifecycle_20261030/integration/followup-continuation-v2/signed-labels')
print(subprocess.check_output(['python3',str(p.parent/'signed-replay/bind.py')],text=True),end='')
files=[p/'Cargo.toml',p/'Cargo.lock',p/'src/lib.rs',p/'tests/saved.rs',p/'check.sh',p/'bind.py']
files+=sorted((p.parent/'signed-replay/fixtures').glob('*.jsonl'))
files+=sorted((p.parent/'signed-replay/fixtures').glob('*.header.json'))
for f in files:print(hashlib.sha256(f.read_bytes()).hexdigest()+'  '+str(f))
