# 024 — Multi-Repo Workspace: Analysis (Phase 1)

> Auxiliary of [`024-multi-repo-workspace.md`](024-multi-repo-workspace.md). Input to the Plan-phase
> design Q&A required by the PRD (`intake/Backlog/prd.md`, O-010).

> **Method note.** Covered: (1) an internal map of single-repo assumptions in `methodology/`,
> `prompts/`, `templates/` (grep + targeted reads; `file:line` anchors); (2) measured sizes and
> methodology versions of the 6 Stateful Spec adopters under `D:\development\public`; (3) web research
> in 8 fronts (F1–F8, F8 = contrary evidence), 2–4 searches each, primary sources fetched where
> reachable. Evidence classes: **(a)** measured empirical result, **(b)** practitioner report,
> **(c)** vendor documentation / position. **Not read in full:** Jaspan et al. 2018 (ACM returned 403;
> abstract only), MemGPT and Cemri et al. (search summaries only), Lost in the Middle (search summary
> only), Gloaguen et al. 2026 (abstract only; finer numbers come from secondary summaries). Cursor
> behavior comes from forum posts, not docs. OpenCode was not researched. Everything left
> **INCONCLUSIVO** is listed in §7. **Evaluation cell of this analysis:** ex ante × artificial (desk
> research + repository inspection). No Workspace design has been tested.

## 1. Requirements summary

**What.** A **Workspace** layer: a folder/repository from which the agent runs, mapping N member
folders/repositories through a **registry**, holding a **pointer index** to each member's memory
(no replication), with its own `AGENTS.md` and methodology config. From a Workspace session the
agent can work on any member, create a new member, and draft an idea until it is actionable (a new
command). A second new command adds a member (updates the registry + indexes the member's memory
pointers). Several sessions may be active at once, at most one per member.

**Why.** Cross-member knowledge sharing. Precedent: learnings from `stand-in` were back-ported into
the methodology by hand (iterations 012, 018–020); the 018 stand-in analysis alone cost ~1.4M tokens
(`.stateful-spec/memory.md:115`, Engrama 019).

**Who.** The developer working across several repositories; AI agents (Claude Code, Cursor, OpenCode
— the three ports — plus the others the wizards recognize, `prompts/initialization/new-project.md:258-273`).

**Boundaries.** No change to single-repo behavior (AC1). Not a daemon or live watcher — consistent
with `methodology/backlog.md:108-113`. Not (by default) a monorepo migration or a clone/sync tool —
see Q-list §6.

## 2. Internal map — single-repo assumptions

### 2.1 Assumptions

| ID | Assumption | Anchor | Workspace impact | Severity |
|----|------------|--------|------------------|----------|
| A1 | One `.stateful-spec/` at "the project root"; every op addresses `.stateful-spec/memory.md` relative to where the agent runs | `methodology/overview.md:120`; `prompts/initialization/new-project.md:295`; `update-project.md:74`; `templates/project/agents-md.md:9-11`; the path appears in all 20 `prompts/operations/*.md` | From the Workspace cwd every op resolves to the Workspace's own memory, not the member's — ops need a notion of **target root** | High |
| A2 | One Open Session per `memory.md` | `templates/project/memory.md:23`; `templates/project/agents-md.md:104`; `prompts/operations/start-session.md:21-30` | AC7 (≤1 per member) fits if sessions stay in the member's memory; the Workspace needs a view of which members are open, and its own shared files become the write-contention point | Medium–High |
| A3 | 15 non-lifecycle ops check `.stateful-spec/memory.md` for an Open Session and log to it | e.g. `create-technical-spec.md:44`, `write-commit-message.md:35`, `review-changes.md:36` (15 files, grep) | Without a resolution rule, a contribution made from the Workspace logs into the wrong iteration — **silently** | High |
| A4 | `NNN` and `O-NNN` are per-repo namespaces | `methodology/history-archiving.md:98-104`; `methodology/backlog.md:40-42` | Cross-member references need qualification (e.g. `member:NNN`); unclear where Workspace-level `O-NNN` live | Medium |
| A5 | Git operations assume one repository (`git mv` archive, branch strategy, commit per item, `advance` refuses the default branch) | `history-archiving.md:78-84`; `qa-phase.md:97-102`; `multi-agent-flow.md:263` | Each member is its own repo (6/6 adopters have `.git`); commits across members are not atomic; the Workspace may or may not be a repo | Medium |
| A6 | Each project carries its own **copy** of the methodology | `new-project.md:308`; `update-project.md:104,149` | Members drift (§2.2): the index cannot assume a uniform memory schema | High (design-shaping) |
| A7 | Agent entry point (`AGENTS.md`) at the repo root; tools discover instruction files from the root down to cwd | `new-project.md:314,377` | Which member files a tool loads depends on topology (subfolder vs sibling) and on the tool (F1) | High |
| A8 | Native command ports live per repo (`.claude/commands/`, `.cursor/rules/`, `.opencode/commands/`) | `new-project.md:332-389`; 20 ops × 3 ports = 60 port files | New Workspace ops (add-member, draft-idea) = new source prompts × 3 ports each (sync rule) | Medium |
| A9 | One active flow per repo, "mirroring the single Open Session" | `multi-agent-flow.md:67-68` | Fine per member; a flow spanning members is undefined | Low |
| A10 | Intake/backlog per project; triage reads `.stateful-spec/intake/` | `start-session.md:32-59` | Where does a drafted idea land — Workspace intake or the target member's? | Medium |
| A11 | Engramas/History Index describe only the own repo; no cross-repo promotion path besides manual back-port | `templates/project/memory.md:59-78`; back-ports in `.stateful-spec/memory.md:116,120` (Engramas 018, `0-archived`/012) | The sharing loop the PRD wants does not exist yet | High |

### 2.2 Measured: the candidate members

The 6 folders under `D:\development\public` with a `.stateful-spec/` (measured 2026-10-02):

| Member | Methodology `.md` files | `history-archiving.md` | `memory.md` bytes | Engramas section | Project Type value |
|--------|------------------------|------------------------|-------------------|------------------|--------------------|
| jandi-colors | 13 | yes | 7,037 | yes | software |
| markdown-harvest | 8 | **no** | 826 | **no** | `library (with binary CLI entrypoint)` |
| ollama-oxide | 8 | **no** | 7,936 | **no** | `library` |
| skills | 12 | yes | 8,266 | **no** | *(line not found)* |
| stand-in | 14 (incl. `multi-agent-flow-v2.md`, absent from source) | yes | 77,789 | yes | software |
| stateful-spec | 1 (reference stub) | n/a | 30,095 | yes | `Documentation / Methodology` |

- Members are on **at least four methodology versions** (8 / 12 / 13 / 14 files). 3 of 6 memories
  have no Engramas section; 4 of 6 Project Type values are outside the registry
  (`methodology/project-types.md:14-18`) or missing.
- `stand-in/.stateful-spec/memory.md`: 77,789 bytes / 10,072 words, of which the Engramas section is
  53,043 bytes (68%). Its `.stateful-spec/` totals ~84 MB.
- The 6 memories sum to **131,949 bytes**. For scale, against documented tool defaults (these caps
  apply to instruction files, not to `memory.md`): Codex concatenates instruction files up to
  **32 KiB** (F1); Claude Code loads auto-memory up to **200 lines / 25 KB** and recommends CLAUDE.md
  under **200 lines** (F1). Replicating the members' memories would be ~4× the Codex default;
  `stand-in` alone is ~2.4×. A pointer row (path + one line)
  is on the order of 100–200 bytes per member — **estimate**, to be measured once a format exists.

## 3. Research findings by front

### F1 — Native multi-root support in agent tools

*Sub-question: how do the tools themselves treat several folders/repos in one session, and what must
the methodology supply on top?*

- **Claude Code (c, official docs).** CLAUDE.md files in cwd and its ancestors load at launch;
  subdirectory files load **on demand** when Claude reads a file there. `--add-dir` grants file
  access; its CLAUDE.md loads only with `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1`, and its
  **AGENTS.md does not load even then**. The `additionalDirectories` setting never loads CLAUDE.md or
  skills. `@import` recurses at most **four hops**; imports outside the working directory need
  approval. "Target under 200 lines per CLAUDE.md file. Longer files consume more context and reduce
  adherence." Memory files are "context, not enforced configuration"; a CLAUDE.md that tells Claude
  in words to read AGENTS.md means "Claude sees AGENTS.md only if it decides to open the file."
- **Codex (c, official docs).** Concatenates `AGENTS.md` from the Git root down to cwd (closer
  files later, so they win); stops at `project_doc_max_bytes` = **32 KiB** by default. The docs page
  (as summarized on fetch) states a single Git repository per run; search does not extend to
  sibling directories.
- **Cursor (b, forum incl. staff answers).** Multi-root workspaces read `.cursor/rules/` from every
  root; glob-scoped rules apply only inside their own root; an `alwaysApply` rule present in several
  roots loads several times; a reported bug: a mix of subdirectory and root rules → none read.
- **AGENTS.md convention (b).** "Closest one takes precedence"; whether the nearest file *replaces*
  or *merges with* the root file is an open question (issue #53, no maintainer reply).

**Implication.** The tools diverge on exactly the mechanism a Workspace would lean on (which member
instruction files load when the agent touches a member). A tool-agnostic methodology cannot rely on
native nested loading; pointer-following must be explicit in the Workspace's instructions (a Guide)
and, ideally, checked (a Sensor).

### F2 — Validated registries for multi-repo setups

*Sub-question: which manifest formats exist, and which fields proved necessary?*

| Tool | Format | Key fields | Class |
|------|--------|------------|-------|
| Google `repo` (Android) | XML manifest | `<remote>`, one `<default>`, `<project name path revision remote groups>` — revision = branch, tag or SHA | (c) |
| vcstool (ROS) | YAML `.repos` | `repositories: <relative path>: {type, url, version}` — the relative path is the key, "avoids collisions by design" | (c) |
| meta | JSON `.meta` | child repos as subdirectories; commands fan out to all | (c) |
| Backstage | YAML `catalog-info.yaml` | the descriptor lives **in the member's own root**; the central catalog stores *locations* and discovers by pattern; duplicate `metadata.name` → one kept | (c) |

**Convergent fields:** a stable key (path or name), a location (path/URL), an optional pin
(revision/version), optional grouping. **Backstage's split** — the member owns its descriptor, the
center holds only locations — is the pointer shape the PRD describes.

### F3 — Pointer-based memory and indexing

*Sub-question: what validated designs keep a small index in context and load bodies on demand?*

- **Anthropic, context engineering (c).** "Just in time" agents "maintain lightweight identifiers
  (file paths, stored queries, web links, etc.) and use these references to dynamically load data
  into context at runtime"; Claude Code's hybrid (CLAUDE.md up front + glob/grep) bypasses "stale
  indexing". Stated trade-off: "runtime exploration is slower than retrieving pre-computed data."
- **Agent Skills, progressive disclosure (c).** Three levels: name + description at startup;
  `SKILL.md` body when relevant; linked files on demand — bundle size "effectively unbounded".
- **llms.txt (b, proposal, J. Howard, 2024-09-03).** A Markdown index: H1 name, blockquote summary,
  H2 sections of link lists.
- **MemGPT (a, Packer et al. 2023; summary only).** Main context vs external context; the model
  pages data in and out via function calls (OS virtual-memory analogy).

**Convergent design:** a small always-loaded index (name + one-line description + location), bodies
loaded on demand. The repo already does this internally: Engramas + the History Index `File` column +
the never-bulk-read cold store (`methodology/history-archiving.md:90-96`).

### F4 — Cost of context (magnitudes)

*Sub-question: what does measured evidence say about loading more context vs focused context?*

- **Gloaguen et al. 2026 (a, arXiv 2602.11988).** Context files "do not generally improve task
  success rates, while increasing inference cost by over 20% on average." Secondary summaries report
  developer-written +4%, LLM-generated −0.5% (SWE-bench Lite) / −2% (AGENTbench), +14–22% reasoning
  tokens — not verified against the full paper.
- **Chroma, Context Rot 2025 (a, 18 models).** "Even a single distractor reduces performance";
  LongMemEval focused (~300 tokens) vs full (~113k tokens): "significantly higher performance on
  focused prompts".
- **Liu et al. 2023, Lost in the Middle (a; summary only).** U-shaped accuracy by position; with
  the relevant document mid-context, GPT-3.5-Turbo scored below its closed-book baseline.
- **Mishra & Mishra 2026 (a, arXiv 2609.05441, 23,440 episodes).** "Full replay was never
  cost-efficient"; implementation choice → up to 60-point success variance; agents acted on correctly
  retrieved values only **55%** of the time.

**Implication (inference by analogy, not a result measured in this setting).** Replicating ~132 KB
of member memory into the Workspace's loaded context resembles the "full prompt / full replay"
condition these results penalize; a pointer index resembles the focused condition. Counter-weight:
retrieval is not use (55%).

### F5 — Cross-project knowledge sharing

*Sub-question: what makes experience from one project reach another, and what fails?*

- **Experience Factory (a/b, Basili, Caldiera & Rombach 1994; NASA SEL).** Separates the project
  organization from an experience organization that analyzes, **packages**, and feeds experience
  back; "a distinct support organization" is required for large-scale reuse. Mapping: members =
  project organization; Workspace = experience factory. This repository already plays that role
  informally (012, 018–020).
- **Memory Transfer Learning (a, Kim et al. 2026, arXiv 2604.14004, 6 coding benchmarks).**
  Cross-domain memory **+3.7%** on average; "high-level insights generalize well, whereas low-level
  traces often induce negative transfer". → share abstracted learnings (Engrama `Learnings` level),
  not raw Session Logs.
- **ShareMem (a, arXiv 2609.32511).** Sharing "helps most when relevant local experience is
  scarce"; pooling "risks transferring preferences that conflict with the receiving user's
  requirements" → cross-member interference (e.g. a Rust convention leaking into a Node member).
- **Contrary — NASA OIG IG-12-012 (2012, audit).** Project managers "do not routinely use LLIS";
  users found it "outdated, not user friendly, and generally unhelpful". A lessons store that relies
  on voluntary lookup fails; surfacing at a decision point is the alternative. (A widely repeated
  "only 30% of lessons are applied" figure had no traceable source and is excluded.)

### F6 — Concurrent sessions

*Sub-question: how is isolation achieved when several agent sessions run at once?*

- **Git worktrees (b, practitioner consensus).** The dominant isolation primitive for parallel
  agents on one repository; conflicts move to merge time. Limits: shared external state (databases,
  ports) still collides; file-level dependencies between concurrent tasks are not resolved.
- **MAST (a, Cemri et al., NeurIPS 2025; summary only).** 1,600+ traces, 7 frameworks, κ = 0.88;
  failures: system design 41.8%, inter-agent misalignment 36.9%, task verification 21.3%.
- **Repository precedent.** Safe shared writes are already specified for the flow tool: validated
  verbs as the only writer, atomic writes, monotonic `seq` (`methodology/multi-agent-flow.md:239-249`).

**Implication.** "≤1 session per member" mirrors the existing per-memory invariant (A2): isolate by
member and coordinate only at the Workspace's shared files (registry/index), which are the
contention point.

### F7 — SDD tools with multi-repo support (taxonomy first)

Positions (Böckeler, martinfowler.com, 2025-10-15): **Stateful Spec — spec-anchored**
(`.stateful-spec/persona-reference.md:80,161`); **Kiro — "mostly spec-first"**; **Spec Kit —
"spec-first only, not spec-anchored over time"**; Tessl — aspires to spec-anchored, explores
spec-as-source.

- **Kiro (c, docs).** Multi-root: **no workspace-level store**. It reads `.kiro/` under each root and
  shows one list labeled by root. Steering is "Always Included" or conditional (same root only);
  hooks fire only for files in their own root; MCP conflicts → "the last defining root" wins.
- **Spec Kit (c).** No official multi-repo support: issue #4583 (2026-09-14) is open, labeled
  "valid and in-scope but deprioritized"; community presets coordinate branches across repos.

**Crossing the quadrant.** Kiro's per-root store + aggregated view is a spec-first answer (nothing
to persist at the center). A spec-anchored method with persistent memory faces a question Kiro does
not: whether the Workspace keeps **its own** memory. AC8 implies it does (own `AGENTS.md` +
methodology config).

### F8 — Contrary evidence

- **Co-location beats indexing for visibility (Jaspan et al. 2018, ICSE-SEIP; abstract only).**
  Monorepo visibility "enables engineers to discover APIs to reuse … and automatically have dependent
  code updated"; multi-repo gives "flexibility … access control and stability". The cheapest sharing
  mechanism may be co-location (a parent folder), not an index layer. Magnitudes not read.
- **Pinned pointers go stale (b, git submodules).** Parent updated, submodule not; state split across
  `.gitmodules` / parent `.git` / child `.git`. Any index that **caches** member state inherits this.
- **Pointers in prose are not followed by construction (c + a).** Claude Code docs (F1): a file
  named in prose is read "only if [Claude] decides to open the file"; Mishra & Mishra: correct
  retrievals acted on 55% of the time. A pointer index is a **Guide**; without a Sensor it is
  instruction without enforcement (persona rule 3.4).
- **Voluntary lessons stores fail (NASA OIG, F5).**
- **Spec sprawl at fleet scale** (TrueFoundry, via `.stateful-spec/persona-reference.md:53`). A
  Workspace multiplies specs across members.

## 4. Complexity breakdown (candidate sub-tasks, to confirm in Plan)

| # | Sub-task | Complexity | Driver |
|---|----------|------------|--------|
| W1 | Registry: format, location, fields | Simple–Medium | Prior art converges (F2); format choice open |
| W2 | Pointer index: what is indexed, refresh trigger, staleness | Complex | Heterogeneous members (§2.2); cache vs pointer (F3, F8) |
| W3 | Target-root resolution for every operation | Complex | 20 source prompts × 3 ports (A1, A3, A8); must be a no-op in single-repo use (AC1) |
| W4 | Session concurrency (≤1 per member + Workspace view) | Medium | Reuses the per-memory invariant; contention only at Workspace files (A2, F6) |
| W5 | New operations: add-member, draft-idea; flows for "work on a member" and "create a member" | Medium | New prompts + 3 ports each; draft-idea overlaps the intake READY gate (A10) |
| W6 | Workspace `AGENTS.md` + methodology config (template) | Medium | Tool loading diverges (F1, A7) |
| W7 | Knowledge-sharing loop (member A → member B / methodology) | Complex | Evidence: abstract-level, pushed at decision points (F5); unspecified in the PRD |
| W8 | No-regression check for single-repo use | Medium | Needs a Sensor (e.g. a dry run of the lifecycle ops on an existing member) |

Overall size: **Large** (`methodology/overview.md:98-103`) — all 5 phases, several milestones.

## 5. Dependencies

- **Upstream (files to change or extend):** `methodology/overview.md`, `roles.md`, `backlog.md`,
  `history-archiving.md`, `multi-agent-flow.md`; `templates/project/agents-md.md`,
  `templates/project/memory.md`; the 20 `prompts/operations/*.md` + 60 ports; the 3 initialization
  wizards (creating a member from the Workspace would reuse `new-project.md`).
- **Downstream:** the 5 adopting repos of §2.2 — any member-side change propagates through
  `update-project` in each. The draft `intake/Backlog/review-iteration-lifecycle.md`: concurrent
  sessions make paused (`review`) iterations more frequent.
- **External (moving):** agent-tool loading behavior (F1) changes between releases (Claude Code:
  `--add-dir` CLAUDE.md loading in 2.1.20; native `AGENTS.md` reading from 2.1.277). The design must
  not pin to one tool's current behavior, and should name a re-validation trigger (a tool release
  that changes instruction loading).

## 6. Open questions for the Plan Q&A (dependency order)

1. **Scope:** shipped to downstream users as part of the methodology (templates + ops), or
   self-use first (like the persona, 022)?
2. **Topology:** Workspace = a parent folder containing members (e.g. `D:\development\public`), a
   separate folder/repo pointing to members anywhere on disk, or both? (F1: on-demand loading works
   only for subfolders.)
3. **Versioning:** is the Workspace itself a git repository (shareable; relative paths) or
   local-only (absolute paths)?
4. **Registry:** fields (location; + pin; + type; + tags) and format (YAML / Markdown table / XML).
5. **Index granularity:** (a) paths only, (b) paths + one-line summary per member, (c) + cached
   learnings. Refresh trigger?
6. **Heterogeneous members:** require the current methodology version (run `update-project` first)
   or tolerate any version (point to files, not sections)?
7. **Non-adopter members:** allowed (a folder without `.stateful-spec/`)? What is indexed then?
8. **Target resolution:** how does an op started in the Workspace know its member — explicit
   argument, cwd, or an "active member" recorded in the Workspace?
9. **Sessions:** member's Open Session + a Workspace mirror, or Workspace-held? What about one change
   spanning two members?
10. **Workspace memory:** does the Workspace keep its own iterations/backlog (for drafts and
    cross-member work), or only registry + index?
11. **Knowledge flow:** what is shared (Learnings? Key Decisions? skills?), push vs pull, who
    promotes, and does promotion into this methodology go through intake → `O-NNN`?
12. **Draft command:** a shaping assistant that ends in an intake item (READY gate) — in which
    intake?
13. **Create member:** wrap `new-project` + register? Where is the folder created?
14. **Tool support:** which agents first (the 3 ports?); may native nested loading be used as an
    optimization, or must the design be fully explicit?
15. **Naming:** "Workspace" collides with VS Code/Cursor/Kiro "multi-root workspace" — keep or rename?

## 7. INCONCLUSIVO

- OpenCode's multi-root / nested-instructions behavior — not researched.
- Whether a member's native commands (`.claude/commands/`, `.cursor/rules/`, `.opencode/commands/`)
  are discoverable from a session launched in the Workspace — not verified for any tool (Claude Code
  docs cover subdirectory skills, not commands).
- YAML vs XML vs Markdown for LLM reading of a registry — no measured comparison found.
- Jaspan et al. 2018 magnitudes — paper not accessible (403); abstract only.
- MAST category percentages, Lost-in-the-Middle numbers, and Gloaguen et al.'s finer numbers
  (+4% / −0.5% / −2% / +14–22%) — secondary summaries; the primary abstracts confirm only the
  direction ("over 20%" cost; no general success improvement).
- Codex "single Git repository per run" — wording from the fetch summary of the docs page, not
  verified verbatim.
- Bytes → tokens (~4 bytes/token) and the 100–200 bytes per pointer row — estimates.
- Whether a pointer index improves this developer's cross-member work — untested; any claim of
  improvement is an ex ante hypothesis until measured in use.

## 8. Removed or made obsolete

Nothing is removed by this analysis; no methodology file changed. Once a design lands, every
statement that `.stateful-spec/` lives at "the project root" (A1 anchors) becomes
context-dependent and must be swept (persona rule 4.5).

# Fontes

**Agent tools (F1)**
- [Claude Code — How Claude remembers your project](https://code.claude.com/docs/en/memory) — ancestors at launch, subdirectories on demand; `@import` ≤4 hops; <200 lines; AGENTS.md rules; "context, not enforced configuration".
- [Claude Code — Monorepo or large codebase](https://code.claude.com/docs/en/large-codebases) — `--add-dir` vs `additionalDirectories` loading table; per-directory CLAUDE.md/skills.
- [Codex — AGENTS.md](https://learn.chatgpt.com/docs/agent-configuration/agents-md) — root→cwd concatenation; 32 KiB default cap.
- [Cursor forum — rules in multi-root workspaces](https://forum.cursor.com/t/rules-from-subdirectories-not-read-in-multi-root-workspace/151107) — per-root rules; mixed-layout bug.
- [Cursor forum — always-applied rules duplicated across roots](https://forum.cursor.com/t/bug-report-always-applied-rules-duplicated-across-workspace-roots-in-multi-root-workspaces/151165) — duplication cost.
- [agents.md issue #53](https://github.com/agentsmd/agents.md/issues/53) — merge vs replace of nested AGENTS.md unresolved.

**Registries (F2)**
- [repo Manifest Format](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md) — XML project/remote/default; revision = branch/tag/SHA.
- [vcstool](https://github.com/dirk-thomas/vcstool) — YAML `.repos`, relative path as collision-free key.
- [meta](https://github.com/mateodelnorte/meta) — JSON `.meta` meta-repo over child repos.
- [Backstage — Descriptor Format](https://backstage.io/docs/features/software-catalog/descriptor-format/) and [GitHub Discovery](https://backstage.io/docs/integrations/github/discovery/) — descriptor in the member's root; central catalog stores locations.

**Pointer-based memory (F3)**
- [Anthropic — Effective context engineering for AI agents](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents) — just-in-time lightweight identifiers; hybrid; slower than pre-computed.
- [Anthropic — Equipping agents with Agent Skills](https://www.anthropic.com/engineering/equipping-agents-for-the-real-world-with-agent-skills) — three-level progressive disclosure.
- [llms.txt proposal](https://llmstxt.org/) — Markdown link index for LLMs.
- [MemGPT (arXiv 2310.08560)](https://arxiv.org/pdf/2310.08560) — main vs external context, paging via function calls.

**Context cost (F4)**
- [Gloaguen et al. — Evaluating AGENTS.md (arXiv 2602.11988)](https://arxiv.org/abs/2602.11988) — no general success gain; >20% cost.
- [Chroma — Context Rot](https://www.trychroma.com/research/context-rot) — distractors degrade; focused ~300 vs full ~113k tokens.
- [Liu et al. — Lost in the Middle (arXiv 2307.03172)](https://arxiv.org/abs/2307.03172) — U-shaped positional accuracy.
- [Mishra & Mishra — When Does Memory Help? (arXiv 2609.05441)](https://arxiv.org/abs/2609.05441) — full replay never cost-efficient; 55% act-on-retrieval.

**Knowledge sharing (F5)**
- [Basili — The Experience Factory: How to Build and Run One](https://www.cs.umd.edu/~basili/publications/proceedings/P77.pdf) — separate experience organization packages and feeds back.
- [Kim et al. — Memory Transfer Learning (arXiv 2604.14004)](https://arxiv.org/abs/2604.14004) — +3.7%; abstract insights transfer, traces negatively transfer.
- [ShareMem (arXiv 2609.32511)](https://arxiv.org/abs/2609.32511) — sharing helps when local experience is scarce; preference interference.
- [NASA OIG IG-12-012](https://oig.nasa.gov/wp-content/uploads/2024/02/IG-12-012.pdf) — LLIS not routinely used; "outdated … generally unhelpful".

**Concurrency (F6)**
- [Augment Code — Git worktrees for parallel agents](https://www.augmentcode.com/guides/git-worktrees-parallel-ai-agent-execution) — worktree isolation; limits.
- [Cemri et al. — Why Do Multi-Agent LLM Systems Fail? (arXiv 2503.13657)](https://arxiv.org/abs/2503.13657) — MAST taxonomy.

**SDD tools (F7)**
- [Böckeler — Understanding SDD: Kiro, spec-kit, and Tessl](https://martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html) — spec-first / spec-anchored / spec-as-source positions.
- [Kiro — Multi-root workspaces](https://kiro.dev/docs/ide/editor/multi-root-workspaces/) — per-root `.kiro/`, aggregated view, no central store.
- [Spec Kit issue #4583](https://github.com/github/spec-kit/issues/4583) — multi-repo request open, deprioritized.

**Contrary evidence (F8)**
- [Jaspan et al. — Advantages and Disadvantages of a Monolithic Repository](https://research.google/pubs/advantages-and-disadvantages-of-a-monolithic-codebase/) — visibility vs flexibility/access control.
- [Ask HN — Why are Git submodules so bad?](https://news.ycombinator.com/item?id=31792303) — stale pins; split state.
