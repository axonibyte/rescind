# 0008: The name sweep, and a public README

- status: closed
- kind: feature
- phase: 5
- opened: 2026-09-11
- closed: 2026-09-12

Phase 5's exit criteria: search crates.io, PyPI, npm and GitHub for
`rue`, `rued` and `rue-core` -- and the SDKs' names, `rue-hook`,
`rue-hook-sdk`, `rue_hook`, `dev.scind:rue-hook`, `Rue.Hook` -- and record the
result in `docs/prior-art.md` before anything is public (ROADMAP 12); a
public README whose prior-art section is 1.2's.

Going public is the owner's decision; the sweep can be done before it.

**Closed.** The name sweep is done and recorded in `docs/prior-art.md` (2026-09-12), which is what ROADMAP 12 asks for before anything is public. `rue` is taken on crates.io (by another programming language called Rue, `xch-dev/rue`, homepage rue-lang.com), on PyPI (an AI testing framework) and on npm (a DI container); `rue-lsp` is taken by that same language's server and `rue-core` by an unrelated UI framework; GitHub holds a second language called Rue at 1193 stars. Free everywhere checked: `rued`, `rue-hook`, `rue-hook-sdk`, `rue_hook`, `tree-sitter-rue`, `dev.scind`, `Rue.Hook`. The record states the discovery problem, the narrower publishing problem (`cargo install rue` is gone; `rued` is free), and three options, and decides none of them: renaming and publishing are both the owner's.

The public README was filed as its own issue (`docs/issues/0016`) on the belief that it did not exist. It did -- 251 lines, with the prior-art section 1.2 asks for -- and what it needed was Phase 5's status and a paragraph about the name, both of which it now has. The mistake is recorded here rather than quietly fixed: the sentence that stood here said the README was "not written", and it was written before this phase began.

**What became of it, 2026-09-13.** This issue's finding was acted on: the
owner renamed the project from `rue` to `rescind` at v0.4.0. Everything above
is left as it was written, naming `rue` throughout, because it is the record
of a sweep of that name and rewriting it would describe a search nobody ran.

Two defects in this sweep's METHOD were found afterwards and are recorded in
`docs/prior-art.md`. It queried a fixed list of six crate names rather than
enumerating the namespace, and so reported three collisions where there were
fourteen; and it never ran the plain-language search a stranger runs, which
returns a definition rather than a list of links and would have settled the
question in one query on the first day. The roadmap's §1.3 standing order now
carries both as rules.
