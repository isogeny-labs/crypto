-- Fixed descriptive native analysis. No clocks and no survivor filtering of the flow.
SELECT json_object('kind','flow','warmup',warmup,'status',status,'rows',COUNT(*))
FROM runs GROUP BY warmup,status ORDER BY warmup,status;

WITH pairs AS (
 SELECT a.record_id aid,b.record_id bid,a.warmup,
   b.record_id IS NULL OR a.total_gae IS NOT b.total_gae OR
   a.algorithm_seed IS NOT b.algorithm_seed OR a.status IS NOT b.status OR
   a.recovered IS NOT b.recovered OR a.method_id IS NOT b.method_id OR
   EXISTS(SELECT phase,adds,doubles,scalar_mults,gae,native_json FROM phases WHERE record_id=a.record_id
          EXCEPT SELECT phase,adds,doubles,scalar_mults,gae,native_json FROM phases WHERE record_id=b.record_id) OR
   EXISTS(SELECT phase,adds,doubles,scalar_mults,gae,native_json FROM phases WHERE record_id=b.record_id
          EXCEPT SELECT phase,adds,doubles,scalar_mults,gae,native_json FROM phases WHERE record_id=a.record_id) OR
   EXISTS(SELECT name,value FROM run_counters WHERE record_id=a.record_id
          EXCEPT SELECT name,value FROM run_counters WHERE record_id=b.record_id) OR
   EXISTS(SELECT name,value FROM run_counters WHERE record_id=b.record_id
          EXCEPT SELECT name,value FROM run_counters WHERE record_id=a.record_id) AS mismatch
 FROM runs a LEFT JOIN runs b ON b.session_id=a.session_id AND b.workload_id=a.workload_id
   AND b.round=a.round AND b.warmup=a.warmup AND b.arm='rho-neg-aa'
 WHERE a.arm='rho-neg'
)
SELECT json_object('kind','aa','warmup',warmup,'pairs',COUNT(*),
                  'mismatches',SUM(mismatch),'pass',COUNT(*)>0 AND SUM(mismatch)=0)
FROM pairs GROUP BY warmup ORDER BY warmup;

SELECT json_object('kind','count_integrity','noninteger_or_large',COUNT(*)) FROM runs
WHERE total_gae IS NULL OR total_gae<>CAST(total_gae AS INTEGER) OR total_gae<0 OR total_gae>9007199254740991;

SELECT json_object('kind','curve','curve',w.curve_slug,'r',c.r,'arm',r.arm,'n',COUNT(*),
 'total_gae',SUM(CAST(r.total_gae AS INTEGER)),'mean_s',AVG(r.s),'min_s',MIN(r.s),'max_s',MAX(r.s),
 'lower_bound_rows',SUM(r.lower_bound),'unpriced_names',GROUP_CONCAT(DISTINCT r.unpriced))
FROM runs r JOIN workloads w USING(workload_id) JOIN curves c ON c.slug=w.curve_slug
WHERE r.warmup=0 GROUP BY w.curve_slug,c.r,r.arm ORDER BY CAST(c.r AS INTEGER),r.arm;

SELECT json_object('kind','phase','curve',w.curve_slug,'arm',r.arm,'phase',p.phase,
 'adds',SUM(p.adds),'doubles',SUM(p.doubles),'gae',SUM(CAST(p.gae AS INTEGER)))
FROM phases p JOIN runs r USING(record_id) JOIN workloads w USING(workload_id)
WHERE r.warmup=0 GROUP BY w.curve_slug,r.arm,p.phase ORDER BY w.curve_slug,r.arm,p.phase;

SELECT json_object('kind','counter','curve',w.curve_slug,'arm',r.arm,'name',q.name,'count',SUM(q.value))
FROM run_counters q JOIN runs r USING(record_id) JOIN workloads w USING(workload_id)
WHERE r.warmup=0 GROUP BY w.curve_slug,r.arm,q.name ORDER BY w.curve_slug,r.arm,q.name;
