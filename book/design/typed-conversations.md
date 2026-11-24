# Typed conversations

The initial product API joins borrowed Rust tools and per-turn typed results over
ACP. The following properties govern its supported scope.

> **`A-scoped-turn-ownership`** (accepted): **Scoped callbacks belong to their execution**
> Property: Only an active execution accesses its bound callbacks; application borrows end when execution is consumed or dropped. Overlapping mutable session executions are rejected.
> Authority: `D-provisional-scoped-turns`
> - Criterion `behavior` [test]: Runtime tests and compiler probes establish non-Send borrowing across await, restored caller access, and rejection of overlapping mutable use.

> **`A-typed-turn-completion`** (accepted): **Typed results require completed turns**
> Property: Success requires supported-schema validation and conversion, a valid candidate, normal terminal response, and settled local callbacks. Result types may change between turns in one conversation.
> Authority: `R-per-turn-result-types`
> - Criterion `behavior` [test]: Tests cover changing types, invalid/corrected/exhausted submissions, missing and duplicate results, abnormal stops, transport loss, and cancellation or callback overlap after candidate acceptance.

> **`A-turn-envelope-routing`** (accepted): **Submitted turn identifiers control callback routing**
> Property: Missing, malformed, and nonmatching identifiers cannot select or invoke current callbacks. Revoked queues cannot reach replacement callbacks. This authenticates no causal origin when a sender supplies the current identifier.
> Authority: `D-provisional-scoped-turns`
> - Criterion `behavior` [test]: Deterministic HTTP tests inject stale and malformed calls during a replacement turn, assert zero mutation, and verify valid calls and revoked queues.

> **`A-cancelled-session-invalidation`** (accepted): **Cancelled sessions cannot resume**
> Property: Cancelling or abandoning a started turn permanently invalidates that session and its dedicated connection; no further prompt or implicit history reset occurs. Dropping an unpolled execution sends no work.
> Authority: `D-initial-cancel-invalidation`
> - Criterion `behavior` [test]: Tests distinguish unpolled and started drops, reject reuse, and inspect peer prompt counts.

> **`A-owned-resource-cleanup`** (accepted): **Owned resources have bounded cleanup**
> Property: Drop revokes local access and signals existing cleanup infrastructure without async waiting or spawning. Awaited shutdown reports a bounded outcome; owned-process guards force termination on driver/runtime loss on supported macOS and Linux hosts. No detached-descendant or asynchronous application-destructor guarantee is made.
> Authority: `D-reviewed-api-baseline`
> - Criterion `behavior` [test]: Instrumented child-process tests observe exit/reaping and listener closure for success, constructor failure, early errors, drops, awaited shutdown and runtime teardown on both supported runtime flavors.

> **`A-supported-contract-boundary`** (accepted): **Advertised supported contracts are enforced**
> Property: Unsupported contracts fail before prompting. Supported input, serialized tool output, and final-result constraints are validated independently of Serde conversion.
> Authority: `D-derived-contracts`
> - Criterion `behavior` [test]: Contract tests cover every supported keyword and unsupported dialects/references/constraints, malformed input and output, and located preflight errors.

> **`A-progress-observation`** (accepted): **Optional observation preserves typed execution**
> Property: Observers execute locally within the polled execution and can borrow non-Send state. Progress is optional, bounded and best-effort, with explicit loss reporting. Permission decisions, validation, tool replies and authoritative typed results do not depend on progress delivery. Dropping observation revokes its queue; observer unwinding initiates owned-connection cleanup. Agent reports are not authenticated originating-turn or callback-execution evidence.
> Authority: `R-progressive-control`
> - Criterion `behavior` [test]: Compiler and runtime tests cover borrowed observers/tools, unchanged typed success/failure, queue limits and loss, polling fairness, callback panic/drop, capture release, and route isolation.

## Initial support

Use Tokio with I/O and time enabled on current-thread or multithread runtimes.
Borrowed callbacks remain local and may be non-Send; owned drivers contain only
owned transport and request/reply state. One session may be open per Agent.
Normal session drop releases its reservation and revokes its endpoint; another
session uses fresh history and credentials. Failed/abandoned started turns close
the connection. Recovery is explicit reconnection.

Expose typed command configuration, the `thinkwell::agent::{Claude, Codex}` presets, fixed typed Tool
descriptors, cloneable unbound Plans, borrowed per-turn bindings and run_in.
Plan::tool constructs a typed tool descriptor and binds its callback in one step.
Plan::run validates the plan locally before opening a temporary session whose
catalog is the plan's bound tools. It executes one turn and drops the session
before returning. Sequential successful runs on an agent use fresh history;
run_in retains explicit-session catalog matching and history. Invalid local
plans send no setup request; occupied agents return Busy. Unpolled runs do
nothing, and setup/turn cancellation or failure retains existing invalidation
rules. The temporary session uses the agent's default model; explicit builders
provide model selection. Both inline tools and bind retain borrowed, non-Send
callbacks and non-cloneable bound plans.
Plan::json borrows any Serialize value only while appending its compact JSON plus
one newline to the prompt, just like text composition. Serialization is immediate
and fallible, returning InvalidValue on failure; no schema derivation is required.
It preserves existing Plan state and introduces no special content block, quoting
protocol, or instruction-trust boundary.
Plan::code(source, language) copies source into a Markdown code block immediately,
returning `Result<Self>`. Its fence uses at least three backticks and is longer than
any backtick run in source. Source bytes are preserved; add a newline before the
closing fence if missing and one after it. An empty label is allowed. Backticks or
control characters in the label return InvalidValue immediately. Like json and text,
this preserves other Plan state, introduces no agent work, and is formatting rather
than syntax validation or an instruction-trust boundary.
Use HTTP MCP 2025-11-25 with a loopback listener, bearer credential, Origin and
body-size checks, and client permission grants restricted to declared tools. Turn/input and
turn/value envelopes are library protocol details, not callback parameters.

Validate the finite object/array/primitive/const/enum/oneOf/anyOf contract subset,
including signed int64 representation and finite f64 values (`format: double`
with `type: number`). Double values must convert to a finite f64; this does not
promise exact representation of every JSON decimal. An f64 contract rejects
serialized nonfinite values, including values encoded as null. Generate
nonrecursive nested derived types
inline for both input/result and output contracts, without requiring application
inline annotations. Recursive types remain unsupported. Unsupported dialects, references,
definitions, formats and constraints are preflight errors. Local contract
validation precedes conversion and follows output serialization. Tool inputs and
final answers use deserialization schemas; tool outputs use serialization schemas
so direction-specific Serde field names describe the actual values on the wire.

Defaults are 120 seconds per turn, 30 seconds per callback, 32 application calls,
three invalid result submissions, 30 seconds for setup, and 12 for shutdown.
Expose configurable nonzero bounds. Do not cap the whole agent lifetime.
Model selection is optional; default selection belongs to the agent. Preset
installation and authentication remain external.

The interoperability targets are Claude ACP 0.23.1 and Codex ACP 2.8.0 with
official Rust ACP SDK 5.4.0. Codex defaults to the adapter's read-only mode;
installation and authentication remain external. The adapter pin does not freeze
its compatible-range Codex dependency. Agent-owned native reads can occur without
ACP permission requests, and user MCP configuration still applies. Reserve the
MCP server name thinkwell for the library. A reported Codex Thinkwell-MCP startup
failure becomes a Configuration error during session setup or a subsequent turn,
including while an ACP request is pending. Reports are matched by session ID;
old-session reports cannot fail a replacement session. Before the new session ID
is known, retain at most 64 distinct reported IDs; excess reports fail setup
closed. Selecting the new session clears that temporary set and retains only
its failure state until replacement or connection teardown.

Permission requests can identify tools directly or refer to an earlier ACP tool
notification. Correlation is scoped to the active session and prompt, recognizes
the tested Claude and Codex naming conventions, and grants AllowOnce only for
declared tools (including the internal result tool). Unknown, conflicting and
terminal call IDs are denied. At 1024 tracked IDs, further new IDs disable grants
for that turn rather than evicting old identities. Correlation state is discarded
at turn completion/connection teardown. This is client permission policy, not
an operating-system sandbox or authenticated originating-tool identity. Support is initially macOS/Linux and Rust 1.100.0; Windows, embedding,
dynamic catalogs, concurrent sessions, independent progress streams, explicit cancellation
handles, and broader JSON Schema support are deferred.


## Wire contracts and validation

Application calls use turn/input envelopes; serialized outputs use a value envelope
in MCP structuredContent, with a matching object outputSchema. Text content retains
the serialized application value. This accommodates primitive outputs while keeping
the MCP output schema object-shaped. Both contracts derive from application types.
The internal result tool uses turn/value with a per-turn schema in the prompt.

HTTP bodies are limited to 256 KiB. Callback/turn deadlines and call budgets bound
individual executions; queued message capacity does not globally limit concurrent
HTTP senders. Header credentials are per session, so reopening cannot authorize
requests using the old credential. A session's routing is revoked on normal drop;
each later session obtains fresh remote history and endpoint credentials.

Every failed started turn closes its connection in this initial conservative
policy, including local exhaustion and abnormal remote completion. Preflight
errors and dropping an unpolled execution leave a usable session unchanged.
Callback errors and callback deadlines are tool errors the agent may handle;
turn deadline, result exhaustion, and call exhaustion terminate the execution.

Const and enum compare finite JSON numbers in their serialized decimal
representation without integer-to-float rounding. Integer contracts separately
reject floating representations. Unsupported schema errors identify the containing
property/items/alternative path. Validate schema dialect metadata before work;
only the 2020-12 dialect is supported.


## Optional progress observation

Plan::observe attaches one synchronous FnMut(&ProgressEvent) observer after plan
construction. The non-cloneable ObservedPlan wrapper supplies run and run_in;
callers clone unobserved base plans before adding captures. Execution remains
lazy and observers are never stored in transport/background tasks. They may
borrow local non-Send data separately from tool captures. Returned/dropped owned
futures release both sets of captures; pinned storage still needs to be dropped.

ProgressEvent and tool status/outcome enums are non_exhaustive. Payloads have
private fields and public accessors, permitting later additions. Events include
user-visible Message chunks, agent-reported AgentTool announcements/partial updates,
local ToolStarted/ToolFinished invocation processing, ResultRejected/ResultAccepted,
and EventsDropped. Local tool processing includes input validation, so a failed
invocation need not have entered the application callback. Agent tools are not
host execution facts; their IDs need not match local invocation IDs.

Progress omits reasoning text, prompts, raw arguments/results, HTTP credentials
and arbitrary error messages. Result rejections use a category-level reason and
submission number; structured schema-path diagnostics are not exposed in this
first version. Agent text/titles can themselves contain application data. Internal
result-tool announcements are hidden; explicit result events describe validation.
A validated candidate still needs normal terminal completion and settled callbacks.

The queue holds at most 128 entries and 64 KiB of event payload (including fixed
entry sizes and owned string lengths); overflow/oversized updates increment a
saturating loss counter. Loss is reported when the retained queue drains. Internal
tool-filter metadata is separately bounded to 128 IDs of at most 512 bytes; exhaustion
omits subsequent agent-tool reports with loss accounting. This does not bound the
agent transport's own buffers or application-owned logs. Unobserved runs create
no event queue or event formatting work.

The local observer driver polls execution before progress and yields after batches
of 16 events. Control traffic never waits for progress space. On normal return,
it closes the queue and delivers the bounded retained tail/loss report. Drop
revokes the queue without observer calls. Slow synchronous callbacks delay polling
and cannot be preempted by an async deadline. Observer panics unwind normally;
an observation guard also closes the connection if unwinding occurs while draining
an already-completed execution. Callback effects are not rolled back.

Temporary-session run includes setup in the observer's lifetime; known-session
notifications during remaining setup may be observed. Notifications before the
session ID is known are not attributed to this observer. Explicit open precedes
run_in and is outside observation. Session IDs filter notifications and each run
gets a distinct queue, but delayed same-session ACP messages cannot be assigned
an authenticated originating-turn identity. Connection-wide observation, streams,
async/fallible observer protocols and cancellation handles remain deferred.
