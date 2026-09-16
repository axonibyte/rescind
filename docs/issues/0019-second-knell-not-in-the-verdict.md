# 0019: A plan's second knell is missing from its verdict

- status: open
- kind: defect
- phase: 6
- opened: 2026-09-16

A plan may hold more than one knell. The verdict reports the first one and
computes its reversible span as though the others were ordinary steps.

`core/src/check.rs:333` derives `point_of_no_return` from `first_knell`, and
`:350` sets

```rust
let back_to = match first_knell {
    Some(n) if n < last_step => Some((last_step, n)),
    _ => None,
};
```

so `reversible_back_to` names the FIRST knell as the step a revert reaches
back to. With a second knell between it and the end, that is false: a knell
carries `undo_locus: :none`, an undo cannot pass it, and the revert stops at
the LAST knell instead. The prose says "step N reversible back to step M"
(`core/src/prose.rs:118`) and states the optimistic branch as fact.

`holds_at` immediately above -- `:313` -- already collects `knell_steps` as a
`Vec` and calls `split_segments` over all of them, so the surrounding function
knows a plan can hold several. Only these two fields read `first_knell`.

**This is live in the project's own acceptance artifact.**
`tenants/t2/expected/node-b-manual/verdict.json` has knells at step 4
(`fence_corpse`) and step 5 (`rollback_ahead_datasets`) and asserts

```json
"point_of_no_return": { "step": 4, ... },
"reversible_back_to": { "from": 10, "to": 4 }
```

-- that reverting from `commit()` reaches back to step 4, across step 5's
`zfs rollback -r`, whose whole declared property is that what it destroys does
not come back. The golden froze the wrong answer, and `node-b-auto` has the
same shape.

**How it was found.** Writing seance's manual promotion ladder as a plan
(`seance/rescind/promote.scind`, epic 1b). The real procedure has two points
of no return -- the fence, which is a power cut, and a conditional
`zfs rollback -r` -- and the verdict named only the fence. The conditional
`when` was ruled out as the cause by re-checking with both knells
unconditional: same output, `to: 8` across a knell at 9.

It then reproduced without being looked for. seance's failback
(`seance/rescind/failback.scind`, epic 2, written from a different source
file) has two knells with no `when` over either -- the reverse `zfs recv -F`
at step 6 and the interim lineage prune at step 10 -- and its verdict says
"step 11 reversible back to step 6". Two real procedures out of two have more
than one point of no return, which is the argument that this is not an edge
case: a plan with one knell is the special case, not the general one.

**Done when.** `reversible_back_to` names the last knell at or before
`last_step`, and the verdict reports every knell rather than one. The prose
for a plan with several says so. `core/tests/check.rs:762` covers a single
knell only and gains a two-knell case; T2's two goldens are regenerated, and
the diff is the point rather than a chore -- it is the project correcting a
claim it had checked in.

Open and NOT settled here: what the right answer is when a later knell is
inside a `when`. The span then depends on a guard that is not known at check
time, and the honest verdict may be two spans with their condition named
rather than one number. The unconditional case is wrong today either way and
does not wait on that question.

A rediscovery row belongs with the fix: `second-knell-not-in-the-span`.

## Fixed in part, 2026-09-16

**The false claim is gone.** `reversible_back_to` now names the LAST knell at
or before the last step, because an undo walks backwards and stops at the first
knell it meets, which going backwards is the last one in the plan.
`reversible_through` still names the FIRST knell and that was always right --
it answers where forward reversibility ends, a different question.

`core/tests/check.rs` gains a two-knell case, mutation-checked: it fails
against the old expression with "a revert stops at the LAST knell, not the
first". `tenants/t2/expected/node-b-manual`'s `verdict.json` and `verdict.txt`
are corrected to `to: 5`; the only delta in the whole golden was that field,
which is what says this was a correction and not a change. `node-b-auto` holds
one knell and is untouched.

**The issue stays open for two things that did not land.**

1. **The verdict still reports one knell rather than every knell.** That adds
   a field, which bumps `VERDICT_VERSION` and regenerates all 50 golden
   verdicts. The span fix removes the untrue sentence on its own, so the
   enumeration should ride with the next schema bump rather than force one.
2. **The conditional case is unresolved.** A knell inside a `when` may not run,
   so the truthful span depends on a guard no one can evaluate at check time.
   The fix reports the pessimistic answer -- the knell might run, so the span
   stops there -- which is the safe direction and not obviously the right one.
   The honest verdict may be two spans with the condition named.
