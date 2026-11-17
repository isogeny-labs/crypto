"""Byte-copy/extraction and manifests only; scientific operations stay in Rust."""
from pathlib import Path
import hashlib,json,shutil
from datetime import datetime,timezone
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_richelot_map_20261117';old=Path('/workspace/cryptanalysis/experiments/chained-transfer-reference/src');b=r/'research/chained_richelot_balanced_chart_20261117'
for n in ['src','vendor','reused-core','original-sources']:(s/n).mkdir(exist_ok=True)
receipt=json.loads((s/'ORIGINAL_INPUTS_BEFORE_CODE_14.json').read_text());known={x['original_path']:x for x in receipt['original_source_files']}
def original(p):
 raw=p.read_bytes();assert hashlib.sha256(raw).hexdigest()==known[str(p.resolve())]['sha256'];return raw
extractions=[]
for name in ['nonsplit.rs','nonsplit_map.rs']:
 raw=original(old/name);prefix=raw[:raw.index(b'#[test]')];segments=[]
 if name=='nonsplit_map.rs':
  unused=b'#[path = "seven_glue.rs"]\nmod seven_glue;\n';at=prefix.index(unused);segments=[(0,at),(at+len(unused),len(prefix))];prefix=prefix.replace(unused,b'',1);tail=(s/'new-code/map14_harness.rs.inc').read_bytes()
 else:segments=[(0,len(prefix))];tail=b'// End of the unchanged constructor prefix; unit14 tests are in map.\n'
 (s/'original-sources'/name).write_bytes(raw);(s/'src'/name).write_bytes(prefix+tail)
 extractions.append({'original':str(old/name),'compiled':f'src/{name}','original_sha256':hashlib.sha256(raw).hexdigest(),'source_byte_intervals_half_open':segments,'prefix_bytes':len(prefix),'prefix_sha256':hashlib.sha256(prefix).hexdigest(),'removed_only':'unused seven_glue registration and original test population' if name=='nonsplit_map.rs' else 'original test population','method_bodies_unchanged':True})
raw=original(old/'closed_fibres.rs');(s/'src/closed_fibres.rs').write_bytes(raw);(s/'original-sources/closed_fibres.rs').write_bytes(raw)
raw=original(b/'src/shifted.rs');(s/'src/shifted.rs').write_bytes(raw+(s/'new-code/shifted14_adapters.rs.inc').read_bytes());(s/'original-sources/shifted12.rs').write_bytes(raw);extractions.append({'original':str(b/'src/shifted.rs'),'compiled':'src/shifted.rs','source_byte_intervals_half_open':[(0,len(raw))],'prefix_bytes':len(raw),'prefix_sha256':hashlib.sha256(raw).hexdigest(),'only_append_new_visibility_adapters':True})
for name in ['fp_poly.rs','fp2.rs']:(s/'vendor'/name).write_bytes(original(b/'vendor'/name))
(s/'reused-core/balanced_core.rs.inc').write_bytes(original(b/'reused-core/balanced_core.rs.inc'))
(s/'src/balanced.rs').write_text('''use crate::{fp2::Fp2Ctx, fp_poly::FpPoly};
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
type Result<T> = std::result::Result<T, String>;
type Key = (Vec<u64>, Vec<u64>, u8);
include!("../reused-core/balanced_core.rs.inc");
#[path = "shifted.rs"]
mod shifted;
pub(crate) use shifted::{witnesses14, Bridge14};
''')
(s/'src/lib.rs').write_text('''//! Frozen operational joint-chart divisor map14; no new field/group engine.
#![forbid(unsafe_code)]
#[rustfmt::skip]
#[allow(clippy::wrong_self_convention)]
#[path = "../vendor/fp2.rs"]
pub mod fp2;
#[rustfmt::skip]
#[path = "../vendor/fp_poly.rs"]
pub mod fp_poly;
#[allow(dead_code)]
pub mod balanced;
pub mod reference {
    pub use crate::fp2::{Fp2, Fp2Ctx};
    pub use crate::fp_poly::FpPoly;
}
#[allow(dead_code)]
mod nonsplit;
''')
for name in ['Cargo.toml','Cargo.lock']:
 text=(b/name).read_text();assert 'chained-richelot-balanced-chart-12' in text
 (s/name).write_text(text.replace('chained-richelot-balanced-chart-12','chained-richelot-operational-map-14'))
(s/'SOURCE_EXTRACTION_14.json').write_text(json.dumps({'time':datetime.now(timezone.utc).isoformat(),'prospective_protocol_public':'e9d833a53265908311f087ca736bfd0bed3c4f9e','prospective_claim':'PCC-20261117-9dec6a','extractions':extractions,'no_compilation_or_native_call':True},indent=2)+'\n')
shutil.copy2('/workspace/scratch/ct729_prepare_map14_source.py',s/'prepare_source14_metadata_only.py')
print('new Rust source14 assembled; inherited method bodies not edited')
