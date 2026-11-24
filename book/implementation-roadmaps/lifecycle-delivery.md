# Lifecycle and change delivery

Roadmap identifier: `lifecycle-delivery`.

Current integration state: milestone 1 was delivered through owner-merged
[PR #2](https://github.com/dherman/thinkwell-rs/pull/2). See the
[shared integration evidence](first-product-slice.md#integrated-completion).
Its earlier local-stage evidence and pending-delivery statements are historical;
they retain their tested scope and do not describe current integration status.
[Milestone 2](#lifecycle-delivery-2) is a separate port-contract experiment on the
fork's `port-phase-adoption` branch, pending upstream review and integration.

Goal: extend the completed scaffold with owner-approved lifecycle and delivery
policy, without product functionality or a detailed port plan.

## Milestone 1. Integrate lifecycle and delivery {#lifecycle-delivery-1}

**Outcome:** The Book governs the Porting Phase, its human-approved transition,
and reviewable delivery; CI protects bootstrap relationships.

**Status:** Complete

**Identifier:** `lifecycle-delivery-1`

**Dependencies:** Completed project scaffolding. The project owner explicitly
approved this bounded assignment and its acceptance criteria in the lifecycle
and change-delivery request on 2026-11-17. The milestone was recorded Ready on that approval and is now In progress.
No port milestone is authorized.

**Design basis:** The owner's accepted lifecycle and delivery direction:
`R-temporary-migration-reference`, `R-porting-transition`, `R-reviewed-delivery`,
`D-thinkwell-identity`, and `D-future-independence`; [lifecycle](../lifecycle.md)
and [change delivery](../change-delivery.md).

**Tasks**

- [x] Record lifecycle authority and isolate migration research.
- [x] Define delivery, review, completion, and recommended GitHub protections.
- [x] Add the PR template and bootstrap consistency checks to existing CI.
- [x] Validate and review the complete change; record evidence and integration gaps.

### Completion criteria

#### Lifecycle and migration boundary {#lifecycle-delivery-1-lifecycle}

Accepted authority defines identity, the Porting Phase, human transition approval,
and future independence. Migration research is removable; historical scaffold
identifiers and evidence remain intact. No product design is introduced.

- **Anchor coverage:** None — project governance and scaffold only.
- **Evidence:** Local content review found the five accepted records, explicit
  transition checklist and future independence, migration-only source revision/API
  research and provenance, and no product functionality or detailed port plan.
  Historical `phase-zero` milestone and criterion identifiers/evidence are unchanged;
  only its display title and historical framing changed. Rendered authority records,
  lifecycle sections, and navigation were inspected. See validation context below.

#### Reviewable delivery {#lifecycle-delivery-1-delivery}

Guidance and the PR template cover approval, separate review, owner merge authority,
durable completion, tested revisions, and post-merge roadmap synchronization.
GitHub protections are recommendations, not claims of enabled settings.

- **Anchor coverage:** None — delivery workflow only.
- **Evidence:** Reviewed the delivery chapter and PR template against the owner's
  requested fields. They distinguish assignment, deliberate review, human approval,
  tested revision, integration, and post-merge synchronization. Recommended GitHub
  settings address CI, PR review, direct updates, merge permission, and stale
  approvals, including GitHub's sole-maintainer self-approval constraint. No settings
  were changed. The owner merged PR #1 and confirmed reviewing it in the synchronization
  conversation on 2026-11-18. See integration evidence below.

#### Checked bootstrap and bounded validation {#lifecycle-delivery-1-validation}

Automated checks protect startup, navigation, pin documentation, and CI entry
points. Documented checks pass and rendered records and navigation are inspected.
Required CI and human review pass before authorized merge; integration and roadmap
synchronization are recorded before durable completion.

- **Anchor coverage:** None — scaffold integrity, not product behavior.
- **Evidence:** All local commands below passed. Two consistency tests check the
  checkout and deliberately broken relationships, including absent guidance,
  navigation drift, mismatched documentation/pins, and direct engine invocation.
  Initial validation left launcher tests unchanged; the CI follow-up below
  serializes fixture lifetimes without changing assertions or production behavior. Rendered navigation, affected authority
  and roadmap records, and local HTML links/fragments passed inspection. Final-head
  and merge-commit Linux CI passed; PR #1 is merged. The owner confirmed human review.
  Integration of this roadmap synchronization remains outstanding; see below.

### Validation and review evidence

Validated 2026-11-17 on macOS arm64, against the uncommitted lifecycle/delivery
working tree based on `6bf7ada5c98b64222bcd495c9c57567a780252cb` on main.
At that validation, no new tested commit existed and changes were local, not
committed, pushed, or integrated. The owner subsequently authorized committing,
pushing, and opening a PR for review on branch `lifecycle-delivery-integration`.
PR #1 subsequently merged; its exact revisions and CI evidence are recorded below.
Durable completion remains pending the remaining delivery-record obligations.
Rust 1.100.0, mdBook 0.5.4, and Anchor Book revision
`9689b00416fd577bfb68a02f1fa310f6088404cf` remain unchanged.

Commands passed:

- `cargo anchor contract`
- `cargo fmt --all -- --check`
- `cargo check --workspace --all-targets --locked`
- `cargo clippy --workspace --all-targets --locked -- -D warnings`
- `cargo test --workspace --locked` (three existing launcher tests and two new
  consistency tests; no Thinkwell behavior tests)
- `cargo anchor check --config anchorbook.toml` (structural/v1, zero errors)
- `mdbook build` (using the documented local tool PATH)
- `cargo test -p anchor-runner --test bootstrap_consistency --locked`
- `git diff --check`; new files also inspected for whitespace

A separate post-implementation pass by the implementing agent reviewed all changed
and new sources, rendered navigation and affected records, and evidence scope.
This is self-review, not fresh-context or independent human review. Findings:
GitHub prevents PR author self-approval, so the recommendation now explains a
narrow owner exception while retaining human review and passing CI; historical
local scaffold completion must not imply durable integration, now explicitly
qualified. The initial consistency test incorrectly assumed a manifest-path alias;
it was corrected to the existing package-based alias, and rejection tests now
first require the unmodified checkout to pass. No launcher changes were needed.
A subsequent review made the negative identity fixture independent of the numeric
engine version. All affected checks were rerun successfully.

Limits: these checks establish scaffold/workflow integrity only. They do not verify
Thinkwell behavior, upstream source rights, external links, or enabled repository
protections. These local checks did not establish GitHub CI or Linux execution;
the subsequent CI evidence below supplies that execution evidence. The engine's
structural check does not validate lifecycle semantics, prose links, or delivery
approval; the human gate remains procedural. No verification runners were added.

### CI failure investigation

The initial head `6fa55d27fd4bcf7e52078f7b991559e48946f839` had different CI outcomes:
[push run 44255081373](https://github.com/dherman/thinkwell-rs/actions/runs/44255081373)
failed in the existing `missing_and_mismatched_engines_install_the_pin` test with
Linux `ExecutableFileBusy` / `Text file busy` (OS error 26), while
[PR run 44255183349](https://github.com/dherman/thinkwell-rs/actions/runs/44255183349)
passed all checks on the same head. The failure occurred starting the fixture
installer, before the new consistency tests ran.

The likely race is concurrent script creation and process spawning in launcher
tests: a child can temporarily inherit a writable script descriptor before exec
closes it, preventing Linux from executing that file. The focused correction holds
a process-local mutex for each fixture's lifetime, including script creation,
execution, and cleanup. Assertions, test coverage, production launcher behavior,
and pinning are unchanged. Lock poisoning is recovered so one failed assertion
does not prevent remaining fixtures from exercising their checks.

Local validation of this follow-up: formatting, workspace compilation, Clippy,
workspace tests, Book structure/rendering, and whitespace checks passed; 20 repeated
launcher test runs with three test threads passed. The failure was not reproduced
on macOS. A local Linux container was unavailable because Docker's daemon was not
running. Subsequent Linux CI passed on the corrected head and merge commit, as
recorded below. These successful runs do not prove that an intermittent failure
can never recur.

### Integration and synchronization evidence

Inspected GitHub on 2026-11-18. Owner `dherman` merged
[PR #1](https://github.com/dherman/thinkwell-rs/pull/1) at
2026-11-18 07:06:13 UTC. The final head was
`c1baf4d65c1d7b236f6580cdcb65bb4ced611086`; the integrated merge commit is
`83ac7e6faf7b4e7dfd74c306436760e44ba46a21`.

Both **Rust and Book checks** executions on the final head passed before merge:
[push run 44255800781](https://github.com/dherman/thinkwell-rs/actions/runs/44255800781)
and [PR run 44255807521](https://github.com/dherman/thinkwell-rs/actions/runs/44255807521).
The [main push run 44256276781](https://github.com/dherman/thinkwell-rs/actions/runs/44256276781)
also passed on the merge commit. These runs supply Linux CI evidence for the
fixture correction and bootstrap checks, not product verification.

The owner confirmed in the synchronization conversation on 2026-11-18: “I
reviewed the PR.” This supplies human-review evidence for PR #1 alongside the
confirmed owner merge. GitHub returns no reviews or PR comments; the review
confirmation is recorded here rather than represented as a GitHub approval.
No protection bypass is inferred. The earlier implementing-agent review and its
findings remain recorded above.

The owner requested this post-merge roadmap synchronization. It is a local,
uncommitted documentation follow-up on `port-design`, based on the merge commit;
its integration is still pending. No product milestone is assigned by this update.
For this documentation follow-up, `cargo anchor check --config anchorbook.toml`
passed structural/v1 with zero errors, mdBook 0.5.4 built successfully, and
`git diff --check` passed. The changed source and rendered integration evidence,
criterion links, and completion record were inspected. No implementation changed;
Cargo behavior tests were not rerun.

### Completion record

Complete. PR #1's implementation and the previously pending synchronization are
integrated: the latter was included in owner-merged PR #2 at
`1b21c83b9b713690d953df72244c7f9fcf8244ce`. PR #1 review and CI evidence remain
above; [PR #2 delivery evidence](first-product-slice.md#integrated-completion)
closes the synchronization gap. No lifecycle transition is implied.

## Milestone 2. Adopt the engine's Port working contract {#lifecycle-delivery-2}

**Outcome:** The configured phase activates reusable port guidance, including engineer questions, source reconstruction, coherent target design, and implementation and transition boundaries.

**Status:** In progress

**Identifier:** `lifecycle-delivery-2`

**Dependencies:** Milestone 1; Anchor Book `8165d599878da6089f367af0d97e88be45631e46`, published on its experimental `port` branch. The fork owner requested this bounded adoption on a branch of `kwdev/thinkwell-rs`; upstream approval and merge remain separate.

**Design basis:** `R-reproducible-checks`, `R-temporary-migration-reference`, `R-porting-transition`, `R-reviewed-delivery`, and the existing [lifecycle](../lifecycle.md) and [change-delivery procedure](../change-delivery.md).

**Tasks**

- [x] Pin the published engine revision and select `port` in the authoritative configuration.
- [x] Retrieve configured guidance at agent startup and reconcile current phase descriptions and pinned authoring references.
- [x] Identify the source repository in migration evidence and make incorporation into owning design explicit in the delivery procedure.
- [x] Extend bootstrap checks, run by existing CI, to detect loss of configured Port startup.
- [x] Run applicable local checks and record their exact scope and results.
- [x] Review the complete change and record findings and remaining integration requirements.
- [ ] Obtain upstream owner review, pass required CI, and integrate before declaring durable completion.

### Completion criteria

#### Port contract and bootstrap {#lifecycle-delivery-2-contract}

The repository launcher installs the exact published revision, configured retrieval includes the Port section once, and the consistency checks reject missing configured retrieval or a different project phase. The common contract remains available for configuration diagnosis.

- **Anchor coverage:** None: development tooling and working guidance; no product anchor changes.
- **Evidence:** Passed locally on 2026-11-24 on `port-phase-adoption`, based on `59b8bd56e64cfd5a0318673421dd176316987c07` plus this adoption change. The real launcher installed `8165d599878da6089f367af0d97e88be45631e46` from GitHub with Rust 1.100.0 and reported the exact pinned identity. `cargo anchor contract --config anchorbook.toml` included the Port section exactly once; the common command remained available separately. `cargo test -p anchor-runner --locked` passed all five launcher and consistency tests, including seeded loss of the configuration argument or the Port phase, and the existing CI entry-point relationships.

#### Coherent guidance and delivery {#lifecycle-delivery-2-guidance}

Active instructions agree on Port and the engine pin, preserve the existing human-controlled lifecycle gate, and direct accepted discoveries into owning design before roadmap implementation. The Book passes structure and rendering checks. Review and integration are recorded without treating this tooling change as product verification or completion of the port.

- **Anchor coverage:** None: project guidance and delivery policy.
- **Evidence:** `cargo fmt --all -- --check`, `cargo check --workspace --all-targets --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, and `cargo test --workspace --locked` passed on Rust 1.100.0; the workspace suite ran 100 tests including doctests, with no failures or ignored tests. The Python capability-probe regression passed one test. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`, `cargo anchor check --config anchorbook.toml`, mdBook 0.5.4's build, and whitespace checking passed. Anchor Book's existing rendered-output acceptance, directed at this Book, checked 37 complete records and 1,056 local links. Separate scheduler, sentiment, unminifier, website, and live-agent checks were not rerun for this tooling and guidance change.
- **Review:** After implementation, the implementing agent reviewed the complete diff, installed-engine output, phase descriptions, pin references, bootstrap failure cases, and rendered records and links. Current guidance consistently selects Port; historical test revisions remain historical. Existing product authority and the human-approved lifecycle transition gate are preserved. No independent-agent or upstream owner approval is claimed. CI, upstream owner review, merge, and synchronization remain outstanding; this change does not establish that existing product design is already fully consolidated.

### Completion record

Pending upstream owner review, required CI, merge, and evidence synchronization.
