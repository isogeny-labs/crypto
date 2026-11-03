// Natural per-curve/per-edge tables of numeric measurements. No witness/RNG fields.
import fs from 'node:fs';
const root=process.argv[2],out=root+'/reports/supplement_tables_20261025';
if(fs.existsSync(out))throw Error('write-once export');fs.mkdirSync(out,{recursive:true});
const csv=(p,h,rows)=>{const s=[h,...rows].map(r=>r.map(v=>v??'').join(',')).join('\n')+'\n';if(Buffer.byteLength(s)>65536)throw Error('artifact policy '+p);fs.mkdirSync(out+'/'+p.split('/').slice(0,-1).join('/'),{recursive:true});fs.writeFileSync(out+'/'+p,s);};
const lines=p=>fs.readFileSync(root+'/'+p,'utf8').trim().split('\n').map(JSON.parse);
const gp=lines('data/order_gp_1525.jsonl');
for(let curve=0;curve<46;curve++){
 const order=lines('runs/order_full_1525/curve_'+curve+'.jsonl'),keys=Object.keys(order[0].counters);
 csv('orders/C'+curve+'_native.csv',['instance','role','returned_order','wall_ns','process_cpu_ns',...keys],order.map(r=>[r.instance,r.role,r.returned_order,r.wall_ns,r.process_cpu_ns,...keys.map(k=>r.counters[k])]));
 const rows=gp.filter(r=>r.curve===curve),gkeys=[...new Set(rows.flatMap(r=>Object.keys(r)))];
 csv('orders/C'+curve+'_pari.csv',gkeys,rows.map(r=>gkeys.map(k=>typeof r[k]==='string'?r[k].replaceAll(',',';'):r[k])));
 const ic=lines('runs/ic/curve_'+curve+'.jsonl');
 for(const stage of ['setup','target','positive_control']){
  const rows=ic.filter(r=>r.stage===stage),counters=Object.keys(rows[0].gb_counters);
  csv('ic/'+stage+'/C'+curve+'.csv',['generator','index','instance','unit_ideal','grid_roots','relations','construction_ns','gb_wall_ns','gb_process_cpu_ns','root_and_relation_ns','independent_checks_ns','whole_probe_wall_ns','top_forms_Hilbert_cutoff',...counters],rows.map(r=>[r.generator,r.index??r.key_index,r.instance??'',r.unit_ideal,r.roots.length,r.relations.length,r.construction_ns,r.gb_wall_ns,r.gb_process_cpu_ns,r.root_and_relation_ns,r.independent_checks_ns,r.whole_probe_wall_ns,r.dreg_top_forms_first_zero_hilbert_degree,...counters.map(k=>r.gb_counters[k])]));
 }
}
const transport=lines('data/transport_measurements.jsonl');
for(let c=0;c<46;c++)csv('transport/C'+c+'.csv',['instance','generator','key_index','repeat','steps','pair_wall_ns','pair_cpu_ns','Fp_multiply_including_inverse_power','Fp_add_sub','Fp_inverse','verified'],transport.filter(r=>r.curve===c).map(r=>[r.instance,r.generator,r.key_index,r.repeat,r.steps,r.pair_wall_ns,r.pair_process_cpu_ns,r.counters.field_multiply_including_inverse_power,r.counters.field_add_sub,r.counters.field_inverse,r.verified_public_P_Q_images]));
const map=lines('data/map_measurements.jsonl'),mkeys=['edge','source','target','instance','generator','key_index','repeat','pair_wall_ns','pair_process_cpu_ns','field_multiply_including_inverse_power','field_add_sub','field_inverse','verified'];
for(let edge=0;edge<90;edge++){const rows=map.filter(r=>r.edge===edge);if(rows.length!==500)throw Error('map coverage');csv('directed_maps/E'+edge+'.csv',mkeys,rows.map(r=>{const x={...r,...r.counters};return mkeys.map(k=>k==='generator'?Math.floor(r.instance/10):k==='key_index'?r.instance%10:x[k]);}));}
const regSetups=lines('sessions/regularity_cert_20261025/setups.jsonl');for(let c=0;c<46;c++){const a=regSetups.filter(x=>x.curve===c),keys=Object.keys(a[0].counters);csv('regularity_setup/C'+c+'.csv',['generator','wall_ns','cpu_ns',...keys],a.map(x=>[x.generator,x.wall_ns,x.process_cpu_ns,...keys.map(k=>x.counters[k])]));}
fs.writeFileSync(out+'/README.md','# Additional numeric projections\n\nOrders: 46 curves, 9200 native and 9200 PARI point checks. IC: all 14720 setup/target/control polynomial systems with timing and original arithmetic/degree counters. Transport: all 23000 full-path pair evaluations and 45000 directed-edge pair evaluations. Natural per-curve, per-stage and per-edge CSV units retain every selected observation. Original scalar-bearing journals remain local; no scalar witness or raw RNG word is projected. Missing measurements stay blank.\n');
console.log(JSON.stringify({out,order_curves:46,IC_systems:14720,transport:transport.length,directed_maps:map.length}));
