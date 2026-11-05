#!/usr/bin/env python3
"""Translate native integer coefficient arrays literally into Lean expressions."""
from pathlib import Path
import json,hashlib
h=Path(__file__).resolve().parent
gpath=h/'runs/global42-01/output/global-recovery.json'
spath=h/'runs/residual42-01/output/residual-systems.json'
data=json.loads(gpath.read_text());spectra={c['id']:c for c in json.loads(spath.read_text())['cases']}
def expr(cs):
 terms=[]
 for i,c in enumerate(cs):
  if c=='0':continue
  term='('+c+' : R)'
  if i:term+=f'*t^{i}'
  terms.append(term)
 return '('+' + '.join(terms)+')' if terms else '(0 : R)'
out=['import CMRecovery42','set_option maxRecDepth 100000','set_option maxHeartbeats 0','namespace CMGlobalCertificates42','variable {R : Type*} [CommRing R]']
mapping=[]
for c in data['cases']:
 if c['status']!='certified':continue
 n=c['n'];name=f'n{n}';s=spectra[c['id']];assert c['modulus']==s['target_modulus']
 i,j=s['CM_exponents'];r=c['modulus']
 fields={'raw':'raw_denominator','red':'reduced_denominator','phi':'phi','redq':'reduction_quotient','inv':'inverse_denominator','invq':'inverse_phi_quotient','invr':'inverse_r_multiple'}
 for short,key in fields.items():out.append(f'def {name}_{short} (t : R) : R := {expr(c[key])}')
 out.append(f'''theorem {name}_raw_formula (t : R) :
    {name}_raw t = 2*(t^{j}-t^{i})*(2*(t^{i})^2+3*t^{i}*t^{j}+2*(t^{j})^2) := by
  dsimp [{name}_raw]
  ring

theorem {name}_denominator_reduction (t : R) :
    {name}_raw t = {name}_red t+{name}_phi t*{name}_redq t := by
  dsimp [{name}_raw, {name}_red, {name}_phi, {name}_redq]
  ring

theorem {name}_integer_inverse_certificate (t : R) :
    {name}_red t*{name}_inv t =
      1+{name}_phi t*{name}_invq t+({r} : R)*{name}_invr t := by
  dsimp [{name}_red, {name}_inv, {name}_phi, {name}_invq, {name}_invr]
  ring

theorem {name}_denominator_unit (t : R) (hp : {name}_phi t=0)
    (hr : ({r} : R)=0) :
    {name}_raw t*{name}_inv t=1 := by
  have hd : {name}_raw t={name}_red t := by
    rw [{name}_denominator_reduction, hp, zero_mul, add_zero]
  rw [hd, {name}_integer_inverse_certificate, hp, zero_mul, hr, zero_mul]
  simp

theorem {name}_CM_block_recovery (f t s : R) (hs : s^2=1)
    (hp : {name}_phi t=0) (hr : ({r} : R)=0) :
    ((t^{i}+2*t^{j}-s*f)*{name}_inv t)*(f^2+s*t^{j}*f+2*(t^{j})^2)-
    ((t^{j}+2*t^{i}-s*f)*{name}_inv t)*(f^2+s*t^{i}*f+2*(t^{i})^2)=1 := by
  have hu := {name}_denominator_unit t hp hr
  rw [{name}_raw_formula] at hu
  exact CMRecovery42.normalized_quadratic_recovery f (t^{i}) (t^{j}) s ({name}_inv t) hs hu
''')
 mapping.append({'case':c['id'],'namespace':'CMGlobalCertificates42.'+name,'declarations':['raw_formula','denominator_reduction','integer_inverse_certificate','denominator_unit','CM_block_recovery'],'scope':'actual integer polynomial identities and conditional CM-block recovery; no geometric array or target map'})
out.append('end CMGlobalCertificates42')
(h/'lean/CMGlobalCertificates42.lean').write_text('\n\n'.join(out)+'\n')
(h/'global-proof-map.json').write_text(json.dumps({'global_native_sha256':hashlib.sha256(gpath.read_bytes()).hexdigest(),'spectral_native_sha256':hashlib.sha256(spath.read_bytes()).hexdigest(),'cases':mapping},indent=2)+'\n')
print('Assembled actual polynomial identities and recovery for',len(mapping),'large cases, fifteen theorems.')
