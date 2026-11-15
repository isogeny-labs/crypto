import ast
import copy
import datetime
import importlib.util
import json
import os
from pathlib import Path
import stat
import tempfile
import types
import unittest
from unittest.mock import patch, Mock
import private_contract as pc

HERE = Path(__file__).resolve().parent
NOW = datetime.datetime(2026,11,3,18,tzinfo=datetime.timezone.utc).timestamp()
def endpoint():
    return dict(schema='software.private-runtime-endpoint/v1', mode='qualification', owner='12345678-1234-1234-1234-123456789abc', socket_path='/private/run/docker.sock',socket_uid=0,socket_gid=0,socket_mode=0o600,socket_device=10,socket_inode=20,daemon_id='owned-daemon',daemon_pid=111,daemon_start_ticks=222,supervisor_pid=333,service_unit='owned.service',service_invocation_id='a'*32,service_cgroup='/system.slice/owned.service',cgroup_parent='/system.slice/owned.service/payload',source_binding_sha256='b'*64,image_id='sha256:'+'c'*64,input_manifest_sha256='d'*64,assembly_receipt_sha256='e'*64,expires_at='2026-11-03T18:05:00Z')
def binding():
    e=endpoint()
    return dict(private_owner=e['owner'],absolute_expiry='2026-11-04T06:00:00Z',final_source_manifest_sha256=e['source_binding_sha256'],image_id=e['image_id'],fixture_manifest_sha256=e['input_manifest_sha256'],private_assembly_receipt_sha256=e['assembly_receipt_sha256'])

class PrivateContractTests(unittest.TestCase):
    def test_valid_qualification_endpoint(self):
        self.assertEqual(pc.validate_endpoint(endpoint(),binding(),NOW),endpoint())
    def test_endpoint_rejections(self):
        for key,value in [('mode','assembly'),('socket_path','/var/run/docker.sock'),('socket_path','/run/docker.sock'),('socket_mode',0o666),('socket_uid',1000),('cgroup_parent','/system.slice/owned.service-other/payload'),('owner','00000000-0000-0000-0000-000000000000'),('source_binding_sha256','f'*64),('assembly_receipt_sha256','f'*64),('expires_at','2026-11-03T17:00:00Z'),('expires_at','2026-11-05T06:00:00Z')]:
            with self.subTest(key=key,value=value):
                e=endpoint();e[key]=value
                with self.assertRaises(pc.ContractError):pc.validate_endpoint(e,binding(),NOW)
    def test_endpoint_one_socket_for_both_transports(self):
        e=endpoint();cli=pc.docker_prefix('/pinned/docker','/empty',e);head=pc.archive_head_argv('/pinned/curl',e,'http://localhost/v1.45/containers/id/archive')
        self.assertEqual(cli[cli.index('--host')+1],'unix://'+e['socket_path'])
        self.assertEqual(head[head.index('--unix-socket')+1],e['socket_path'])
        self.assertNotIn('sudo',' '.join(cli+head))
    def test_socket_replacement_rejects(self):
        e=endpoint();fake=types.SimpleNamespace(st_mode=stat.S_IFSOCK|0o600,st_uid=0,st_gid=0,st_dev=10,st_ino=21)
        with patch.object(pc,'no_symlinks'),patch.object(Path,'lstat',return_value=fake):
            with self.assertRaisesRegex(pc.ContractError,'socket changed'):pc.socket_identity(e)
    def test_component_ancestry(self):
        self.assertTrue(pc.below('/a/unit/payload/id','/a/unit'))
        for child in ['/a/unit-other/id','/a/unit']:
            self.assertFalse(pc.below(child,'/a/unit'))
        for child in ['/a/unit/../escape','relative','/a//unit/id']:
            with self.assertRaises(pc.ContractError):pc.below(child,'/a/unit')
    def test_proc_identity_pid_reuse(self):
        def procstat(ticks):return '42 (gate with (spaces)) S '+'0 '*18+str(ticks)+' 0\n'
        with patch.object(Path,'read_text',side_effect=[procstat(1),'0::/a/unit/gate\n',procstat(2)]):
            with self.assertRaisesRegex(pc.ContractError,'identity changed'):pc.process_identity(42)
    def test_proc_identity_cpu_fields_can_change(self):
        first='42 (gate) S '+'0 '*18+'8 0\n';second=first.replace('S 0','R 1',1)
        with patch.object(Path,'read_text',side_effect=[first,'0::/a/unit/gate\n',second]):
            self.assertEqual(pc.process_identity(42)['start_ticks'],8)
    def test_host_ancestry_and_inode(self):
        with tempfile.TemporaryDirectory() as td:
            root=Path(td);e=endpoint();cg=e['cgroup_parent']+'/container';p=root/cg.lstrip('/');p.mkdir(parents=True)
            for name,value in [('cgroup.procs','42\n'),('pids.max','1\n'),('pids.current','1\n')]: (p/name).write_text(value)
            st=p.stat();gate=dict(cgroup_device=st.st_dev,cgroup_inode=st.st_ino,nonce=e['owner']);c=dict(Id='f'*64,State=dict(Pid=42,Running=True),HostConfig=dict(CgroupParent=e['cgroup_parent']))
            observed=dict(pid=42,start_ticks=77,cgroup=cg)
            with patch.object(pc,'process_identity',return_value=observed.copy()):
                self.assertEqual(pc.check_host_ancestry(e,c,gate,cgroup_root=root)['start_ticks'],77)
                wrong=dict(gate,cgroup_inode=st.st_ino+1)
                with self.assertRaisesRegex(pc.ContractError,'inode'):pc.check_host_ancestry(e,c,wrong,cgroup_root=root)
                (p/'pids.current').write_text('2\n')
                with self.assertRaisesRegex(pc.ContractError,'PID controller'):pc.check_host_ancestry(e,c,gate,cgroup_root=root)
            with patch.object(pc,'process_identity',return_value=dict(observed,cgroup='/outside/container')):
                with self.assertRaisesRegex(pc.ContractError,'escaped'):pc.check_host_ancestry(e,c,gate,cgroup_root=root)
    def test_held_gate_release_order_and_denial(self):
        tree=ast.parse((HERE/'supervise.py').read_text());main=next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name=='main');release=next(n for n in main.body if isinstance(n,ast.FunctionDef) and n.name=='release_gate')
        with tempfile.TemporaryDirectory() as td:
            order=[];nonce=endpoint()['owner'];container=dict(State=dict(Running=True),Id='f'*64);gate=dict(nonce=nonce)
            def cli(label,*args,**kw):
                order.append(label)
                return (0,(json.dumps(gate)+'\n').encode() if label.startswith('held-gate') else b'',None)
            def ancestry(*a):order.append('ancestry');return dict(pid=42,start_ticks=1,nonce=nonce)
            def require(ok,msg):
                if not ok:raise ValueError(msg)
            g=dict(require=require,identity=lambda:container,cli=cli,time=types.SimpleNamespace(monotonic=lambda:1,sleep=lambda _:None),active_deadline=20,gate_line=lambda *a:b'',strict=json.loads,endpoint=endpoint(),check_host_ancestry=ancestry,live_endpoint=lambda _:order.append('endpoint'),out=Path(td),P=Path,os=os,json=json,owned=container['Id'],host_preexec_observations=[])
            exec(compile(ast.Module(body=[release],type_ignores=[]),'extracted-production-release','exec'),g)
            g['release_gate']('Q1',nonce)
            self.assertEqual(order,['held-gate-logs-Q1','ancestry','ancestry','endpoint','release-Q1'])
            self.assertEqual((Path(td)/'release-Q1/gate-release').read_text(),nonce+'\n')
            self.assertEqual(len(g['host_preexec_observations']),1)
            order.clear()
            g['check_host_ancestry']=Mock(side_effect=pc.ContractError('escaped'))
            with self.assertRaises(pc.ContractError):g['release_gate']('Q2',nonce)
            self.assertFalse(any(x.startswith('release-') for x in order))
    def test_final_success_boundary(self):
        expiry='2026-11-03T18:05:00Z'
        self.assertTrue(pc.success_boundary(None,299,300,NOW,expiry))
        for signal,mono,wall in [(15,299,NOW),(None,300,NOW),(None,299,NOW+300)]:
            self.assertFalse(pc.success_boundary(signal,mono,300,wall,expiry))
    def test_partial_release_observation_static(self):
        gate=(HERE/'gate.c').read_text()
        # Native tests of delayed bytes/metadata remain mandatory and unexecuted.
        self.assertIn('if(value==1)return 1;',gate)
        self.assertNotIn('if(value)return value==1;',gate)
        self.assertIn('now.tv_sec-start.tv_sec>15',gate)
        self.assertIn('n==37&&memcmp(b,nonce,36)==0',gate)
    def test_original_controls_and_gate_order_static(self):
        source=(HERE/'supervise.py').read_text();gate=(HERE/'gate.c').read_text()
        ast.parse(source)
        for text in ["'--pids-limit','1'","'--memory','512m'","'--cpus','1'","rc==0 and not diff.strip()","passed==['Q0','Q1','Q2','Q3','Q4']","head_contract_sha256","active_deadline=time.monotonic()+20","deadline-30"]:self.assertIn(text,source)
        self.assertNotIn('/var/run/docker.sock',source)
        self.assertLess(gate.index('if(!await_release(work'),gate.index('execve(v[0]'))
        self.assertIn('preexisting release token',gate)
        self.assertIn('st.st_uid==0&&st.st_gid==0',gate)
        self.assertIn('st.st_size==37',gate)
        self.assertIn('sigaction(SIGTERM',gate)
        self.assertNotIn('fork(',gate)
        b=json.loads((HERE/'binding.template.json').read_text())
        self.assertIs(b['admitted'],False);self.assertIs(b['qualification_admitted'],False);self.assertIsNone(b['gate_binary_sha256'])

if __name__=='__main__':unittest.main(verbosity=2)
