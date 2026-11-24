# Contributing

Read the [Book](book/SUMMARY.md), particularly
[working on the project](book/working-on-the-project.md) and the
[current boundary](book/boundary.md), before changes. Work requires a human-approved
milestone; lifecycle integration does not authorize implementing the port.
See [change delivery](book/change-delivery.md) for review and merge requirements.

## Install the pinned tools

Run from the repository root with Git and rustup installed:

```sh
rustup toolchain install 1.100.0 --profile minimal --component rustfmt --component clippy
cargo anchor --version
cargo install mdbook --version 0.5.4 --locked --root target/tools
export PATH="$PWD/target/tools/bin:$PATH"
rustc --version
mdbook --version
```

The toolchain file selects Rust 1.100.0, including formatting and Clippy. It is a
development baseline, not an MSRV declaration. Keep `Cargo.lock` version controlled.
The expected engine identity is
`anchorbook 0.1.0 (3bd59b7c1e352e94bafcf133dc05c9a7560a48df)`; mdBook is `0.5.4`.
`cargo anchor` verifies the repository-local engine on every run and automatically
installs/replaces it from the exact revision when needed. The pin and expected
identity live in `tools/anchor/src/main.rs`. No ambient anchorbook is invoked.
The pin includes the experimental Port phase from the engine's `port` branch.
Configured contract retrieval supplies its port guidance. The full SHA keeps
installation independent of later changes to that branch.
Git, rustup/Cargo, and network access are needed for first use or recovery; a
matching local engine is reused without installation. The alias uses a small,
dependency-free development binary, not a Thinkwell runtime dependency.
Tools and generated output stay under ignored `target/`. Re-export PATH in each
new shell. No global Rust default change is required.

## Local checks

After installing tools and setting PATH, run:

```sh
cargo anchor contract --config anchorbook.toml
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
python3 -m unittest discover -s experiments/acp-integration -p 'test_probe_agent.py'
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo anchor check --config anchorbook.toml
mdbook build
cargo test -p anchor-runner --test bootstrap_consistency --locked
git diff --check
```

The library has deterministic contract, ACP/HTTP subprocess, ownership, and lifecycle
tests. They require Python 3 and Unix ps on macOS/Linux, with bounded deadlines;
they never invoke a real model. The development launcher has focused bootstrap
tests. The bootstrap-consistency integration test checks startup, Book
navigation, documented engine pins, and CI/rendering entry points without duplicating
installation tests. It runs in workspace tests and has the explicit command above
for focused diagnostics. Product test results cover their named behavioral cases, not all supported-agent
behavior. Linux CI is distinct from a local macOS pass. The opt-in
`cargo run -p thinkwell --locked --example live_check` exercises the pinned
Claude adapter and existing authentication; it is not part of ordinary tests.
`check` evaluates Book structure, not semantic validity or completion.
The mdBook preprocessor additionally validates authority records and roadmap
references. Inspect generated navigation, records, and links under
`target/anchorbook/`; a build is not a complete content review.

The Rustdoc check builds workspace API documentation without dependency docs and
treats documentation warnings (including broken intra-doc links) as errors. Local
output is under `target/doc/`, with the library entry at `target/doc/thinkwell/index.html`.
Doctests remain part of `cargo test`. The Thinkwell crate denies missing public
API documentation. Document public types, variants, fields, and methods, and
include concrete examples for major APIs. Write for newcomers: explain what an
API helps them do, introduce concepts through small examples, and put advanced
constraints after the basic workflow. Examples requiring an installed agent
or model credentials should use `no_run` so CI still compiles them without
inference; use ordinary runnable examples for local configuration and value logic.

Each CI run uploads the generated tree as the `rustdoc` artifact, retained for
14 days. In GitHub Actions, open the CI run, download `rustdoc` from its Artifacts
section, extract the archive, and open `thinkwell/index.html`. Keep the whole
extracted tree together for styles, search, and source links. GitHub artifacts
are downloads, not a hosted documentation website.

Use `crate::` for imports from the current crate in examples and documentation.
Keep explicit crate names for dependencies and imports across crate boundaries:
a package's binary, library, integration tests, examples, and doctests are separate
crates.

Use default rustfmt and Clippy behavior. There is no custom lint configuration.
CI runs the Cargo/Book and bootstrap checks above and whitespace checking on Ubuntu 24.04.
It installs exact tool versions and uses a SHA-pinned checkout action. Runner images
can change; this is not a hermetic or bit-for-bit reproducibility guarantee.
Local results do not establish that GitHub CI has run.

## Meeting scheduler example

The standalone [meeting scheduler](examples/meeting-scheduler/README.md) has its
own workspace and lockfile; root workspace tests do not build its native solver.
Its separate CI job and local checks explicitly run:

```sh
export CARGO_TARGET_DIR="$PWD/target/meeting-scheduler"
cargo fmt --manifest-path examples/meeting-scheduler/Cargo.toml -- --check
cargo check --manifest-path examples/meeting-scheduler/Cargo.toml --all-targets --locked
cargo clippy --manifest-path examples/meeting-scheduler/Cargo.toml --all-targets --locked -- -D warnings
cargo test --manifest-path examples/meeting-scheduler/Cargo.toml --locked
```

Install CMake 3.26+ (CI uses 3.31.10), a C++ compiler suitable for Z3, and Python 3.
The vendored native build uses the example lockfile; it does not require system
Z3. Tests use a scripted agent and never require authentication. Live execution
is opt-in; see the walkthrough for commands and caveats. Native solver debug
traces and build products are ignored. The example's dependencies are not library
runtime dependencies.

The scheduler CI job caches Cargo dependencies and their build artifacts,
including native Z3, using a commit-pinned `Swatinem/rust-cache` action. Its
workspace mapping explicitly selects `target/meeting-scheduler`. The action's
Rust/toolchain and dependency keys are supplemented with Ubuntu 24.04 and a hash
of C/C++ compiler, CMake, linker, and libc version output. Changes to native tools
therefore invalidate the cache. Cache misses still perform the locked vendored
build and all checks; hits never skip compilation commands or tests. A first run
populates the cache, and eviction or branch visibility can cause later misses.
Confirm speedup from a subsequent run's restore log and compilation timing;
configuration alone is not evidence that Z3 was reused.

## Sentiment analysis example

The standalone [sentiment example](examples/sentiment-analysis/README.md) has its
own workspace and lockfile. CI explicitly runs its checks without inference:

```sh
export CARGO_TARGET_DIR="$PWD/target/sentiment-analysis"
cargo fmt --manifest-path examples/sentiment-analysis/Cargo.toml -- --check
cargo check --manifest-path examples/sentiment-analysis/Cargo.toml --all-targets --locked
cargo clippy --manifest-path examples/sentiment-analysis/Cargo.toml --all-targets --locked -- -D warnings
cargo test --manifest-path examples/sentiment-analysis/Cargo.toml --locked
```

Python 3 is needed for the scripted agent; no native solver library or model
credentials are required for tests. Live execution is opt-in. VADER and its
lexicons remain dependency-owned and are not library runtime dependencies.

## JavaScript unminifier example

The standalone [unminifier](examples/javascript-unminifier/README.md) isolates Oxc
and its lockfile from the library workspace. Its separate CI job runs:

```sh
export CARGO_TARGET_DIR="$PWD/target/unminifier"
cargo fmt --manifest-path examples/javascript-unminifier/Cargo.toml -- --check
cargo check --manifest-path examples/javascript-unminifier/Cargo.toml --all-targets --locked
cargo clippy --manifest-path examples/javascript-unminifier/Cargo.toml --all-targets --locked -- -D warnings
cargo test --manifest-path examples/javascript-unminifier/Cargo.toml --locked
```

Tests require Python 3 and Unix process support, but no credentials or model calls.
Live inference is opt-in; the walkthrough documents installation, command arguments,
new-file output and teaching-example assumptions. No JavaScript runtime is required by
ordinary checks; locally recorded behavior checks used Node only on original fixtures.

## Website checks

The **Documentation site** CI job runs the [website checks](website/README.md#validate)
on pushes and pull requests. From the repository root:

```sh
python3 website/scripts/check_examples.py
```

From `website/`, using Node 24.12.0:

```sh
npx --yes mint@4.3.84 validate --telemetry false
npx --yes mint@4.3.84 broken-links --telemetry false
```

Complete tutorial programs are compiled without running inference. Source excerpts
are compared with repository code; navigation, local links, MDX and the Mintlify
build are checked. Dependencies resolve from the tutorial's manifest into ignored
`target/website-examples`; this is not a locked release build. CI has a 15-minute
job timeout and no deployment step. It doesn't establish browser appearance,
external-service availability or real-model behavior.

## Updating tools

Inspect the candidate engine's CLI, configuration reader, compiled contract,
format documentation, and fixtures before upgrading. Features described in its
Book may still be planned. Select a full Git commit SHA and its compatible mdBook
version; install with `--locked`. Update the engine pin and expected identity in
`tools/anchor/src/main.rs`, the mdBook installation/version check in this file and
`.github/workflows/ci.yml`, and the pinned authoring links
in the Book. Update `rust-toolchain.toml` and the matching setup instructions when
changing Rust. Tool pins are setup choices, not product architecture or MSRV policy.

Read the new contract, reconcile changed formats and guidance, run all local
checks, inspect rendered records and links, and record results and limitations
in the assigned roadmap. Do not add the planned `[engine]` configuration until a
reviewed revision actually supports it. Do not install a moving branch in CI.

## Licensing and contributions

Original project code and documentation are dual-licensed under the
[MIT License](LICENSE-MIT) or [Apache License, Version 2.0](LICENSE-APACHE), at the
recipient's option. Both workspace packages inherit `MIT OR Apache-2.0` from the
root Cargo manifest.

Unless explicitly agreed otherwise, any contribution you intentionally submit
for inclusion in this project is provided under both licenses, without additional
terms or conditions. Submit only material you have the right to contribute on
those terms. Identify third-party material and its applicable license separately;
this contribution policy does not override its existing terms.

Before copying or adapting upstream source, verify the applicable license,
copyright holders, and notices for those files. Preserve required copyright and
permission notices and record the source revision and affected portions. MIT-derived
material retains its MIT notice obligations under either project license option.
Do not attribute an Apache patent grant to an upstream author solely because we
incorporate their MIT-licensed material. Dependencies and development tools retain
their own licenses; the project's dual license does not relicense them.

## Change and publication boundaries

Update durable knowledge and criterion evidence alongside the work. Record exact
commands, results, revision context, and remaining gaps; preserve stable record
and milestone identifiers. Follow the licensing and provenance requirements above
before any source adaptation.

Commits, pushes, publishing, releases, and repository settings require explicit
authorization. CI here only validates; it does not deploy or publish.
