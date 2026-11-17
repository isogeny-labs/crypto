from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-cached-shifts';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name in ('interval.rs','packed-helper.rs'):
  t=t.replace('cache_only_batches:usize}','cache_only_batches:usize,polynomial_setups:usize,progression_setups:usize}')
  t=t.replace('cache_only_batches:0})','cache_only_batches:0,polynomial_setups:0,progression_setups:0})')
  t=t.replace('if self.progression.is_none(){','if self.progression.is_none(){self.progression_setups+=1;')
  t=t.replace('if self.polynomial.is_none(){','if self.polynomial.is_none(){self.polynomial_setups+=1;')
 if f.name=='stream-adapter.rs':
  isolated=t.replace('fn recover_traced(', 'fn recover_isolated(')
  session=t.replace('impl StreamModes{','impl RecoveryModes{')
  wrapper="""impl StreamModes{fn recover_traced(&self,source:&source::Source,curve:group::Curve,base:group::Point,target:Option<group::Point>,radii:&[u64],cap:usize,extra:&[u64],max_calls:usize)->Result<(Option<u64>,PullStats,&'static str),Error>{RecoveryModes::from_modes(self)?.recover_traced(source,curve,base,target,radii,cap,extra,max_calls)}}"""
  t=isolated+session+wrapper
 if f.name=='streamed.rs':t+='\ninclude!("context.rs");\n'
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){context_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
s=(src/'streamed.rs').read_text()
coeff=s[s.index('struct CoeffStream'):s.index('/// No coefficient')]
coeff=coeff.replace('struct CoeffStream{',"struct SharedCoeffStream<'a>{").replace('impl CoeffStream{',"impl<'a> SharedCoeffStream<'a>{").replace('products:IntervalProducts',"products:&'a RecoveryProducts")
coeff=coeff.replace('cap:usize)->Result<Self,Error>{',"cap:usize,products:&'a RecoveryProducts)->Result<Self,Error>{",1)
coeff=coeff.replace('let mut products=IntervalProducts::new(r,m as u64,cap)?;', 'if (r,m,cap)!=(products.r,products.m,products.cap){return Err(Error::Domain)}')
methods=s[s.index(' fn stream(&self'):s.index('include!("stream-checks.rs")')]
methods=methods.replace('Result<CoeffStream,Error>{CoeffStream::new(self.r,self.m,e,self.block)}',"Result<SharedCoeffStream<'_>,Error>{SharedCoeffStream::new(self.r,self.m,e,self.block,&self.products)}")
head="""struct RecoveryProducts{r:u64,m:usize,cap:usize,inner:std::cell::RefCell<IntervalProducts>}
impl RecoveryProducts{
 fn new(r:u64,m:usize,cap:usize)->Result<Self,Error>{Ok(Self{r,m,cap,inner:std::cell::RefCell::new(IntervalProducts::new(r,m as u64,cap)?)})}
 fn products(&self,requests:&[(u64,u64)])->Result<Vec<u64>,Error>{self.inner.try_borrow_mut().map_err(|_|Error::Arithmetic)?.products(requests)}
}
struct RecoveryModes{r:u64,m:usize,zeta:u64,block:usize,products:RecoveryProducts}
impl RecoveryModes{
 fn from_modes(m:&StreamModes)->Result<Self,Error>{Ok(Self{r:m.r,m:m.m,zeta:m.zeta,block:m.block,products:RecoveryProducts::new(m.r,m.m,m.block)?})}
"""
(p/'context.rs').write_text(coeff+head+methods+'\ninclude!("context-checks.rs");\n');outs.append('context.rs')
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
