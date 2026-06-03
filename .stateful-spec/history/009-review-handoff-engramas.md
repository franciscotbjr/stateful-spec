# Review Handoff: 008 — Sistema de Engramas

> **Este arquivo é um handoff de review, não uma iteração de feature.** Documenta
> a revisão crítica da iteração [008](008-engramas-compiled-memory.md) para que o
> próximo agente/desenvolvedor possa retomar as correções sem refazer a análise.
> A iteração 008 permanece a Open Session; as correções abaixo devem ser
> registradas no Session Log dela.

## Metadata

- **Tipo:** review-handoff
- **Alvo da review:** `008-engramas-compiled-memory.md` (status `review`)
- **Data:** 2026-06-02
- **Revisor:** análise sob `resume-session` (revisão implacável solicitada)
- **Veredito:** **Reprovado para fechamento** — caixas de Quality Check falsas e feature deployada pela metade.

## Escopo revisado

- `impl.md` (especificação original do pedido)
- `.stateful-spec/history/008-engramas-compiled-memory.md`
- `git diff` do working tree (9 arquivos, +58/−9)
- Verificação cruzada dos ports `.cursor/` · `.claude/` · `.opencode/`

## Veredito

A iteração está marcada `review` com **100% das caixas marcadas**, mas pelo menos
**duas marcações são falsas** (verificáveis no diff) e a feature está **deployada
pela metade**: o comportamento novo existe nos prompts-fonte, mas **não** nos
comandos nativos que os agentes realmente executam. O núcleo conceitual também não
resolve o crescimento assintótico descrito no `impl.md` — apenas reduz a constante.

## Objetivos do `impl.md` — atingidos?

| Objetivo declarado no `impl.md` | Status | Nota |
|---|---|---|
| `resume-session` não lê mais toda a `history/` (nos **prompts-fonte**) | ✅ | `resume-session.md:49` |
| Seção compilada ("Engramas") em `memory.md` | ✅ | Estrutura criada |
| Drill-down só quando necessário | ⚠️ | Existe, mas sem heurística (B-3 / M-3) |
| Atualização nas operações de ciclo de vida | ⚠️ | Só nos fontes, **não nos ports** (B-1) |
| "Não afetar outras partes da metodologia exceto se necessário" | ❌ | Quebrou sync; mudou status do 007 junto (B-1 / G-5) |
| "Investigar outros modelos de sumarização na web de forma implacável" | ❌ | **Sem evidência alguma** (B-2) |
| Persona: documentar todas as alternativas consideradas | ❌ | Alternativas de *modelo de sumarização* não documentadas (B-2) |

Dois pedidos **explícitos** do prompt original foram silenciosamente ignorados.

---

## Defeitos BLOQUEANTES

### B-1 — Regra de sincronização violada (feature pela metade) 🔴

Restrição inegociável do repositório (`project-definition.md:144`, CLAUDE.md, AGENTS.md):

> *"When modifying source prompts in `prompts/operations/`, the AI must also update
> the corresponding `.cursor/rules/`, `.claude/commands/`, and `.opencode/commands/`."*

A iteração modificou **4** prompts-fonte (`resume`, `start`, `save`, `end`) e **não
tocou em nenhum port**.

**Evidência verificada:**

```
.claude/commands/resume-session.md:49     → ainda "Read all files."
.cursor/rules/resume-session.mdc:50       → ainda "Read all files."
.opencode/commands/resume-session.md:49   → ainda "Read all files."
grep "Engramas" .claude/ .cursor/ .opencode/  → 0 ocorrências
git status .claude/ .cursor/ .opencode/        → limpo (intocado)
```

**Consequência:** o comando `/resume-session` nativo **ainda manda ler toda a
`history/`**. Para os três agentes nativos (o caminho real de execução), a feature
**não existe** e a economia de tokens não é entregue.

A Decisão *"Pastas `.claude/`, `.cursor/`, `.opencode/` fora do escopo"* é um **erro
de categoria**: confunde a *instância de dados* Engramas (`.stateful-spec/memory.md`
— de fato fora de escopo) com os *ports dos prompts de operação* (cópias tool-native
dos prompts que acabaram de mudar — governados por regra de sync obrigatória).

### B-2 — Investigação obrigatória (web research) não realizada 🔴

`impl.md` §Investigação exige investigar modelos de sumarização "na web de forma
implacável"; §Persona exige documentar "todas as alternativas consideradas e os
motivos". As `References` da iteração citam **apenas o gist do karpathy** (já dado no
prompt). Nenhuma alternativa real de arquitetura de memória foi avaliada
(hierarchical/recursive summarization, rolling summary, RAG sobre history,
MemGPT/Letta, progressive disclosure). "Decisions Made" documenta detalhes de
implementação (lote=5, map-reduce), não a escolha do **modelo**.

---

## Defeitos GRAVES

### G-1 — Boilerplate em português num template que deve ser inglês 🟠

Quality Check linha 50 da iteração afirma *"Conventions followed (**English**…)"* —
marcada `[x]`. **Falso.** O bloco Engramas em `templates/project/memory.md` (que
**ships para projetos downstream**) está em português: *"Memória compilada do
histórico…"* + colunas *Resumo / Decisões-chave / Aprendizados* + placeholder
*"[1-2 frases do que foi feito]"*. Atestação de convenção demonstravelmente
incorreta. (No `.stateful-spec/memory.md` deste repo o português é defensável; no
**template**, não.)

### G-2 — Instrução escrita contradiz os dados de exemplo 🟠

`start-session.md` (novo) manda *"Synthesize the `Resumo` from the iteration's
Description (1-2 sentences)"*, mas a linha 008 em `memory.md` traz
`Resumo = _Em andamento_` — não uma síntese. Pior: a Decisão *"Agente sintetiza do
arquivo inteiro"* **contradiz** o algoritmo map-reduce-sobre-Session-Log escrito em
AGENTS.md e nos prompts. Fonte da verdade ambígua → drift garantido.

### G-3 — "Append" (instrução) vs. "prepend" (dados) 🟠

`start-session.md` diz *"**Append** a new row"*, mas a tabela real é *newest-first*
(008 no topo). Appendar coloca no fim — oposto da prática. Próximo agente insere na
posição errada.

### G-4 — O modelo não resolve o crescimento assintótico, só a constante 🟠

Falha de design mais profunda. O `impl.md` descreve o problema como o número de
**arquivos history crescendo ao longo das iterações**. Porém:

- O map-reduce comprime **dentro de uma iteração** (entradas do Session Log) — eixo errado.
- A **tabela Engramas cresce 1 linha por iteração, sem limite**. Após N iterações são
  N linhas lidas em **todo** `resume-session` — mesmo problema de crescimento
  ilimitado, com fator constante menor.
- **Sem compactação de segundo nível** (rollup hierárquico, aging, tiers), que é
  justamente o que a inspiração LLM-Wiki/memória hierárquica pediria.
- "Map-reduce sempre com lotes de 5" **degenera**: Session Log típico tem 2–5
  entradas (o do 008 tem 2) → um único lote → cerimônia sem benefício.

### G-5 — Escopo vazado: fechamento do 007 no diff do 008 🟠

O diff vira `007` de `review`→`done` e preenche a data de conclusão, dentro do
working tree da feature 008, **sem registro no Session Log do 008**. Mistura o
encerramento de uma iteração anterior numa feature não relacionada — viola o cuidado
do `impl.md` ("outras partes não podem ser afetadas exceto se necessário").

---

## Defeitos MENORES / smells

- **M-1** — `1.5.` como número de passo em `end-session.md`: passo fracionário em
  lista ordenada é gambiarra; renumerar 1→2→3.
- **M-2** — Regra do AGENTS.md (`AGENTS.md:56`) é um parágrafo único denso que
  duplica o algoritmo de save/end-session e diz "keep in sync" sem nunca podar
  (reforça G-4).
- **M-3** — Drill-down sem heurística: *"only when needed"* é subjetivo; sem critério
  (ex.: "se for modificar arquivo tocado pela iteração N, leia N") o agente cauteloso
  lê tudo (anula a feature) e o econômico nunca lê (decide mal).
- **M-4** — AC diz "001-007", Session Log diz "001-008": descrição inconsistente.
- **M-5** — Custo não medido: feature de performance sem **nenhuma** quantificação de
  tokens. Com N pequeno, a seção Engramas (lida toda sessão) pode custar **mais** que
  ler `history/` uma vez por resume. Break-even não analisado.
- **M-6** — `resume-session` ainda lê **toda** a `methodology/` (`resume-session.md:48`,
  item 3); os próprios Blockers admitem que é custo igual ou maior, deixado de fora.
  Otimização parcial cujo maior dreno permanece intocado.

---

## O que ficou bom (para ser justo)

- Estrutura da tabela Engramas (`# / Resumo / Decisões-chave / Aprendizados`,
  vinculada ao History Index pelo `#`) é limpa; *progressive disclosure* é a ideia certa.
- Documentação propagada de forma coerente entre `overview.md`, `AGENTS.md` e os
  prompts-fonte (dentro do subconjunto tocado).
- Backward-compat preservada; diffs cirúrgicos, sem reescritas.
- População retroativa de 001–007 feita (com as ressalvas de G-2).

---

## Ações priorizadas para liberar o fechamento

1. **(Bloqueante B-1)** Sincronizar os **4 prompts** para `.cursor/rules/`,
   `.claude/commands/`, `.opencode/commands/` (preservar frontmatter `.mdc`).
2. **(Bloqueante B-2)** Entregar a investigação de modelos de sumarização exigida,
   com tabela alternativas × motivo — ou registrar honestamente o descarte e o porquê.
3. **(Grave G-1)** Traduzir o boilerplate Engramas do **template** para inglês;
   corrigir a Quality Check falsa.
4. **(Grave G-2/G-3)** Reconciliar instrução × dados: fonte da verdade única entre
   "synthesize from Description" e "map-reduce sobre Session Log"; corrigir "append"
   vs. ordem newest-first.
5. **(Grave G-4)** Endereçar (ou documentar explicitamente como limitação) o
   crescimento ilimitado da tabela Engramas — rollup hierárquico / tiers.
6. **(Menor)** Separar fechamento do 007; corrigir `1.5.`; adicionar heurística de
   drill-down; reconciliar "001-007"/"001-008".

## Open questions para o desenvolvedor

- Os ports (B-1) devem ser sincronizados nesta mesma iteração 008 ou numa de
  follow-up dedicada?
- A investigação de alternativas (B-2) é requisito de aceitação ou pode ser
  formalmente dispensada (registrando a dispensa)?
- Compactação de segundo nível (G-4) entra agora ou vira backlog explícito na
  metodologia?

## Referências

- Iteração revisada: `008-engramas-compiled-memory.md`
- Especificação original: `impl.md` (raiz do repo)
- Regra de sync violada: `project-definition.md:144`, `AGENTS.md`, `CLAUDE.md`
