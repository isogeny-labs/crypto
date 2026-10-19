#!/usr/bin/env python3
"""Local client hook: IAM invoke only; never send prompt/transcript contents."""
import argparse
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import time


def invoke(config, action):
    for attempt in range(config.get('attempts',5)):
        with tempfile.NamedTemporaryFile() as output:
            command = ['aws', '--region', config['region'], '--cli-connect-timeout', '3',
                       '--cli-read-timeout', '10']
            if config.get('profile'):
                command += ['--profile', config['profile']]
            command += ['lambda', 'invoke', '--function-name', config['function'],
                        '--cli-binary-format', 'raw-in-base64-out',
                        '--payload', json.dumps({'action': action}), output.name]
            result = subprocess.run(command, capture_output=True, text=True, timeout=18)
            if result.returncode:
                raise RuntimeError('AWS wake request failed; check the client AWS profile and InvokeFunction permission')
            meta = json.loads(result.stdout)
            if meta.get('FunctionError'):
                raise RuntimeError('Spot lifecycle function failed; inspect its CloudWatch logs')
            response = json.loads(Path(output.name).read_text())
        if response.get('state') != 'busy':
            return response
        time.sleep(.5)
    raise RuntimeError('Lifecycle controller busy; retry the wake request')


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--config', type=Path, default=Path.home()/'.config/spot-system/client.json')
    p.add_argument('--action', choices=['wake', 'release', 'status', 'ssh-key'], default='wake')
    p.add_argument('--hook', action='store_true')
    p.add_argument('--wait', type=int, default=0, metavar='SECONDS')
    a = p.parse_args()
    try:
        config = json.loads(a.config.expanduser().read_text())
        if a.hook:
            config['attempts'] = 1
        response = invoke(config, a.action)
        if a.wait and a.action == 'wake' and response.get('state') != 'staged':
            deadline = time.monotonic() + a.wait
            while time.monotonic() < deadline:
                response = invoke(config, 'status')
                if response.get('state') == 'ready':
                    break
                time.sleep(5)
            else:
                raise RuntimeError('Spot capacity/endpoint is not ready within the wait limit')
        if a.hook:
            if response.get('state') == 'staged' and a.action == 'wake':
                print('spot-system: startup is staged pending workload migration; no worker requested', file=sys.stderr)
        else:
            print(json.dumps(response))
        return 0
    except (OSError, ValueError, KeyError, RuntimeError, subprocess.SubprocessError) as exc:
        print('spot-system: '+str(exc), file=sys.stderr)
        return 0 if a.hook else 1


if __name__ == '__main__':
    raise SystemExit(main())
