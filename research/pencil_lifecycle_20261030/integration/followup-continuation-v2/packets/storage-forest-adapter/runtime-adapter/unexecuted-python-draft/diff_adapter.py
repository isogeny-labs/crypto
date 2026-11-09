"""Pure qualification diff prerequisite adapter. No process or filesystem calls."""
import re
from mount_diff import mount_diff
CONTRACT = "software.qualification-exact-mount-diff/v1"

def require(condition, message):
    if not condition:
        raise ValueError(message)

def exact_int(value, expected):
    return type(value) is int and value == expected

def qualify_diff(case, returncode, raw, container, expected):
    require(case in {"Q1", "Q2", "Q3"}, "active qualification case")
    require(type(container) is dict and type(expected) is dict, "identity objects")
    required = {"container_id", "image_id", "container_name", "owner_nonce", "fixtures_source", "gate_source"}
    require(set(expected) == required and all(type(v) is str for v in expected.values()), "expected identity schema")
    require(re.fullmatch(r"[0-9a-f]{64}", expected["container_id"]) is not None, "full container ID")
    require(re.fullmatch(r"sha256:[0-9a-f]{64}", expected["image_id"]) is not None, "image ID")
    require(re.fullmatch(r"[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}", expected["owner_nonce"]) is not None, "owner nonce")
    require(expected["container_name"] == "isolab-runtime-q-" + expected["owner_nonce"], "owner name")
    require(all(expected[k].startswith("/") for k in ("fixtures_source", "gate_source")), "absolute mount sources")
    require(container.get("Id") == expected["container_id"] and container.get("Image") == expected["image_id"] and container.get("Name") == "/" + expected["container_name"], "container identity")
    config = container.get("Config", {})
    require(type(config) is dict and type(config.get("Labels")) is dict, "config labels")
    require(config["Labels"].get("isolab.runtime-qualification.owner") == expected["owner_nonce"], "owner label")
    require(config.get("User") == "1000:1000" and config.get("Entrypoint") == ["/qualification/exec-gate"], "user and entrypoint")
    state = container.get("State", {})
    require(type(state) is dict and state.get("Running") is False and exact_int(state.get("ExitCode"), {"Q1":0,"Q2":2,"Q3":125}[case]), "terminal state")
    hc = container.get("HostConfig", {})
    require(type(hc) is dict and hc.get("ReadonlyRootfs") is True and hc.get("NetworkMode") == "none", "readonly root/network")
    require(all(exact_int(hc.get(k),v) for k,v in {"PidsLimit":1,"Memory":512*1024**2,"MemorySwap":512*1024**2,"NanoCpus":10**9}.items()), "resource limits")
    require(hc.get("CapDrop") == ["ALL"] and hc.get("SecurityOpt") == ["no-new-privileges"], "privilege restrictions")
    require(hc.get("Privileged") is False and hc.get("PidMode", "") == "" and hc.get("UTSMode", "") == "" and hc.get("IpcMode") == "private" and hc.get("CgroupnsMode") == "private", "namespace restrictions")
    require(not hc.get("Devices") and not hc.get("DeviceRequests") and not hc.get("CapAdd"), "devices/capabilities")
    tmpfs = hc.get("Tmpfs", {})
    require(type(tmpfs) is dict and set(tmpfs) == {"/tmp", "/work"}, "tmpfs destinations")
    options = {"rw", "noexec", "nosuid", "nodev", "size=16m", "mode=1777"}
    require(all(type(v) is str and set(v.split(",")) == options for v in tmpfs.values()), "tmpfs options")
    mounts = container.get("Mounts")
    require(type(mounts) is list and all(type(m) is dict and type(m.get("Destination")) is str for m in mounts), "mount list")
    require(len({m["Destination"] for m in mounts}) == len(mounts), "duplicate mount destination")
    require(all(m.get("Type") in {"bind", "tmpfs"} for m in mounts), "mount type")
    require(all(m["Destination"] in {"/tmp", "/work"} for m in mounts if m["Type"] == "tmpfs"), "tmpfs mount destination")
    binds = [m for m in mounts if m["Type"] == "bind"]
    wanted = {"/fixtures": expected["fixtures_source"], "/qualification/exec-gate": expected["gate_source"]}
    require(len(binds) == 2 and all(m["Destination"] in wanted and m.get("Source") == wanted[m["Destination"] and m.get("RW") is False for m in binds), "readonly bind identity")
    require(exact_int(returncode, 0), "diff transport exit")
    parsed = mount_diff(raw)
    return {"schema": CONTRACT, "case": case, "container_id": expected["container_id"], "prerequisites_checked": True, "diff": parsed, "writer_attribution": None, "no_writes_claim": False}
