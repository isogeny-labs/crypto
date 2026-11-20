use hec_native_ic::fp::Fp;
pub type Row=Vec<(u32,u64)>;
pub type Id=usize;
enum Node{Zero,Leaf(Row),Sum{a:Id,ca:u64,b:Id,cb:u64}}
pub struct Circuit{f:Fp,dim:usize,nodes:Vec<Node>,pub roots:Vec<Id>,pub leaf_entries:usize}
impl Circuit{
 pub fn new(dim:usize,p:u64)->Self{Self{f:Fp::new(p),dim,nodes:vec![Node::Zero],roots:vec![],leaf_entries:0}}
 pub fn leaf(&mut self,r:Row)->Id{assert!(r.iter().all(|&(i,a)|(i as usize)<self.dim&&a<self.f.p));self.leaf_entries+=r.len();let id=self.nodes.len();self.nodes.push(Node::Leaf(r));id}
 pub fn combine(&mut self,a:Id,ca:u64,b:Id,cb:u64)->Id{assert!(a<self.nodes.len()&&b<self.nodes.len());let(ca,cb)=(if a==0{0}else{ca},if b==0{0}else{cb});if ca==0&&cb==0{return 0}if ca==1&&cb==0{return a}if ca==0&&cb==1{return b}let id=self.nodes.len();self.nodes.push(Node::Sum{a,ca,b,cb});id}
 pub fn nodes(&self)->usize{self.nodes.len()}
 pub fn sums(&self)->usize{self.nodes.iter().filter(|n|matches!(n,Node::Sum{..})).count()}
 pub fn apply(&self,x:&[u64])->Vec<u64>{assert_eq!(x.len(),self.dim);let mut z=vec![0;self.nodes.len()];for(i,n)in self.nodes.iter().enumerate(){z[i]=match n{Node::Zero=>0,Node::Leaf(r)=>r.iter().fold(0,|v,&(c,a)|self.f.add(v,self.f.mul(a,x[c as usize]))),Node::Sum{a,ca,b,cb}=>self.f.add(self.f.mul(*ca,z[*a]),self.f.mul(*cb,z[*b]))};}self.roots.iter().map(|&r|z[r]).collect()}
 pub fn transpose(&self,y:&[u64])->Vec<u64>{assert_eq!(y.len(),self.roots.len());let mut z=vec![0;self.nodes.len()];let mut x=vec![0;self.dim];for(&r,&v)in self.roots.iter().zip(y){z[r]=self.f.add(z[r],v);}for(i,n)in self.nodes.iter().enumerate().rev(){match n{Node::Zero=>{},Node::Leaf(r)=>for&(c,a)in r{x[c as usize]=self.f.add(x[c as usize],self.f.mul(a,z[i]));},Node::Sum{a,ca,b,cb}=>{z[*a]=self.f.add(z[*a],self.f.mul(*ca,z[i]));z[*b]=self.f.add(z[*b],self.f.mul(*cb,z[i]));}}}x}
 pub fn expand(&self,id:Id)->Row{assert!(id<self.nodes.len());// Expansion only for controls, via reverse accumulation.
 let mut z=vec![0;self.nodes.len()];let mut x=vec![0;self.dim];z[id]=1;for(i,n)in self.nodes.iter().enumerate().rev(){match n{Node::Zero=>{},Node::Leaf(r)=>for&(c,a)in r{x[c as usize]=self.f.add(x[c as usize],self.f.mul(a,z[i]));},Node::Sum{a,ca,b,cb}=>{z[*a]=self.f.add(z[*a],self.f.mul(*ca,z[i]));z[*b]=self.f.add(z[*b],self.f.mul(*cb,z[i]));}}}x.into_iter().enumerate().filter(|&(_,a)|a!=0).map(|(i,a)|(i as u32,a)).collect()}
}
pub struct Graph{f:Fp,parent:Vec<usize>,size:Vec<usize>,scale:Vec<u64>,potential:Vec<Id>,pub circuit:Circuit,pub finds:usize,pub links:usize}
impl Graph{
 pub fn new(n:usize,dim:usize,p:u64)->Self{Self{f:Fp::new(p),parent:(0..=n).collect(),size:vec![1;n+1],scale:vec![1;n+1],potential:vec![0;n+1],circuit:Circuit::new(dim,p),finds:0,links:0}}
 pub fn find(&mut self,v:usize)->(usize,u64,Id){self.finds+=1;let p=self.parent[v];if p==v{return(v,1,0)}let(r,c,e)=self.find(p);let s=self.scale[v];let out=self.circuit.combine(self.potential[v],1,e,s);let c=self.f.mul(s,c);self.parent[v]=r;self.scale[v]=c;self.potential[v]=out;(r,c,out)}
 pub fn insert(&mut self,large:&[(usize,u64)],row:Row)->Option<Id>{assert!(large.len()<=2);assert!(large.iter().all(|&(i,a)|i>0&&i<self.parent.len()&&a>0&&a<self.f.p));let row=self.circuit.leaf(row);let result=if large.is_empty(){Some(row)}else{let(u,alpha)=large[0];let(v,beta)=if large.len()==2{large[1]}else{(0,1)};let(ru,su,eu)=self.find(u);let(rv,sv,ev)=self.find(v);let p=self.f.mul(alpha,su);let q=self.f.mul(beta,sv);let r=self.circuit.combine(row,1,eu,alpha);let r=self.circuit.combine(r,1,ev,beta);
 if ru==rv{if ru==0||self.f.add(p,q)==0{Some(r)}else{let inv=self.f.inv(self.f.add(p,q));self.parent[ru]=0;self.scale[ru]=1;self.potential[ru]=self.circuit.combine(r,self.f.neg(inv),0,0);self.size[0]+=self.size[ru];self.links+=1;None}}else{let(child,root,c,den)=if ru==0||(rv!=0&&self.size[ru]>=self.size[rv]){(rv,ru,p,q)}else{(ru,rv,q,p)};let inv=self.f.inv(den);self.parent[child]=root;self.scale[child]=self.f.neg(self.f.mul(c,inv));self.potential[child]=self.circuit.combine(r,self.f.neg(inv),0,0);self.size[root]+=self.size[child];self.links+=1;None}};if let Some(id)=result{self.circuit.roots.push(id);}assert!(self.circuit.nodes()<5_000_000,"circuit_node_cap");result}
}
