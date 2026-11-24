# Agent startup

1. Run `cargo anchor contract --config anchorbook.toml` from the repository root. The launcher checks and
   installs the pinned engine automatically; see `CONTRIBUTING.md` for prerequisites.
2. Read `README.md`, `CONTRIBUTING.md`, and `book/working-on-the-project.md`.
3. Read `book/SUMMARY.md` and every listed chapter in full from this checkout.
4. Run `cargo anchor check --config anchorbook.toml`.
5. Before implementation, report readiness using the contract's template:
   current phase, assigned milestone, open decisions, check results, and evidence gaps.
6. Begin implementation only on a human-approved Ready milestone; record it as
   In progress and continue only within that assignment. Existing approval need
   not be requested again. A roadmap alone does not authorize work.
7. Reconcile durable discoveries, changed decisions, and evidence into the Book.

Detailed project truth belongs in the Book. The current lifecycle state is the
Porting Phase; legacy TypeScript behavior is migration evidence, not permanent
Thinkwell authority. Distinguish observations from accepted requirements. This
workflow update authorizes no detailed port milestone. Ending the Porting Phase
needs separate human approval; future phases retain no active legacy dependency.
See the Book's lifecycle and change-delivery guidance.
Do not commit, push, publish, or change repository settings without explicit
authorization. Verify applicable licensing and attribution before adapting source.
