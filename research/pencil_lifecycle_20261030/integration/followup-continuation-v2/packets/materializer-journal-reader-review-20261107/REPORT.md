# Independent materializer journal reader review

[pcc:unverified] Scoped software source and deterministic fixture review.

The reader validates the acknowledged storage prefix against the exact producer ABI. All 21 independent adversarial unit fixture methods passed in 0.004 seconds under a 25-second wrapper. These are local software tests, not materializer execution, fault qualification or a scientific benchmark.

The reviewer checked bounded prefix bytes, SHA256 and strict record count; strict JSON/UTF8 and duplicate/nonfinite rejection; typed inline equality; directory and leaf symlink rejection; FIFO rejection; and passed-final no-tail consistency. Failed final may retain an unacknowledged suffix.

Initial source inspection found incomplete-final suffix acceptance. The producer restricted final status to passed or failed. The first 18 fixture methods ran after that correction, so no executed initial failure is claimed. Root additionally identified an ambiguous complete label; final output now says storage_complete and explicitly reports cleanup semantics and terminal custody unverified.

A synthetic receipt reporting an error still passes storage matching. This deliberate case demonstrates the output boundary: matching bytes do not authenticate successful cleanup, source identity, process termination or a completed materializer. Missing journal pointers do not prove absence of children.

No concurrent mutation or durability fault experiment ran. The file identity check is not a transactional snapshot. Existing scientific deadlines and execution controls remain unchanged.

KB tools were unavailable in the exposed runtime; the exact repository producer and previous source review were inspected as primary sources. No novelty claim is made.

Bindings, review findings and test command are in review.json, review_plan.json and adversarial_tests.py. Preread plan was saved before candidate inspection.
