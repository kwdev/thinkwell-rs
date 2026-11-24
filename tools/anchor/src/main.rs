//! Repository-local launcher for the pinned Anchor Book engine.

use std::{
    env,
    ffi::OsStr,
    io,
    path::Path,
    process::{self, Command, Stdio},
};

const REVISION: &str = "8165d599878da6089f367af0d97e88be45631e46";
const REPOSITORY: &str = "https://github.com/anchor-book/anchor-book";

fn matches_pin(binary: &Path) -> bool {
    Command::new(binary)
        .arg("--version")
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .is_ok_and(|output| {
            output.status.success()
                && String::from_utf8_lossy(&output.stdout).trim()
                    == format!("anchorbook 0.1.0 ({REVISION})")
        })
}

fn ensure_engine(root: &Path, cargo: &OsStr) -> io::Result<std::path::PathBuf> {
    let install_root = root.join("target/tools");
    let binary = install_root
        .join("bin")
        .join(format!("anchorbook{}", env::consts::EXE_SUFFIX));
    if !matches_pin(&binary) {
        eprintln!("Installing pinned Anchor Book engine ({REVISION})...");
        let status = Command::new(cargo)
            .current_dir(root)
            .args([
                "install", "--git", REPOSITORY, "--rev", REVISION, "--locked", "--force", "--root",
            ])
            .arg(&install_root)
            .arg("anchor-book")
            .stdin(Stdio::null())
            .stdout(Stdio::from(io::stderr()))
            .stderr(Stdio::inherit())
            .status()?;
        if !status.success() {
            return Err(io::Error::other(format!(
                "Anchor Book installation failed: {status}"
            )));
        }
        if !matches_pin(&binary) {
            return Err(io::Error::other(
                "Installed Anchor Book does not match the pinned identity",
            ));
        }
    }
    Ok(binary)
}

fn run() -> io::Result<i32> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    // Cargo supplies its own executable when running this alias. Avoid resolving
    // another cargo from PATH during bootstrap.
    let cargo =
        env::var_os("CARGO").ok_or_else(|| io::Error::other("Run this tool with cargo anchor"))?;
    let binary = ensure_engine(root, &cargo)?;
    let status = Command::new(binary).args(env::args_os().skip(1)).status()?;
    Ok(status.code().unwrap_or(1))
}

fn main() {
    match run() {
        Ok(code) => process::exit(code),
        Err(error) => {
            eprintln!("cargo anchor: {error}");
            process::exit(1);
        }
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::{
        fs,
        os::unix::fs::PermissionsExt,
        path::PathBuf,
        sync::{
            Mutex, MutexGuard,
            atomic::{AtomicUsize, Ordering},
        },
    };

    static NEXT: AtomicUsize = AtomicUsize::new(0);

    // A concurrent process spawn can inherit another test's writable script
    // descriptor until exec closes it, causing Linux ETXTBSY when that script
    // is executed. Serialize writing and spawning for each fixture's lifetime.
    static FIXTURES: Mutex<()> = Mutex::new(());

    struct Fixture {
        root: PathBuf,
        _guard: MutexGuard<'static, ()>,
    }

    impl Fixture {
        fn new() -> Self {
            let guard = FIXTURES.lock().unwrap_or_else(|error| error.into_inner());
            let root = env::temp_dir().join(format!(
                "anchor launcher {} {}",
                process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(root.join("target/tools/bin")).unwrap();
            Self {
                root,
                _guard: guard,
            }
        }

        fn script(&self, path: &str, contents: &str) -> PathBuf {
            let path = self.root.join(path);
            fs::write(&path, format!("#!/bin/sh\n{contents}\n")).unwrap();
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
            path
        }

        fn engine(&self) -> PathBuf {
            self.script(
                "target/tools/bin/anchorbook",
                &format!("echo 'anchorbook 0.1.0 ({REVISION})'"),
            )
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    #[test]
    fn matching_engine_needs_no_installer() {
        let fixture = Fixture::new();
        let engine = fixture.engine();
        assert_eq!(
            ensure_engine(&fixture.root, OsStr::new("nonexistent-installer")).unwrap(),
            engine
        );
    }

    #[test]
    fn missing_and_mismatched_engines_install_the_pin() {
        for mismatched in [false, true] {
            let fixture = Fixture::new();
            if mismatched {
                fixture.script("target/tools/bin/anchorbook", "echo wrong-version");
            }
            let installer = fixture.script("cargo", &format!(
                "printf '%s\\n' \"$@\" > arguments\ncat > target/tools/bin/anchorbook <<'ENGINE'\n#!/bin/sh\necho 'anchorbook 0.1.0 ({REVISION})'\nENGINE\nchmod +x target/tools/bin/anchorbook"
            ));
            let binary = ensure_engine(&fixture.root, installer.as_os_str()).unwrap();
            assert!(matches_pin(&binary));
            let arguments = fs::read_to_string(fixture.root.join("arguments")).unwrap();
            let expected = format!(
                "install\n--git\n{REPOSITORY}\n--rev\n{REVISION}\n--locked\n--force\n--root\n{}\nanchor-book\n",
                fixture.root.join("target/tools").display()
            );
            assert_eq!(arguments, expected);
        }
    }

    #[test]
    fn failed_install_and_wrong_post_install_identity_are_rejected() {
        let fixture = Fixture::new();
        for script in ["exit 17", "exit 0"] {
            let installer = fixture.script("cargo", script);
            assert!(ensure_engine(&fixture.root, installer.as_os_str()).is_err());
        }
        fixture.script("target/tools/bin/anchorbook", "echo wrong-version");
        let installer = fixture.script("cargo", "exit 0");
        assert!(ensure_engine(&fixture.root, installer.as_os_str()).is_err());
    }
}
