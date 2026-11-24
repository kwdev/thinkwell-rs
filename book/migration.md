# Porting research (migration-only)

This chapter is temporary migration evidence during the [Porting Phase](lifecycle.md).
The source is the [TypeScript Thinkwell repository](https://github.com/dherman/thinkwell);
this repository develops the Rust target. The inspected revisions below identify
research inputs and do not establish an ongoing compatibility baseline.
Observed legacy behavior is not an accepted Thinkwell requirement. This chapter
includes inherited scaffolding research and the fresh API review below; neither
selects a compatibility baseline. No ongoing revision tracking is required.

## Inspected source and concepts

The Phase 0 research inspected Thinkwell's README, AGENTS guidance, manifests,
public exports, examples, and completed MVP, functional-plan, and conductor RFDs.
The source snapshot was
[`2393101fd66b8471f33bb3a4908aeee919fdf0ce`](https://github.com/dherman/thinkwell/tree/2393101fd66b8471f33bb3a4908aeee919fdf0ce).

Its public surface includes agent opening and lifecycle, sessions, immutable
fluent plans, structured output schemas, tools, skills, and streaming thought
events. The examples combine ordinary deterministic code with model calls and
typed results. These concepts inform later design research; they do not settle
Rust compatibility or implementation choices.

The TypeScript monorepo separates its high-level library, ACP handling, protocol
types, and conductor, alongside editor tooling and examples. Historical designs
describe an external Rust conductor and its later TypeScript replacement.
Neither historical implementation constraints nor package boundaries determine
this Rust workspace's eventual crates.

Reference entry points:

- [Public API exports](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/packages/thinkwell/src/index.ts)
- [MVP design](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/doc/rfd/completed/mvp.md)
- [Immutable plans](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/doc/rfd/completed/functional-plan.md)
- [Conductor design](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/doc/rfd/completed/conductor.md)
- [Examples](https://github.com/dherman/thinkwell/tree/2393101fd66b8471f33bb3a4908aeee919fdf0ce/examples)

## API design review

At the owner's request on 2026-11-18, inspected the current upstream HEAD,
`2393101fd66b8471f33bb3a4908aeee919fdf0ce`, which matches the earlier snapshot.
Read both READMEs, all six top-level TypeScript examples (greeting, summarization,
classification, sentiment, scheduling, and unminification), the website quickstart
and Agent/Plan/Session API pages, and the agent-api, functional-plan,
schema-providers, and conductor RFDs. Also inspected schema generation, Plan,
and Session source, explicit-configuration design, and the opening build-command
design sections. This was document/source research: no examples or tests ran and
no implementation was copied. Historical RFD proposals and documentation are
intent evidence, not proof of implementation behavior.

### Programming model observed

- **Application code owns composition.** The
  [scheduler](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/examples/src/schedule.ts)
  uses the model to translate availability into structured constraints, Z3 to
  solve them, then another model call to explain the result. The
  [unminifier](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/examples/src/unminify.ts)
  interleaves model judgment with deterministic formatting and AST changes;
  application code batches and limits concurrent requests.
- **Typed results are the normal entry point.** Even the greeting and summary
  examples declare result types. Classification returns a discriminated union
  consumed by an ordinary switch. Field documentation and constraints contribute
  to the schema shown to the model.
- **Tools are local callbacks.** Greeting provides a no-input time callback;
  sentiment wraps an ordinary library with a typed input callback. Callers attach
  tools beside the prompt without constructing an MCP server themselves.
- **Control is incremental.** The
  [Agent API](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/website/api/agent.mdx)
  starts with named agents and permits custom commands. Independent calls use
  ephemeral conversational context on a reused connection; explicit sessions
  preserve context with the same prompt-building interface.
- **Plans are composable values.** The functional-plan RFD explicitly motivates
  immutability through reusable base plans, independent variants, and helpers that
  do not modify their inputs. Prompt methods distinguish instructions, quoted
  material, and code. These formatting conventions are not a security boundary.
- **Observation is optional.** The
  [Plan API](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/website/api/plan.mdx)
  offers `run()` for the final typed value and `stream()` for events plus a separate
  result promise. Skills add stored or programmatic instructions with on-demand
  disclosure. Neither requires a different application orchestration language.

### Schema generation and runtime integration

The [schema-provider design](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/doc/rfd/completed/schema-providers.md)
separates schema production from the runtime API. `SchemaProvider<T>` and
`schemaOf<T>()` permit alternatives to generated schemas. The optional type-first
path discovers `@JSONSchema` annotations and injects `Type.Schema`; compiler,
loader, declaration-generation, and editor integration support that convenience.
The schema machinery serves both tool contracts and final result types.

Runtime MCP integration serves a different purpose: exposing local callbacks and
carrying the structured `return_result` invocation. The conductor design bridges
MCP-over-ACP to HTTP for agents lacking the native transport. Replacing schema
generation does not itself decide the Rust transport or conductor design.

The inspected Plan source wraps non-object result schemas for tool compatibility
and performs limited required-field checks before accepting a result; it does
not establish full JSON Schema validation. Session closure is local bookkeeping
in the inspected Session source. These details qualify documentation claims and
must not silently become Rust guarantees. This review did not audit all MCP
validation, concurrency, agent support, or failure paths.

The owner's Rust direction is to use derives for type-derived contracts without
an additional schema build tool; its durable definition is in
[design](design/index.md#type-derived-contracts). The remaining observations guide
API exploration, without accepting every legacy method, mechanism, or behavior.

## Example selection for the next milestone {#example-selection}

At the owner's request following PR #2, refreshed upstream at the same revision
`2393101fd66b8471f33bb3a4908aeee919fdf0ce`. The repository has six runnable examples
under examples/src and four tutorial sources under website/examples. The supplied
[example index](https://thinkwell.sh/examples) identifies greeting, sentiment,
unminification, and scheduling. The web fetch of the scheduler page was unavailable;
its repository MDX source supplies the tutorial evidence below. No browser session,
upstream example execution, or model inference was needed for this planning pass.

| Example and source | Main lesson | Rust candidate / disposition |
| --- | --- | --- |
| [Greeting](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/examples/src/greeting.ts) and hello-world tutorial | A local time tool contributes facts to a typed greeting; the script also streams progress. | `chrono` or `time` for local date/time; IANA-zone naming needs separate consideration. Useful small tool example, but little new coverage beyond current docs; omit streaming unless separately assigned. |
| [Summarization](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/examples/src/summarize.ts) | Read text and ask for a typed summary. | Standard Rust file I/O plus existing Plan/Serde/Schemars; simple onboarding candidate. |
| [Classification](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/examples/src/classify.ts) | Match a typed discriminated result in ordinary application code. | A Serde tagged enum and Rust match; no extra algorithm crate required. |
| [Sentiment](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/examples/src/sentiment.ts) and sentiment-analysis tutorial | Wrap a deterministic package in a tool; the agent chooses passages and synthesizes a report. | `vader_sentiment` 0.1.1 is a pure-Rust candidate, not an AFINN-compatible replacement for npm `sentiment`. Good later borrowed-tool example. |
| [Scheduler](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/examples/src/schedule.ts) and meeting-scheduler tutorial | Typed language-to-constraints conversion, deterministic solving, then another typed prompt. | `z3` 0.23.0 wraps the same solver as npm `z3-solver`. Recommended first application example; native setup must be tested. |
| [Unminifier](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/examples/src/unminify.ts) and javascript-unminifier tutorial | Model naming judgments between formatting, AST transformations, and scoped renaming; bounded parallel batches. | Oxc parser/semantic/codegen is a candidate; SWC is another. Scope-aware rename and collision correctness require a larger investigation, so defer. |

The scheduler best exposes the code/prompt alternation requirement without
requiring new library API. It does not exercise borrowed application tools; the
sentiment example is the natural complementary candidate. Do not add artificial
tools to scheduling solely to cover every library feature in one program.

### Dependency findings

Published metadata and local cargo package sources were inspected on 2026-11-20:

- [`z3` 0.23.0](https://docs.rs/z3/0.23.0/z3/) is MIT-licensed and declares Rust
  1.88.0, within this project's pinned compiler. Its API has integer/Boolean terms,
  Solver, Sat/Unsat/Unknown results, model evaluation, timeout parameters, and
  unknown-reason reporting. It requires a native Z3 library; this is a real setup
  difference from the npm/WASM package. Default system linking, vendored source,
  vcpkg, and downloaded GitHub releases are documented. `bundled` is a deprecated
  alias; prefer an explicit native setup decision. The proposed example prefers
  a locked vendored build, isolated from normal library compilation.
- [`vader_sentiment` 0.1.1](https://docs.rs/vader_sentiment/0.1.1/vader_sentiment/)
  is MIT-licensed and returns positive/negative/neutral proportions and a compound
  score. It uses VADER rules, not the AFINN score/comparative/token-list semantics
  of npm `sentiment`. Its latest published version dates to 2021. Build and
  behavioral verification, floating-value schema handling, and adapted teaching
  language remain necessary before selecting it; numerical parity is not promised.
- [`oxc` 0.159.0](https://docs.rs/oxc/0.159.0/oxc/) is MIT-licensed and declares
  Rust 1.98.0. Parser, semantic, and codegen features make it a plausible replacement
  for parts of Babel/Prettier. [`swc_ecma_parser` 47.1.0](https://docs.rs/swc_ecma_parser/47.1.0/swc_ecma_parser/)
  is Apache-2.0 and belongs to an alternative parser/visitor/codegen ecosystem.
  Neither was built or established as a drop-in rename/formatting replacement.
  Standard iterator batching and bounded async work can replace lodash/p-limit
  mechanics, but concurrency would need to respect Thinkwell's session constraints.
- `chrono` 0.4.45 and `time` 0.3.55 are MIT OR Apache-2.0 candidates for greeting
  date/time formatting. A complete timezone-name/localization equivalent to the
  JavaScript Intl calls was not established.

Crates are candidates, not additions to the library dependency set. Exact
versions and native prerequisites must be locked and tested in an approved
example assignment. Package license metadata is not clearance to copy arbitrary
upstream examples, lexicons, or third-party JavaScript fixtures.

### Deliberate scheduler differences proposed for Rust

The TypeScript solver skips attendees with empty availability, treats all non-sat
outcomes as no solution, and permits hour 17 in its general domain. The Rust
proposal instead treats empty availability as no feasible meeting, preserves
Unknown/timeout as inconclusive, and fits a full one-hour meeting inside the
business day. The model's parsed roster must match the input roster exactly.
The solver proves only the parsed constraints, not faithful natural-language
interpretation. Display the parsed windows and canonical solver result separately
from the generated explanation.

Numeric minimum/maximum keywords and schema references in the original tutorial
are not supported by the current Rust validator. An ignored scratch check showed
that nested records and a weekday enum marked `#[schemars(inline)]`, using i64
hours, pass the existing support validator and decoder. Proposed Rust range and
roster validation is explicit application code. This avoids selecting broader
library schema support implicitly while retaining meaningful validation.

The corresponding [Rust milestone proposal](implementation-roadmaps/meeting-scheduler.md)
states independent application rules and acceptance criteria. This inventory and
all upstream comparisons remain removable migration evidence. No source,
prompt text, tutorial prose, or data was adapted in this planning work; the
upstream examples directory still has no independently established license notice.

## Source provenance

The four core package manifests and the high-level package README declare MIT.
The inspected tree has no root license file; its explicit license file is under
the VS Code extension. That MIT text requires copyright and permission notices
to accompany copies or substantial portions. Its location alone does not establish
applicability to every source file or any third-party material.

Before copying or adapting implementation, independently establish the applicable
license, copyright holders, and required notices for the selected files. Preserve
those notices and document the source revision and affected portions. Choosing
Apache-2.0 for this project does not remove upstream MIT notice obligations or
establish an Apache patent grant from MIT-only upstream authors. Separately
identified third-party material retains its own terms. Phase 0 copies no
substantive upstream implementation; no upstream code attribution is claimed yet.

## Unresolved migration questions

Which capabilities and observable behavior, APIs, tests, examples, and documentation
should be selected for the port after this review? Which differences should
become explicit Rust-native decisions? No compatibility scope is accepted.
For each adaptation, establish file-specific rights, notices, source revision, and
affected portions before copying; the project license alone is not clearance.

Keep TypeScript inventories, parity analysis, source links, and migration notes
here (or in migration-only chapters), with accepted Rust requirements and designs
in their durable owning chapters. Do not create runtime, build, test, packaging,
submodule, CI, release, or contributor workflow dependencies on the legacy repository.

## Removal

At the human-approved transition, remove this chapter and its SUMMARY entry and
incoming migration links. First retain any legally required notices and provenance
in durable records alongside adapted material. No substantive implementation has
been adapted so far. Migration research must not be necessary to interpret the
remaining requirements, designs, tests, or delivery records.


## Progress API refresh

For the owner-assigned progress design, re-read the pinned upstream
[greeting example](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/examples/src/greeting.ts)
and [Plan API documentation](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/website/api/plan.mdx).
Greeting displays message chunks from stream(), then separately awaits result.
The docs describe eager execution, independent event iteration/result lifecycles,
message/thought/tool/plan events and continued execution after early iterator exit.
These are source observations, not Rust requirements or newly verified TS runtime
behavior. No source was copied or adapted. The Rust proposal keeps typed results
separate from progress but recommends lazy execution and a borrowed synchronous
observer initially. Migration links stay here; the Rust design stands independently.


## Unminifier refresh {#unminifier-refresh}

The assigned investigation re-read the pinned
[TypeScript source](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/examples/src/unminify.ts)
and [tutorial source](https://github.com/dherman/thinkwell/blob/2393101fd66b8471f33bb3a4908aeee919fdf0ce/website/examples/javascript-unminifier.mdx).
The published tutorial entry is
[JavaScript Unminifier](https://thinkwell.sh/examples/javascript-unminifier);
this refresh read the repository MDX, not the live page. The source formats with
Prettier, uses ast-grep for a particular UMD wrapper, asks the model to discover
functions, analyzes batches of 30 with concurrency limited to five, deduplicates
suggested names, then invokes Babel program-scope renaming. The tutorial omits
the source's later duplicate-name suffix pass. These are source observations;
neither the TypeScript example nor its rename safety was tested here.

The Rust proposal deliberately uses parser-discovered identities, sequential
batches and fresh names reserved against existing bindings and unresolved names.
It omits UMD conversion and uses original fixtures. The proposal lives in the
[unminifier investigation](implementation-roadmaps/unminifier.md); these migration
references are not permanent Rust requirements.

Cargo-published source inspection covered oxc, oxc_semantic and oxc_codegen
0.159.0 (MIT, Rust 1.98.0), and swc_ecma_transforms_base 52.0.0 (Apache-2.0,
no declared rust-version). Package VCS revisions are respectively
`d9483516f84068f867e0a3949ac0ec53ef82aa63` for oxc and
`fa67ecc2e89b2d89e6c30b31d4adb11e0dfa5b08` for SWC transforms.
Inspected Oxc Scoping::rename_symbol and Codegen::with_scoping, including object
shorthand printing; inspected SWC resolver documentation and rename/renamer APIs.
Oxc's rename preconditions require collision checks by the caller. Package metadata
and source mechanisms establish candidacy, not a compiled or verified pipeline.
No source, prompts, tutorial text or third-party fixtures were adapted, and no
repository dependency was added. The existing upstream file-license uncertainty
still applies before any future copying.
