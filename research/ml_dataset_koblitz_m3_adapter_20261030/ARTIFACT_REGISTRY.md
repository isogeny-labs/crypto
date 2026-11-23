# Koblitz reserved artifact identities

The four path/role/encoding triples remain those frozen in DESIGN.md. Canonical
admission requires all four regardless of worker completion flags. Matching uses
both path and role, rejecting missing, duplicate, swapped, aliased, relocated or
wrong-encoding records. Reserved names or roles outside the frozen registry fail.

Live copies under `live/payloads/` retain collector identity
`unregistered-partial-artifact` with encoding `native`; their mathematical
identity is authenticated from the committed capture chain. Such live copies do
not satisfy canonical artifact requirements. A live copy and its canonical copy
may coexist, but duplicate paths or canonical identity on a live copy fail.

Incomplete or censored prefixes require their separate admission and cannot gain
mathematical labels from the identity check. The registry does not check file
bytes or mathematical content: manifest hash verification, bounded strict
parsing, original-input verification, capture matching, witness publication and
outer outcome reconciliation remain mandatory.
