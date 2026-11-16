from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;b=p.parent/'feature-stable-envelope'
fs=['modes.rs','reference.rs','adapter.rs','orbit.rs','dynamic.rs','envelope.rs']
for f in fs:
 s=(b/f).read_text()
 if f=='envelope.rs':s=s.replace('fn main(){','fn envelope_checks(){')+'\ninclude!("trace.rs");\n'
 (p/f).write_text(s)
# Preserve the entire existing dynamic routine, changing only closure construction.
s=(b/'envelope.rs').read_text();start=s.index('fn envelope_history(');end=s.index('\ninclude!("dynamic.rs");',start)
body=s[start:end].replace('fn envelope_history(','fn trace_history(')
old='let(envelope,stats)=stable_envelope(p,&state,initial,zeta,m,r,cap)?;'
new='let data=trace_image(p,&state,m,r,cap,cap)?;let envelope=data.envelope;let stats=EnvelopeStats::default();if p.divrem(initial,&envelope).map_err(|_|Error::Arithmetic)?.1!=[0]{return Err(Error::Arithmetic)}'
assert old in body;body=body.replace(old,new)
(p/'history.rs').write_text(body)
s=(b/'dynamic.rs').read_text().replace('recover_enveloped(','recover_traced(').replace('envelope_history(','trace_history(')
(p/'trace-adapter.rs').write_text(s)
(p/'generation.json').write_text(json.dumps({'sources':{str(b/f):hashlib.sha256((b/f).read_bytes()).hexdigest() for f in fs},'outputs':{f:hashlib.sha256((p/f).read_bytes()).hexdigest() for f in fs+['history.rs','trace-adapter.rs']}},indent=2)+'\n')

# Generate the blocked alternative from the preserved sequential image function.
t=(p/'trace.rs').read_text();start=t.index('fn trace_image(');end=t.index('\ninclude!("block-projection.rs");',start)
a=t[start:end].replace('fn trace_image(','fn trace_image_blocked(').replace('max_output:usize)->','max_output:usize,max_workspace:usize)->')
lo=a.index(' // -Q');hi=a.index(' let characteristic=',lo)
a=a[:lo]+' let b=power_mod(p,g,m)?;let moments=blocked_moments(p,g,&b,r,max_workspace)?;\n'+a[hi:]
(p/'blocked-image.rs').write_text(a)
h=(p/'history.rs').read_text().replace('trace_image(p,&state,m,r,cap,cap)?','trace_image_blocked(p,&state,m,r,cap,cap,cap.checked_mul(cap).ok_or(Error::Limit)?)?')
(p/'history.rs').write_text(h)
receipt=json.loads((p/'generation.json').read_text());receipt['sources'][str(p/'trace.rs')]=hashlib.sha256((p/'trace.rs').read_bytes()).hexdigest()
for f in ['blocked-image.rs','history.rs']:receipt['outputs'][f]=hashlib.sha256((p/f).read_bytes()).hexdigest()
(p/'generation.json').write_text(json.dumps(receipt,indent=2)+'\n')
