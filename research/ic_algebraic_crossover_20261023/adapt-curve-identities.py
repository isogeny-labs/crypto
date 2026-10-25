"""Versioned metadata-only identity adapter; no field/curve arithmetic or reruns.

Use the repository's exact shared EC1 metadata reference. Preserve frozen
producer preimages and labels; do not silently reinterpret them as global IDs.
"""
from pathlib import Path
import argparse
import hashlib
import json
import sys

study=Path(__file__).resolve().parent
root=study.parent.parent
sys.path.insert(0,str(root/'tools'))
from curve_identity import curve_identity,inspect_manifest,digest  # noqa: E402

def load(relative):
    p=study/relative
    return json.loads(p.read_text()),hashlib.sha256(p.read_bytes()).hexdigest()

records=[]
def bind(source,source_sha,producer_label,preimage_kind,field,curve,extra):
    identity=curve_identity(field,curve,'bin')
    curve=dict(curve,curve_id=identity['curve_id'])
    manifest={'field':field,'curve':curve}
    assert inspect_manifest(manifest)==identity
    records.append({'source':source,'source_sha256':source_sha,'producer_label':producer_label,
        'producer_preimage_kind':preimage_kind,'producer_alias_global_compatibility':'requires_this_versioned_adapter',
        'canonical_identity':identity,'canonical_manifest':manifest,'registry_status':'unregistered_research_model',**extra})

source='pipeline-receipts/fixture-subgroups-enumerated-20261025/receipt.json'
data,sha=load(source)
for old in data['curve_models'].values():
    f=old['field'];c=old['curve'];label=c['curve_id']
    original={'field':f,'curve':{k:v for k,v in c.items() if k!='curve_id'}}
    assert label.endswith(digest(original)[:12])
    field={'characteristic':f['p'],'degree':f['n'],'representation':f['basis'],
        'modulus':f['modulus'],'element_encoding':f['element_encoding']}
    curve={'model':c['model'],'coefficients':c['coefficients'],'group_order':c['order'],
        'subgroup_order':c['r'],'cofactor':c['cofactor'],'generator':c['generator'],'target_group':c['target_group']}
    bind(source,sha,label,'native-field-p-n-basis-curve-r/v1',field,curve,
         {'producer_preimage_sha256':digest(original),'producer_preimage':original})

source='pipeline-receipts/qualified-queries-20261025/summary.json'
data,sha=load(source)
for old in data['models']:
    original={k:v for k,v in old.items() if k not in ('record_sha256','ec1_record_id')}
    assert digest(original)==old['record_sha256']
    field={'characteristic':2,'degree':old['n'],'representation':'polynomial',
        'modulus':old['field_modulus'],'element_encoding':'unsigned-integer-polynomial-bits'}
    curve={'model':'y^2+x*y=x^3+a2*x^2+a6','coefficients':{'a1':1,'a2':old['a'],'a3':0,'a4':0,'a6':old['b']},
        'group_order':old['group_order'],'subgroup_order':old['r'],'cofactor':old['cofactor'],
        'generator':old['generator'],'target_group':old['subgroup']}
    bind(source,sha,old['ec1_record_id'],'flat-ic-qualified-curve/v1',field,curve,
         {'producer_preimage_sha256':old['record_sha256'],'producer_preimage':original,'icv1_slug':old['icv1']})

for m in (16,17,18):
    source=f'pipeline-receipts/wide-core-audit-20261025/m{m}/output.json'
    old,sha=load(source)
    field={'characteristic':2,'degree':old['n'],'representation':'polynomial',
        'modulus':old['polynomial_modulus_hex'],'element_encoding':'hex-string-polynomial-bits'}
    curve={'model':'y^2+x*y=x^3+a2*x^2+a6',
        'coefficients':{'a1':'0x1','a2':old['a_hex'],'a3':'0x0','a4':'0x0','a6':old['b_hex']},
        'group_order':old['group_order_decimal'],'subgroup_order':old['r_decimal'],'cofactor':old['cofactor_decimal'],
        'order_encoding':'decimal-string',
        'generator':{'x':old['generator']['polynomial_x_hex'],'y':old['generator']['polynomial_y_hex']},
        'target_group':'prime-order-subgroup-generated-by-G'}
    bind(source,sha,old['curve_name'],'native-icv1-wide-core/v1',field,curve,
         {'icv1':old['icv1'],'icv1_slug':old['curve_name']})

assert len(records)==25
assert len({r['canonical_identity']['curve_uid'] for r in records})==25
document={'schema':'ic-algebraic-curve-identity-adapter/v1',
    'scope':'Metadata-only adapter for 18 reference/twist models, four qualified-query models and three wide-core models; source records, targets and measurement labels unchanged.',
    'warning':'Producer EC1-shaped labels use different preimages from tools/curve_identity.py. They are preserved as provisional/source labels, never silently treated as canonical global aliases. The versioned canonical manifests below supply full UIDs; registry admission and mathematical equivalence are separate.',
    'shared_reference':'tools/curve_identity.py','records':records}
text=json.dumps(document,indent=2,ensure_ascii=False)+'\n'
target=study/'curve-identity-adapter.v1.json'
args=argparse.ArgumentParser()
args.add_argument('--check',action='store_true')
if args.parse_args().check:
    assert target.read_text()==text,'curve identity metadata adapter is stale'
    print('PASS: all 25 source-bound metadata adapters and canonical aliases/full UIDs agree')
else:
    target.write_text(text)
    print('Wrote 25 versioned metadata adapters; frozen records unchanged')
