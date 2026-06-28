# Project Memory

> This file is the AI's entry point for understanding the project's current state. Keep it updated as work progresses.

## Project Summary

- **Project:** Stateful Spec
- **Description:** A specification-driven development methodology for AI-assisted software projects
- **Last Updated:** 2026-06-28
- **Current Status:** Active development

## Active Work

> What is currently in progress? Reference the iteration file.

- _(none)_ — no iteration in progress. Last delivery: 012 + 013 (closed 2026-06-28, committed `b4bdd10`). Possible next: this repo's own self-adoption (scaffold `.stateful-spec/intake/` + `backlog.md`).

## Open Session

> When a session is active, this section points to the current iteration file.
> Managed by `start-session` and `end-session` — do not edit manually.

- _(none)_

## Recent Completions

> Last 3-5 completed iterations for quick context.

| # | Name | Type | Completed |
|---|------|------|-----------|
| 013 | flow-packages | feature | 2026-06-28 |
| 012 | reverse-update-from-stand-in | feature | 2026-06-28 |
| 011 | multi-agent-flow | feature | 2026-06-03 |
| 010 | reconcile-loose-ends | chore | 2026-06-03 |
| 008 | engramas-compiled-memory | feature | 2026-06-03 |

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

| # | Name | Type | Status | File |
|---|------|------|--------|------|
| 013 | flow-packages | feature | done | `history/013-flow-packages.md` |
| 012 | reverse-update-from-stand-in | feature | done | `history/012-reverse-update-from-stand-in.md` |
| 011 | multi-agent-flow | feature | done | `history/011-multi-agent-flow.md` |
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
| 013 | Implementação de referência **opcional** do contrato de fluxo multi-agente em `packages/`: crate Rust `stateful-spec-flow` + gêmeo Node/TS `@stateful-spec/flow` (ambos zero-dep), com paridade Rust≡TS via fixtures compartilhadas; a metodologia segue usável **sem** os packages, com a regra "pedir permissão antes de usar" e uma emenda à restrição documentation-only para isolar `packages/`. | Core + spawn **configurável** (sem bundles hardcoded) — o package sabe COMO spawnar, o projeto fornece O QUE via `flow.conf`; packages opcionais/não-vinculantes (agente deve saber + pedir permissão); emendar documentation-only p/ carve-out de `packages/`; fontes+testes, publish manual (sem CI). | Paridade Rust≡TS é garantível por fixtures compartilhadas rodadas pelas duas suítes (cargo 54+7+1; node 4+7+5), verificado do zero; a curadoria removeu spawn bundles/cargo-fmt/opencode/PT sem perder o contrato (13 verbos+exit codes); revisão que re-roda os gates (não confia no self-review) confirma contagens exatas. |
| 012 | Back-port curado e **agnóstico** das evoluções de metodologia do downstream `stand-in`: 3 novos docs universais (`history-archiving`, `backlog`/intake, `qa-phase`), `multi-agent-flow.md` reescrito como doc único de **dois modos** (2 e 3 agentes) + contrato de tooling abstrato, e fiação das estruturas de apoio nas operações de sessão (+3 ports cada) e templates — documentation-only preservado, 5 fases intactas. | Multi-agent flow = doc único de dois modos (sem v1/v2), descartando a bagagem histórica do stand-in; estruturas de apoio (archiving/intake-backlog/QA) universais a todos os tipos, multi-agent-flow segue software-only; nenhum código portado — protocolos como contratos abstratos. | Curadoria agnóstica zera o leakage de stand-in (grep + revisão independente); a regra de sync source↔3 ports escala — cada adição verificada nos 3 ports; a migração estrutural do próprio repo (self-adoption) deve ser deferida ao fechamento revisado, não embarcada na sessão de implementação. |
| 011 | Formalizou um modo opcional de **fluxo autônomo de dois agentes** (PM/Arquiteto + Eng. Sênior) coordenado por um arquivo de estado dedicado (`.stateful-spec/flow-state.md`): novo protocolo `methodology/multi-agent-flow.md`, operação `start-multi-agent-flow` (+3 ports), templates `flow-state.md`/`review-handoff.md`, e notas aditivas em `roles.md` e nas ops de sessão. Software-only; reusa o ciclo de 5 fases; generaliza o padrão que a iteração 011 do `stand-in` provou à mão. | Loop autônomo por polling sobre arquivo de estado com máquina de estados granular e turn-based (sem colisão); humano aprova o plano 1× e só gateia ações irreversíveis, com teto de `review_round`=3→BLOCKED; umbrella = 1 iteração/1 Engrama, cada marco é uma sessão, layout flat em `history/`. | Padrão de marco (spec→impl+gate→handoff→fix→commit) generaliza limpo num modo aditivo sem tocar nas 5 fases; investigação web (resíduo do prompt de engramas) dispensada explicitamente pelo dev e registrada honestamente (lição da 009); regra de sync respeitada desde o início (ports escritos junto com a fonte). |
| 010 | Reconciliação de pontas soltas pós-008: indexou o review-handoff 009 (commitado, PR #25) na History Index e registrou a feature Engramas no CHANGELOG `[Unreleased]`. | Indexar 009 como `review-handoff` (mantendo o arquivo) para preservar trilha de auditoria visível; 009 não recebe linha de Engramas por não ter Session Log próprio. | History Index e tabela Engramas divergem de propósito para artefatos não-feature — review-handoffs são indexados mas não compilados em engramas. |
| 008 | Sistema de compactação de memória com seção Engramas, algoritmo map-reduce sobre Session Log, drill-down por engramas em vez de ler history/ inteiro, e two-tier compaction para prevenir crescimento ilimitado da tabela. | Two-tier compaction com Archive row mantém tabela em N+1 linhas; ports de operações sincronizados via regra de sync obrigatória; pesquisa de alternativas documentada (hierarchical, MemGPT/Letta, RAG, gist tokens, progressive disclosure). | A regra de sync do project-definition obriga portar mudanças para .cursor/.claude/.opencode; a two-tier compaction resolve o problema assintótico real que o modelo flat não resolvia. |
| 007 | Suporte a múltiplos tipos de projeto (software, skills, studies) com registro central `project-types.md`. Criou templates, operações e presets específicos por tipo, além de detecção nos wizards de inicialização. | Project Type como campo load-bearing com seções condicionais; software mantido como default para backward compatibility | O repositório da metodologia permanece como software/documentation-only — não reclassificado |
| 006 | Correção do AGENTS.md para ser universal (não apenas Cursor). Generalizou imports de CLAUDE.md para AGENTS.md e unificou as regras de agentes. | — | — |
| 005 | Sistema de session lifecycle com start-session e end-session. Iteration tracking com Session Log para registrar contribuições de diferentes operações sob a mesma iteração. | Sessões explícitas com Open Session flag em memory.md; operações registram timestamp + operação + sumário no Session Log | Closed stale session de iteração já mergeada via PR |
| 004 | Suporte ao Claude Code com comandos nativos em `.claude/commands/` e entry point CLAUDE.md. Operações disponíveis como `/resume-session`, `/save-session`, etc. | Operation prompts espelhados como comandos nativos em múltiplas ferramentas (Cursor, Claude Code) | — |
| 003 | Wizard `update-project.md` para atualizar a metodologia em projetos já configurados. Permite refresh de prompts, regras e templates a partir do upstream. | — | — |
| 0-archived | As primeiras iterações estabeleceram as fundações da metodologia: correção dos prompts de inicialização do Cursor para seguir a estrutura de diretórios correta (001) e a obrigatoriedade de iteration tracking para todo trabalho não-trivial via arquivos `history/NNN-name.md` (002). Conteúdo completo preservado em `history/.archived/memory.md`. | Iteration file como artefato permanente de tracking; `memory.md` atualizado ao iniciar/completar trabalho (002). | — |
