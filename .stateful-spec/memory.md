# Project Memory

> This file is the AI's entry point for understanding the project's current state. Keep it updated as work progresses.

## Project Summary

- **Project:** Stateful Spec
- **Description:** A specification-driven development methodology for AI-assisted software projects
- **Last Updated:** 2026-06-28
- **Current Status:** Active development

## Active Work

> What is currently in progress? Reference the iteration file.

- [ ] Validate published references (crates.io + npm) → `history/017-validate-published-references.md`

## Open Session

> When a session is active, this section points to the current iteration file.
> Managed by `start-session` and `end-session` — do not edit manually.

- Currently open: 017 Validate published references → `history/017-validate-published-references.md`

## Recent Completions

> Last 3-5 completed iterations for quick context.

| # | Name | Type | Completed |
|---|------|------|-----------|
| 016 | publish-flow-ts-npm | chore | 2026-06-28 |
| 015 | publish-flow-rs-crates-io | chore | 2026-06-28 |
| 014 | self-adoption | chore | 2026-06-28 |
| 013 | flow-packages | feature | 2026-06-28 |
| 012 | reverse-update-from-stand-in | feature | 2026-06-28 |

## Key Decisions

> Important decisions that affect how the AI should work on this project. For detailed ADRs, see `history/` files.

- This project is documentation-only (Markdown). There is no application code, build system, or runtime dependencies.
- Operation prompts are placed as native Cursor rule files (`.cursor/rules/<name>.mdc`) rather than in `.stateful-spec/operations/`.
- `.stateful-spec/methodology/` references the root `methodology/` directory rather than copying it, because this repo IS the methodology source. This avoids duplication drift in a self-referential project.
- Non-trivial work must use an iteration file under `.stateful-spec/history/` (see `AGENTS.md` **Iteration tracking**); `resume-session` and `save-session` prompts describe direct-task entry and retroactive saves.

## Constraints & Reminders

> Things the AI must always remember when working on this project.

- All files are Markdown (`.md`) using kebab-case naming
- Follow existing directory structure conventions
- CHANGELOG follows Keep a Changelog format
- No build system, no CI, no linter — quality is manual review
- Branch strategy: main + feature branches with PRs

## History Index

> Complete list of iterations. Newest first.
>
> <!-- RAW_HISTORY = 3 --> `RAW_HISTORY = 3`: `history/` keeps the central files of the last 3
> **closed** iterations plus the open one; older centrals live in `history/.archived/` (indexed here
> via the `File` column, never bulk-read). The next `NNN` scans **both** directories. See
> `methodology/history-archiving.md`.

| # | Name | Type | Status | File |
|---|------|------|--------|------|
| 017 | validate-published-references | chore | in-progress | `history/017-validate-published-references.md` |
| 016 | publish-flow-ts-npm | chore | done | `history/016-publish-flow-ts-npm.md` |
| 015 | publish-flow-rs-crates-io | chore | done | `history/015-publish-flow-rs-crates-io.md` |
| 014 | self-adoption | chore | done | `history/014-self-adoption.md` |
| 013 | flow-packages | feature | done | `history/.archived/013-flow-packages.md` |
| 012 | reverse-update-from-stand-in | feature | done | `history/.archived/012-reverse-update-from-stand-in.md` |
| 011 | multi-agent-flow | feature | done | `history/.archived/011-multi-agent-flow.md` |
| 010 | reconcile-loose-ends | chore | done | `history/.archived/010-reconcile-loose-ends.md` |
| 009 | review-handoff-engramas | review-handoff | done | `history/.archived/009-review-handoff-engramas.md` |
| 008 | engramas-compiled-memory | feature | done | `history/.archived/008-engramas-compiled-memory.md` |
| 007 | multi-project-type-support | feature | done | `history/.archived/007-multi-project-type-support.md` |
| 006 | universal-agents-md | fix | done | `history/.archived/006-universal-agents-md.md` |
| 005 | new-session-management | feature | done | `history/.archived/005-new-session-management.md` |
| 004 | add-claude-code-support | feature | done | `history/.archived/004-add-claude-code-support.md` |
| 002 | require-iteration-tracking | chore | done | `history/.archived/002-require-iteration-tracking.md` |
| 003 | add-update-project-wizard | feature | done | `history/.archived/003-add-update-project-wizard.md` |
| 001 | fix-cursor-init-prompts | chore | done | `history/.archived/001-fix-cursor-init-prompts.md` |

## Engramas

> Compiled memory from history. The agent reads this section for context without
> loading all `history/` files. Each row links to the History Index via `#`.
> If the engram is insufficient, the agent should consult the corresponding
> history file.
>
> **Two-tier compaction:** The table holds at most `N` active rows (default 10)
> plus one Archive row. When the active row count exceeds N, the oldest row
> merges into the Archive row at the bottom.

<!-- N = 10 -->

| # | Summary | Key Decisions | Learnings |
|---|---------|---------------|------------|
| 017 | _In progress_ | — | — |
| 016 | Publicou no npm a implementação de referência **opcional** `@stateful-spec/flow` (`packages/flow-ts/`) v0.1.0 — contraparte Node/TS da publicação 015 no crates.io, dirigida por um PRD externo promovido como O-003: bump das devDeps (typescript 6.0.3, @types/node 26.0.1) e do engine (node `>=24`), `"types": ["node"]` exigido pelo TS 6.0, 16 testes + dry-run limpos, então publish público. | Interpretar "flow-rs to npm" como **flow-ts → npm** (flow-rs já foi p/ crates.io na 015; npm é o registro do gêmeo Node/TS); seguir as versões vinculantes do PRD verbatim (ts 6.0.3 / @types/node 26.0.1 / node `>=24`); publicar com `--access public` via **automation token** por env (bypass de 2FA), nunca commitado. | TS 6.0 mudou o default de `types` — não auto-inclui mais `@types/*`, então pacotes Node precisam declarar `"types": ["node"]` ou os imports `node:*` quebram; publicar pacote escopado exige a **org/escopo existir antes** (404 "Scope not found") e token que faça **bypass de 2FA** (token "Publish" comum cai em 403); debugar auth colando tokens vivos no chat vaza segredos — usar um único automation token em env e rotacionar (o PRD guardava o token em plaintext). |
| 015 | Publicou no crates.io a implementação de referência **opcional** `stateful-spec-flow` (`packages/flow-rs/`) v0.1.0, dirigida por um PRD externo promovido da intake como O-002: bump do `Cargo.toml` para edition 2024 / MSRV 1.96, refino de `keywords`/`categories` para descoberta, 62 testes + dry-run limpos, então publish. | Seguir as propriedades vinculantes do PRD verbatim (edition 2024, MSRV 1.96) em vez dos 2021/1.74 anteriores; refinar keywords p/ termos de maior tráfego (cli/state-machine/workflow/agents) mantendo a keyword de marca e os dois slugs de categoria válidos; commitar o prep num branch e publicar de uma árvore limpa (sem `--allow-dirty`), token só via env. | Publish no crates.io é permanente (yank ≠ delete) — gatear atrás de confirmação explícita; segredo em PRD de intake (token em plaintext) é risco real — usar via env, nunca commitar, e sinalizar rotação; o bump p/ edition 2024 neste crate zero-dep foi no-op de código (cargo check/test limpos). |
| 014 | Self-adoption: o repositório (fonte da metodologia) passou a praticar as estruturas que publica — criou seu próprio `intake/` (Backlog/Discovery/QA) + `backlog.md` (`O-001`), atualizou a árvore do `project-definition.md` (`packages/` + artefatos 007/008/012/013) e documentou `RAW_HISTORY=3` no `memory.md`. | Uma fonte auto-referencial deve praticar o que entrega — sem o inbox, a triagem de intake era um no-op silencioso; self-adoption registrada como `O-001` (promoted → 014), exercitando o pipeline intake→backlog→roadmap. | Um audit "como usuário da própria metodologia" revela lacunas que a mera implementação da metodologia não pega; a *Repository Structure* do `project-definition.md` defasa ao adicionar áreas grandes (ex.: `packages/`) e exige refresh explícito. |
| 013 | Implementação de referência **opcional** do contrato de fluxo multi-agente em `packages/`: crate Rust `stateful-spec-flow` + gêmeo Node/TS `@stateful-spec/flow` (ambos zero-dep), com paridade Rust≡TS via fixtures compartilhadas; a metodologia segue usável **sem** os packages, com a regra "pedir permissão antes de usar" e uma emenda à restrição documentation-only para isolar `packages/`. | Core + spawn **configurável** (sem bundles hardcoded) — o package sabe COMO spawnar, o projeto fornece O QUE via `flow.conf`; packages opcionais/não-vinculantes (agente deve saber + pedir permissão); emendar documentation-only p/ carve-out de `packages/`; fontes+testes, publish manual (sem CI). | Paridade Rust≡TS é garantível por fixtures compartilhadas rodadas pelas duas suítes (cargo 54+7+1; node 4+7+5), verificado do zero; a curadoria removeu spawn bundles/cargo-fmt/opencode/PT sem perder o contrato (13 verbos+exit codes); revisão que re-roda os gates (não confia no self-review) confirma contagens exatas. |
| 012 | Back-port curado e **agnóstico** das evoluções de metodologia do downstream `stand-in`: 3 novos docs universais (`history-archiving`, `backlog`/intake, `qa-phase`), `multi-agent-flow.md` reescrito como doc único de **dois modos** (2 e 3 agentes) + contrato de tooling abstrato, e fiação das estruturas de apoio nas operações de sessão (+3 ports cada) e templates — documentation-only preservado, 5 fases intactas. | Multi-agent flow = doc único de dois modos (sem v1/v2), descartando a bagagem histórica do stand-in; estruturas de apoio (archiving/intake-backlog/QA) universais a todos os tipos, multi-agent-flow segue software-only; nenhum código portado — protocolos como contratos abstratos. | Curadoria agnóstica zera o leakage de stand-in (grep + revisão independente); a regra de sync source↔3 ports escala — cada adição verificada nos 3 ports; a migração estrutural do próprio repo (self-adoption) deve ser deferida ao fechamento revisado, não embarcada na sessão de implementação. |
| 011 | Formalizou um modo opcional de **fluxo autônomo de dois agentes** (PM/Arquiteto + Eng. Sênior) coordenado por um arquivo de estado dedicado (`.stateful-spec/flow-state.md`): novo protocolo `methodology/multi-agent-flow.md`, operação `start-multi-agent-flow` (+3 ports), templates `flow-state.md`/`review-handoff.md`, e notas aditivas em `roles.md` e nas ops de sessão. Software-only; reusa o ciclo de 5 fases; generaliza o padrão que a iteração 011 do `stand-in` provou à mão. | Loop autônomo por polling sobre arquivo de estado com máquina de estados granular e turn-based (sem colisão); humano aprova o plano 1× e só gateia ações irreversíveis, com teto de `review_round`=3→BLOCKED; umbrella = 1 iteração/1 Engrama, cada marco é uma sessão, layout flat em `history/`. | Padrão de marco (spec→impl+gate→handoff→fix→commit) generaliza limpo num modo aditivo sem tocar nas 5 fases; investigação web (resíduo do prompt de engramas) dispensada explicitamente pelo dev e registrada honestamente (lição da 009); regra de sync respeitada desde o início (ports escritos junto com a fonte). |
| 010 | Reconciliação de pontas soltas pós-008: indexou o review-handoff 009 (commitado, PR #25) na History Index e registrou a feature Engramas no CHANGELOG `[Unreleased]`. | Indexar 009 como `review-handoff` (mantendo o arquivo) para preservar trilha de auditoria visível; 009 não recebe linha de Engramas por não ter Session Log próprio. | History Index e tabela Engramas divergem de propósito para artefatos não-feature — review-handoffs são indexados mas não compilados em engramas. |
| 008 | Sistema de compactação de memória com seção Engramas, algoritmo map-reduce sobre Session Log, drill-down por engramas em vez de ler history/ inteiro, e two-tier compaction para prevenir crescimento ilimitado da tabela. | Two-tier compaction com Archive row mantém tabela em N+1 linhas; ports de operações sincronizados via regra de sync obrigatória; pesquisa de alternativas documentada (hierarchical, MemGPT/Letta, RAG, gist tokens, progressive disclosure). | A regra de sync do project-definition obriga portar mudanças para .cursor/.claude/.opencode; a two-tier compaction resolve o problema assintótico real que o modelo flat não resolvia. |
| 007 | Suporte a múltiplos tipos de projeto (software, skills, studies) com registro central `project-types.md`. Criou templates, operações e presets específicos por tipo, além de detecção nos wizards de inicialização. | Project Type como campo load-bearing com seções condicionais; software mantido como default para backward compatibility | O repositório da metodologia permanece como software/documentation-only — não reclassificado |
| 0-archived | As primeiras iterações estabeleceram as fundações da metodologia: correção dos prompts de inicialização do Cursor (001), a obrigatoriedade de iteration tracking via arquivos `history/NNN-name.md` (002) e o wizard `update-project` para refresh da metodologia em projetos já configurados (003); em seguida veio o suporte ao Claude Code com comandos nativos em `.claude/commands/` e entry point CLAUDE.md (004); o sistema de session lifecycle (`start-session`/`end-session`) com Session Log para registrar contribuições de várias operações sob a mesma iteração (005); e a generalização do `AGENTS.md` para ser universal (não só Cursor), unificando os imports CLAUDE.md→AGENTS.md e as regras de agentes (006). Conteúdo completo preservado em `history/.archived/memory.md`. | Iteration file como artefato permanente de tracking; `memory.md` atualizado ao iniciar/completar trabalho (002); operation prompts espelhados como comandos nativos em múltiplas ferramentas — Cursor, Claude Code (004); sessões explícitas com Open Session flag em `memory.md`, cada operação registra timestamp + operação + sumário no Session Log (005). | Closed stale session de iteração já mergeada via PR (005). |
