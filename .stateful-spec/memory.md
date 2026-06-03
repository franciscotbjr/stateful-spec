# Project Memory

> This file is the AI's entry point for understanding the project's current state. Keep it updated as work progresses.

## Project Summary

- **Project:** Stateful Spec
- **Description:** A specification-driven development methodology for AI-assisted software projects
- **Last Updated:** 2026-06-03
- **Current Status:** Active development

## Active Work

> What is currently in progress? Reference the iteration file.

_(none)_

## Open Session

> When a session is active, this section points to the current iteration file.
> Managed by `start-session` and `end-session` — do not edit manually.

_(none)_

## Recent Completions

> Last 3-5 completed iterations for quick context.

| # | Name | Type | Completed |
|---|------|------|-----------|
| 008 | engramas-compiled-memory | feature | 2026-06-03 |
| 007 | multi-project-type-support | feature | 2026-06-02 |
| 005 | new-session-management | feature | 2026-05-03 |
| 004 | add-claude-code-support | feature | 2026-04-25 |
| 002 | require-iteration-tracking | chore | 2026-04-04 |

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
| 008 | engramas-compiled-memory | feature | done | `history/008-engramas-compiled-memory.md` |
| 007 | multi-project-type-support | feature | done | `history/007-multi-project-type-support.md` |
| 006 | universal-agents-md | fix | done | `history/006-universal-agents-md.md` |
| 005 | new-session-management | feature | done | `history/005-new-session-management.md` |
| 004 | add-claude-code-support | feature | done | `history/004-add-claude-code-support.md` |
| 002 | require-iteration-tracking | chore | done | `history/002-require-iteration-tracking.md` |
| 003 | add-update-project-wizard | feature | done | `history/003-add-update-project-wizard.md` |
| 001 | fix-cursor-init-prompts | chore | done | `history/001-fix-cursor-init-prompts.md` |

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
| 008 | Sistema de compactação de memória com seção Engramas, algoritmo map-reduce sobre Session Log, drill-down por engramas em vez de ler history/ inteiro, e two-tier compaction para prevenir crescimento ilimitado da tabela. | Two-tier compaction com Archive row mantém tabela em N+1 linhas; ports de operações sincronizados via regra de sync obrigatória; pesquisa de alternativas documentada (hierarchical, MemGPT/Letta, RAG, gist tokens, progressive disclosure). | A regra de sync do project-definition obriga portar mudanças para .cursor/.claude/.opencode; a two-tier compaction resolve o problema assintótico real que o modelo flat não resolvia. |
| 007 | Suporte a múltiplos tipos de projeto (software, skills, studies) com registro central `project-types.md`. Criou templates, operações e presets específicos por tipo, além de detecção nos wizards de inicialização. | Project Type como campo load-bearing com seções condicionais; software mantido como default para backward compatibility | O repositório da metodologia permanece como software/documentation-only — não reclassificado |
| 006 | Correção do AGENTS.md para ser universal (não apenas Cursor). Generalizou imports de CLAUDE.md para AGENTS.md e unificou as regras de agentes. | — | — |
| 005 | Sistema de session lifecycle com start-session e end-session. Iteration tracking com Session Log para registrar contribuições de diferentes operações sob a mesma iteração. | Sessões explícitas com Open Session flag em memory.md; operações registram timestamp + operação + sumário no Session Log | Closed stale session de iteração já mergeada via PR |
| 004 | Suporte ao Claude Code com comandos nativos em `.claude/commands/` e entry point CLAUDE.md. Operações disponíveis como `/resume-session`, `/save-session`, etc. | Operation prompts espelhados como comandos nativos em múltiplas ferramentas (Cursor, Claude Code) | — |
| 003 | Wizard `update-project.md` para atualizar a metodologia em projetos já configurados. Permite refresh de prompts, regras e templates a partir do upstream. | — | — |
| 002 | Iteration tracking obrigatório para todo trabalho não-trivial. Arquivos `history/NNN-name.md` com template padronizado de iteration. | Iteration file como artefato permanente de tracking; memory.md atualizado ao iniciar/completar trabalho | — |
| 001 | Correção nos prompts de inicialização do Cursor para seguir a estrutura de diretórios correta. Ajuste nos paths e referências. | — | — |
| 0-archived | — | — | — |
