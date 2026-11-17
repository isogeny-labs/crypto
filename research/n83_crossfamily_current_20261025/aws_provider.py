"""Independent EC2 API and archived-host binding for the N83 panel."""

from datetime import datetime
import json


def timestamp(value):
    parsed = datetime.fromisoformat(value.replace('Z', '+00:00'))
    if parsed.tzinfo is None:
        raise ValueError('provider timestamp lacks a timezone')
    return parsed


def verify(snapshot, strata, head):
    controller = snapshot['controller']
    instances = snapshot['instances']
    if (snapshot.get('schema') !=
            'n83.crossfamily-aws-provider-snapshot/v1' or
            snapshot.get('snapshot_source') not in (
                'ec2_describe_instances_at_launch',
                'ec2_describe_instances_at_collection') or
            controller.get('schema') != 'n83.crossfamily-aws-controller/v1' or
            controller.get('account_id') != '590183823895' or
            controller.get('region') != 'us-west-2' or
            controller.get('source_head_sha') != head or
            len(controller.get('strata', [])) != 2 or
            len(instances) != 2 or
            len(strata) != 2):
        raise ValueError('AWS provider snapshot is incomplete')
    by_id = {item['instance_id']: item for item in instances}
    if len(by_id) != 2 or set(by_id) != set(controller['instance_ids']):
        raise ValueError('AWS API instances differ from allocation')
    intervals = []
    zones = set()
    for stratum, item in enumerate(strata):
        host = item['host']
        results = item['results']
        allocation = json.loads((results / 'aws_allocation.json').read_text())
        fixed = controller['strata'][stratum]
        instance_id = fixed['instance_id']
        api = by_id.get(instance_id)
        if api is None:
            raise ValueError('AWS provider instance missing')
        tags = api['tags']
        if (fixed['stratum'] != stratum or
                allocation['stratum'] != stratum or
                allocation['instance_id'] != instance_id or
                allocation['run_id'] != controller['run_id'] or
                host.get('parent_instance_id') != instance_id or
                host.get('provider') != 'aws_ec2_spot' or
                host.get('git_sha') != head or
                api['instance_type'] != 'g5g.4xlarge' or
                api['instance_lifecycle'] != 'spot' or
                api['image_id'] != allocation['image_id'] or
                api['availability_zone'] != fixed['zone'] or
                api['availability_zone'] != allocation['availability_zone'] or
                api['subnet_id'] != fixed['subnet_id'] or
                api['account_id'] != controller['account_id'] or
                tags.get('Project') != 'N83HighKCurrent' or
                tags.get('RunId') != controller['run_id'] or
                tags.get('Stratum') != str(stratum) or
                tags.get('SourceHead') != head or
                item['analysis']['provider_receipt']['status'] !=
                'PASS_AWS_ALLOCATION'):
            raise ValueError('AWS API, host, or allocation identity differs')
        zones.add(fixed['zone'])
        usages = [json.loads(path.read_text()) for path in
                  results.glob('points/*/runs/*/usage.json')]
        if not usages:
            raise ValueError('AWS host has no measured intervals')
        start = timestamp(host['recorded_utc'])
        end = max(timestamp(usage['finished_utc']) for usage in usages)
        launch = timestamp(api['launch_time'])
        if not launch <= start < end:
            raise ValueError('AWS launch and measured interval differ')
        intervals.append((start, end))
    if not zones or max(interval[0] for interval in intervals) >= min(
            interval[1] for interval in intervals):
        raise ValueError('AWS strata did not overlap in time')
    return {'status': 'PASS_TWO_DISTINCT_CONCURRENT_AWS_SPOT_INSTANCES',
            'instance_ids': sorted(by_id), 'availability_zones': sorted(zones),
            'run_id': controller['run_id'],
            'snapshot_captured_utc': snapshot['captured_utc']}
