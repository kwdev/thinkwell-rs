# Change delivery

This procedure implements `R-reviewed-delivery`. The root `CONTRIBUTING.md` owns
installation and validation commands; this chapter owns delivery decisions.

## Planning approval

Work requires an accepted design basis and a human-approved Ready milestone.
Record it In progress before implementation and stay within its assignment.
A roadmap alone does not authorize implementation. Scaffold choices do not imply
product decisions. Lifecycle integration authorizes no detailed port milestone.

## Deliberate review

After implementation and before merge, perform a review pass separate from the
implementation work. The sole maintainer, a fresh-context agent, or a separate
review agent may perform it. When an agent implemented the change, prefer a
different agent or a fresh context for review. A second human reviewer is not required.

Review the complete diff: code, tests, Book records, evidence, and roadmap updates.
Before review, incorporate agreed discoveries into their owning design chapters
and reconcile affected authority, anchors, and roadmap work. Replace duplicated
roadmap definitions with links to that design; readers must be able to understand
the target without reconstructing research or milestone history.
Check accepted authority, scope, criterion coverage, limitations, and the tested
revision. Record the reviewer or context, significant findings and their resolution
in the PR and owning roadmap evidence. Material follow-up changes require renewed
review of their effects. Agent review never substitutes for required human review.
The project owner retains final approval and merge authority.

## Merge and durable completion

Only the project owner, or a maintainer explicitly authorized by the owner for that
scope, may merge or declare durable completion. Agents need explicit authorization
for those actions. Local implementation, a commit, passing local tests, or a draft
PR is not durable completion. Required CI and human review must pass before merge.
Use the Anchor-aware PR template to connect the change to authority and evidence.

Evidence identifies the tested revision (or base commit plus explicitly uncommitted
working-tree state), commands, results, untested scope, and integration state. Do
not describe local results as CI results. Before merging, confirm required checks
and review apply to the actual revision being integrated.

After merge, synchronize the owning roadmap with the integrated revision, criterion
evidence, findings, remaining gaps, and completion status **before starting the next
milestone**. When synchronization needs a follow-up change, deliver it under the
same approved assignment and record its integration. Leave work In progress until
its outcome, criteria, and durable integration hold. Historical scaffolding's
bounded local completion records retain their original meaning.

Ending the Porting Phase requires its own explicit [transition approval](lifecycle.md#human-approved-transition-gate).
No generic mutation, proof, nextest, or compatibility obligations are added here;
product verification methods need accepted guarantees and anchors.

## Recommended GitHub protections (not enabled by this document)

A maintainer should configure a ruleset or branch protection for the primary branch:

- Require the existing **Rust and Book checks** CI job to pass for the revision
  being merged; confirm its exact check context in GitHub after CI runs.
- Require a pull request and review before merge. Require one approving review
  where an eligible reviewer is available. GitHub authors cannot approve their own
  PRs: for sole-maintainer delivery, document the owner's deliberate human review
  and use only a narrowly scoped owner bypass if necessary, retaining passing CI.
  Do not treat agent review as a GitHub human approval or silently waive review.
- Restrict direct updates, force pushes, and deletion of the primary branch;
  limit any bypass to the project owner and document its use.
- Permit merges only by the owner or explicitly authorized maintainers.
- Dismiss stale approvals after material changes; obtain renewed review of the
  resulting change before merge. Prefer GitHub's dismissal-on-new-commits option
  so this does not depend solely on detecting material changes manually.

These are recommended settings, not verified repository configuration. This work
does not enable protections, alter permissions, publish, or authorize a push.
