// compiles+runs each oracle_probes/*.c on host linux-gnu builds and captures
// its stdout (generated rust consts) into $OUT_DIR/<stem>_{glibc,musl}.rs for
// the integration tests to include!. the uefi build early-returns: no c is
// compiled and no rustc-link-* directives are emitted, so nothing here can
// reach the shipped rlib. musl is best-effort: without musl-gcc the stub
// just sets AVAILABLE = false.
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    if !target.ends_with("linux-gnu") {
        return;
    }
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let probe_dir = manifest.join("oracle_probes");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed=oracle_probes");
    if !probe_dir.is_dir() {
        return;
    }
    let cc_path = cc::Build::new().get_compiler().path().to_path_buf();
    for entry in fs::read_dir(&probe_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("c") {
            continue;
        }
        let stem = path.file_stem().unwrap().to_str().unwrap().to_string();
        println!("cargo:rerun-if-changed=oracle_probes/{}.c", stem);
        build_and_run(&cc_path, &path, &out_dir, &stem, "glibc");
        let musl = PathBuf::from("musl-gcc");
        if which(&musl) {
            build_and_run(&musl, &path, &out_dir, &stem, "musl");
        } else {
            fs::write(
                out_dir.join(format!("{}_musl.rs", stem)),
                "pub const AVAILABLE: bool = false;\n",
            )
            .unwrap();
        }
    }
}

fn which(p: &Path) -> bool {
    Command::new(p)
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn build_and_run(cc: &Path, src: &Path, out: &Path, stem: &str, libc: &str) {
    let exe = out.join(format!("{}_{}", stem, libc));
    let st = Command::new(cc)
        .arg(src)
        .arg("-O2")
        .arg("-o")
        .arg(&exe)
        .status()
        .expect("failed to spawn C compiler");
    assert!(
        st.success(),
        "compiling {} ({}) failed",
        src.display(),
        libc
    );
    let o = Command::new(&exe).output().expect("failed to run oracle");
    assert!(
        o.status.success(),
        "oracle {} exited nonzero",
        exe.display()
    );
    fs::write(out.join(format!("{}_{}.rs", stem, libc)), o.stdout).unwrap();
}
