# Introduction

Thinkwell combines deterministic code with LLM-powered reasoning. This repository
is the future of Thinkwell, implemented in idiomatic Rust. The unpublished library
is named `thinkwell`; publication is not authorized by that choice.

The [Anchor Book method](https://anchor-book.github.io/anchor-book/) organizes
version-controlled project knowledge. Authority records state requirements and
accepted decisions; design chapters define the intended system; roadmaps record
assigned work, results, and gaps. Acceptance, implementation, and verification
are distinct.

The current lifecycle state is the [Porting Phase](lifecycle.md). The engine's
working mode is `port` in `anchorbook.toml`, so configured contract retrieval
includes the port procedure. Temporary source research lives in
[migration-only material](migration.md).

## Licensing

Original code and documentation are licensed under `MIT OR Apache-2.0`, as recorded
in `D-dual-licensing`. Complete texts are in the repository-root `LICENSE-MIT` and
`LICENSE-APACHE`; both Cargo packages inherit the shared declaration. The root
`CONTRIBUTING.md` defines contribution terms and source-adaptation obligations.
Separately identified third-party material retains its terms and required notices.
