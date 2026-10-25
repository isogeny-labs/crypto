#!/usr/bin/env python3
"""Wake the fixed SSH endpoint and pin its public key through authenticated AWS."""
import argparse
import json
import os
from pathlib import Path
import select
import socket
import sys
import time

from wake import invoke


def connect(config, host, port, known_hosts, timeout=900):
    if host != '3.134.144.201' or port != 22:
        raise ValueError('Proxy is restricted to the configured experiment endpoint')
    invoke(config,'wake')
    deadline=time.monotonic()+timeout
    while time.monotonic()<deadline:
        status=invoke(config,'status')
        if status.get('state')=='ready':
            key=invoke(config,'ssh-key')
            if key.get('state')=='ready' and key['instance_id']==status['endpoint_instance_id']:
                fields=key['host_key'].split()
                if len(fields)!=2 or fields[0]!='ssh-ed25519' or '\n' in key['host_key']:
                    raise ValueError('Unexpected SSH public key')
                known_hosts.parent.mkdir(parents=True,exist_ok=True)
                temporary=known_hosts.with_suffix('.tmp.'+str(os.getpid()))
                temporary.write_text(host+' '+key['host_key']+'\n')
                temporary.chmod(0o600);os.replace(temporary,known_hosts)
                try:
                    return socket.create_connection((host,port),timeout=10)
                except OSError:
                    pass
        time.sleep(3)
    raise TimeoutError('Spot capacity or SSH endpoint did not become ready')


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--config',type=Path,required=True)
    p.add_argument('--known-hosts',type=Path,required=True)
    p.add_argument('host');p.add_argument('port',type=int)
    args=p.parse_args()
    try:
        config=json.loads(args.config.read_text())
        with connect(config,args.host,args.port,args.known_hosts) as remote:
            remote.settimeout(None)
            inputs=[remote,sys.stdin.buffer]
            while remote in inputs:
                readable,_,_=select.select(inputs,[],[])
                for current in readable:
                    if current is remote:
                        data=remote.recv(65536)
                        if not data:return 0
                        sys.stdout.buffer.write(data);sys.stdout.buffer.flush()
                    else:
                        data=os.read(0,65536)
                        if data:remote.sendall(data)
                        else:remote.shutdown(socket.SHUT_WR);inputs.remove(sys.stdin.buffer)
    except (OSError,ValueError,KeyError,RuntimeError) as exc:
        print('spot-system SSH: '+str(exc),file=sys.stderr);return 1
    return 0


if __name__=='__main__':raise SystemExit(main())
