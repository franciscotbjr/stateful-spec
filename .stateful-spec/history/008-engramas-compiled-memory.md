# Iteration: 008 — Sistema de Engramas para Memory.md

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** feature
- **Status:** done
- **Created:** 2026-06-02
- **Completed:** 2026-06-03
- **Author:** Francisco Tarcizo Bomfim Júnior

## Description

Adicionar mecanismo de "compactação de memória" ao Stateful Spec, inspirado no
padrão LLM Wiki. Uma nova seção **Engramas** em `memory.md` armazena sumários
compilados de cada iteração do histórico, permitindo que o `resume-session`
obtenha contexto suficiente lendo apenas `memory.md` — sem precisar carregar
todos os arquivos da pasta `history/` na janela de contexto.

## Acceptance Criteria

- [x] `resume-session.md` não instrui mais a ler todos os arquivos de `history/`
- [x] `resume-session.md` instrui a usar Engramas e só fazer drill-down se necessário
- [x] `start-session.md` inicializa linha de Engramas na criação da iteração
- [x] `save-session.md` atualiza Engramas via map-reduce ao salvar
- [x] `end-session.md` finaliza Engramas via map-reduce ao fechar
- [x] `templates/project/memory.md` contém seção Engramas após History Index
- [x] `methodology/overview.md` referencia Engramas em Key Files + Why This Matters
- [x] `AGENTS.md` contém regra central de manutenção de Engramas
- [x] `.stateful-spec/memory.md` tem seção Engramas populada com iterações 001-008
- [x] Conteúdo existente de `resume-session.md` não relacionado a history/ permanece intacto
- [x] Operações não-lifecycle (spec, debug, etc.) permanecem inalteradas

## Implementation Tasks

- [x] Criar seção Engramas em `templates/project/memory.md`
- [x] Adicionar regra central de Engramas em `AGENTS.md`
- [x] Atualizar `methodology/overview.md` (Key Files + Why This Matters)
- [x] Atualizar `prompts/operations/resume-session.md` (STEP 1 + direct-task + drill-down)
- [x] Atualizar `prompts/operations/start-session.md` (initialize Engramas row)
- [x] Atualizar `prompts/operations/save-session.md` (update Engramas via map-reduce)
- [x] Atualizar `prompts/operations/end-session.md` (finalize Engramas via map-reduce)
- [x] Popular Engramas em `.stateful-spec/memory.md` com iterações 001-008

## Quality Checks

> Standard checks from the Project Definition. Verify before marking done.

- [x] Conventions followed (English, kebab-case, ATX headings, GFM tables, placeholder syntax)
- [x] No application code, build tooling, or CI introduced (documentation-only)
- [x] All internal links resolve
- [x] Backward compatibility preserved (software subsections carry current content verbatim)

## Session Log

> Timestamped entries recording each operation performed during this session.
> Agents append entries automatically when an Open Session is active.

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-06-02 | analyze+plan+specify | Definição completa via questionamento exaustivo: seção Engramas (tabela #/Resumo/Decisões/Aprendizados), algoritmo map-reduce sempre com lotes de 5, trigger nas lifecycle ops, drill-down por instrução aberta, escopo apenas history/. |
| 2026-06-02 | implement | Implementação completa (8 arquivos): template memory.md + AGENTS.md + overview.md + 4 lifecycle prompts (resume/start/save/end) + instância memory.md populada com engramas 001-008. |
| 2026-06-03 | review | Revisão crítica (file 009) encontrou 2 bloqueantes (B-1: ports não sincronizados, B-2: web research não feita) e 5 graves (G-1: template em português, G-2/G-3: instruções contraditórias, G-4: crescimento assintótico não resolvido, G-5: fechamento do 007 vazado no diff). |
| 2026-06-03 | implement | Correção completa dos defeitos: (1) sync de 4 prompts para 12 port files (.claude/.cursor/.opencode), (2) pesquisa de alternativas de sumarização documentada (hierarchical, MemGPT/Letta, RAG, gist tokens, progressive disclosure), (3) tradução do template Engramas para inglês, (4) reconciliação de instruções (insert top + Resumo=_In progress_ no start-session), (5) two-tier compaction com Archive row (N=10 ativas + 1 archive) resolvendo crescimento assintótico, (6) renumeração de 1.5. no end-session, fix do AC 001-007→001-008. AGENTS.md, source prompts, ports, template, e memory.md atualizados. |
| 2026-06-03 | review+fix | Segunda rodada certificada: tradução completa dos nomes de coluna PT→EN (Summary/Key Decisions/Learnings) nos 13 arquivos (4 fontes + AGENTS.md + 9 ports), fix do AC 001-007→001-008, typos (Sumarization→Summarization; frase truncada no engram 008). Certificação confirmou 2 bloqueantes e 5 graves resolvidos; ports espelham os fontes. |
| 2026-06-03 | end-session | Session closed. Feature Engramas concluída — compactação de memória via map-reduce + two-tier compaction (Archive row), ports sincronizados, investigação de modelos documentada. Todos os critérios de aceite e quality checks atendidos. |

## Decisions Made

> Decisions made during this iteration. Include rationale.

| Decision | Rationale | Date |
|----------|-----------|------|
| Técnica map-reduce sempre | Consistência no comportamento, independente do tamanho da iteration | 2026-06-02 |
| Lote de 5 Session Log entries no map | Lotes menores = sumários parciais mais precisos | 2026-06-02 |
| Agente sintetiza do arquivo inteiro | Captura nuances que o mapeamento direto perderia | 2026-06-02 |
| Escopo apenas history/ (não methodology/) | methodology/ é conjunto fixo, não cresce com o tempo. Foco no problema principal. | 2026-06-02 |
| Regra central no AGENTS.md + menção nas lifecycle ops | Evita replicar instrução em ~15 operation prompts | 2026-06-02 |
| Sincronizar ports de prompts-fonte para `.cursor/`, `.claude/`, `.opencode/` | Regra de sync em project-definition.md:144 é inegociável — ports são cópias tool-native dos prompts de operação, não instâncias de dados | 2026-06-03 |
| Two-tier compaction com Archive row | Tabela engranas crescia 1 linha por iteração sem limite — o mesmo problema assintótico original com constante menor. Solução: N rows ativas (default 10) + 1 archive row, merge automático quando excede N. | 2026-06-03 |
| `start-session` insere `_In progress_` em vez de sintetizar da Description | O conteúdo real só é conhecido após o trabalho — a síntese deve vir no save/end via map-reduce sobre o Session Log. | 2026-06-03 |

### Summarization Models Investigation (B-2)

| Model | Source | Pros | Cons | Verdict |
|-------|--------|------|------|---------|
| **LLM Wiki (Karpathy)** | [gist.github.com/karpathy](https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f) | index.md catalog, append-only log.md, 3-layer architecture (raw→wiki→schema), compounding artifact | No summarization tiers; same flat growth problem | Inspiration — adapted core ideas |
| **Hierarchical/Rolling Summary** | Known pattern in DB aging (time-series compaction) | Bounded context, fixed cost regardless of history size | Loss of detail on older iterations; merge logic complexity | Adopted as Tier 2 (Archive row) |
| **MemGPT/Letta (OS memory)** | [letta.com](https://letta.com), [memgpt.ai](https://memgpt.ai) | Context Constitution principles, sleep-time compute, managed memory tiers, continual learning | Heavy infrastructure (DB, job queue, Rust core); designed for live agents, not static markdown docs | Not applicable — too heavy for documentation-only project |
| **RAG over history** | Common pattern (embeddings + vector search) | Semantic retrieval, only loads context relevant to current task | Requires embedding infrastructure (vector DB, embedding model); retrieval quality varies; adds tooling burden | Overkill — adds infrastructure dependency the methodology avoids |
| **Gist tokens (Mu et al. 2023)** | [arXiv:2304.08467](https://arxiv.org/abs/2304.08467) | 26x prompt compression via trained gist tokens, up to 40% FLOP reduction | Requires model training/fine-tuning; designed for runtime prompt compression, not persistent memory summarization | Not applicable — wrong category (runtime compression, not structural memory) |
| **Progressive disclosure** | Common UX pattern (overview → detail on demand) | Multi-resolution; agent reads what's needed, skips what's not | Agent must decide depth heuristically; risk of under-reading (misses critical context) | Adopted — engram row (overview) → history file (detail) drill-down |

## Blockers & Notes

> Anything that blocked progress or is worth remembering.

- O `resume-session` atual também lê toda a `methodology/` via STEP 1 — otimizar isso fica para uma iteração futura.
- Remoção de iterações (não prevista atualmente) deve ser considerada se/quando adicionada — o Engramas precisará suportar remoção de linhas.
- **G-5 scope leak acknowledged:** A iteração original fechou o 007 (review→done) dentro do mesmo diff sem registro no Session Log do 008. O estado já foi consolidado — manter o 007 como done.
- **Known limitation (G-4):** O modelo original não resolvia o crescimento assintótico — apenas reduzia a constante (tabela Engramas crescia 1 linha por iteração, sem limite). Resolvido com two-tier compaction (N ativas + 1 archive).

## References

- **Especificação:** diálogo de questionamento exaustivo (design tree), 2026-06-02
- **Inspiração:** LLM Wiki — https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f
- **PR/MR:** —
- **Commits:** —
- **Related Issues:** —
