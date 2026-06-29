# Archived Engramas (cold store)

> Folded Engrama rows, preserved **verbatim** before they collapsed into the `0-archived`
> synthesis row in `../../memory.md`. Append-only, newest at the bottom; one section per folded
> iteration. **Never bulk-read** — open a section only when the `0-archived` synthesis is
> insufficient (see `methodology/history-archiving.md`).

## 001 — folded 2026-06-28

| # | Summary | Key Decisions | Learnings |
|---|---------|---------------|-----------|
| 001 | Correção nos prompts de inicialização do Cursor para seguir a estrutura de diretórios correta. Ajuste nos paths e referências. | — | — |

## 002 — folded 2026-06-28

| # | Summary | Key Decisions | Learnings |
|---|---------|---------------|-----------|
| 002 | Iteration tracking obrigatório para todo trabalho não-trivial. Arquivos `history/NNN-name.md` com template padronizado de iteration. | Iteration file como artefato permanente de tracking; memory.md atualizado ao iniciar/completar trabalho | — |

## 003 — folded 2026-06-28

| # | Summary | Key Decisions | Learnings |
|---|---------|---------------|-----------|
| 003 | Wizard `update-project.md` para atualizar a metodologia em projetos já configurados. Permite refresh de prompts, regras e templates a partir do upstream. | — | — |

## 004 — folded 2026-06-28

| # | Summary | Key Decisions | Learnings |
|---|---------|---------------|-----------|
| 004 | Suporte ao Claude Code com comandos nativos em `.claude/commands/` e entry point CLAUDE.md. Operações disponíveis como `/resume-session`, `/save-session`, etc. | Operation prompts espelhados como comandos nativos em múltiplas ferramentas (Cursor, Claude Code) | — |

## 005 — folded 2026-06-28

| # | Summary | Key Decisions | Learnings |
|---|---------|---------------|-----------|
| 005 | Sistema de session lifecycle com start-session e end-session. Iteration tracking com Session Log para registrar contribuições de diferentes operações sob a mesma iteração. | Sessões explícitas com Open Session flag em memory.md; operações registram timestamp + operação + sumário no Session Log | Closed stale session de iteração já mergeada via PR |

## 006 — folded 2026-06-28

| # | Summary | Key Decisions | Learnings |
|---|---------|---------------|-----------|
| 006 | Correção do AGENTS.md para ser universal (não apenas Cursor). Generalizou imports de CLAUDE.md para AGENTS.md e unificou as regras de agentes. | — | — |

## 007 — folded 2026-06-28

| # | Summary | Key Decisions | Learnings |
|---|---------|---------------|-----------|
| 007 | Suporte a múltiplos tipos de projeto (software, skills, studies) com registro central `project-types.md`. Criou templates, operações e presets específicos por tipo, além de detecção nos wizards de inicialização. | Project Type como campo load-bearing com seções condicionais; software mantido como default para backward compatibility | O repositório da metodologia permanece como software/documentation-only — não reclassificado |

## 008 — folded 2026-06-28

| # | Summary | Key Decisions | Learnings |
|---|---------|---------------|-----------|
| 008 | Sistema de compactação de memória com seção Engramas, algoritmo map-reduce sobre Session Log, drill-down por engramas em vez de ler history/ inteiro, e two-tier compaction para prevenir crescimento ilimitado da tabela. | Two-tier compaction com Archive row mantém tabela em N+1 linhas; ports de operações sincronizados via regra de sync obrigatória; pesquisa de alternativas documentada (hierarchical, MemGPT/Letta, RAG, gist tokens, progressive disclosure). | A regra de sync do project-definition obriga portar mudanças para .cursor/.claude/.opencode; a two-tier compaction resolve o problema assintótico real que o modelo flat não resolvia. |

## 010 — folded 2026-06-28

| # | Summary | Key Decisions | Learnings |
|---|---------|---------------|-----------|
| 010 | Reconciliação de pontas soltas pós-008: indexou o review-handoff 009 (commitado, PR #25) na History Index e registrou a feature Engramas no CHANGELOG `[Unreleased]`. | Indexar 009 como `review-handoff` (mantendo o arquivo) para preservar trilha de auditoria visível; 009 não recebe linha de Engramas por não ter Session Log próprio. | History Index e tabela Engramas divergem de propósito para artefatos não-feature — review-handoffs são indexados mas não compilados em engramas. |
