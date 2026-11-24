# Lifecycle

The current project lifecycle state is **Porting Phase**, governed by
`D-thinkwell-identity`, `R-temporary-migration-reference`,
`R-porting-transition`, and `D-future-independence`.
This repository is the future of Thinkwell. During porting, distinguish the legacy
TypeScript implementation from the emerging Rust implementation. After transition,
unqualified Thinkwell refers to this project and its Rust implementation.

The engine working mode is `port` in `anchorbook.toml`. Configured contract
retrieval supplies the common guidance plus the port procedure. The lifecycle
gate below continues to govern when this project's port may end; selecting an
engine phase does not establish its completion.
Historical Phase 0 records describe completed scaffolding, not a product phase.
Future phases may be named as needed without a numbered sequence.

## Porting Phase

Legacy observable behavior, APIs, tests, examples, and documentation may inform
migration decisions; they do not permanently govern Thinkwell architecture or
behavior. Requirements and design must be explicit Rust-native Book records.
Differences must be deliberate, documented decisions, not accidental drift or
an assumed indefinite parity obligation.

Keep source inventories, links, parity analysis, and migration notes in removable
migration-only chapters. Do not create a runtime, build, test, packaging, Git
submodule, CI, release, or contributor workflow dependency on the legacy repository.
Preserve applicable licenses, copyright, provenance, and attribution for adaptations.
Compatibility scope, guarantees, architecture, anchors, and implementation slices
remain subject to research and separate human approval. No detailed port roadmap
is authorized by lifecycle integration.

## Human-approved transition gate

A completed roadmap never ends the Porting Phase automatically. Prepare evidence
for each item below, then obtain the project owner's explicit transition approval:

- Every capability selected for the port is implemented, deliberately rejected,
  or explicitly deferred through a Rust-native decision.
- Requirements and design are understandable without the TypeScript repository.
- Rust tests independently define and verify supported behavior.
- No runtime, build, test, CI, packaging, release, or contributor workflow depends
  on the TypeScript repository.
- No active agent instruction or roadmap requires TypeScript parity or synchronization.
- All TypeScript-specific migration material is identified for removal from the
  active Book and repository state, including navigation and incoming links.
- Legally required notices and provenance records are retained.
- The TypeScript implementation is ready to be deprecated.
- A human explicitly approves the transition, separately from milestone completion.

Record the owner, approval, reviewed revision, checklist evidence, and integration
state in the transition's owning delivery/roadmap record. In that approved change,
remove migration-only material and references, update active guidance and lifecycle
authority to the new state, and record deprecation. Approval cannot be inferred from
local checks, merge of an unrelated change, or roadmap completion.

## Future phases

Thinkwell is governed by this repository's Book, decisions, code, tests, and delivery
records. The TypeScript implementation is deprecated; there is no continuing parity,
compatibility, synchronization, or tracking obligation.

Active agent instructions, requirements, designs, roadmaps, CI, builds, tests, and
releases neither depend on nor refer developers back to the TypeScript implementation.
Migration-only documentation is removed from the active repository state, except
legally required notices or provenance. Reconcile historical records' active links
and instructions during transition while preserving stable identifiers and truthful
evidence; Git history retains removed migration research. Future lifecycle states
can be introduced as needed without a numbered sequence.
