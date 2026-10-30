"""Prefreeze population amendment; apply before freeze, never to a frozen record."""
from adapter import REPRESENTATION, require, PINS, sha, CHECKER_DEPENDENCIES
from pathlib import Path

def amend(draft):
    require(draft['freeze']['status']=='draft','never amend a frozen protocol')
    draft['sample_size']['instances']=1
    draft['sample_size']['power']['note']='Exactly two software workflow invocations, one per arm, one replicate and no warmups. No statistical population, significance, performance or power inference. Native candidate cells and copied controls are subordinate bounded operations, never padded arm rows.'
    draft['arms'][0]['description']='One invoked native-selection workflow: plan/preflight, bounded fixed-order search across at most16 candidate cells and immutable retries for a qualifying pair. Failed/censored outcomes and unrun cells remain in hashed subordinate ledgers.'
    draft['arms'][1]['description']='One invoked copied-control workflow: after native-selection terminates, check prerequisite; if unavailable record failed missing-precondition with zero native control calls; otherwise invoke exactly four copied-source controls unless interrupted or failed. All attempts retained.'
    f=draft['extensions']['ml_dataset_sat_fixture']
    f['pre_amendment_runtime_preflight']=dict(draft['sample_size']['preflight_runtime'])
    f['runtime_preflight_scope']='Retained empirical inputs are per-command observations, not measured entire workflows. Before freeze derive and document a workflow estimate from those observations and the frozen16-cell24-pattern4-control ceilings; do not relabel a measured cell or copied export as a measured workflow.'
    draft['sample_size']['preflight_runtime']={'fixed_seconds':None,'per_sample_seconds':None,'estimated_total_seconds':None,'threads':1,'measured_on':None}
    f['pending_before_freeze'].append('derive workflow-scale runtime estimate explicitly from retained per-command observations; new population invalidates previous per-sample interpretation')
    f['export_representation']=REPRESENTATION
    f['unified_export_tooling']={'path':'/home/ec2-user/ml-dataset-work-20261028/protocol-tooling-f755085ff951',
        'checker_python':'/home/ec2-user/ml-dataset-work-20261028/protocol-checker-venv/bin/python',
        'checker_python_sha256':sha(Path('/home/ec2-user/ml-dataset-work-20261028/protocol-checker-venv/bin/python')), 'file_sha256':dict(PINS),'dependencies':dict(CHECKER_DEPENDENCIES),'dependency_provenance':'Installed pinned protocol-checker-venv metadata observed2026-10-30; driver admission and adapter require exact versions and Draft202012Validator before native work'}
    f['arm_population']={'native-sat':'exactly one driver workflow invocation after protocol admission; subordinate candidate ceiling16, native pattern ceiling24',
        'copied-source-controls':'exactly one driver workflow invocation after selection terminates, including explicit missing-precondition return with zero native calls; subordinate control ceiling4'}
    f['unified_export_admission']='Both workflow invocations and terminal receipts required. Failure before workflow invocation or missing source preflight remains raw-only, never fabricated export rows. Every native command belongs exactly once to a workflow; every original cell is started or unrun. Official schema/files/flow gate mandatory before fixture success.'
    draft['randomization']['instance_sampling']['rule']='One deterministic software fixture workflow identity0 per arm. The frozen16-cell native table and4 conditional copied-input controls are subordinate ledgers, not measured samples or independent instances.'
    prior_primary=draft['metrics']['primary']
    for metric in prior_primary:
        if metric['id']=='verified-decisions':
            metric.update(definition='Sum verified decision occurrences from completed checked candidate audits within the native workflow; absent for copied-control workflow; never count unrun or unfinished candidates as zero measurements', source='driver candidates verified_decisions bound to audit and export manifests', aggregation='per_row')
        elif metric['id']=='qualified-transition':
            metric.update(definition='Boolean selected checked transition within native-selection workflow; absent for copied-control workflow', source='driver selected transition_witness', aggregation='per_row')
    draft['metrics']['primary']=[{'id':'workflow-invocations','name':'Workflow invocations','unit':'driver workflow invocation','kind':'counted','definition':'One recorded workflow entry and terminal receipt per arm; does not count native solver attempts','source':'driver receipt workflows','aggregation':'per_row','resolution':'1','uncertainty':'none_exact'}]
    draft['metrics']['secondary']=[{'id':'cli-command-receipts','name':'CLI command receipts','unit':'recorded CLI command attempt','kind':'counted','definition':'Count command receipts assigned exactly once to this workflow; includes failures and interrupted commands, distinguishes zero-control-call failed prerequisites','source':'driver receipt commands and workflows.command_names','aggregation':'per_row','resolution':'1','uncertainty':'none_exact'}]
    draft['metrics']['secondary'].extend(prior_primary)
    return draft
