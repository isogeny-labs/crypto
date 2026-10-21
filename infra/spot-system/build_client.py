#!/usr/bin/env python3
"""Package a workstation installer containing no AWS credentials or server keys."""
import argparse
import io
import json
from pathlib import Path
import tarfile


def build(function, destination, region='us-east-2'):
    here = Path(__file__).resolve().parent
    files = {name: (here/name).read_bytes() for name in ['install_hooks.py', 'wake.py', 'ssh_proxy.py', 'install_client.sh']}
    files['deployment.json'] = json.dumps({'function': function, 'region': region}, indent=2).encode()
    files['caller-policy.json'] = json.dumps({'Version':'2012-10-17', 'Statement':[
        {'Effect':'Allow', 'Action':'lambda:InvokeFunction', 'Resource':function}]}, indent=2).encode()
    files['setup.py'] = b'''#!/usr/bin/env python3
import argparse,json
from pathlib import Path
from install_hooks import install
p=argparse.ArgumentParser()
p.add_argument('--home',type=Path,default=Path.home())
p.add_argument('--profile')
a=p.parse_args()
d=json.loads(Path(__file__).with_name('deployment.json').read_text())
install(a.home.expanduser(),d['function'],d['region'],a.profile)
print('AWS CLI and local InvokeFunction access are required. Review Codex /hooks before expecting it to run.')
'''
    with tarfile.open(destination, 'w:gz') as archive:
        for name, data in files.items():
            entry = tarfile.TarInfo('spot-client/'+name)
            entry.mode = 0o644
            entry.size = len(data)
            archive.addfile(entry, io.BytesIO(data))
    print('Client installer:', destination)


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    p.add_argument('--function', required=True)
    p.add_argument('--region', default='us-east-2')
    p.add_argument('--output', type=Path, default=Path(__file__).with_name('spot-client.tar.gz'))
    a = p.parse_args()
    build(a.function, a.output, a.region)
