# 0018: `check` passes a plan whose knell can never be acknowledged

- status: open
- kind: defect
- phase: 6
- opened: 2026-09-15

`rescind check` and the engine read the authenticator list from two
different places, and nothing compares them.

- **Check** enumerates `humans()` and resolves a named authenticator over
  the `[authenticators]` table of the **inventory file**
  (`surface/src/resolve/site.rs:619` builds it; `core/src/check.rs:290`
  binds it as `auths`).
- **The engine** enumerates the same thing over the **`approval via:`
  binding** (`engine/src/gates.rs:296`), and with no binding declared
  `request_challenge` and `approve_proof` refuse outright: "the site
  declares no approval binding" (`engine/src/gates.rs:361`, `:397`, `:525`).

So a site that declares `[authenticators]` in its inventory and **no
`approval via:` line** produces a clean verdict for a plan with a knell, and
that knell is unacknowledgeable the instant it is reached. The plan does not
fail early; it applies, runs up to the irreversible step, and stops
somewhere no proof can be given. `daemon/src/run.rs:180` returns
`Ok(None)` for a missing binding without comment, so the daemon starts
normally too.

**How it was found.** Converting a real seance procedure
(`seance/rescind/migrate.scind`, a planned guest migration whose step 4 is
`zfs recv -F` into a live dataset). The text checks byte-identically with
and without its `approval via: hook(:authority)` line -- verified by
removing the line and the `hooks do` block and re-running `check`, which
printed the same verdict including "acknowledged by humans()".

**Why it is a defect and not a documented gap.** `docs/LANGUAGE.md:282`
already states the intended rule -- "`approval via:` names the binding that
publishes the authenticators a gate may name" -- which is the engine's
reading, not check's. Check is implementing something the language
reference does not describe, and the divergence is invisible until run time.

**Done when.** A plan carrying any gate or knell `ack:` other than `:none`
is refused at check time when the site declares no `approval via:` binding,
with a diagnostic naming the binding as the missing thing. Adding an E-code
is the usual four-place change by the gate's own guard (`proto/`'s
`Rescind.Proto.Diagnostics`, `core/src/diagnostics.rs`, the section 6.7
table in `docs/ROADMAP.md`, `docs/LANGUAGE.md`) plus a positive and a
negative golden.

Open in the same breath, and NOT settled here: whether check should also
require the two lists to agree -- an inventory naming `oncall` and a binding
publishing only `platform_a` is the same class of surprise one layer down,
but the binding's list is not knowable at check time without calling the
hook, so the honest answer may be that the inventory table is a declaration
the binding is checked against at boot rather than at check.

A rediscovery row belongs with the fix: `knell-unacknowledgeable-checks-clean`.
