# Boundary and non-goals

The long-term goal is Thinkwell implemented in idiomatic Rust. Supported user-facing
concepts, behavior, and guarantees require explicit design and human review.

The completed scaffolding supplies the workspace, development launcher, Book, and
CI. The [first product slice](implementation-roadmaps/first-product-slice.md) is now
approved for local implementation. Its scope is typed persistent conversations,
borrowed per-turn tools, finite schema validation, and owned subprocess cleanup
using Tokio and HTTP MCP. Product properties live in
[typed conversations](design/typed-conversations.md); implementation evidence and
remaining Linux CI/review/integration gaps live in the roadmap.

The earlier lifecycle-delivery assignment did not authorize product work; the
separate product milestone approval does. Dynamic catalogs, concurrent sessions,
shared host embedding, broad provider presets, arbitrary JSON Schema, progress
streams, Windows support, installation/authentication automation, and publication
remain outside this first slice. Commit, push, merge, release, repository settings,
and ending the Porting Phase retain their separate authorization boundaries.

The lifecycle remains Porting Phase and the engine working mode is `port`.
Accepted product anchors state intended guarantees, not proof that
every criterion has been verified or that delivery is complete.
