-- 0014: class groups of imaginary quadratic orders and the order of the class
-- of a prime above a small l (src/import/class_group.rs).
--
-- Cl(O) depends only on disc(O) = f^2 d_K, so one row per discriminant holds
-- it for every curve whose End(E) has that discriminant: join on
-- endomorphism_ring.discriminant. Additive only: two new tables.

CREATE TABLE ec.order_class_group (
  discriminant           ec.zint PRIMARY KEY CHECK (discriminant < 0),  -- D = f^2 d_K
  fundamental_discriminant ec.zint NOT NULL,                             -- d_K
  conductor_f            ec.pos NOT NULL,                                -- [O_K : O]
  class_number           ec.pos NOT NULL,                                -- h(D) = #Cl(O)
  class_group            ec.pos[] NOT NULL,                              -- invariant factors n_1 | n_2 | ..., [] when trivial
  class_number_fundamental ec.pos NOT NULL,                              -- h(d_K)
  unit_index             ec.pos NOT NULL,                                -- [O_K^* : O^*]
  two_rank               integer NOT NULL CHECK (two_rank >= 0),         -- dim Cl(O)[2]
  cross_checks           text[] NOT NULL,                                -- independent identities that held
  status                 ec.fact_status NOT NULL DEFAULT 'unknown',
  method                 text,
  provenance_id          bigint REFERENCES ec.provenance(id),
  CHECK (discriminant = conductor_f * conductor_f * fundamental_discriminant)
);
COMMENT ON TABLE ec.order_class_group IS
  'Cl(O) of the imaginary quadratic order of discriminant D = f^2 d_K: class number, invariant factors (ascending, each dividing the next), h(d_K), unit index and 2-rank. One row per discriminant; join endomorphism_ring.discriminant.';

CREATE TABLE ec.order_ell_class (
  discriminant  ec.zint NOT NULL REFERENCES ec.order_class_group(discriminant),
  ell           ec.pos NOT NULL,
  splitting     text NOT NULL CHECK (splitting IN ('split', 'ramified')),   -- of ell in O (ell does not divide f)
  prime_form    ec.zint[] NOT NULL,                                          -- (ell, b, c), 0 <= b <= ell: the form of the prime above ell
  class_order   ec.pos NOT NULL,                                             -- order of [prime above ell] in Cl(O)
  cross_checks  text[] NOT NULL,
  status        ec.fact_status NOT NULL DEFAULT 'unknown',
  method        text,
  provenance_id bigint REFERENCES ec.provenance(id),
  PRIMARY KEY (discriminant, ell),
  CHECK (cardinality(prime_form) = 3 AND prime_form[1] = ell),
  CHECK (splitting <> 'ramified' OR class_order <= 2)
);
COMMENT ON TABLE ec.order_ell_class IS
  'Order of the class of a prime ideal above ell in Cl(O), for ell split or ramified in O and prime to the conductor (both primes above a split ell have inverse classes, so the order is one number). Equals the crater size of the ell-volcano of a curve with End(E) = O.';
