#!/usr/bin/env bash
# boots one .efi across a device/topology matrix; each cell runs
# scripts/qemu-assert.sh with its expected values. the -smp 1 cell is the
# ap off-by-one discriminator and is mandatory.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
ASSERT="$SCRIPT_DIR/qemu-assert.sh"
ESP_DIR="$REPO_ROOT/testing/esp"

RED='\033[0;31m'; GREEN='\033[0;32m'; BLUE='\033[0;34m'; NC='\033[0m'
info() { echo -e "${BLUE}[matrix]${NC} $*"; }
ok()   { echo -e "${GREEN}[matrix]${NC} $*"; }
err()  { echo -e "${RED}[matrix]${NC} $*" >&2; }

usage() {
  cat <<EOF
Usage: $(basename "$0") <efi-file> [options]
  --timeout <sec>     per-cell boot timeout (default 240)
  --no-kvm            force TCG (CI: no KVM on runners)
  --kvm               force KVM (fail if unavailable)
  --ovmf-code <path>  OVMF_CODE.fd (autodetected if omitted)
  --ovmf-vars <path>  OVMF_VARS.fd (autodetected if omitted)
Exit 0 iff every cell's manifest assert passes.
EOF
}

EFI=""; TIMEOUT=240; USE_KVM="auto"; OVMF_CODE=""; OVMF_VARS=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --timeout) TIMEOUT="$2"; shift 2;;
    --no-kvm) USE_KVM="none"; shift;;
    --kvm) USE_KVM="force"; shift;;
    --ovmf-code) OVMF_CODE="$2"; shift 2;;
    --ovmf-vars) OVMF_VARS="$2"; shift 2;;
    -h|--help) usage; exit 0;;
    -*) err "unknown option $1"; usage; exit 2;;
    *) if [[ -z "$EFI" ]]; then EFI="$1"; else err "extra arg $1"; exit 2; fi; shift;;
  esac
done
[[ -n "$EFI" && -f "$EFI" ]] || { err "efi file missing: ${EFI:-<none>}"; exit 2; }
[[ -x "$ASSERT" ]] || { err "assert lib not executable: $ASSERT"; exit 2; }
command -v qemu-system-x86_64 >/dev/null || { err "qemu-system-x86_64 not found"; exit 2; }

autodetect() { local -n cands="$1"; local p; for p in "${cands[@]}"; do [[ -f "$p" ]] && { echo "$p"; return 0; }; done; return 1; }
CODE_CANDS=(/usr/share/edk2/x64/OVMF_CODE.4m.fd /usr/share/OVMF/OVMF_CODE_4M.fd /usr/share/OVMF/OVMF_CODE.4m.fd /usr/share/OVMF/OVMF_CODE.fd /usr/share/edk2-ovmf/OVMF_CODE.fd)
VARS_CANDS=(/usr/share/edk2/x64/OVMF_VARS.4m.fd /usr/share/OVMF/OVMF_VARS_4M.fd /usr/share/OVMF/OVMF_VARS.4m.fd /usr/share/OVMF/OVMF_VARS.fd /usr/share/edk2-ovmf/OVMF_VARS.fd)
[[ -n "$OVMF_CODE" ]] || OVMF_CODE="$(autodetect CODE_CANDS)" || { err "OVMF_CODE not found"; exit 2; }
[[ -n "$OVMF_VARS" ]] || OVMF_VARS="$(autodetect VARS_CANDS)" || { err "OVMF_VARS not found"; exit 2; }
info "OVMF_CODE=$OVMF_CODE"; info "OVMF_VARS=$OVMF_VARS"

mkdir -p "$ESP_DIR/EFI/BOOT"
# skip the copy when the input already is the staged esp file
if [[ "$(realpath "$EFI")" != "$(realpath "$ESP_DIR/EFI/BOOT/BOOTX64.EFI" 2>/dev/null || true)" ]]; then
  cp "$EFI" "$ESP_DIR/EFI/BOOT/BOOTX64.EFI"
fi
info "staged $(basename "$EFI") -> $ESP_DIR/EFI/BOOT/BOOTX64.EFI"

kvm_args() {
  case "$USE_KVM" in
    force) [[ -r /dev/kvm && -w /dev/kvm ]] || { err "KVM requested, /dev/kvm unusable"; exit 2; }; echo "-enable-kvm";;
    auto)  { [[ -r /dev/kvm && -w /dev/kvm ]] && echo "-enable-kvm"; } || true;;
    none)  true;;
  esac
}

# name | smp | expect_reclaim(0/1) | net(e1000e|"") | usb(usb-kbd|"")
CELLS=(
  "smp1|1|1||"
  "smp4|4|1||"
  "smp4-usbkbd|4|1||usb-kbd"
  "smp4-e1000e|4|1|e1000e|"
)

run_cell() {
  local name="$1" smp="$2" reclaim="$3" net="$4" usb="$5"
  local serial_log vars_copy
  serial_log="$(mktemp --suffix=.serial.log)"
  vars_copy="$(mktemp --suffix=.OVMF_VARS.fd)"
  cp "$OVMF_VARS" "$vars_copy"
  local qargs=(-machine q35 -m 512M -smp "$smp"
    -drive "if=pflash,format=raw,unit=0,readonly=on,file=$OVMF_CODE"
    -drive "if=pflash,format=raw,unit=1,file=$vars_copy"
    -drive "format=raw,file=fat:rw:$ESP_DIR"
    -display none -no-reboot -no-shutdown -serial "file:$serial_log")
  if [[ "$net" == "e1000e" ]]; then qargs+=(-netdev user,id=net0 -device e1000e,netdev=net0); else qargs+=(-net none); fi
  if [[ "$usb" == "usb-kbd" ]]; then qargs+=(-device qemu-xhci,id=xhci -device usb-kbd,bus=xhci.0); fi
  # shellcheck disable=SC2046
  qargs+=($(kvm_args))
  info "cell '$name': -smp $smp net='${net:-none}' usb='${usb:-none}' (timeout ${TIMEOUT}s)"
  qemu-system-x86_64 "${qargs[@]}" & local pid=$!
  local start; start=$(date +%s)
  while true; do
    local el=$(( $(date +%s) - start ))
    grep -qF 'MORPHEUSX_BOOT_OK' "$serial_log" 2>/dev/null && break
    kill -0 "$pid" 2>/dev/null || break
    [[ $el -ge $TIMEOUT ]] && { err "cell '$name': timeout ${TIMEOUT}s"; break; }
    sleep 2
  done
  kill "$pid" 2>/dev/null || true; wait "$pid" 2>/dev/null || true
  local aargs=(--smp "$smp" --label "$name")
  [[ "$reclaim" == "1" ]] && aargs+=(--expect-reclaim)
  [[ "$net" == "e1000e" ]] && aargs+=(--device e1000e)
  [[ "$usb" == "usb-kbd" ]] && aargs+=(--device usb-kbd)
  local rc=0; "$ASSERT" "$serial_log" "${aargs[@]}" || rc=$?
  rm -f "$vars_copy" "$serial_log"; return $rc
}

fails=0; total=0
for cell in "${CELLS[@]}"; do
  IFS='|' read -r name smp reclaim net usb <<<"$cell"
  total=$((total+1))
  if run_cell "$name" "$smp" "$reclaim" "$net" "$usb"; then ok "cell '$name' PASSED"
  else err "cell '$name' FAILED"; fails=$((fails+1)); fi
done
echo ""
if [[ $fails -eq 0 ]]; then ok "matrix: all $total cell(s) passed"; exit 0
else err "matrix: $fails/$total cell(s) failed"; exit 1; fi
