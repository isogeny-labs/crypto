"""Metadata only: extend the immutable v1 adapter with two checked legacy models."""
from pathlib import Path
import argparse
import hashlib
import json
import sys

study=Path(__file__).resolve().parent
sys.path.insert(0,str(study.parent.parent/'tools'))
from curve_identity import curve_identity,inspect_manifest  # noqa: E402

base_path=study/'curve-identity-adapter.v1.json'
base=json.loads(base_path.read_text())
source='pipeline-receipts/publication-followup-audit-v3-20261026/output.json'
p=study/source
data=json.loads(p.read_text())
assert data['status']=='native_profile_accounting_and_partial_scalar_checks_pass'
records=list(base['records'])
for old in data['partial_quartics']:
    assert old['wide_factory_generator_matches_recorded'] is True
    raw=study/f"pipeline-runs/quartic-m{old['m']}-local-counts-20261025-01/records.jsonl"
    assert hashlib.sha256(raw.read_bytes()).hexdigest()==old['records_sha256']
    first=json.loads(raw.read_text().splitlines()[0])
    field={'characteristic':2,'degree':old['n'],'representation':'polynomial',
           'modulus':old['polynomial_modulus_hex'],'element_encoding':'hex-string-polynomial-bits'}
    generator=old['wide_factory_generator_polynomial']
    curve={'model':'y^2+x*y=x^3+a2*x^2+a6',
           'coefficients':{'a1':'0x1','a2':old['a_hex'],'a3':'0x0','a4':'0x0','a6':old['b_hex']},
           'group_order':old['order_decimal'],'subgroup_order':old['r_decimal'],
           'cofactor':str(first['workload']['curve']['cofactor']),'order_encoding':'decimal-string',
           'generator':{'x':generator[0],'y':generator[1]},'target_group':'prime-order-subgroup-generated-by-G'}
    identity=curve_identity(field,curve,'bin')
    manifest={'field':field,'curve':dict(curve,curve_id=identity['curve_id'])}
    assert inspect_manifest(manifest)==identity
    records.append({'source':source,'source_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),
                    'producer_label':old['curve_name'],'icv1':old['icv1'],'canonical_identity':identity,
                    'canonical_manifest':manifest,'registry_status':'unregistered_research_model',
                    'legacy_point_encoding':'packed tower coordinates, converted by native audit to the canonical polynomial basis',
                    'original_records_sha256':old['records_sha256']})
assert len(records)==27
assert len({r['canonical_identity']['curve_uid'] for r in records})==27
document={'schema':'ic-algebraic-curve-identity-adapter/v2','base_adapter':'curve-identity-adapter.v1.json',
          'base_sha256':hashlib.sha256(base_path.read_bytes()).hexdigest(),'records':records,
          'scope':'Preserve the 25 v1 records unchanged and add two source-bound partial quartic models; no registry admission, isomorphism or full-method equivalence is inferred.'}
text=json.dumps(document,indent=2,ensure_ascii=False)+'\n'
parser=argparse.ArgumentParser();parser.add_argument('--check',action='store_true')
target=study/'curve-identity-adapter.v2.json'
if parser.parse_args().check:
    assert target.read_text()==text
    print('PASS: all 27 source-bound metadata identities; v1 records unchanged')
else:
    target.write_text(text)
    print('Wrote v2 metadata adapter; v1 remains unchanged')
