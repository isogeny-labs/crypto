#!/usr/bin/env python3
"""Merge opt-in user hooks on the machine running the coding client."""
import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
import shlex
import shutil


def install(home, function, region='us-east-2', profile=None):
    runtime = home / '.local/share/spot-system'
    config_path = home / '.config/spot-system/client.json'
    # Validate all existing documents before making the first mutation.
    documents = {}
    for relative in ['.claude/settings.json', '.codex/hooks.json']:
        path = home / relative
        data = json.loads(path.read_text()) if path.exists() else {}
        if not isinstance(data, dict) or not isinstance(data.get('hooks', {}), dict):
            raise ValueError('Invalid existing hook document: '+str(path))
        for event in ['SessionStart', 'UserPromptSubmit', 'Stop', 'SessionEnd']:
            if not isinstance(data.get('hooks', {}).get(event, []), list):
                raise ValueError('Invalid existing hook event: '+event)
        documents[relative] = data
    if config_path.exists():
        current = json.loads(config_path.read_text())
        if current.get('function') != function or current.get('region') != region:
            raise ValueError('Existing client points at another deployment; review before replacing')
    ssh=home/'.ssh/config'
    ssh_existing=ssh.read_text() if ssh.exists() else ''
    start='# BEGIN spot-system managed SSH\n';end='# END spot-system managed SSH\n'
    if start in ssh_existing:
        before,rest=ssh_existing.split(start,1)
        if end not in rest:raise ValueError('Incomplete existing Spot SSH block')
        ssh_existing=before+rest.split(end,1)[1]
    runtime.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(Path(__file__).with_name('wake.py'), runtime/'wake.py')
    shutil.copyfile(Path(__file__).with_name('ssh_proxy.py'), runtime/'ssh_proxy.py')
    config_path.parent.mkdir(parents=True, exist_ok=True)
    config_path.write_text(json.dumps({'function': function, 'region': region, 'profile': profile}, indent=2)+'\n')
    command = 'python3 '+shlex.quote(str(runtime/'wake.py'))+' --config '+shlex.quote(str(config_path))
    for relative in ['.claude/settings.json', '.codex/hooks.json']:
        path = home / relative
        data = documents[relative]
        original = json.dumps(data, sort_keys=True)
        hooks = data.setdefault('hooks', {})
        for event, action in [('SessionStart','wake'), ('UserPromptSubmit','wake'),
                              ('Stop','release'), ('SessionEnd','release')]:
            item = {'hooks': [{'type': 'command', 'command': command+' --hook --action '+action, 'timeout': 3 if event=='SessionEnd' else 25}]}
            existing = hooks.setdefault(event, [])
            if item not in existing:
                existing.append(item)
        if original != json.dumps(data, sort_keys=True):
            path.parent.mkdir(parents=True, exist_ok=True)
            if path.exists():
                shutil.copyfile(path, str(path)+'.spot-backup-'+datetime.now(timezone.utc).strftime('%Y%m%d%H%M%S%f'))
            path.write_text(json.dumps(data, indent=2)+'\n')
    plugin = home / '.config/opencode/plugins/spot-system.js'
    plugin.parent.mkdir(parents=True, exist_ok=True)
    code = '''import { execFile } from "node:child_process";
import { promisify } from "node:util";
const execute = promisify(execFile);
const wake = async (action) => {
  try { await execute("python3", [SCRIPT, "--config", CONFIG, "--action", action, "--hook"], {timeout: 24000}); }
  catch (error) { console.error("spot-system: wake hook failed; check local AWS access"); }
};
export const SpotSystem = async () => ({
  event: async ({event}) => {
    if (event.type === "session.created") await wake("wake");
    if (event.type === "session.idle" || event.type === "session.deleted") await wake("release");
    if (event.type === "message.updated" && event.properties?.info?.role === "user") await wake("wake");
  }
});
'''.replace('SCRIPT', json.dumps(str(runtime/'wake.py'))).replace('CONFIG', json.dumps(str(config_path)))
    if plugin.exists() and plugin.read_text() != code:
        shutil.copyfile(plugin, str(plugin)+'.spot-backup-'+datetime.now(timezone.utc).strftime('%Y%m%d%H%M%S%f'))
    plugin.write_text(code)
    known=home/'.config/spot-system/known_hosts'
    proxy='python3 '+shlex.quote(str(runtime/'ssh_proxy.py'))+' --config '+shlex.quote(str(config_path))+' --known-hosts '+shlex.quote(str(known))+' %h %p'
    block=start+'Host 3.134.144.201\n  ProxyCommand '+proxy+'\n  UserKnownHostsFile '+json.dumps(str(known))+'\n  StrictHostKeyChecking yes\nHost *\n'+end
    ssh.parent.mkdir(parents=True,exist_ok=True)
    if ssh.exists():shutil.copyfile(ssh,str(ssh)+'.spot-backup-'+datetime.now(timezone.utc).strftime('%Y%m%d%H%M%S%f'))
    ssh.write_text(block+ssh_existing);ssh.chmod(0o600)
    print('Hooks prepared in '+str(home)+'. Review Codex /hooks to trust the exact definitions.')


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    p.add_argument('--home', type=Path, default=Path.home())
    p.add_argument('--function', required=True)
    p.add_argument('--region', default='us-east-2')
    p.add_argument('--profile')
    a = p.parse_args()
    install(a.home.expanduser(), a.function, a.region, a.profile)
