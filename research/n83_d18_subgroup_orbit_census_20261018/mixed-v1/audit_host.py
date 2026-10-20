#!/usr/bin/env python3
"""Independently bind mixed-panel source, S3 inputs, host wall and cleanup."""

from __future__ import annotations

from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import re
import subprocess

from panel_supervisor import PLAN

ROOT = Path(__file__).resolve().parent
SOURCE = 'cad5fb22d7d768c27ef495e6ef9f84259443f49d'
BINARY = 'a2c2414f1eb52b8982d62c3b58eac1bf768160a569c937824ce6c05290e1b097'
ARCHIVES = {
    'U': ('2457a977ea9c05a11cc60878bdb3b29f6d905dc8db9aa26e52913a702d308369',
          '1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d',
          83_701_045, 34_163.898474),
    'D': ('347d465bef217ae5f76e9b664710440a735a26211bf3957c4be4bcb4b46345a7',
          'e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682',
          81_844_623, 69_218.884625),
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def read_json(path: Path):
    return json.loads(path.read_text())


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def utc(text: str) -> datetime:
    return datetime.fromisoformat(text.replace('Z', '+00:00')).astimezone(timezone.utc)


def audit() -> dict:
    setup = ROOT / 'host-setup'
    panel = ROOT / 'panel-02'
    package = read_json(ROOT / 'source-package.json')
    snapshot = read_json(ROOT / 'snapshot-attestation.json')
    require(snapshot['source_commit'] == SOURCE and snapshot['status_clean'] is True,
            'snapshot source attestation')
    require(package['source_commit'] == SOURCE and len(package['compiled_source_sha256']) == 7,
            'source package identity')
    for name, expected in package['compiled_source_sha256'].items():
        shown = subprocess.run(['git', 'show', f'{SOURCE}:{name}'], cwd=ROOT,
                               capture_output=True, check=True).stdout
        require(hashlib.sha256(shown).hexdigest() == expected,
                f'committed compiled source differs: {name}')
    transfer_lines = [line.split('  ', 1)
                      for line in (setup / 'transfer-sha256.txt').read_text().splitlines()]
    require(len(transfer_lines) == 5, 'host transfer manifest count')
    by_name = {name: digest for digest, name in transfer_lines}
    require(len(by_name) == 5, 'duplicate host transfer name')
    require(by_name['source-cad5fb2-minimal.tar.gz'] == package['minimal_source_archive_sha256']
            and by_name['snapshot-cad5fb2.tar.gz']
            == '75452343c54ed46e74ffdffad51bed870c1c7ed3437acff684d27842db0397a2'
            and by_name['inputs-clean.tar']
            == '7a8d4cd6b2b4a0da5fbc76f77ddbd3c58c22d3bf708ff863e5c4d1ad5a1d2331'
            and by_name['host-setup.sh'] == sha256(ROOT / 'host-setup.sh')
            and by_name['run-mixed-docker.sh'] == sha256(ROOT / 'run-mixed-docker.sh'),
            'host transfer/source bindings')
    require('PASS seven compiled-source bytes and snapshot'
            in (setup / 'host-setup.stdout').read_text(),
            'host source snapshot attestation absent')
    built_digest = (setup / 'binary-sha256.txt').read_text().split()[0]
    require(built_digest == BINARY, 'host binary differs')
    input_lines = (setup / 'input-sha256.txt').read_text().splitlines()
    require(len(input_lines) == 2, 'host input archive count')
    input_hashes = {('U' if '/U/' in name else 'D' if '/D/' in name else '?'): digest
                    for digest, name in (line.split('  ', 1) for line in input_lines)}
    require(set(input_hashes) == {'U', 'D'}, 'host input archive paths')
    for arm, (digest, object_id, size, construction_ms) in ARCHIVES.items():
        require(input_hashes[arm] == digest, f'{arm}: host archive hash')
        manifest = read_json(ROOT / 'input-receipts' / arm / 'manifest.json')
        if arm == 'U':
            require(len(manifest['bases']) == 1, 'U base count')
            base = manifest['bases'][0]
            require(base['construction_elapsed_ms_informational_L0'] == construction_ms,
                    'U construction time')
            require(read_json(ROOT / 'input-receipts/U/upload-receipt.json')['status'] == 'PASS'
                    and sha256(ROOT / 'input-receipts/U/probe-corpus.json')
                    == '760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd',
                    'U custody/public corpus')
        else:
            base = manifest
            require(base['construction_elapsed_ms_informational_L0'] == construction_ms,
                    'D construction time')
            custody = read_json(ROOT / 'input-receipts/D/s3-custody.json')
            require(custody['status'] == 'PASS' and custody['fresh_get']['sha256'] == digest,
                    'D custody')
        require(base['compressed_blake3'] == object_id and base['bytes'] == size
                and base['columns'] == 16_627 and base['points'] == 2_760_082,
                f'{arm}: factor-base manifest identity')
        require(read_json(ROOT / 'input-receipts' / arm / 'replay.json')['status'] == 'PASS',
                f'{arm}: independent archive replay')
        head = read_json(setup / f'{arm.lower()}-s3-head.json')
        require(head['ContentLength'] == size and head['ServerSideEncryption'] == 'AES256',
                f'{arm}: fresh S3 custody HEAD')
    ordered = read_json(panel / 'audit.json')
    require(ordered['status'] == 'AUDITED_BOUNDED' and len(ordered['rows']) == len(PLAN)
            and ordered['raw_file_count'] == 340
            and [r['name'] for r in ordered['rows']] == [r[0] for r in PLAN]
            and all(r['status'] == 'UNKNOWN_budget' for r in ordered['rows'] if r['mode'] == 'natural')
            and ordered['candidate_id'] is None
            and ordered['selected_best_total_runtime'] is None,
            'complete ordered panel audit')
    failed = read_json(ROOT / 'panel-01/panel-status.json')
    require(failed['status'] == 'INCOMPLETE' and 'host manifest failed' in failed['error']
            and 'No such file or directory' in failed['error'],
            'first setup failure receipt')
    instance = read_json(setup / 'instance-terminated.json')['Reservations'][0]['Instances'][0]
    request = read_json(setup / 'spot-request-final.json')['SpotInstanceRequests'][0]
    clean = read_json(setup / 'cloud-cleanup.json')
    require(instance['State']['Name'] == 'terminated'
            and instance['InstanceId'] == clean['instance_id']
            and request['Type'] == 'one-time' and request['State'] == 'cancelled'
            and not read_json(setup / 'keypair-after.json')['KeyPairs']
            and not read_json(setup / 'security-group-after.json')['SecurityGroups']
            and clean['temporary_keypairs_remaining'] == 0
            and clean['temporary_security_groups_remaining'] == 0
            and clean['local_ssh_private_key_removed'] is True,
            'cloud cleanup')
    launch = utc(instance['LaunchTime'])
    match = re.fullmatch(r'User initiated \((\d{4}-\d\d-\d\d \d\d:\d\d:\d\d) GMT\)',
                         instance['StateTransitionReason'])
    require(match is not None, 'termination time absent')
    stop = datetime.strptime(match.group(1), '%Y-%m-%d %H:%M:%S').replace(tzinfo=timezone.utc)
    ready = utc((setup / 'ready.utc').read_text().strip())
    first = utc(ordered['rows'][0]['start_utc'])
    last = utc(ordered['rows'][-1]['end_utc'])
    require(launch < ready < first < last < stop, 'host/panel UTC chronology')
    provisioned = (stop - launch).total_seconds()
    arm_wall = ordered['charged_arm_host_wall_s']
    require(0 < arm_wall < provisioned, 'host accounting interval')
    price = read_json(setup / 'spot-price-observed.json')['SpotPriceHistory']
    require(len(price) == 1 and price[0]['InstanceType'] == 'c7i.2xlarge'
            and price[0]['AvailabilityZone'] == 'us-west-2b',
            'spot observation identity')
    point_price = float(price[0]['SpotPrice'])
    report = {
        'schema': 'n83.mixed-orbit-m4-host-audit/v1', 'status': 'PASS',
        'source_commit': SOURCE, 'binary_sha256': BINARY,
        'u_archive_sha256': ARCHIVES['U'][0], 'd_archive_sha256': ARCHIVES['D'][0],
        'u_construction_ms_informational_L0': ARCHIVES['U'][3],
        'd_construction_ms_informational_L0': ARCHIVES['D'][3],
        'panel_audit_sha256': sha256(panel / 'audit.json'),
        'raw_files_sha256': sha256(panel / 'raw-files.sha256'),
        'panel_arm_count': len(PLAN), 'natural_capped_count': 8,
        'provisioned_start_utc': launch.isoformat(),
        'provisioned_termination_request_utc': stop.isoformat(),
        'charged_provisioned_host_wall_s': provisioned,
        'summed_arm_host_wall_s': arm_wall,
        'charged_setup_replay_transfer_idle_wall_s': provisioned - arm_wall,
        'spot_point_price_usd_per_hour': point_price,
        'spot_point_estimate_compute_usd': round(provisioned / 3600 * point_price, 6),
        'exact_aws_invoice_usd': None,
        'candidate_id': None, 'total_index_calculus_runtime_ms': None,
        'selected_best_total_runtime': None,
    }
    (ROOT / 'host-audit.json').write_text(json.dumps(report, sort_keys=True, indent=2) + '\n')
    return report


if __name__ == '__main__':
    print(json.dumps(audit(), sort_keys=True))
