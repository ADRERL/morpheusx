#!/usr/bin/env bash
# strip ansi from a captured serial log and assert the boot manifest. shared
# by qemu-e2e.sh and qemu-matrix.sh so the check logic cannot drift.
#
# usage: qemu-assert.sh <serial-log> [--smp N] [--expect-reclaim] [--device NAME]... [--label TEXT]
# hard checks: MORPHEUSX_BOOT_OK present; none of [MEM] CORRUPT /
#   [MEM] VALIDATE: corrupt / probable loop / [PANIC] / unexpected [ERR].
# --smp N: every 'smp (K cpu online)' must have K==N, at least one present.
# --expect-reclaim: '[MEM] reclaimed pages=0x..' present with value > 0.
# --device is informational only, never fails. exit: 0 pass / 1 fail / 2 usage.
set -euo pipefail
SELF="$(basename "$0")"

usage() { grep '^#' "$0" | sed 's/^# \{0,1\}//'; }

LOG=""; SMP=""; EXPECT_RECLAIM=0; LABEL=""; DEVICES=()
while [[ $# -gt 0 ]]; do
  case "$1" in
    --smp) SMP="$2"; shift 2;;
    --expect-reclaim) EXPECT_RECLAIM=1; shift;;
    --device) DEVICES+=("$2"); shift 2;;
    --label) LABEL="$2"; shift 2;;
    -h|--help) usage; exit 0;;
    -*) echo "$SELF: unknown option $1" >&2; exit 2;;
    *) if [[ -z "$LOG" ]]; then LOG="$1"; else echo "$SELF: extra arg $1" >&2; exit 2; fi; shift;;
  esac
done
[[ -n "$LOG" ]] || { echo "$SELF: no serial-log given" >&2; exit 2; }
[[ -f "$LOG" ]] || { echo "$SELF: serial-log not found: $LOG" >&2; exit 2; }

# strip ansi sgr/cursor escapes so every pattern below is a plain fixed string
CLEAN="$(mktemp)"
trap 'rm -f "$CLEAN"' EXIT
sed -E 's/\x1b\[[0-9;?]*[a-zA-Z]//g' "$LOG" > "$CLEAN"

fail=0
tag="${LABEL:+[$LABEL] }"
pass() { echo "  ${tag}PASS: $*"; }
bad()  { echo "  ${tag}FAIL: $*" >&2; fail=1; }
info() { echo "  ${tag}INFO: $*"; }
count() { grep -cF "$1" "$CLEAN" || true; }

# boot completion token
if grep -qF 'MORPHEUSX_BOOT_OK' "$CLEAN"; then pass "MORPHEUSX_BOOT_OK present"
else bad "MORPHEUSX_BOOT_OK missing"; fi

# free-list corruption tokens must be absent
for pat in '[MEM] CORRUPT' '[MEM] VALIDATE: corrupt' 'probable loop'; do
  n=$(count "$pat")
  if [[ "$n" -eq 0 ]]; then pass "no '$pat'"; else bad "$n occurrence(s) of '$pat'"; fi
done

# panic is always fatal. [ERR] is fatal too, except the known-benign diskless
# boot error (no root disk in qemu/ci -> kernel mounts a fresh ram root)
n=$(count '[PANIC]')
if [[ "$n" -eq 0 ]]; then pass "no '[PANIC]'"; else bad "$n occurrence(s) of '[PANIC]'"; fi
nerr=$( { grep -aF '[ERR]' "$CLEAN" || true; } | { grep -avF 'failed to mount any root filesystem' || true; } | wc -l | tr -d ' ')
if [[ "$nerr" -eq 0 ]]; then pass "no unexpected '[ERR]'"; else bad "$nerr unexpected '[ERR]' line(s)"; fi

# reclaim value is '0x' + 16 hex digits; nonzero iff a non-'0' digit follows
# the leading zeros
if [[ "$EXPECT_RECLAIM" -eq 1 ]]; then
  if grep -Eq '\[MEM\] reclaimed pages=0x0*[1-9a-fA-F]' "$CLEAN"; then
    pass "reclaimed pages > 0"
  else
    bad "reclaimed pages missing or zero"
  fi
fi

# both the detected-cores row and the ap-bringup row print 'smp (N cpu online)';
# on a healthy boot every N equals the expected -smp
if [[ -n "$SMP" ]]; then
  mapfile -t counts < <(grep -Eo 'smp \([0-9]+ cpu online\)' "$CLEAN" | grep -Eo '[0-9]+')
  if [[ "${#counts[@]}" -eq 0 ]]; then
    bad "no 'smp (N cpu online)' line found (expected $SMP)"
  else
    mismatch=0
    for c in "${counts[@]}"; do [[ "$c" == "$SMP" ]] || mismatch=1; done
    if [[ "$mismatch" -eq 0 ]]; then pass "smp online == $SMP (${#counts[@]} line(s))"
    else bad "smp online mismatch: saw [${counts[*]}], expected all == $SMP"; fi
  fi
fi

# device presence is informational only: boot may not probe every device
for d in "${DEVICES[@]:-}"; do
  [[ -n "$d" ]] || continue
  case "$d" in
    e1000e)  if grep -qF '[e1000e]' "$CLEAN"; then info "device e1000e: driver output present"; else info "device e1000e: no driver output"; fi;;
    usb-kbd) if grep -qiE 'xhci|hid' "$CLEAN"; then info "device usb-kbd: xhci/hid output present"; else info "device usb-kbd: no xhci/hid output"; fi;;
    *) info "device $d: no token rule";;
  esac
done

if [[ "$fail" -eq 0 ]]; then echo "  ${tag}ALL ASSERTIONS PASSED"; exit 0
else echo "  ${tag}ASSERTIONS FAILED" >&2; exit 1; fi
