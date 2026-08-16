# test corpus

fixtures read from disk by differential/known-answer tests, one family per
directory, each with a sibling `PROVENANCE.md`.

## rules

- small and stripped: target < 256 KB per file; anything bigger ships as
  `.img.xz` or a header slice cut to the smallest span the parser needs.
- provenance mandatory: every fixture gets a `PROVENANCE.md` row with source
  tool/url + exact command, capture date, sha-256, byte size, license, and a
  one-line why.
- regenerate, don't trust: generated fixtures ship a deterministic `regen.sh`
  (pinned tool version, fixed seed/guids, `SOURCE_DATE_EPOCH`) so a rebuild
  can be diffed against the committed bytes.
- prefer generating in ci: if a host tool can mint the fixture hermetically,
  generate at run time and commit nothing, or commit only a golden fallback
  for tool-absent runners with a visible skip message.
- fixtures with no external referent (helix on-disk blobs, the MXISO manifest,
  the SYS_* numbers) go under `oracle-unverified/`, whose readme states the
  framing is self-referential.

## families

- `gpt/`   gpt partition images + sgdisk/parted dumps
- `fat32/` mkfs.fat images / esp header slices
- `iso/`   genisoimage/xorriso isos + real distro header slices

## PROVENANCE.md row template

```text
### <filename>
- source: <tool vX.Y.Z | URL>
- command: <exact command that produced it>
- date: <YYYY-MM-DD>
- sha256: <hex>
- bytes: <n>
- license: <SPDX or "self-generated">
- why: <one line: which parser path / quirk this exercises>
```
