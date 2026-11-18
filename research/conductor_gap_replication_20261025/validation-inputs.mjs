// Metadata export only. All arithmetic checks are in the independent C++ programs.
import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {createHash} from 'node:crypto';
const root='research/conductor_gap_replication_20261025';
const load=p=>JSON.parse(readFileSync(root+'/'+p,'utf8'));
const dir=process.argv[2]??root+'/validation';mkdirSync(dir,{recursive:true});
const fixtures=['panel,class,d,bits,p,t,order,r,ell,frobenius_conductor,kernel_degree'];
for(const c of load('prepared-a/a-classes.json')){
  const x=c.pair.class;
  fixtures.push(['A',c.class,c.d,c.bits,x.p,x.t,x.order,x.n,x.gap_prime,x.frobenius_conductor,1].join(','));
}
for(const c of load('prepared-b-v2/b-classes.json'))
  fixtures.push(['B',c.class,c.d,0,c.p,c.t,c.r,c.r,c.gap_prime,c.gap_prime,c.kernel_abscissa_degree].join(','));
writeFileSync(dir+'/fixtures.csv',fixtures.join('\n')+'\n',{flag:'wx'});
const vertices=['class,volcano,vertex,level,p,a,b,j,parent,nbr0,nbr1,nbr2'];
const sources=[];
for(let ci=0;ci<8;ci++)for(let vi=0;vi<4;vi++){
  const path=`prepared-a/a-class${ci}/volcano${vi}/curves.jsonl`;
  const raw=readFileSync(root+'/'+path);
  sources.push({path,sha256:createHash('sha256').update(raw).digest('hex')});
  for(const c of raw.toString().trim().split('\n').map(JSON.parse).filter(x=>x.model===0))
    vertices.push([ci,vi,c.vertex,c.level,c.p,c.a,c.b,c.j,c.parent??-1,...[0,1,2].map(i=>c.nbrs[i]??-1)].join(','));
}
writeFileSync(dir+'/vertices.csv',vertices.join('\n')+'\n',{flag:'wx'});
writeFileSync(dir+'/export-sources.json',JSON.stringify(sources,null,2)+'\n',{flag:'wx'});
