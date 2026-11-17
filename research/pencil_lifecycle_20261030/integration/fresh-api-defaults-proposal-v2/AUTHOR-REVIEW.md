# Additive fresh selector correction

Independent review identified mutable indexing on a downwardAPI resourceFieldRef item as inserting fieldRef:null. Replaced with get_mut traversal. Added an exact unchanged fixture with resourceFieldRef in both environment and downward volume items, plus an explicit missing fieldRef assertion. Existing API roundtrip/mutation fixture remains.

This incremental patch is strictly on 33748bb7; prior patches and receipts remain unchanged. Targeted fresh_ software fixtures passed in a private controller tree. Source formatted with rustfmt. No root edit/build, cloud, watchdog or scientific execution. Root combined suite remains a separate qualification.
