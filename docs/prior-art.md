# Prior art: the falsification sweep

The claim under test (docs/ROADMAP.md section 1.1): rescind is the first plan
language in which "this can be undone" is a compile-time verdict rather than
a comment. Section 1.3's standing order is that before Phase 0 exits, one
person spends a day trying to break the claim and records what was found.
This is that record. Swept 2026-09-06 by web search over the roadmap's
section 1.2 table plus adjacent fields it did not name; each entry states
what was checked and whether the claim survives as written, survives
narrowed, or falls.

## Verdict

The claim survives, narrowed in one place. Two bodies of work decide
reversibility offline and were not in the roadmap's table: action
reversibility in AI planning, and the compensation calculi. Neither is a
language for operations against hosts, and neither states the things rescind's
verdict states (undo locus, cost, arming order, who must act, how long); but
"compile-time verdict on undoability" is not new in the abstract, and the
claim should say "for operations against a world, from declared footprints"
rather than imply the idea is unprecedented. The roadmap's section 1.1 is
the owner's to reword; the sentence proposed is in the last section.

## Candidates and deltas

### Action reversibility in AI planning (new to the table; narrows the claim)

Eiter, Erdem and Faber, "Undoing the effects of action sequences", Journal
of Applied Logic 6(3), 2008, introduce reverse plans: whether the effects
of an action sequence can be undone by another sequence, decided over a
planning domain. Morak, Chrpa, Faber and Fišer, "On the Reversibility of
Actions in Planning", KR 2020, generalize this (uniform reversibility, a
reverse plan that works from every state) and show the decision is at least
as hard as planning, PSPACE-hard unrestricted. Med, Chrpa, Morak and Faber
extend it to non-deterministic actions (ICAPS 2024; KR 2025), and ASP
encodings exist (Faber et al., 2021 onward).

What was checked: the KR 2020 page and abstract; the 2008 paper's abstract;
the 2024 and 2025 follow-ups' titles and abstracts.

Delta: this is a compile-time decision about undoability, so the abstract
idea predates rescind. It decides by search over a STRIPS-like world model in
which every action's effects are fully known, and its answer is a reverse
plan or its absence. Rescind does not search: it computes from declarations
(footprint kinds, undo body and locus, refusal mode, backstop and its arming
order) and its verdict states where the undo runs, past which step it
cannot, what the step of no return costs, who must acknowledge it, and how
long the plan is bounded. The planning work has no notion of locus, cost,
gate, wane or reach. The claim narrows to "for operations against hosts,
from declared footprints, with locus and cost in the verdict".

### Compensation calculi (new to the table; the claim survives)

Bruni, Melgratti and Montanari, "Theoretical foundations for compensations
in flow composition languages", POPL 2005; the Sagas calculi and
compensating CSP (Butler, Hoare, Ferreira); Lanese et al. on the expressive
power of compensation primitives; work on static versus dynamic
compensations, where termination is decidable with static compensations and
not with dynamic ones.

What was checked: the POPL 2005 abstract; the survey chapters' abstracts;
the static-versus-dynamic decidability result.

Delta: these give semantics to compensation and prove properties of the
calculi (expressiveness, decidability). None types the footprint an
activity touches or decides, for a given program, whether its compensations
compose; the "static compensation" of the calculi means the compensation is
fixed at installation, not that anything is checked. Rescind is the checker
these calculi lack, and its footprint algebra is what makes the check
possible.

### BPEL compensation handlers, model-checked (new to the table; survives)

Formal semantics for WS-BPEL fault, compensation and termination handlers,
and model checking of processes against them (several papers, 2006 onward;
a BIP-based compositional semantics).

Delta: model checking of a given process against properties someone wrote
down, after translation. Not a language-level verdict, no footprints, no
locus. Adjacent in spirit to rescind's E-codes, different in kind.

### Sagas and compensating transactions (in the table; survives)

Garcia-Molina and Salem, 1987, and the microservice saga pattern as
Temporal, Cadence and others implement it: local transactions with
hand-written compensations run in reverse order on failure, compensations
required to be idempotent.

What was checked: Temporal's saga documentation and blog posts, including
a documented failure where a compensation ran before the step it undoes had
taken effect.

Delta unchanged: nothing about a compensation is checked before it runs;
there is no undo that outlives the engine, no point of no return, no
locus. The documented compensation-before-effect failure is exactly the
class rescind's LIFO interference query and step numbering exist to refuse.

### `commit confirmed` (in the table; survives)

Junos `commit confirmed` (1 to 65535 minutes, default 10), IOS-XR
`commit confirmed`, IOS-XE `configure terminal revert timer`.

Delta unchanged: one device, one kind of change, one trigger. Rescind's T3 is
this generalized to any op with a target-standalone undo, and the reach rule
proves the arming order the devices get for free by being the thing they
change.

### Database migrations (in the table; survives, with a precision)

Rails `ActiveRecord::Migration`: `change` methods are reversed by a
`CommandRecorder` that knows which commands have inverses;
`IrreversibleMigration` is raised when a migration is moving down. Flyway
and Liquibase undo scripts are written by hand.

Precision: Rails does keep a fixed list of reversible commands, which is a
static notion. But the check happens at rollback time, not at migration
time, and nothing is said about two migrations' footprints. Delta stands.

### Infrastructure "atomic transaction" frameworks and patents (new; survives)

US 8935570 B2, "Automating infrastructure workflows as atomic transactions"
(filed 2012, granted 2015; SunGard Availability Services, now 11:11
Systems): flows with paired `do()` and `undo()` methods, `undo()` formulated
at runtime from a captured `CurrentState()`. US 10565536 and US 11087258,
"Automated process reversal". Recorded here for awareness; no reading of
their claims is offered.

Delta: runtime pairing of forward and reverse transactions with state
capture; no static verification of the undo is claimed. Rescind's verdict is
computed before anything runs.

### A programming language with a fully undoable core (added 2026-09-13; narrows the claim)

US 7174481, 7203866, 7734958, 7966605 and 8112671, "Method and apparatus for
a programming language having fully undoable, timed reactive instructions"
(AT&T; priority 2001, grants 2007-2012; inventors Di Fabbrizio and
Klarlund). A reactive language with "a fully undoable core language portion
and a conventional language portion", whose programs "fully recover to any
previous program execution state". It carries a `local protect` primitive
that "prevents a thread from being recovered to a point before" it, and it
notifies an external program when core-language executions are undone.

Recorded for awareness; no reading of the claims is offered. What was read
is the abstract and the description of one of the five.

Delta, and it is smaller than the deltas above. **Undo is decided at run
time** here, by event queues and checkpointing; rescind's verdict is computed
before anything runs, from declarations. It recovers **program execution
state** inside a language runtime, with a notification boundary to an
external program, rather than operating on hosts it does not own from
declared footprints; and it has no cost, no acknowledgement, no arming order
and no bound.

**What it takes away.** A point past which undo cannot reach is NOT rescind's
idea: `local protect` is exactly that, in a language, from a 2001 priority
date. What survives is computing and reporting that point **statically,
before execution**, which is narrower than section 1.1's "past which step it
cannot" invites a reader to assume. And "a language whose core is fully
undoable" is a quarter of a century old.

**How it was found, because the method matters more than the entry.** Not by
the falsification sweep, which searched the section 1.2 table and adjacent
fields and recorded three infrastructure-workflow patents while missing a
five-patent family whose title is literally a programming language with
undoable instructions. It surfaced in a single plain-language web search run
for an unrelated reason -- checking what a candidate project NAME already
meant. The sweep's structured queries over an author-chosen list could not
see it; one sentence typed the way a stranger would type it did.

### Reversible DSLs outside operations (new; survives)

RASQ, a DSL for reversible robot assembly sequences with formal semantics
and reverse execution to back out of an error; Eel, a language for partially
reversible programs with logged trace information for the non-invertible
parts; Ψ-Lisp and the reversible-computing lineage.

Delta: reverse execution at runtime, of programs, over state the language
owns. No footprints on a world the language does not own, no locus, no
cost.

### Reversible programming languages (in the table; survives)

Janus (1982; Yokoyama et al., 2008), RFun (2012), and the Reversible
Computation conference series through 2024.

Delta unchanged: language-level reversibility of computation, not of
effects on a world; the totality ethic is borrowed, the subject is not.

### Agent workflow verification and transactional tool use (new; survives)

Agentproof (Xavier et al., arXiv 2026): static structural checks and
temporal safety policies, compiled to automata, over workflow graphs
extracted from agent frameworks. Atomix (Mohammadi et al., arXiv 2026):
progress-aware transactions for agent tool use, with a commit gate that
lets irreversible effects out only at commit and compensates reversible
ones on abort. Squidie, an Elixir workflow runtime with steps marked
`:irreversible`.

Delta: Agentproof verifies reachability and temporal safety, not
reversibility, footprints or locus. Atomix's commit gate is rescind's `knell`
and `commit()` discipline at runtime, with no verdict beforehand. Squidie
marks a step irreversible and does nothing with the mark statically. The
vocabulary is converging on rescind's from a different direction, which is
evidence the problem is real and the verdict is the missing piece.

### Ansible, Terraform, Kubernetes, NixOS (in the table; survive)

Ansible has no rollback beyond hand-written playbooks and roles such as
ansistrano.rollback. Terraform and Kubernetes converge; `rollout undo`
redeploys a prior revision. NixOS generations roll back one footprint kind.

Delta unchanged.

### Miniscript (in the table; survives)

Policy satisfiability, minimum-satisfaction analysis and refusal of mixed
timelocks at compile time.

Delta unchanged: policy, not operations. It is the model rescind's gates borrow,
including the refusal of a policy the compiler cannot reason about (E0508,
E0509).

### Lenses and bidirectional transformations; Ecto.Multi; Metafont and Dhall

Unchanged from the table. Checked by search for anything newer; nothing
that changes the delta.

## Proposed rewording of the claim

"Rescind is the first language for operations against hosts in which 'this can
be undone' is a compile-time verdict rather than a comment: computed from
declared footprints, undo loci and refusal modes rather than by search over
a world model, and stating where the undo runs, past which step it cannot,
what that step costs and who must acknowledge it, and how long the plan is
bounded. Deciding undoability offline is not new (action reversibility in
planning; compensation calculi); deciding it for a plan language from
declarations, with locus and cost in the answer, is."

## Sources

- Eiter, Erdem, Faber, "Undoing the effects of action sequences", J. Applied Logic 6(3), 2008. https://www.sciencedirect.com/science/article/pii/S1570868307000328
- Morak, Chrpa, Faber, Fišer, "On the Reversibility of Actions in Planning", KR 2020. https://proceedings.kr.org/2020/65/
- Chrpa et al., "Universal and Uniform Action Reversibility", KR 2021. https://proceedings.kr.org/2021/63/kr2021-0063-chrpa-et-al.pdf
- Med, Chrpa, Morak, Faber, "Weak and Strong Reversibility of Non-deterministic Actions", ICAPS 2024. https://ojs.aaai.org/index.php/ICAPS/article/view/31496
- Med et al., "Non-deterministic Action Reversibility: Complexity Results", KR 2025. https://proceedings.kr.org/2025/45/kr2025-0045-med-et-al.pdf
- Bruni, Melgratti, Montanari, "Theoretical foundations for compensations in flow composition languages", POPL 2005. https://dl.acm.org/doi/10.1145/1040305.1040323
- "A Process Calculus Analysis of Compensations". https://link.springer.com/chapter/10.1007/978-3-642-00945-7_6
- "On the Expressive Power of Primitives for Compensation Handling". https://link.springer.com/chapter/10.1007/978-3-642-11957-6_20
- "Formal analysis of BPEL workflows with compensation by model checking". https://www.researchgate.net/publication/228703533_Formal_analysis_of_BPEL_workflows_with_compensation_by_model_checking
- Temporal, "Saga Pattern". https://docs.temporal.io/design-patterns/saga-pattern
- Temporal, "Saga Compensating Transactions". https://temporal.io/blog/compensating-actions-part-of-a-complete-breakfast-with-sagas
- Rails API, `ActiveRecord::IrreversibleMigration`. https://api.rubyonrails.org/classes/ActiveRecord/IrreversibleMigration.html
- Junos `commit confirmed` example. https://www.networkcuriosity.com/junos-commit-confirmed-example/
- "How Cisco (IOS/IOS XE) Implements Juniper like Commit and Rollback Behavior". https://iosxrjunos.wordpress.com/2025/05/16/how-cisco-ios-ios-xe-implements-juniper-like-commit-and-rollback-behavior/
- US 8935570 B2, "Automating infrastructure workflows as atomic transactions". https://patents.google.com/patent/US8935570B2/en
- US 10565536, US 11087258, "Automated process reversal". https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/10565536
- "Towards a Domain-Specific Language for Reversible Assembly Sequences" (RASQ). https://link.springer.com/chapter/10.1007/978-3-319-20860-2_7
- "Toward an Energy Efficient Language and Compiler for (Partially) Reversible Algorithms" (Eel). https://arxiv.org/pdf/1605.08475
- Yokoyama et al., "Principles of a reversible programming language" (Janus). https://dl.acm.org/doi/10.1145/1366230.1366239
- "Interpretation and programming of the reversible functional language RFUN". https://dl.acm.org/doi/10.1145/2897336.2897345
- Xavier et al., "Agentproof: Static Verification of Agent Workflow Graphs", 2026. https://arxiv.org/abs/2603.20356
- Mohammadi et al., "Atomix: Timely, Transactional Tool Use for Reliable Agentic Workflows", 2026. https://arxiv.org/abs/2602.14849
- Squidie, workflow automation runtime for Elixir. https://elixirforum.com/t/squidie-workflow-automation-runtime-for-elixir-applications/75162
- ansistrano/rollback. https://github.com/ansistrano/rollback
- rust-miniscript, mixed timelock detection. https://github.com/rust-bitcoin/rust-miniscript/pull/121
- Blockstream, "Don't Mix Your Timelocks". https://medium.com/blockstream/dont-mix-your-timelocks-d9939b665094

---

# The name sweep, and the rename it caused

ROADMAP section 12 requires a search of crates.io, PyPI, npm and GitHub for
the project's name and the SDKs' publishing names, recorded here **before
anything is public**. Swept for `rue` on 2026-09-12; swept again for
`rescind` on 2026-09-13, after the first sweep's finding was acted on.

## What the first sweep found, and what it missed

**`rue` was taken for a programming language, twice**, and the two package
names it would have published first were held by one of them:
`github.com/xch-dev/rue` (a typed Chia language targeting CLVM, holding
`rue` and `rue-lsp` on crates.io) and `github.com/rue-language/rue` (an
experimental systems language implemented in Rust, ~1,200 stars, its own
domain, actively developed).

**It understated the collision, and the method is why.** The sweep queried a
FIXED LIST of six names -- the three ROADMAP section 12 named, plus the SDK
names -- and reported that `rue`, `rue-core` and `rue-lsp` were taken. It
never enumerated the namespace. Enumerating it on 2026-09-13 found
**seventeen `rue*` crates, fourteen of them one language's toolchain**:
`rue-parser`, `rue-lexer`, `rue-compiler`, `rue-ast`, `rue-hir`, `rue-lir`,
`rue-types`, `rue-diagnostic`, `rue-options`, `rue-cli`, `rue-lsp`,
`rue-clvm`, `rue-typing`, `rue-formatter`, and `rue` itself. With one
instance of each name checked, "this name is taken" and "this entire
namespace is another language's toolchain" give the same answer.

**And it never ran the search a stranger runs.** Registry APIs answer *is
the name taken*. They do not answer *what does this name already mean*, and
those are different questions: searching `rue crate rust` returns a
definition -- "Rue is a typed programming language which gets compiled to
CLVM bytecode" -- not a list of links. The first sweep reasoned about
discovery in a paragraph instead of observing it. The owner found the
collision in one search, which is the whole of the argument for running one.

## The rename

The project was renamed **`rue` -> `rescind`** on 2026-09-13, at v0.4.0,
with the repository renamed in place so no history was lost. The name is a
legal verb -- to annul an order, restoring what it disturbed -- and the
language's file extension `.scind` is its root: *scindere*, to cut;
*re-scind*, to cut back. The file describes the cut and the tool takes it
back.

`rue` survives in exactly two places on purpose, and both are recorded
where they live: the version keyword, which is accepted and answered with
**E0610** naming the change, so the frozen upgrade vectors still check; and
the journal and request **domain separators**, which are mixed into every
hash and signature ever written and whose stability is the only property
that matters about them.

## The sweep for `rescind`, 2026-09-13

Checked the way the first sweep should have been: namespace enumeration, the
shape names a toolchain reaches for, every registry the project would ever
publish to, and the plain-language searches.

| where | result |
|---|---|
| crates.io | **zero crates** in the whole `rescind*` namespace; `rescindd`, `rescind-cli`, `rescind-lsp`, `rescind-core`, `rescind-hook`, `rescind-engine` all free |
| PyPI | free, with `rescind-hook` and `rescind_hook` free |
| npm | `tree-sitter-rescind` and `rescind-cli` free; the bare name is a tombstone (see below) |
| Hex, RubyGems, Packagist | free, with `rescind_hook` free |
| Maven Central | `dev.rescind` free |
| NuGet | `rescind`, `rescind.hook` free |
| Go proxy, Homebrew, Debian, AUR, FreeBSD ports | free |
| Docker Hub | no images |
| GitHub | 25 repositories mention the word in total; the largest has 4 stars; none is a language or a developer tool |
| DNS | `rescind.dev`, `rescind.sh`, `rescind-lang.dev` unregistered; `.com/.io/.org/.net/.app` registered and parked with no content |
| plain search | no language, no tool, no company. `rescind programming language` returns the CATEGORY -- reversible programming languages -- rather than a competitor |

**Four things that are not clean, recorded rather than smoothed over.**

1. **ReScript.** A real, well-known language (compiles to JavaScript).
   `rescind` and `rescript` differ by two letters and share the `resc-`
   prefix. This is a mishearing and autocorrect risk, not a namespace or
   legal one, and the owner accepted it deliberately. No registry, GitHub or
   DNS check would have surfaced it; the plain search did.
2. **The npm bare name cannot be published.** It is a tombstone: version
   `10.0.0`, published 2014-04-05 with the description "v. To make void",
   unpublished eighty-two minutes later by npm's own founder. Zero versions
   remain. It does not matter -- what this project publishes to npm is
   `tree-sitter-rescind`.
3. **The GitHub handle `Rescind` is taken** by an account with no
   repositories and no followers, untouched since 2022. The organisation
   name is unavailable; `axonibyte/rescind` was free and is what is used.
4. **No trademark search was done.** The USPTO's search is not publicly
   queryable and a web search is not a clearance. Nothing here claims a
   mark, and searching the word returns trademark *cancellation* law,
   because "rescind" is a legal verb -- which is also a mild permanent
   search headwind: `rescind <technical term>` pulls in documentation about
   removing things.

## What the method changed

The first sweep's two defects are now the rule for the next one, whatever it
is for: **enumerate the namespace rather than a list of names you thought
of**, and **run the search a stranger would run**, because it answers a
different question than a registry API does. Section 1.3's standing order
carries both.

## Sources

- crates.io: `https://crates.io/api/v1/crates?q=<name>` (namespace), and
  `/api/v1/crates/<name>` per shape name
- PyPI: `https://pypi.org/pypi/<name>/json`
- npm: `https://registry.npmjs.org/<name>`
- Hex: `https://hex.pm/api/packages/<name>`; RubyGems:
  `https://rubygems.org/api/v1/gems/<name>.json`
- Maven Central: `https://search.maven.org/solrsearch/select?q=g:dev.rescind`
- NuGet: `https://azuresearch-usnc.nuget.org/query?q=packageid:<name>`
- Go: `https://proxy.golang.org/<module>/@v/list`; Homebrew:
  `https://formulae.brew.sh/api/formula/<name>.json`
- Debian: `https://sources.debian.org/api/search/<name>/`; AUR:
  `https://aur.archlinux.org/rpc/v5/info?arg[]=<name>`
- GitHub: `https://api.github.com/search/repositories?q=<name>`
- Linguist, for the file extension:
  `https://raw.githubusercontent.com/github-linguist/linguist/main/lib/linguist/languages.yml`
- Plain-language web search, which is the one that found both the collision
  and, separately, the patent family in the section above
