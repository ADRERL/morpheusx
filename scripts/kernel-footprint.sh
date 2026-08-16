#!/usr/bin/env bash
# kernel footprint report: what actually ends up in BOOTX64.EFI.
# prints pe section sizes, loc compiled into the kernel graph (workspace +
# external deps), and with --map the biggest symbols via an lld link map.
#
# usage: kernel-footprint.sh [efi-file] [--map] [--top N]
#   efi-file  defaults to testing/esp/EFI/BOOT/BOOTX64.EFI
#   --map     rebuild the bootloader with /lldmap and list biggest symbols
#             (full uefi rebuild, takes a few minutes)
#   --top N   symbols to list in map mode (default 30)
set -euo pipefail
cd "$(dirname "$0")/.."

EFI="testing/esp/EFI/BOOT/BOOTX64.EFI"
DO_MAP=0
TOP=30
while [[ $# -gt 0 ]]; do
  case "$1" in
    --map) DO_MAP=1; shift;;
    --top) TOP="$2"; shift 2;;
    -h|--help) grep '^#' "$0" | sed 's/^# \{0,1\}//'; exit 0;;
    *) EFI="$1"; shift;;
  esac
done

echo "== pe sections =="
if [[ -f "$EFI" ]]; then
  ls -l "$EFI" | awk '{printf "  image: %s  (%.2f MiB)\n", $NF, $5/1048576}'
  objdump -h "$EFI" | awk '
    /^ *[0-9]+ / { sz = strtonum("0x" $3); total += sz;
      printf "  %-10s %10d bytes  (%8.1f KiB)\n", $2, sz, sz/1024 }
    END { printf "  %-10s %10d bytes  (%8.1f KiB)\n", "total", total, total/1024 }'
else
  echo "  $EFI not found - run testing/build.sh first"
fi

echo ""
echo "== loc compiled into the kernel graph =="
python3 - <<'PY'
import json, subprocess, os, sys
from pathlib import Path

def sh(cmd):
    return subprocess.run(cmd, shell=True, capture_output=True, text=True).stdout

# crates that actually link into the image: no dev, no build, no proc-macro deps
graph = set(sh("cargo tree -e no-dev,no-build,no-proc-macro "
               "--target x86_64-unknown-uefi -p morpheus-bootloader "
               "--prefix none").split())
meta = json.loads(sh("cargo metadata --format-version 1"))
ws_ids = set(meta["workspace_members"])

def loc_rs(path):
    # non-blank, non-comment-only lines; stop at a trailing inline test mod
    n = 0
    try:
        for line in open(path, errors="replace"):
            s = line.strip()
            if s.startswith("#[cfg(test)]"):
                break
            if s and not s.startswith("//"):
                n += 1
    except OSError:
        pass
    return n

SKIP_DIRS = {"tests", "benches", "examples", "oracle_probes", "host_tool", "target"}

def count_crate(root, src_only):
    rs = asm = 0
    base = root / "src" if src_only and (root / "src").is_dir() else root
    for p in base.rglob("*"):
        if any(part in SKIP_DIRS for part in p.relative_to(root).parts):
            continue
        if p.suffix == ".rs":
            rs += loc_rs(p)
        elif p.suffix in (".s", ".S", ".asm"):
            asm += sum(1 for l in open(p, errors="replace") if l.strip())
    return rs, asm

ws, ext = [], []
for pkg in meta["packages"]:
    if pkg["name"] not in graph:
        continue
    root = Path(pkg["manifest_path"]).parent
    if pkg["id"] in ws_ids:
        ws.append((pkg["name"], *count_crate(root, False)))
    else:
        ext.append((pkg["name"], *count_crate(root, True)))

ws.sort(key=lambda t: -(t[1] + t[2]))
ext.sort(key=lambda t: -(t[1] + t[2]))
wr = sum(t[1] for t in ws); wa = sum(t[2] for t in ws)
er = sum(t[1] for t in ext)

print("  workspace crates (rust + asm loc, tests excluded):")
for name, rs, asm in ws:
    a = f" + {asm} asm" if asm else ""
    print(f"    {name:<28} {rs:>7}{a}")
print(f"    {'workspace total':<28} {wr:>7} + {wa} asm")
print("")
print("  external deps in the image (src/ loc):")
for name, rs, _ in ext:
    print(f"    {name:<28} {rs:>7}")
print(f"    {'external total':<28} {er:>7}")
print("")
print(f"  grand total linked into the kernel: {wr + wa + er} loc "
      f"({wr + wa} ours, {er} external)")
PY

if [[ "$DO_MAP" -eq 1 ]]; then
  echo ""
  echo "== symbol attribution (lld map, release uefi build) =="
  MAP="target/kernel-lld.map"
  if [[ ! -f "$MAP" ]]; then
    RUSTFLAGS="${RUSTFLAGS:-} -Clink-arg=/lldmap:${PWD}/${MAP}" \
      cargo build --target x86_64-unknown-uefi -p morpheus-bootloader --release >/dev/null 2>&1
  fi
  TOP="$TOP" MAP="$MAP" python3 - <<'PY'
import os, re, collections

# map layout: fragment lines carry the size (addr size align obj:(section));
# the zero-size symbol lines that follow name what lives in the fragment.
frag_re = re.compile(r"^([0-9a-f]+) ([0-9a-f]+)\s+\d+\s+\S+:\((\S+)\)")
sym_re = re.compile(r"^([0-9a-f]+) 0{8}\s+0\s+(\S.*)$")
sect_re = re.compile(r"^([0-9a-f]+) ([0-9a-f]+)\s+\d+\s+(\.\S+)$")

frags, sections = [], []
pend = None
for line in open(os.environ["MAP"]):
    m = sect_re.match(line)
    if m:
        sections.append((m.group(3), int(m.group(2), 16)))
        continue
    m = frag_re.match(line)
    if m:
        pend = [int(m.group(1), 16), int(m.group(2), 16), m.group(3), None]
        frags.append(pend)
        continue
    m = sym_re.match(line)
    if m and pend and int(m.group(1), 16) == pend[0] and pend[3] is None:
        pend[3] = m.group(2).strip()

top = int(os.environ["TOP"])
frags.sort(key=lambda f: -f[1])
print(f"  top {top} fragments:")
for addr, size, sect, sym in frags[:top]:
    print(f"    {size/1024:9.1f} KiB  {sect:<12} {sym or '(no symbol)'}")

crate = collections.Counter()
for _, size, _, sym in frags:
    name = (sym or "?").split("::")[0].split("<")[-1].strip("&(")
    crate[name] += size
print("")
print("  per-crate rollup (by symbol prefix, all sections):")
for name, size in crate.most_common(20):
    print(f"    {size/1024:9.1f} KiB  {name}")
PY
fi
