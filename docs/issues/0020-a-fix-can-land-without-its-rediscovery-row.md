# 0020: A fix can land without its rediscovery row, and nothing says so

- status: open
- kind: defect
- phase: -
- opened: 2026-09-18

The rediscovery table is the project's answer to "a protection whose removal
changes nothing is a comment with an if statement around it"
(`tools/rediscovery/table.tsv`). A row names a patch that reverts a protection
and the selector that must then fail, so a check is only believed once its
removal has been watched to break something.

**Nothing asks whether a NEW check has a row.** `tools/rediscovery/check-patches.sh`
is a gate phase and it asks the opposite question -- that every EXISTING row's
patch still applies to the working tree, so a refactor that moves a check fails
the gate until the patch is redone. That is the right guard for rows that exist.
It is silent about the ones that do not.

**It has already cost the project two days.** Issue #0019 asked for a row in the
same breath as the fix -- "a rediscovery row belongs with the fix:
`second-knell-not-in-the-span`" -- and a65f693 shipped the fix, its two-knell
test, and the corrected acceptance golden, with no row. The gate passed. So
between 2026-09-16 and 2026-09-18 the project held a protection against a false
`reversible_back_to` and could not demonstrate that the test protecting it bit
anything. It did bite, as it turned out (`10861c2`, 1 rediscovered, 0 not) --
but that was luck confirmed after the fact, not a property the project had.

**What makes this worth an issue rather than more care.** The instruction was
written down, in the issue, by the person who then did not follow it. This is
the shape of defect the project removes rather than re-resolves to avoid: a rule
that holds only while somebody remembers. The rediscovery table exists because
tests are believed too easily; a table that is itself populated on the honour
system inherits the problem it was built to solve.

**Done when** a commit cannot quietly add a protection and no row. What that
check should be is the open question and the reason this is not already fixed:

1. **"Every new `#[test]` needs a row" is false and must not be built.** Most
   tests are not protections in the rediscovery sense, and a guard that demands
   a row for each would either be ignored or answered with junk rows -- which is
   worse than no rows, because the table would then assert protections nobody
   verified.
2. **A count baseline is the wrong instrument here.** It would fire on ordinary
   test-writing and say nothing about whether the new test is a protection.
3. **The narrow version that IS decidable, and the one to try first:** an issue
   whose own text names a rediscovery row must have that row in
   `tools/rediscovery/table.tsv` before the issue may be CLOSED.
   `tools/lint-issues.sh` already parses every issue file, so this is a grep for
   the row name it names and a lookup in the table. It would not have caught
   #0019 in time -- #0019 is still open -- so it is a partial answer and must be
   described as one rather than as the fix.

A partial guard sold as complete is worse than none, so whichever lands says in
its own text which case it catches and which it does not.
