#!/usr/bin/env python3
"""Recover one raw N83 artifact through the RunPod PTY gateway."""

import argparse
import base64
import hashlib
import subprocess
from pathlib import Path


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--user', required=True)
    parser.add_argument('--remote', required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--sha256')
    args = parser.parse_args()
    if not (args.remote == '/workspace/raw_parts_manifest.json' or (args.remote.startswith('/workspace/raw.part.') and args.remote.removeprefix('/workspace/raw.part.').isdigit())):
        raise ValueError('remote path must be an N83 screen raw part or manifest')
    if any(char in args.remote for char in "'\n\r"):
        raise ValueError('invalid remote path')
    if args.output.exists():
        raise FileExistsError(args.output)
    begin, end = b'N83_RAW_BEGIN_7b8e6a2f', b'N83_RAW_END_7b8e6a2f'
    ssh = [
        'ssh', '-tt', '-i', '/Users/adamburan/.ssh/id_ed25519',
        '-o', 'IdentitiesOnly=yes', '-o', 'BatchMode=yes',
        '-o', 'ConnectTimeout=8', '-o',
        'UserKnownHostsFile=/private/tmp/n83_runpod_known_hosts',
        '-o', 'StrictHostKeyChecking=accept-new',
        f'{args.user}@ssh.runpod.io',
    ]
    command = ("stty -echo -ixon\nprintf N83_RAW_BEGIN_7b8e6a2f; "
               f"base64 -w0 '{args.remote}'; "
               "printf N83_RAW_END_7b8e6a2f\nexit\n")
    capture = args.output.with_suffix(args.output.suffix + '.pty')
    if capture.exists():
        raise FileExistsError(capture)
    with capture.open('wb') as stream:
        status = subprocess.run(ssh, input=command.encode(), stdout=stream,
                                stderr=subprocess.STDOUT, check=False)
    if status.returncode:
        raise RuntimeError(f'gateway exit {status.returncode}; capture {capture}')
    raw = capture.read_bytes()
    start = raw.rfind(begin)
    finish = raw.find(end, start + len(begin)) if start >= 0 else -1
    if start < 0 or finish < 0:
        raise ValueError(f'gateway markers missing; capture {capture}')
    encoded = raw[start + len(begin):finish]
    result = base64.b64decode(encoded, validate=True)
    digest = hashlib.sha256(result).hexdigest()
    if args.sha256 and digest != args.sha256:
        raise ValueError(f'digest mismatch: {digest} != {args.sha256}')
    args.output.write_bytes(result)
    capture.unlink()
    print(f'{args.output} bytes={len(result)} sha256={digest}')


if __name__ == '__main__':
    main()
