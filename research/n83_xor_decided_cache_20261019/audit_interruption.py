#!/usr/bin/env python3
"""Charge and classify the retained first Spot interruption without ranking it."""

from __future__ import annotations

from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import re


ROOT = Path(__file__).resolve().parent
ATTEMPT = ROOT / 'attempt-01'
SOURCE = '8cde893f774cace3e022438e168a0485b7268a57'
BINARY = 'e95658ff67c876a6ae80d9c9c6cd89726b32947d48c653208efadbc1cc2950c3'


def require(ok: bool, message: str) -> None:
    if not ok:
        raise ValueError(message)


def read(path: Path):
    return json.loads(path.read_text())


def utc(text: str) -> datetime:
    return datetime.fromisoformat(text.replace('Z', '+00:00')).astimezone(timezone.utc)


def audit() -> dict:
    panel = ATTEMPT / 'panel-01'
    cloud = ATTEMPT / 'cloud'
    prior = read(panel / 'audit.json')
    require(prior['status'] == 'AUDITED_INCOMPLETE'
            and prior['source_commit'] == SOURCE and prior['binary_sha256'] == BINARY
            and len(prior['rows']) == 8
            and all(r['status'] in {'PASS_PREFLIGHT', 'PASS_CONSTRUCTED_CONTROL'}
                    for r in prior['rows'])
            and prior['candidate_id'] is None
            and prior['selected_best_total_runtime'] is None,
            'completed controls or incomplete audit differ')
    require(read(panel / 'panel-status.json')['status'] == 'INCOMPLETE'
            and 'SSH lost' in read(panel / 'panel-status.json')['error'],
            'supervisor failure differs')
    event_rows = [json.loads(line) for line in
                  (panel / 'panel-events.jsonl').read_text().splitlines()]
    last_control = next(e for e in reversed(event_rows)
                        if e.get('arm') == 'a-uudd-on'
                        and e['event'] == 'mirrored_verified')
    natural_start = next(e for e in event_rows
                         if e.get('arm') == 'n-uudd-1-off' and e['event'] == 'start')
    partial = next(e for e in event_rows
                   if e.get('arm') == 'n-uudd-1-off' and e['event'] == 'partial_mirror')
    natural_end = next(e for e in event_rows
                       if e.get('arm') == 'n-uudd-1-off' and e['event'] == 'host_command_end')
    require(utc(last_control['utc']) < utc(natural_start['utc'])
            < utc(partial['utc']) < utc(natural_end['utc'])
            and partial['copy_exit'] == 0 and natural_end['exit'] == 255,
            'interruption chronology or partial mirror differs')
    partial_root = panel / 'partial/n-uudd-1-off/0060s/n-uudd-1-off'
    require(partial_root.is_dir() and (partial_root / 'start.utc').is_file()
            and not (panel / 'spot-arms/n-uudd-1-off').exists(),
            'natural interruption must remain partial')
    manifest = (panel / 'raw-files.sha256').read_text().splitlines()
    require(bool(manifest), 'completed-arm raw manifest absent')
    for line in manifest:
        digest, relative = line.split('  ', 1)
        path = panel / relative
        require(path.is_file() and hashlib.sha256(path.read_bytes()).hexdigest() == digest,
                f'completed-arm raw hash differs: {relative}')
    stopped = read(cloud / 'instance-terminated.json')['Reservations'][0]['Instances'][0]
    interrupted = read(cloud / 'instance-interrupted.json')['Reservations'][0]['Instances'][0]
    request = read(cloud / 'spot-interrupted.json')['SpotInstanceRequests'][0]
    cleanup = read(cloud / 'cloud-cleanup.json')
    require(stopped['State']['Name'] == 'terminated'
            and interrupted['StateReason']['Code'] == 'Server.SpotInstanceTermination'
            and request['Status']['Code'] == 'instance-terminated-no-capacity'
            and cleanup['status'] == 'PASS'
            and cleanup['instance_id'] == stopped['InstanceId']
            and cleanup['temporary_keypairs_remaining'] == 0
            and cleanup['temporary_security_groups_remaining'] == 0
            and cleanup['local_ssh_private_key_removed'] is True,
            'AWS Spot interruption or cleanup differs')
    match = re.fullmatch(r'Service initiated \((\d{4}-\d\d-\d\d \d\d:\d\d:\d\d) GMT\)',
                         stopped['StateTransitionReason'])
    require(match is not None, 'service termination time absent')
    launched = utc(stopped['LaunchTime'])
    terminated = datetime.strptime(match.group(1), '%Y-%m-%d %H:%M:%S').replace(
        tzinfo=timezone.utc)
    wall = (terminated - launched).total_seconds()
    require(wall > prior['charged_arm_host_wall_s'] > 0,
            'provisioned host interval does not charge completed arms')
    price = read(ROOT / 'host-setup/spot-price-observed.json')['SpotPriceHistory'][0]
    require(price['AvailabilityZone'] == 'us-west-2b'
            and price['InstanceType'] == 'c7i.2xlarge',
            'first host Spot price identity differs')
    report = {
        'schema': 'n83.xor-cache-interruption-audit/v1',
        'status': 'INFRA_SPOT_INTERRUPTION',
        'source_commit': SOURCE,
        'binary_sha256': BINARY,
        'panel_audit_sha256': hashlib.sha256((panel / 'audit.json').read_bytes()).hexdigest(),
        'completed_admission_arms': len(prior['rows']),
        'natural_arm_elapsed_s_before_ssh_loss': natural_end['elapsed_s'],
        'natural_partial_mirror_count': 1,
        'spot_status_code': request['Status']['Code'],
        'charged_provisioned_host_wall_s': wall,
        'spot_point_price_usd_per_hour': float(price['SpotPrice']),
        'spot_point_estimate_compute_usd': round(wall / 3600 * float(price['SpotPrice']), 6),
        'exact_aws_invoice_usd': None,
        'candidate_id': None,
        'total_index_calculus_runtime_ms': None,
        'selected_best_total_runtime': None,
    }
    (ATTEMPT / 'audit.json').write_text(json.dumps(report, sort_keys=True, indent=2) + '\n')
    return report


if __name__ == '__main__':
    print(json.dumps(audit(), sort_keys=True))
