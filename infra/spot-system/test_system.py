import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time
import unittest
from unittest.mock import Mock, patch
from types import SimpleNamespace

from build_template import build
from controller import handler, reconcile, retain_confirmed_termination
from runner import read_jobs
from install_services import discovery_units, units


class AddressTests(unittest.TestCase):
    def test_release_retains_successor_death_without_claiming_source_terminated(self):
        db,ec2=Mock(),Mock()
        ec2.describe_instances.return_value={'Reservations':[{'Instances':[{'State':{'Name':'terminated'}}]}]}
        record={'Owner':{'S':'previous'},'Origin':{'S':'source'},
                'OriginReleased':{'BOOL':True},'ReleaseReceipt':{'S':'a'*64}}
        retain_confirmed_termination(db,ec2,'table','source',{'legacy-owner':record})
        ec2.describe_instances.assert_called_once_with(InstanceIds=['previous'])
        self.assertTrue(record['OwnerTerminated']['BOOL'])
        self.assertNotIn('OriginTerminated',record)

    def test_released_source_still_requires_ready_matching_recovery_owner(self):
        owner={'Origin':{'S':'source'},'OriginReleased':{'BOOL':True},
               'ReleaseReceipt':{'S':'a'*64},'Owner':{'S':'i-new'},'Ready':{'BOOL':True}}
        self.ec2.describe_instances.return_value['Reservations'][0]['Instances'][1]['Tags'].append({'Key':'LegacyReady','Value':'true'})
        result=reconcile(self.ec2,self.asg,'g','e',True,'source',owner)
        self.assertEqual(result['instance'],'i-new')
        self.asg.set_desired_capacity.assert_called_once()

    def test_partial_release_never_moves_protected_source_address(self):
        self.ec2.describe_instances.return_value={'Reservations':[{'Instances':[{'State':{'Name':'running'}}]}]}
        released={'Origin':{'S':'source'},'OriginReleased':{'BOOL':True},'ReleaseReceipt':{'S':'a'*64}}
        result=reconcile(self.ec2,self.asg,'g','e',True,'source',[released,{'Origin':{'S':'source'}}])
        self.assertEqual(result['state'],'protected-source-active')
        self.ec2.associate_address.assert_not_called()

    def test_only_confirmed_termination_is_recorded_with_current_owner_condition(self):
        for state in ['running','stopped','shutting-down','terminated']:
            db,ec2=Mock(),Mock()
            ec2.describe_instances.return_value={'Reservations':[{'Instances':[{'State':{'Name':state}}]}]}
            record={'Owner':{'S':'source'},'Origin':{'S':'source'},'Complete':{'BOOL':True}}
            retain_confirmed_termination(db,ec2,'table','source',{'legacy-owner':record})
            if state!='terminated':db.update_item.assert_not_called()
            else:
                call=db.update_item.call_args.kwargs
                self.assertEqual(call['ConditionExpression'],'#owner = :owner AND #origin = :origin')
                self.assertNotIn('Complete',call['UpdateExpression'])
                self.assertTrue(record['OwnerTerminated']['BOOL'])
                self.assertTrue(record['Complete']['BOOL'])

    def test_unconfirmed_absence_never_records_death(self):
        db,ec2=Mock(),Mock()
        ec2.describe_instances.return_value={'Reservations':[]}
        with self.assertRaises(RuntimeError):
            retain_confirmed_termination(db,ec2,'table','source',{'legacy-owner':{'Owner':{'S':'source'}}})
        db.update_item.assert_not_called()

    def test_durable_origin_and_owner_deaths_need_no_ec2_lookup(self):
        db,ec2=Mock(),Mock()
        record={'Owner':{'S':'previous'},'Origin':{'S':'source'},
                'OriginTerminated':{'BOOL':True},'OwnerTerminated':{'BOOL':True}}
        retain_confirmed_termination(db,ec2,'table','source',{'legacy-owner':record})
        ec2.describe_instances.assert_not_called()
        db.update_item.assert_not_called()

    def setUp(self):
        self.ec2, self.asg = Mock(), Mock()
        self.asg.describe_auto_scaling_groups.return_value = {"AutoScalingGroups": [{"Instances": [
            {"InstanceId": "i-old", "LifecycleState": "InService", "HealthStatus": "Healthy"},
            {"InstanceId": "i-new", "LifecycleState": "InService", "HealthStatus": "Healthy"}]}]}
        self.ec2.describe_instances.return_value = {"Reservations": [{"Instances": [
            {"InstanceId": "i-old", "LaunchTime": 1, "State": {"Name": "running"},
             "Tags": [{"Key": "SpotReady", "Value": "true"}]},
            {"InstanceId": "i-new", "LaunchTime": 2, "State": {"Name": "running"},
             "Tags": [{"Key": "SpotReady", "Value": "true"}]}]}]}
        self.ec2.describe_instance_status.return_value = {"InstanceStatuses": [
            {"InstanceId": i, "SystemStatus": {"Status": "ok"}, "InstanceStatus": {"Status": "ok"}}
            for i in ("i-old", "i-new")]}
        self.ec2.describe_addresses.return_value = {"Addresses": [{"InstanceId": "i-old"}]}

    def test_staged_never_moves_address(self):
        self.assertEqual(reconcile(self.ec2, self.asg, "g", "e", False)["state"], "staged")
        self.ec2.associate_address.assert_not_called()
        self.asg.describe_auto_scaling_groups.assert_not_called()

    def test_ready_replacement_receives_address(self):
        self.assertEqual(reconcile(self.ec2, self.asg, "g", "e", True)["instance"], "i-new")
        self.ec2.associate_address.assert_called_once_with(
            AllocationId="e", InstanceId="i-new", AllowReassociation=True)

    def test_unready_replacement_cannot_steal_address(self):
        self.ec2.describe_instances.return_value["Reservations"][0]["Instances"][1]["Tags"] = []
        self.assertEqual(reconcile(self.ec2, self.asg, "g", "e", True)["state"], "attached")
        self.ec2.associate_address.assert_not_called()

    def test_unhealthy_replacement_cannot_steal_address(self):
        self.ec2.describe_instance_status.return_value["InstanceStatuses"][1]["SystemStatus"]["Status"] = "impaired"
        self.assertEqual(reconcile(self.ec2, self.asg, "g", "e", True)["state"], "attached")

    def test_no_capacity_keeps_existing_address(self):
        self.asg.describe_auto_scaling_groups.return_value["AutoScalingGroups"][0]["Instances"] = []
        self.assertEqual(reconcile(self.ec2, self.asg, "g", "e", True)["state"], "waiting-for-capacity")
        self.ec2.associate_address.assert_not_called()

    def test_protected_running_source_keeps_ip_even_with_ready_replacements(self):
        self.ec2.describe_instances.return_value = {'Reservations':[{'Instances':[{'InstanceId':'source','State':{'Name':'running'}}]}]}
        result = reconcile(self.ec2,self.asg,'g','e',True,'source',{})
        self.assertEqual(result['state'],'protected-source-active')
        self.ec2.associate_address.assert_not_called()
        self.asg.set_desired_capacity.assert_not_called()

    def test_terminated_source_requires_matching_recovery_owner_and_readiness(self):
        ownership={'OriginTerminated':{'BOOL':True},'Owner':{'S':'i-old'},'Ready':{'BOOL':True}}
        self.ec2.describe_instances.return_value['Reservations'][0]['Instances'][0]['Tags'].append({'Key':'LegacyReady','Value':'true'})
        result=reconcile(self.ec2,self.asg,'g','e',True,'source',ownership)
        self.assertEqual(result['instance'],'i-old')
        self.ec2.associate_address.assert_not_called()
        self.asg.set_desired_capacity.assert_called_once()

    def test_legacy_not_ready_cannot_receive_ip(self):
        ownership={'OriginTerminated':{'BOOL':True},'Owner':{'S':'i-new'}}
        self.assertEqual(reconcile(self.ec2,self.asg,'g','e',True,'source',ownership)['state'],'waiting-for-bootstrap')
        self.ec2.associate_address.assert_not_called()

    def test_every_incomplete_namespace_must_be_ready_on_same_worker(self):
        for instance in self.ec2.describe_instances.return_value['Reservations'][0]['Instances']:
            instance['Tags'].append({'Key':'LegacyReady','Value':'true'})
        def owner(instance,ready=True):
            return {'OriginTerminated':{'BOOL':True},'Owner':{'S':instance},'Ready':{'BOOL':ready}}
        for owners in [[owner('i-old'),owner('i-new')], [owner('i-new'),owner('i-new',False)]]:
            self.assertEqual(reconcile(self.ec2,self.asg,'g','e',True,'source',owners)['state'],'waiting-for-bootstrap')
            self.ec2.associate_address.assert_not_called()
        self.assertEqual(reconcile(self.ec2,self.asg,'g','e',True,'source',
            [{'Complete':{'BOOL':True}},owner('i-new')])['instance'],'i-new')


WORKLOAD = '''import json,os,time
from pathlib import Path
p=Path(os.environ['SPOT_CHECKPOINT_DIR'])/'progress.json'
p.parent.joinpath('pid').write_text(str(os.getpid()))
n=json.loads(p.read_text()) if p.exists() else 0
while n<40:
 n+=1
 tmp=p.with_suffix('.tmp')
 with tmp.open('w') as f:
  json.dump(n,f); f.flush(); os.fsync(f.fileno())
 os.replace(tmp,p)
 time.sleep(.03)
Path(os.environ['SPOT_OUTPUT_DIR']).joinpath('answer').write_text(str(n))
'''


class ResumeTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.job = {"id": "experiment", "cwd": ".", "argv": [sys.executable, "-c", WORKLOAD],
                    "checkpoint_contract": "atomic-files-and-sigterm-v1"}
        (self.root / "jobs.json").write_text(json.dumps([self.job]))
        self.procs = []
        self.addCleanup(self.cleanup)

    def cleanup(self):
        for p in self.procs:
            if p.poll() is None:
                p.terminate()
                p.wait(timeout=10)

    def start(self):
        p = subprocess.Popen([sys.executable, str(Path(__file__).with_name('runner.py')),
                              '--root', str(self.root), '--no-imds', '--once'],
                             stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        self.procs.append(p)
        return p

    def wait_checkpoint(self):
        p = self.root / "state/experiment/checkpoint/progress.json"
        deadline = time.monotonic() + 8
        while time.monotonic() < deadline:
            if p.exists() and json.loads(p.read_text()) >= 5:
                return p
            time.sleep(.02)
        self.fail("No checkpoint produced")

    def verify_resume(self):
        self.assertEqual(self.start().wait(timeout=10), 0)
        self.assertEqual((self.root / 'state/experiment/output/answer').read_text(), '40')
        self.assertTrue((self.root / 'state/experiment/complete.json').exists())

    def test_sigterm_resumes_from_durable_checkpoint(self):
        first = self.start()
        checkpoint = self.wait_checkpoint()
        first.terminate()
        first.wait(timeout=10)
        saved = json.loads(checkpoint.read_text())
        self.assertGreaterEqual(saved, 5)
        self.assertLess(saved, 40)
        self.assertFalse((self.root / 'state/experiment/complete.json').exists())
        self.verify_resume()

    def test_abrupt_worker_death_resumes(self):
        first = self.start()
        checkpoint = self.wait_checkpoint()
        worker = int(checkpoint.parent.joinpath('pid').read_text())
        os.kill(worker, signal.SIGKILL)
        self.assertNotEqual(first.wait(timeout=10), 0)
        self.verify_resume()

    def test_replacement_waits_for_old_supervisor(self):
        old = self.start()
        self.wait_checkpoint()
        replacement = self.start()
        time.sleep(.1)
        self.assertIsNone(replacement.poll())
        old.terminate()
        old.wait(timeout=10)
        self.assertEqual(replacement.wait(timeout=10), 0)

    def test_completed_job_not_rerun(self):
        self.verify_resume()
        answer = self.root / 'state/experiment/output/answer'
        timestamp = answer.stat().st_mtime_ns
        self.assertEqual(self.start().wait(timeout=5), 0)
        self.assertEqual(answer.stat().st_mtime_ns, timestamp)

    def test_spec_change_cannot_reuse_checkpoint(self):
        self.verify_resume()
        self.job['argv'].append('different-input')
        (self.root / 'jobs.json').write_text(json.dumps([self.job]))
        self.assertNotEqual(self.start().wait(timeout=5), 0)

    def test_corrupt_application_checkpoint_does_not_restart_from_zero(self):
        state = self.root / 'state/experiment/checkpoint'
        state.mkdir(parents=True)
        (state / 'progress.json').write_text('corrupt')
        self.assertNotEqual(self.start().wait(timeout=5), 0)
        self.assertEqual((state / 'progress.json').read_text(), 'corrupt')
        self.assertFalse((state.parent / 'complete.json').exists())

    def test_missing_checkpoint_contract_rejected(self):
        del self.job['checkpoint_contract']
        (self.root / 'jobs.json').write_text(json.dumps([self.job]))
        with self.assertRaises(ValueError):
            read_jobs(self.root)

    def test_directory_escape_rejected(self):
        self.job['cwd'] = '..'
        (self.root / 'jobs.json').write_text(json.dumps([self.job]))
        with self.assertRaises(ValueError):
            read_jobs(self.root)


class DeploymentTests(unittest.TestCase):
    def test_legacy_restore_waits_for_codex_workspace_readiness(self):
        services=units(Path('/opt/spot'),Path('/opt/spot/bin'),Path('/srv/experiments/work'),legacy_origin='i-source')
        self.assertIn('Type=notify',services['spot-codex-recovery'])
        self.assertIn('ConditionPathExists=/srv/experiments/agent-migration/agent.json',services['spot-codex-recovery'])
        for name in ['spot-legacy-recovery','spot-diem-recovery']:
            self.assertIn('After=network-online.target spot-codex-recovery.service',services[name])
            self.assertIn('Requires=spot-codex-recovery.service',services[name])

    def test_compute_lifecycle_is_independent_of_ip_cutover(self):
        template = build()
        self.assertEqual(template['Conditions']['LifecycleActive'],
                         {'Fn::Equals': [{'Ref': 'LifecycleEnabled'}, 'true']})
        self.assertEqual(template['Resources']['Controller']['Properties']['Environment']['Variables']['ENABLED'],
                         {'Ref': 'Cutover'})

    def test_isolated_worker_root_applies_to_every_service(self):
        root = Path('/srv/experiments/spot-worker')
        text = units(Path('/opt/spot'), Path('/opt/spot/bin'), root/'work', root=root)
        self.assertIn('--root '+str(root)+'/nodes', text['spot-nodes'])
        self.assertIn('--root '+str(root), text['spot-experiments'])
        self.assertIn('--nodes '+str(root)+'/nodes', text['spot-activity'])
        self.assertIn('--output '+str(root)+'/discovery/current-host.json',
                      discovery_units(Path('/opt/spot'), [], root)['spot-checkpoint-discovery.service'])

    def test_bootstrap_proves_recovery_before_marking_instance_ready(self):
        bootstrap = build()['Resources']['Template']['Properties']['LaunchTemplateData']['UserData']['Fn::Base64']['Fn::Sub']
        self.assertLess(bootstrap.index('/opt/spot/bin/checkpoint-selftest'), bootstrap.index('aws ec2 create-tags'))
        self.assertIn("SPOT_SELFTEST_PARENT='${WorkRoot}'", bootstrap)

    def test_discovery_timer_only_runs_read_only_inventory(self):
        units = discovery_units(Path('/opt/spot'), [Path('/work/with spaces')])
        service = units['spot-checkpoint-discovery.service']
        self.assertIn('User=ec2-user', service)
        self.assertIn('--search-root "/work/with spaces"', service)
        self.assertIn('--output /srv/experiments/discovery/current-host.json', service)
        self.assertIn('OnUnitInactiveSec=900', units['spot-checkpoint-discovery.timer'])
        self.assertIn('--max-hash-bytes 68719476736', units['spot-checkpoint-discovery.service'])
        self.assertNotIn('runner.py', service)

    def test_safe_staging_and_durable_storage(self):
        t = build()
        self.assertNotIn('DesiredCapacity', t['Resources']['Group']['Properties'])
        self.assertEqual(t['Resources']['Group']['Properties']['MinSize'], '0')
        self.assertEqual(t['Parameters']['Cutover']['Default'], 'false')
        self.assertEqual(t['Parameters']['PreserveHostKeys']['Default'], 'false')
        self.assertEqual(t['Resources']['FileSystem']['DeletionPolicy'], 'Retain')
        group = t['Resources']['Group']['Properties']
        self.assertTrue(group['CapacityRebalance'])
        self.assertEqual(group['MixedInstancesPolicy']['InstancesDistribution']['OnDemandPercentageAboveBaseCapacity'], 0)
        data = t['Resources']['Template']['Properties']['LaunchTemplateData']['UserData']['Fn::Base64']['Fn::Sub']
        self.assertLessEqual(len(data.encode()), 16384)


class LeaseTests(unittest.TestCase):
    def setUp(self):
        self.db = Mock()
        self.ec2 = Mock()
        self.asg = Mock()
        self.sdk = SimpleNamespace(client=lambda name: {
            'dynamodb': self.db, 'ec2': self.ec2, 'autoscaling': self.asg}[name])
        class ConditionalFailure(Exception):
            pass
        self.db.exceptions.ConditionalCheckFailedException = ConditionalFailure
        self.asg.describe_auto_scaling_groups.return_value = {
            'AutoScalingGroups': [{'Instances': []}]}
        self.env = {'ENABLED': 'true', 'LOCK_TABLE': 'locks', 'GROUP': 'group', 'ALLOCATION': 'eip'}

    def test_busy_lease_prevents_ip_reconciliation(self):
        self.db.put_item.side_effect = self.db.exceptions.ConditionalCheckFailedException()
        with patch.dict(sys.modules, {'boto3': self.sdk}), patch.dict(os.environ, self.env):
            result = handler({}, SimpleNamespace(aws_request_id='request'))
        self.assertEqual(result['state'], 'another-controller-active')
        self.asg.describe_auto_scaling_groups.assert_not_called()
        self.db.delete_item.assert_not_called()

    def test_acquired_lease_released_after_failure(self):
        self.asg.describe_auto_scaling_groups.side_effect = RuntimeError('AWS unavailable')
        with patch.dict(sys.modules, {'boto3': self.sdk}), patch.dict(os.environ, self.env):
            with self.assertRaises(RuntimeError):
                handler({}, SimpleNamespace(aws_request_id='request'))
        self.db.delete_item.assert_called_once()
        release = self.db.delete_item.call_args.kwargs
        self.assertEqual(release['ExpressionAttributeValues'][':owner']['S'], 'request')


if __name__ == '__main__':
    unittest.main()
