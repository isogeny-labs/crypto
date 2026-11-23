-- Exact native SQLite analysis. Every required integrity check aborts on zero.
CREATE TABLE required(ok INTEGER NOT NULL CHECK(ok=1));
INSERT INTO required SELECT COUNT(*)=1 FROM rows WHERE json_extract(payload,'$.kind')='scope';
INSERT INTO required SELECT COUNT(*)=1 FROM rows WHERE json_extract(payload,'$.kind')='total';
INSERT INTO required SELECT json_extract(payload,'$.extension_degree')=1 AND
  json_extract(payload,'$.exclusive_prime_limit') BETWEEN 5 AND 500 FROM rows WHERE json_extract(payload,'$.kind')='scope';
CREATE TABLE numbers AS WITH RECURSIVE ns(n) AS (VALUES(0) UNION ALL SELECT n+1 FROM ns WHERE n<1024) SELECT n FROM ns;
CREATE TABLE primes AS SELECT n AS p FROM numbers q WHERE n>=2 AND n<
  (SELECT json_extract(payload,'$.exclusive_prime_limit') FROM rows WHERE json_extract(payload,'$.kind')='scope')
  AND NOT EXISTS(SELECT 1 FROM numbers d WHERE d.n>=2 AND d.n*d.n<=q.n AND q.n%d.n=0);
CREATE TABLE models AS SELECT id,json_extract(payload,'$.p') AS p,
  json_extract(payload,'$.a') AS coefficients,json_extract(payload,'$.a[3]') AS a,
  json_extract(payload,'$.a[4]') AS b,json_extract(payload,'$.orbit_size') AS weight,
  json_extract(payload,'$.N') AS N,json_extract(payload,'$.trace') AS trace,
  json_extract(payload,'$.source_supersingular') AS ss,
  json_extract(payload,'$.divisor_max_embedding_degree') AS divisor_max,
  json_extract(payload,'$.r2_killed') AS killed2,json_extract(payload,'$.r3_killed') AS killed3,payload
  FROM rows WHERE json_extract(payload,'$.kind')='model';
CREATE UNIQUE INDEX unique_models ON models(p,coefficients);
INSERT INTO required SELECT COUNT(*)=0 FROM models WHERE p NOT IN(SELECT p FROM primes) OR weight<1 OR N<1 OR trace!=p+1-N OR ss!=(trace%p=0);
INSERT INTO required SELECT COUNT(*)=0 FROM models WHERE N>CASE WHEN p>=5 THEN 2*p+1 ELSE p*p+1 END;
INSERT INTO required SELECT COUNT(*)=0 FROM models WHERE p>=5 AND
  (json_extract(payload,'$.a[0]')!=0 OR json_extract(payload,'$.a[1]')!=0 OR json_extract(payload,'$.a[2]')!=0);
INSERT INTO required SELECT COUNT(*)=0 FROM models,json_each(models.coefficients) AS c
  WHERE json_array_length(coefficients)!=5 OR c.value<0 OR c.value>=models.p;
INSERT INTO required SELECT NOT EXISTS(SELECT p FROM primes EXCEPT
  SELECT json_extract(payload,'$.p') FROM rows WHERE json_extract(payload,'$.kind')='field') AND
  NOT EXISTS(SELECT json_extract(payload,'$.p') FROM rows WHERE json_extract(payload,'$.kind')='field' EXCEPT SELECT p FROM primes);
INSERT INTO required SELECT (SELECT COUNT(*) FROM primes)=
  (SELECT COUNT(*) FROM rows WHERE json_extract(payload,'$.kind')='field');

-- Reconstruct every short scaling orbit with ordinary integer arithmetic.
CREATE TABLE scales AS SELECT p,n AS u,(n*n)%p AS u2,
  (((n*n)%p)*((n*n)%p))%p AS u4,
  (((((n*n)%p)*((n*n)%p))%p)*((n*n)%p))%p AS u6
  FROM primes JOIN numbers ON n>0 AND n<p WHERE p>=5;
CREATE INDEX scales_p ON scales(p);
CREATE TABLE covered(p INTEGER,a INTEGER,b INTEGER,owner INTEGER,UNIQUE(p,a,b));
INSERT OR IGNORE INTO covered SELECT m.p,(s.u4*m.a)%m.p,(s.u6*m.b)%m.p,m.id
  FROM models m JOIN scales s ON s.p=m.p WHERE m.p>=5 ORDER BY m.id,s.u;
CREATE INDEX covered_owner ON covered(owner);
INSERT INTO required SELECT COUNT(*)=0 FROM models m WHERE p>=5 AND
  weight!=(SELECT COUNT(*) FROM covered c WHERE c.owner=m.id);
INSERT INTO required SELECT COUNT(*)=0 FROM covered WHERE (4*a*a*a+27*b*b)%p=0;
CREATE TABLE expected_short_counts AS SELECT q.p,COUNT(*) AS n FROM primes q
  JOIN numbers x ON x.n<q.p JOIN numbers y ON y.n<q.p
  WHERE q.p>=5 AND (4*x.n*x.n*x.n+27*y.n*y.n)%q.p!=0 GROUP BY q.p;
INSERT INTO required SELECT COUNT(*)=0 FROM expected_short_counts e WHERE
  e.n!=(SELECT COUNT(*) FROM covered c WHERE c.p=e.p);
INSERT INTO required SELECT COUNT(*)=0 FROM primes q WHERE q.p<5 AND
  (SELECT COUNT(*) FROM models m WHERE m.p=q.p)!=(q.p-1)*q.p*q.p*q.p*q.p;
INSERT INTO required SELECT COUNT(*)=0 FROM models WHERE p<5 AND weight!=1;

-- Reconstruct #E, E[2] and E[3] from integer residue roots, independently
-- of the census's table-field point additions and scalar multiplication.
CREATE TABLE square_counts AS SELECT q.p,(y.n*y.n)%q.p AS value,COUNT(*) AS roots
  FROM primes q JOIN numbers y ON y.n<q.p GROUP BY q.p,value;
CREATE UNIQUE INDEX square_lookup ON square_counts(p,value);
CREATE TABLE polynomial_counts AS SELECT m.id,1+SUM(COALESCE(s.roots,0)) AS N,
  1+SUM(CASE WHEN (x.n*x.n*x.n+m.a*x.n+m.b)%m.p=0 THEN 1 ELSE 0 END) AS killed2,
  1+SUM(CASE WHEN (3*x.n*x.n*x.n*x.n+6*m.a*x.n*x.n+12*m.b*x.n-m.a*m.a)%m.p=0
    THEN COALESCE(s.roots,0) ELSE 0 END) AS killed3
  FROM models m JOIN numbers x ON x.n<m.p
  LEFT JOIN square_counts s ON s.p=m.p AND s.value=(x.n*x.n*x.n+m.a*x.n+m.b)%m.p
  WHERE m.p>=5 GROUP BY m.id;
CREATE UNIQUE INDEX polynomial_lookup ON polynomial_counts(id);
INSERT INTO required SELECT COUNT(*)=0 FROM models m JOIN polynomial_counts c ON c.id=m.id
  WHERE m.N!=c.N OR m.killed2!=c.killed2 OR m.killed3!=c.killed3;

CREATE TABLE factors AS SELECT m.id,m.p,m.weight,m.N,m.ss,
  json_extract(f.value,'$.r') AS r,json_extract(f.value,'$.killed') AS killed,
  json_extract(f.value,'$.prime_to_p') AS coprime,json_extract(f.value,'$.r_div_p_minus_one') AS unit,
  json_extract(f.value,'$.r2_div_N') AS r2,json_extract(f.value,'$.full_size') AS full,
  json_extract(f.value,'$.qualified_k1_mismatch') AS qualified_mismatch,
  json_extract(f.value,'$.unrestricted_k1_mismatch') AS unrestricted_mismatch,
  json_extract(f.value,'$.embedding_degree') AS degree
  FROM models m,json_each(m.payload,'$.factors') f;
CREATE UNIQUE INDEX unique_factors ON factors(id,r);
INSERT INTO required SELECT COUNT(*)=0 FROM factors f WHERE r<2 OR N%r!=0 OR
  EXISTS(SELECT 1 FROM numbers d WHERE d.n>=2 AND d.n*d.n<=f.r AND f.r%d.n=0) OR
  killed<1 OR killed>N OR coprime!=(p!=r) OR unit!=((p-1)%r=0) OR
  r2!=(N%(r*r)=0) OR full!=(killed=r*r) OR
  qualified_mismatch!=(coprime AND NOT unit AND full) OR
  unrestricted_mismatch!=(coprime AND full!=unit);
INSERT INTO required SELECT COUNT(*)=0 FROM models m JOIN numbers d ON d.n>=2 AND d.n<=m.N
  WHERE m.N%d.n=0 AND NOT EXISTS(SELECT 1 FROM numbers z WHERE z.n>=2 AND z.n*z.n<=d.n AND d.n%z.n=0)
  AND NOT EXISTS(SELECT 1 FROM factors f WHERE f.id=m.id AND f.r=d.n);
CREATE TABLE divisors AS SELECT m.id,m.p,m.weight,m.N,json_extract(d.value,'$.d') AS d,
  json_extract(d.value,'$.embedding_degree') AS degree FROM models m,json_each(m.payload,'$.ss_divisors') d;
CREATE UNIQUE INDEX unique_divisors ON divisors(id,d);
INSERT INTO required SELECT COUNT(*)=0 FROM divisors WHERE d<2 OR N%d!=0 OR d%p=0;
INSERT INTO required SELECT COUNT(*)=0 FROM models m JOIN numbers d ON d.n>=2 AND d.n<=m.N
  WHERE m.ss AND m.N%d.n=0 AND d.n%m.p!=0 AND
  NOT EXISTS(SELECT 1 FROM divisors s WHERE s.id=m.id AND s.d=d.n);
INSERT INTO required SELECT COUNT(*)=0 FROM models m WHERE divisor_max!=
  COALESCE((SELECT MAX(degree) FROM divisors d WHERE d.id=m.id),0);

-- Recompute every distinct required modular order, including composites.
CREATE TABLE order_inputs AS SELECT DISTINCT p,r AS n FROM factors WHERE coprime
  UNION SELECT DISTINCT p,d AS n FROM divisors;
CREATE TABLE modular_orders AS WITH RECURSIVE states(p,n,k,x) AS (
  SELECT p,n,1,p%n FROM order_inputs UNION ALL
  SELECT p,n,k+1,(x*p)%n FROM states WHERE x!=1 AND k<n
) SELECT p,n,k FROM states WHERE x=1;
CREATE UNIQUE INDEX modular_order_lookup ON modular_orders(p,n);
INSERT INTO required SELECT (SELECT COUNT(*) FROM modular_orders)=(SELECT COUNT(*) FROM order_inputs);
INSERT INTO required SELECT COUNT(*)=0 FROM factors f LEFT JOIN modular_orders o ON o.p=f.p AND o.n=f.r
  WHERE (f.coprime AND (o.k IS NULL OR f.degree!=o.k)) OR (NOT f.coprime AND f.degree!=0);
INSERT INTO required SELECT COUNT(*)=0 FROM divisors d JOIN modular_orders o ON o.p=d.p AND o.n=d.d WHERE d.degree!=o.k;

CREATE TABLE derived AS SELECT json_object(
 'models',(SELECT SUM(weight) FROM models),'representatives',(SELECT COUNT(*) FROM models),
 'prime_cases',(SELECT SUM(weight) FROM factors),'prime_to_p',(SELECT SUM(CASE WHEN coprime THEN weight ELSE 0 END) FROM factors),
 'characteristic_cases',(SELECT SUM(CASE WHEN NOT coprime THEN weight ELSE 0 END) FROM factors),
 'qualified',(SELECT SUM(CASE WHEN coprime AND NOT unit THEN weight ELSE 0 END) FROM factors),
 'qualified_mismatches',(SELECT SUM(CASE WHEN coprime AND NOT unit AND full THEN weight ELSE 0 END) FROM factors),
 'omitted_condition_failures',(SELECT SUM(CASE WHEN coprime AND full!=unit THEN weight ELSE 0 END) FROM factors),
 'omitted_r2_failures',(SELECT SUM(CASE WHEN coprime AND full!=unit AND r2 THEN weight ELSE 0 END) FROM factors),
 'necessity_mismatches',(SELECT SUM(CASE WHEN coprime AND full AND NOT unit THEN weight ELSE 0 END) FROM factors),
 'ss_models',(SELECT SUM(CASE WHEN ss THEN weight ELSE 0 END) FROM models),
 'ss_prime_cases',(SELECT SUM(CASE WHEN ss AND coprime THEN weight ELSE 0 END) FROM factors),
 'ss_equal_two_failures',(SELECT SUM(CASE WHEN ss AND coprime AND degree!=2 THEN weight ELSE 0 END) FROM factors),
 'ss_prime_bound_failures',(SELECT SUM(CASE WHEN ss AND coprime AND degree>CASE WHEN p>3 THEN 2 ELSE 6 END THEN weight ELSE 0 END) FROM factors),
 'ss_divisor_bound_failures',COALESCE((SELECT SUM(CASE WHEN degree>CASE WHEN p>3 THEN 2 ELSE 6 END THEN weight ELSE 0 END) FROM divisors),0),
 'root2_checks',(SELECT COUNT(*) FROM models WHERE p>=5),'root3_checks',(SELECT COUNT(*) FROM models WHERE p>=5),
 'order_checks',(SELECT SUM(N) FROM models),'point_rows',(SELECT SUM(N) FROM models),
 'max_ss_prime_degree',COALESCE((SELECT MAX(degree) FROM factors WHERE ss AND coprime),0),
 'max_ss_divisor_degree',COALESCE((SELECT MAX(degree) FROM divisors),0)) AS payload;
INSERT INTO required SELECT COUNT(*)=0 FROM derived,json_each(derived.payload) v,
  rows t WHERE json_extract(t.payload,'$.kind')='total' AND json_extract(t.payload,'$.'||v.key) IS NOT v.value;
CREATE TABLE full_by_prime AS SELECT r,SUM(weight) AS n FROM factors WHERE coprime AND full GROUP BY r;
INSERT INTO required SELECT NOT EXISTS(SELECT CAST(j.key AS INTEGER),j.value FROM rows t,json_each(t.payload,'$.full_by_prime') j
  WHERE json_extract(t.payload,'$.kind')='total' EXCEPT SELECT r,n FROM full_by_prime) AND
  NOT EXISTS(SELECT r,n FROM full_by_prime EXCEPT SELECT CAST(j.key AS INTEGER),j.value FROM rows t,json_each(t.payload,'$.full_by_prime') j WHERE json_extract(t.payload,'$.kind')='total');
SELECT json_object('kind','integrity','pass',1,'required_checks',(SELECT COUNT(*) FROM required),
  'prime_fields',(SELECT COUNT(*) FROM primes),'covered_short_models',(SELECT COUNT(*) FROM covered),
  'independent_polynomial_rows',(SELECT COUNT(*) FROM polynomial_counts),
  'modular_orders',(SELECT COUNT(*) FROM modular_orders));
SELECT json_set(payload,'$.kind','independent_totals') FROM derived;
SELECT json_object('kind','full_by_prime','r',r,'models',n) FROM full_by_prime ORDER BY r;
SELECT payload FROM rows WHERE json_extract(payload,'$.kind')='field';
