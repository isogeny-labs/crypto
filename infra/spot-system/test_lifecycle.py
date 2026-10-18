import copy
from contextlib import contextmanager
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import Mock, patch

from lifecycle import run
from install_hooks import install
import worker_state


class MemoryDB:
    class ConditionalFailure(Exception):
        pass

    def __init__(self):
        self.items = {}
        self.exceptions = Mock(ConditionalCheckFailedException=self.ConditionalFailure)

    def get_item(self, **kw):
        return {'Item': copy.deepcopy(self.items.get(kw['Key']['Id']['S'], {}))}

    def put_item(self, **kw):
        item = kw['Item']
        key = item['Id']['S']
        if kw.get('ConditionExpression'):
            existing = self.items.get(key)
            if existing and int(existing['Expires']['N']) >= int(kw['ExpressionAttributeValues'][':now']['N']):
                raise self.ConditionalFailure()
        self.items[key] = copy.deepcopy(item)

    def delete_item(self, **kw):
        del self.items[kw['Key']['Id']['S']]


class LifecycleTests(unittest.TestCase):
    def setUp(self):
        self.db, self.asg = MemoryDB(), Mock()
        self.group = {'DesiredCapacity': 1, 'Instances': [{'InstanceId': 'i-worker', 'LifecycleState': 'InService'}]}
        self.asg.describe_auto_scaling_groups.side_effect = lambda **_: {'AutoScalingGroups': [self.group]}

    def invoke(self, action='tick', now=2000, enabled=True):
        return run(self.db, self.asg, {'action': action}, 'request', now, 'table', 'group', enabled)

    def idle(self, now, **extra):
        self.db.items['activity#i-worker'] = {'Id': {'S': 'activity#i-worker'}, 'Observed': {'N': str(now)},
            'LastBusy': {'N': '2000'},
            'Busy': {'BOOL': False}, 'Unknown': {'BOOL': False}, 'Pending': {'N': '0'}, **extra}

    def test_staging_never_starts_or_stops(self):
        self.assertEqual(self.invoke('wake', enabled=False)['state'], 'staged')
        self.assertFalse(self.db.items)
        self.asg.set_desired_capacity.assert_not_called()

    def test_prompt_starts_and_cancels_idle(self):
        self.db.items['control'] = {'Id': {'S': 'control'}, 'IdleSince': {'N': '1'}, 'StoppingAt': {'N': '2'}}
        self.assertEqual(self.invoke('wake')['state'], 'starting')
        self.assertNotIn('IdleSince', self.db.items['control'])
        self.assertNotIn('StoppingAt', self.db.items['control'])
        self.asg.set_desired_capacity.assert_called_once_with(AutoScalingGroupName='group', DesiredCapacity=1, HonorCooldown=False)

    def test_exact_thirty_minutes_of_confirmed_idle(self):
        self.idle(2000)
        self.assertEqual(self.invoke(now=2000)['remaining_seconds'], 1800)
        self.idle(3799)
        self.assertEqual(self.invoke(now=3799)['remaining_seconds'], 1)
        self.asg.set_desired_capacity.assert_not_called()
        self.idle(3800)
        self.assertEqual(self.invoke(now=3800)['state'], 'scaled-to-zero')
        self.assertIn('StoppingAt', self.db.items['control'])
        self.asg.set_desired_capacity.assert_called_once_with(AutoScalingGroupName='group', DesiredCapacity=0, HonorCooldown=False)

    def test_new_prompt_resets_shutdown_window(self):
        self.idle(2000)
        self.invoke(now=2000)
        self.invoke('wake', now=3799)
        self.asg.set_desired_capacity.reset_mock()
        self.idle(3800)
        self.assertEqual(self.invoke(now=3800)['state'], 'idle')
        self.asg.set_desired_capacity.assert_not_called()

    def test_short_job_between_timer_ticks_restarts_idle_window(self):
        self.idle(2000)
        self.invoke(now=2000)
        self.idle(3800, LastBusy={'N':'3790'})
        response = self.invoke(now=3800)
        self.assertEqual(response['state'], 'idle')
        self.assertEqual(response['remaining_seconds'], 1790)
        self.asg.set_desired_capacity.assert_not_called()

    def test_busy_pending_unknown_and_stale_each_cancel_idle(self):
        for override in [{'Busy': {'BOOL': True}}, {'Pending': {'N': '1'}}, {'Unknown': {'BOOL': True}},
                         {'Observed': {'N': '1000'}}]:
            with self.subTest(override=override):
                self.db.items['control'] = {'Id': {'S': 'control'}, 'IdleSince': {'N': '1'}}
                self.idle(2000, **override)
                self.assertEqual(self.invoke()['state'], 'working-or-unconfirmed')
                self.assertNotIn('IdleSince', self.db.items['control'])
        self.asg.set_desired_capacity.assert_not_called()

    def test_missing_report_never_stops(self):
        self.assertEqual(self.invoke()['state'], 'working-or-unconfirmed')
        self.asg.set_desired_capacity.assert_not_called()

    def test_missing_instance_during_capacity_shortage_never_stops(self):
        self.group['Instances'] = []
        self.assertEqual(self.invoke()['state'], 'working-or-unconfirmed')

    def test_booting_replacement_prevents_stop(self):
        self.idle(2000)
        self.group['Instances'].append({'InstanceId': 'i-new', 'LifecycleState': 'Pending'})
        self.assertEqual(self.invoke()['state'], 'working-or-unconfirmed')

    def test_missing_required_report_fields_prevents_stop(self):
        self.idle(2000)
        del self.db.items['activity#i-worker']['Unknown']
        self.assertEqual(self.invoke()['state'], 'working-or-unconfirmed')

    def test_worker_clock_ahead_prevents_stop(self):
        self.idle(3000)
        self.assertEqual(self.invoke(now=2000)['state'], 'working-or-unconfirmed')

    def test_busy_controller_requests_retry_not_dropped_prompt(self):
        self.db.items['lifecycle-lock'] = {'Id': {'S': 'lifecycle-lock'}, 'Expires': {'N': '2100'}}
        self.assertTrue(self.invoke('wake')['retry'])
        self.asg.set_desired_capacity.assert_not_called()

    def test_scaling_error_preserves_demand_for_next_tick(self):
        self.asg.set_desired_capacity.side_effect = RuntimeError('AWS temporarily unavailable')
        with self.assertRaises(RuntimeError):
            self.invoke('wake')
        self.assertEqual(self.db.items['control']['WakeUntil']['N'], '3800')
        self.asg.set_desired_capacity.side_effect = None
        self.group['DesiredCapacity'] = 0
        self.assertEqual(self.invoke(now=2010)['state'], 'starting')

    def test_session_end_never_immediately_stops_experiments(self):
        self.assertEqual(self.invoke('release')['state'], 'idle-timer-managed')
        self.asg.set_desired_capacity.assert_not_called()


class HookTests(unittest.TestCase):
    def test_existing_hooks_preserved_and_install_idempotent(self):
        with tempfile.TemporaryDirectory() as d:
            home = Path(d)
            path = home / '.claude/settings.json'
            path.parent.mkdir()
            existing = {'hooks': {'UserPromptSubmit': [{'hooks': [{'type': 'command', 'command': 'existing-hook'}]}]},
                        'unrelated': 'retained'}
            path.write_text(json.dumps(existing))
            install(home, 'arn:function')
            first = path.read_text()
            install(home, 'arn:function')
            self.assertEqual(path.read_text(), first)
            actual = json.loads(first)
            self.assertEqual(actual['unrelated'], 'retained')
            self.assertEqual(actual['hooks']['UserPromptSubmit'][0], existing['hooks']['UserPromptSubmit'][0])
            self.assertEqual(len(actual['hooks']['UserPromptSubmit']), 2)
            self.assertTrue((home/'.config/opencode/plugins/spot-system.js').exists())

    def test_invalid_existing_config_is_not_partially_rewritten(self):
        with tempfile.TemporaryDirectory() as d:
            home = Path(d)
            path = home / '.codex/hooks.json'
            path.parent.mkdir()
            path.write_text('{invalid')
            with self.assertRaises(ValueError):
                install(home, 'arn:function')
            self.assertEqual(path.read_text(), '{invalid')
            self.assertFalse((home/'.claude/settings.json').exists())
            self.assertFalse((home/'.config/spot-system/client.json').exists())

    def test_draining_worker_cannot_admit_new_job(self):
        @contextmanager
        def held():
            yield
        with patch.object(worker_state, 'lease', held), patch.object(worker_state, 'call') as api:
            api.return_value = {'Item': {'StoppingAt': {'N': '1'}}}
            with self.assertRaisesRegex(RuntimeError, 'draining'):
                worker_state.admit(Path('/unused'))
            self.assertEqual(api.call_count, 1)


if __name__ == '__main__':
    unittest.main()
