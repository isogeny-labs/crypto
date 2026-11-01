#!/usr/bin/env python3
"""Bind the two independently selected N83 online candidates."""

import hashlib
import json
from pathlib import Path

from research.n83_final_extended_20261018 import arms as extended_arms
from research.n83_final_extended_20261018 import run as extended_run
from research.n83_final_extended_20261018 import select as extended_select
from research.n83_root4_frontier_current_20261018 import arms as root4_arms
from research.n83_root4_frontier_current_20261018 import run as root4_run
from research.n83_root4_frontier_current_20261018 import select as root4_select

HERE = Path(__file__).resolve().parent
SOURCE_SHA256 = root4_run.SOURCE_SHA256
EXTENDED_ARMS = (*extended_arms.SMALL, *extended_arms.MID,
                 *extended_arms.six_run.ARMS)
ROOT4_ARMS = root4_arms.ARMS


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def manifest(arm):
    candidate_id = arm[3]
    if arm in EXTENDED_ARMS:
        return extended_arms.manifest(candidate_id)
    if arm in ROOT4_ARMS:
        return root4_arms.manifest(arm)
    raise ValueError('candidate is outside frozen upstream arms')


def upstream(validated_path, selection_path, schema, target_sha, selector,
             allowed_ids):
    validated = json.loads(validated_path.read_text())
    selection = json.loads(selection_path.read_text())
    if (validated.get('schema') != schema or
            validated.get('source_sha256') != SOURCE_SHA256 or
            validated.get('targets_sha256') != target_sha or
            selection != selector.select(validated)):
        raise ValueError('upstream arithmetic, provider or selector differs')
    selected = selection['online']['selected_candidate_id']
    if selected is not None and (selected not in allowed_ids or
                                 selection['online']['status'] != 'SELECTED' or
                                 selection.get('admission_status') != 'PASS' or
                                 validated.get('evidence_status') !=
                                 'PASS_INDEPENDENT_REPLAY' or
                                 validated.get('different_cloud_instances') is not True or
                                 validated.get('same_candidate_bases_across_strata') is not True):
        raise ValueError('upstream selected an unknown candidate')
    return validated, selection, selected


def resolve(extended_validated, extended_selection,
            root4_validated, root4_selection):
    first = upstream(extended_validated, extended_selection,
                     'n83.final-extended-validated/v1',
                     extended_run.TARGETS_SHA256, extended_select,
                     {arm[3] for arm in EXTENDED_ARMS})
    second = upstream(root4_validated, root4_selection,
                      'n83.root4-frontier-validated/v1',
                      root4_run.TARGETS_SHA256, root4_select,
                      {arm[3] for arm in ROOT4_ARMS})
    selected = (first[2], second[2])
    evidence = {
        'schema': 'n83.global-bridge-upstream/v1',
        'source_sha256': SOURCE_SHA256,
        'extended_run_id': first[0]['run_id'],
        'extended_head_sha': first[0]['head_sha'],
        'extended_validated_sha256': digest(extended_validated),
        'extended_selection_sha256': digest(extended_selection),
        'root4_run_id': second[0]['run_id'],
        'root4_head_sha': second[0]['head_sha'],
        'root4_validated_sha256': digest(root4_validated),
        'root4_selection_sha256': digest(root4_selection),
        'upstream_online_ids': list(selected),
        'upstream_admission_statuses':
            [first[1]['admission_status'], second[1]['admission_status']],
    }
    if None in selected:
        return (), {**evidence, 'status': 'UPSTREAM_UNRESOLVED',
                    'ordered_candidate_ids': []}
    if selected[0] == selected[1]:
        root4_arm = next(arm for arm in ROOT4_ARMS if arm[3] == selected[1])
        first_manifest = extended_arms.manifest(selected[0])[0]
        second_manifest = root4_arms.manifest(root4_arm)[0]
        if digest(first_manifest) != digest(second_manifest):
            raise ValueError('identical candidate ID has different manifests')
        return (), {**evidence, 'status': 'CONVERGED_IDENTICAL',
                    'ordered_candidate_ids': [selected[0]],
                    'candidate_manifest_sha256':
                    {selected[0]: digest(first_manifest)}}
    first_arm = next(arm for arm in EXTENDED_ARMS if arm[3] == selected[0])
    second_arm = next(arm for arm in ROOT4_ARMS if arm[3] == selected[1])
    arms = (first_arm, second_arm)
    paths = (extended_arms.manifest(selected[0])[0],
             root4_arms.manifest(second_arm)[0])
    if (len({arm[0] for arm in arms}) != 2 or
            any(json.loads(path.read_text())['candidate_id'] != arm[3]
                for path, arm in zip(paths, arms))):
        raise ValueError('distinct candidate names or manifests differ')
    manifests = {arm[3]: digest(path) for arm, path in zip(arms, paths)}
    return arms, {**evidence, 'status': 'TWO_DISTINCT_ADMITTED',
                  'ordered_candidate_ids': list(selected),
                  'candidate_manifest_sha256': manifests,
                  'candidate_manifest_paths':
                  [str(path.relative_to(HERE.parents[1])) for path in paths]}


def choose():
    return resolve(HERE / 'extended_validated.json',
                   HERE / 'extended_selection.json',
                   HERE / 'root4_validated.json',
                   HERE / 'root4_selection.json')
