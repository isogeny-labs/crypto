# Forward map to actual KM classes, including reference overlaps

Frozen before data,2026-11-17. Same fixed P-192 data and native development
boundary as COMPLETE_MAPS_PROTOCOL.md;600seconds/1GiB,no timings or paid run.
The pole audit PCC-20261117-905d71 records no denominator factors of degree
<=3. Thus every valid Fp reduced Mumford input has denominator-unit support.
Recheck this audit predicate before claiming that scope.

Encode the qualified cycle in parameter z=x+lambda*y by substituting its
actual x(z),y(z) into all target section monomials. Do not treat z as x.
For d=1,2,3,pad with (3-d)*Eref plus three auxiliary points. If reference
support occurs in the image, count its exact multiplicity m in the cycle
support and add jets of orders m through m+(3-d)-1. Use an x-local chart
when F_y!=0,otherwise a y-local chart when F_x!=0,with the exact curve lift.
This adds divisor multiplicities rather than adding duplicate constraints.

Choose auxiliary points separately for each input,backward through the
retained pool,excluding85 evaluation points,the three reference points,
the pole line and image z-support. For degree<=9,the last exclusion removes
at most36 pool points,so the226-point pool has ample points for three.
Compare the resulting degree12 RR space with its baseline3*Eref+auxiliary;
both have dimension22. Zero maps directly to the identity.

Check all eight direct forward fixtures (zero,second branch and six pinned
source classes). Compare six regular images to the old KM bridge,check branch
order2,negation,doubling and P+pullG. Two representation controls use actual
quartic points: the earlier three-point vertical collision cycle,and a
length3 cycle concentrated at the first reference point. The latter requires
adding padding multiplicity to make order5 at that reference point.
Compare both controls with direct rational point classes in the KM Jacobian.
Retain all source/image/class-space/auxiliary/overlap records and exact counts.

This completes a forward class interface for reduced Fp Mumford inputs at the
specified representation boundary. It does not turn bounded qualifiers into
independent verification or implement inverse decoding of opaque KM spaces.
