# XP-20261029-packed-filter, PROTOCOL.md section 4 "Trigger", conditions 2 to 4, evaluated over l2-gate.json.
# Run as: jq -e -f trigger.jq l2-gate.json   (exit status 0 prints true; exit status 1 prints false)
def l2_ok:
  test("^[0-9]+ ticks of foreign work on the run CPU during the run$")
  or test("^[0-9]+ ticks of hypervisor steal on the run CPU$")
  or test("^[0-9]+ busy ticks on the run CPU.s idle siblings and the rest of the reservation$")
  or test("^run-queue delay [0-9]+ µs is [0-9.]+% of the solve$")
  or test("^[0-9]+ involuntary context switches (during a [0-9.]+ ms solve|over the process) \\(at most [0-9.]+\\)$")
  or test("^[0-9]+ interrupts on the run CPU during the solve \\(.*\\)$")
  or test("^[0-9]+ foreign task slices on the run CPU during the solve \\([0-9]+ µs\\)$");
(.invalid_outcomes == [])
and ((.below_l2 | type) == "array")
and ((.below_l2 | length) > 0)
and all(.below_l2[];
      (.isolation.level == "L1")
      and ((.isolation.blockers | type) == "array")
      and all(.isolation.blockers[];
            (type == "string")
            and (startswith("L3: ") or (startswith("L2: ") and (.[4:] | l2_ok)))))
