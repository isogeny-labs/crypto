# Vertex and description accounting, before native implementation

Source review identifies a cost term that edge mass alone omits: every group
child contains one new vertex for every packet, including packets with no
retained port in that group. Measure old vertices, new packet vertices and
zero-port packet vertices separately for every constructed child.

The predicted aggregate is
`sum child.vertices = parent.left.len + parent.right.len + radix*packets`.
Old vertices partition by their labels, including isolates; each group has
the same number of new packet vertices. The abstract old-label counting
uses the existing Multiset.sum_count_eq_card theorem; add the shared named
composition with the constant packet term before its study consumer.

For the balanced fixture, predict4096 packets,262144 total new packet
vertices and254016 zero-port packet vertices, plus16386 old vertices
(the matching fixture deliberately includes two isolates). Thus total
child edges16384 does not describe the vertex/description workload.
Check these predicted values against actual native objects. They are not
observed memory bytes or total bit work.

The prior native plan/addenda remain unchanged. All new observables and
predictions precede native code/execution. Separate all sweep,balanced and
negative-probe denominators. Default Mathlib pin remains unchanged.
