import io
import json
import os
from pathlib import Path
from types import SimpleNamespace
import tempfile
import tarfile
import unittest
from unittest.mock import patch

import legacy_recovery as recovery
import criu_export


class FencingTests(unittest.TestCase):
    def test_expired_original_deadline_never_restores_with_a_fresh_budget(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=Path(tmp);real_stat=Path.stat
            def stat(path,*args,**kwargs):
                if path==root:return SimpleNamespace(st_uid=0,st_mode=0o40700)
                return real_stat(path,*args,**kwargs)
            args=SimpleNamespace(root=root,status=root/'status.json',table='table',origin='origin',
                owner_key='legacy-owner-test',host_owner_key='legacy-owner-host-test',
                pointer_key='pointer',criu=Path('/not-executed'),backup_timer='test.timer',once=True)
            index={'batch':'old-checkpoint','completed':[], 'objects':[{'pid':123}],
                'recovery':{'jobs':[{'pid':123}], 'contracts':{'123':{'deadline_utc':1}},
                            'prefix':'checkpoints/criu-validation/deadline'}}
            with patch.object(Path,'stat',stat),patch.object(recovery,'instance_id',return_value='successor'), \
                    patch.object(recovery,'acquire',return_value=True),patch.object(recovery,'api',return_value={}) as api, \
                    patch.object(recovery,'discover',return_value=index),patch.object(recovery,'restore') as restore, \
                    patch.object(recovery.subprocess,'run'),patch.object(criu_export,'upload_verified') as upload:
                recovery.run(args)
            restore.assert_not_called()
            self.assertEqual(json.loads((root/'status.json').read_text())['state'],'failed')
            self.assertEqual(json.loads((root/'terminal.json').read_text())['last_recoverable_batch'],'old-checkpoint')
            upload.assert_called_once()
            self.assertTrue(api.call_args.kwargs['expression_attribute_values'][':failed']['BOOL'])

    def test_confirmed_owner_death_survives_ec2_record_expiry(self):
        writes=[]
        def db(op,**kw):
            if op=='get-item':return {'Item':{'Owner':{'S':'previous'},'Origin':{'S':'origin'},
                'OriginTerminated':{'BOOL':True},'OwnerTerminated':{'BOOL':True}}}
            writes.append(kw);return {}
        def expired(**kw):self.fail('Previously confirmed deaths need no expiring EC2 record')
        self.assertTrue(recovery.acquire(db,expired,'table','owner-key','origin','successor'))
        self.assertNotIn('OwnerTerminated',writes[0]['item'])
        self.assertEqual(writes[0]['expression_attribute_values'][':prior'],{'S':'previous'})

    def test_source_cannot_restore_its_own_work(self):
        def forbidden(*args,**kw):self.fail('Source must not attempt to claim or restore')
        self.assertFalse(recovery.acquire(forbidden,forbidden,'table','owner-key','origin','origin'))

    def test_same_owner_preserves_ready_and_complete_receipts(self):
        def db(op,**kw):
            self.assertEqual(op,'get-item')
            return {'Item':{'Owner':{'S':'successor'},'Origin':{'S':'origin'},
                'OriginTerminated':{'BOOL':True},'Ready':{'BOOL':True}}}
        def forbidden(**kw):self.fail('Confirmed origin termination should not need another lookup')
        self.assertTrue(recovery.acquire(db,forbidden,'table','owner-key','origin','successor'))

    def test_running_or_unconfirmed_previous_owner_cannot_be_replaced(self):
        for state in ['running','stopped','shutting-down','pending','stopping']:
            writes=[]
            def db(op, **kw):
                if op=='get-item':return {'Item':{'Owner':{'S':'previous'},'Origin':{'S':'origin'}}}
                writes.append(kw);return {}
            ec2=lambda **kw:{'Reservations':[{'Instances':[{'State':{'Name':state}}]}]}
            self.assertFalse(recovery.acquire(db,ec2,'table','owner-key','origin','successor'))
            self.assertEqual(writes,[])

    def test_terminated_owner_can_be_replaced_only_with_compare_and_swap(self):
        writes=[]
        def db(op, **kw):
            if op=='get-item':return {'Item':{'Owner':{'S':'previous'},'Origin':{'S':'origin'},'OriginTerminated':{'BOOL':True}}}
            writes.append(kw);return {}
        ec2=lambda **kw:{'Reservations':[{'Instances':[{'State':{'Name':'terminated'}}]}]}
        self.assertTrue(recovery.acquire(db,ec2,'table','owner-key','origin','successor'))
        self.assertEqual(writes[0]['expression_attribute_values'][':prior'],{'S':'previous'})
        self.assertEqual(writes[0]['condition_expression'],'#owner = :prior')

    def test_expired_lease_is_never_a_recovery_permission(self):
        def db(op, **kw):
            if op=='get-item':return {'Item':{'Owner':{'S':'previous'},'Origin':{'S':'origin'},'Expires':{'N':'1'}}}
            self.fail('Tried to take ownership')
        ec2=lambda **kw:{'Reservations':[{'Instances':[{'State':{'Name':'running'}}]}]}
        self.assertFalse(recovery.acquire(db,ec2,'table','owner-key','origin','successor'))

    def test_complete_receipt_requires_all_expected_summaries_and_departed_process(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=Path(tmp);out=root/'stdout';err=root/'stderr';err.write_text('')
            contract={'stdout':str(out),'stderr':str(err),'shape':'random','summaries':{'0':2,'1':2}}
            job={'pid':123,'cwd':str(root)}
            out.write_text(json.dumps({'shape':'random','instance':0,'measured':2,'summary':True})+'\n')
            with patch('criu_local.same_process',return_value=False):
                self.assertIsNone(criu_export.completion(job,contract))
                with out.open('a') as f:f.write(json.dumps({'shape':'random','instance':1,'measured':2,'summary':True})+'\n')
                self.assertEqual(criu_export.completion(job,contract)['state'],'complete')
            with patch('criu_local.same_process',return_value=True):
                self.assertIsNone(criu_export.completion(job,contract))

    def test_archive_traversal_links_and_duplicate_members_are_rejected(self):
        import subprocess
        for name,kind in [('../outside','file'),('/outside','file'),('link','symlink'),('duplicate','duplicate')]:
            with tempfile.TemporaryDirectory() as tmp:
                root=Path(tmp);archive=root/'fixture.tar';zipped=root/'fixture.tar.zst'
                with tarfile.open(archive,'w') as t:
                    for _ in range(2 if kind=='duplicate' else 1):
                        item=tarfile.TarInfo(name)
                        if kind=='symlink':item.type=tarfile.SYMTYPE;item.linkname='/outside';t.addfile(item)
                        else:item.size=1;t.addfile(item,io.BytesIO(b'x'))
                subprocess.run(['zstd','-q',str(archive),'-o',str(zipped)],check=True)
                with patch.object(recovery.os,'chown'), self.assertRaises(ValueError):
                    recovery.extract(zipped,root/'extracted')

    def test_jsonl_completion_rejects_old_partial_and_live_outputs(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=Path(tmp);out=root/'stdout';out.write_text('human readable progress\n')
            terminal=root/'terminal.jsonl'
            contract={'kind':'jsonl-records','stdout':str(out),'stderr':str(out),
                'completion_file':str(terminal),'expected_records':52,'registered_utc':1000}
            job={'pid':123}
            terminal.write_text('{}\n'*52);os.utime(terminal,(999,999))
            with patch('criu_local.same_process',return_value=False):
                self.assertIsNone(criu_export.completion(job,contract))
                terminal.write_text('{}\n'*51)
                self.assertIsNone(criu_export.completion(job,contract))
                terminal.write_text('{}\n'*52)
                complete=criu_export.completion(job,contract)
                self.assertEqual(complete['state'],'complete')
                self.assertEqual(len(complete['outputs']),2)
            with patch('criu_local.same_process',return_value=True):
                self.assertIsNone(criu_export.completion(job,contract))


if __name__=='__main__':unittest.main()
