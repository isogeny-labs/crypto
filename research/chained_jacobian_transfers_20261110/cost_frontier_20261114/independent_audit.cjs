#!/usr/bin/env node
// Independent schema-and-difference check, count-only; not a PCC verdict.
const fs = require('node:fs');
const fixture = process.argv[2] ||
  'research/chained_jacobian_transfers_20261110/evidence/public-n4-complete-chain-checkpoint-01/01-literal-setup-and-selected-vectors.json';
const root = JSON.parse(fs.readFileSync(fixture, 'utf8'));
const fail = (condition, why) => { if (!condition) throw new Error(why); };
const rows = root.all48_actual_selected_vectors?.all48;
fail(Array.isArray(rows) && rows.length === 48, 'exactly 48 selected rows');
fail(Object.keys(root).sort().join() === ['actual_setup','all48_actual_selected_vectors','scope'].sort().join(), 'root schema');
const C13 = 'degree13_elliptic_evaluation', C169 = 'degree169_elliptic_evaluation';
const D = 'norm_point_doubling';
const routes = ['direct_composite','literal_sequential_Jacobian_maps','strongest_retained_lift_chain'];
const originalNames = ['correction_dictionary_lookup','correction_difference',C13,C169,
'explicit_half_check','external_full_order','external_norm_and_geometry',
'external_packet_parse','final_actual_norm','final_full_order',
'generic_halving_factorization','generic_ordinate','infinity_halves',
'inverse_full_group_scalar','matching_corrections_checked',
'quadratic_source_phi','quadratic_target_phi','source_lift_addition',
'target_correction_addition','two_torsion_halves'];
const deltas = {};
const directions = {}, sectors = {}, seen = new Set();
const saved = {'direct_composite':0, 'literal_sequential_Jacobian_maps':0,
  'strongest_retained_lift_chain':0};
rows.forEach((row, i) => {
  fail(Object.keys(row).sort().join() === [...routes, 'from','to','input','sector',
    'scalar_OUTSIDE_runtime','whole_outputs_equal'].sort().join(), 'row schema '+i);
  const edge = row.from+'->'+row.to;
  fail(edge === '0->2' || edge === '2->0', 'edge '+i);
  fail(Number.isInteger(row.sector) && row.sector>=0 && row.sector<4,'sector '+i);
  fail(Number.isInteger(row.scalar_OUTSIDE_runtime) &&
       Array.isArray(row.input) && Array.isArray(row.whole_outputs_equal),'point '+i);
  const id = JSON.stringify([edge,row.sector,row.scalar_OUTSIDE_runtime,row.input]);
  fail(!seen.has(id),'duplicate '+i);
  seen.add(id);
  directions[edge] = (directions[edge]||0)+1;
  sectors[row.sector] = (sectors[row.sector]||0)+1;
  const transformed = {};
  routes.forEach((route, r) => {
    const obj = row[route];
    fail(Object.keys(obj).sort().join()===originalNames.slice().sort().join(),'counter schema '+i+route);
    fail(originalNames.every(k=>Number.isInteger(obj[k])&&obj[k]>=0),'counter '+i+route);
    const kind = r===0?C169:C13, other = r===0?C13:C169;
    fail(obj[kind]===(r===0?2:4)&&obj[other]===0,'original map count '+i+route);
    transformed[route] = {...obj, [kind]:r===0?1:2, [D]:r===1?2:1};
    saved[route]+=r===0?1:2;
  });
  for(const k of [...originalNames,D]){
    const direct = transformed[routes[0]][k], literal=transformed[routes[1]][k],
      retained=transformed[routes[2]][k];
    fail(literal>=retained,'componentwise literal '+i+k);
    if(k!==C13 && k!==C169)fail(direct===retained,'nonmap difference '+i+k);
    if(!(k in deltas)) deltas[k]=[literal-retained,literal-retained];
    deltas[k][0]=Math.min(deltas[k][0],literal-retained);
    deltas[k][1]=Math.max(deltas[k][1],literal-retained);
  }
  fail(transformed[routes[0]][C169]===1&&transformed[routes[2]][C13]===2,'crossover');
});
fail(directions['0->2']===24&&directions['2->0']===24,'directions');
fail(Object.keys(sectors).length===4&&Object.values(sectors).every(x=>x===12),'sectors');
fail(saved.direct_composite===48&&saved.literal_sequential_Jacobian_maps===96&&
saved.strongest_retained_lift_chain===96,'saved map counts');
console.log(JSON.stringify({vectors:48,directions,sectors,saved_map_evaluations:saved,
direct_minus_retained:'C169 - 2*C13, all other recorded counters equal',
literal_componentwise_dominated:true,
literal_minus_retained_ranges:Object.fromEntries(Object.entries(deltas).filter(([k,v])=>v[1]>0))}));
