# 025 — Relatório do M1 (marcador de versão)

> **Nota de método.** O Implement (tarefas a, b e c) e o Verify do M1 foram feitos em 2026-10-05 numa
> única sessão, sem o developer presente e com autorização para comitar ao fim de cada fase. Toda a
> verificação é **ex ante e artificial**: revisão do diff, buscas com grep e raciocínio sobre cenários
> de uso. Nada foi testado num projeto adotante real. Ficam **INCONCLUSIVOS**: (1) se o marcador
> atende o Workspace, o que só o cenário de ponta a ponta do M7 mede; (2) se um agente segue os novos
> passos dos wizards numa execução real, porque nenhum wizard foi executado. Este relatório resume a
> iteração; a fonte completa continua sendo `history/025-version-marker.md` (Decisions Made S1–S4b,
> Session Log e Blockers & Notes).

## 1. Resumo

O M1 do O-010 acrescenta um marcador de versão da metodologia: o arquivo
`.stateful-spec/methodology-version.md` registra no campo `methodology_version` a release do Stateful
Spec de onde vieram a metodologia e os prompts de operação de um projeto. Os três wizards de
inicialização gravam o marcador, o `update-project` também o lê, o `overview.md` documenta a regra e
este repositório passou a ter o próprio marcador (`2.0.0`). O diff de produto tem 8 arquivos, com +36
e −4 linhas. Os 7 critérios de aceite foram verificados, o checklist §6 deu 7/7 PASS e a iteração
está em `review`, à espera da sua aprovação.

## 2. Commits

Todos estão na branch `feat/multi-context`. Não fiz push nem abri PR.

| Commit | Fase | Conteúdo |
|--------|------|----------|
| `bd55295` | Implement (a) | Template do marcador, marcador deste repositório e regra Release Cut |
| `337808f` | Implement (b) | Os três wizards de `prompts/initialization/` |
| `05f2e1f` | Implement (c) | `overview.md` e `CHANGELOG.md` |
| `718abf8` | Verify | Critérios e Quality Checks com âncoras, AC1, §6 e uma correção no `update-project` |
| `b14c6e6` | save-session | Status `review`, Engrama 025, Active Work e History Index |

## 3. O que mudou, por arquivo

As âncoras abaixo valem no `HEAD` deste relatório.

| Arquivo | Mudança | Spec · Decisão |
|---------|---------|----------------|
| `templates/project/methodology-version.md` (novo) | Frontmatter `methodology_version: {{METHODOLOGY_VERSION}}` e uma linha de corpo: o que o arquivo registra, quem o grava, que não se edita à mão e onde fica a regra (`:1-5`) | 1 · S1, S1b |
| `.stateful-spec/methodology-version.md` (novo) | `methodology_version: 2.0.0`; o corpo diz que este arquivo é reescrito a cada corte de release (`:1-5`) | 6 · S3c |
| `.stateful-spec/project-definition.md` | Regra **Release Cut** no Deployment (`:157`); o marcador entrou na árvore (`:56`, `:73`) | 7 · S3c |
| `prompts/initialization/new-project.md` | Item 7 do "Always create": cria o marcador com a versão lida do `CHANGELOG.md` da origem; sem `X.Y.Z` válido, não cria e avisa (`:317`) | 2 · S2, S2b, S3, F1 |
| `prompts/initialization/onboard-existing.md` | O mesmo item 7 (`:164`). No setup parcial, só grava o marcador se a mesma execução copia `methodology/`; caso contrário, aponta para o `update-project` (`:166`) | 3 · S3, F2 |
| `prompts/initialization/update-project.md` | O STEP 0 define a versão da origem (`:54`); o inventário do STEP 2 ganhou a linha "Methodology version" (`:110`); o STEP 4 mostra a transição (`:161`); o item 5 do STEP 6 grava só nos escopos 2 e 3, mesmo quando a versão é menor que a atual (`:214-216`) | 4 · S3, S3b |
| `methodology/overview.md` | O marcador entrou na árvore (`:127`) e em Key Files (`:154`), e ganhou a subseção *Methodology Version* (`:158-165`). A R6 não aparece | 5 · S4, S4b |
| `CHANGELOG.md` | Entrada em `[Unreleased]` → `Added` (`:11`) | 8 · AC7 |

## 4. Como o mecanismo funciona

- **Fonte da versão:** o primeiro `## [X.Y.Z]` do `CHANGELOG.md` da origem, pulando `## [Unreleased]`.
  O marcador registra uma **linha de release**, não o conteúdo exato. Por isso, até o corte da 3.0.0,
  um projeto configurado a partir da `main` grava `2.0.0`, mesmo já trazendo o marcador e o restante
  do `[Unreleased]` (S2b, limitação declarada).
- **Quem grava:** `new-project`, `onboard-existing` e `update-project`, este último só nos escopos 2
  e 3. Nenhum deles grava sem ter lido um `X.Y.Z` válido. Neste repositório, quem grava é a regra
  Release Cut, aplicada no mesmo commit que renomeia o `[Unreleased]`.
- **Quem lê:** no M1, só o `update-project`, no inventário (STEP 2) e na transição (STEP 4). Nenhum
  prompt de operação lê o marcador. O add-member (M4) e a R6 (M2) serão os próximos leitores.
- **Sem marcador:** o projeto conta como ≤ 2.0.0. A comparação é numérica por `MAJOR.MINOR.PATCH`, e
  a ausência de marcador fica abaixo de qualquer marcador.

## 5. Verificação

**Critérios de aceite:** 7/7. Cada um está marcado na 025 com a âncora que o comprova.

**AC1, sem regressão (PASS, ex ante e artificial).** O grep mostra que nenhum arquivo de
`prompts/operations/` lê o marcador. Para um projeto adotado na 2.0.0, sem marcador:

1. As operações do dia a dia (`resume-session`, `save-session` etc.) não mudam.
2. O `update-project` no escopo 1 mostra "no marker (≤ 2.0.0)" e a transição `no marker → 2.0.0`,
   avisa que o marcador fica inalterado e traz uma subseção nova no `overview.md` que nenhum passo
   usa.
3. Nos escopos 2 e 3, o `update-project` cria um arquivo novo com `2.0.0`.
4. O `onboard-existing` pula para o STEP 5 e não grava nada.

O `new-project` passa a criar um arquivo a mais.

**Checklist §6 da persona (passo separado):** 7/7 PASS, com dois riscos declarados:

- As âncoras das decisões S1–S4b foram medidas em `74c393e`, e várias mudaram com o próprio Implement
  (por exemplo, `CHANGELOG.md:63` → `:64`).
- "Nunca gravar sem ler", "não editar à mão" e a regra Release Cut são **Guides sem Sensor
  determinístico**, porque o repositório não tem CI por desenho. O único Sensor é você ler a transição
  no STEP 4; neste repositório, conferi à mão que o marcador `2.0.0` bate com `CHANGELOG.md:64`.

## 6. Defeitos achados nas críticas e corrigidos

| Fase | Defeito | Correção |
|------|---------|----------|
| Implement (a) | O corpo do template dizia "last refreshed from", o que fica falso depois de um `update-project` de escopo 1, que não grava o marcador | O texto passou a ser "methodology and operation prompts were set up or last refreshed from" |
| Implement (b) | O STEP 4 do `update-project` mostrava `2.0.0 → 3.0.0` também nos escopos 1, 4 e 5, em que o marcador não muda | O passo agora diz que, nesses escopos, o marcador fica inalterado |
| Verify | O STEP 4 usava "version" em dois sentidos: "read the source version … at the chosen version" | Passou a dizer "from the chosen tag, branch, commit, or folder" |

## 7. Escolhas feitas sem você (para revisar)

1. **Status `review` em vez de `done`.** O `save-session` manda marcar `done` quando os critérios são
   atendidos, mas a aprovação é sua e você pediu este relatório para analisar. O `end-session` fecha a
   iteração depois da sua revisão. Como a sessão continua aberta, a lacuna do draft
   `intake/Backlog/review-iteration-lifecycle.md` não é acionada.
2. **O corpo do marcador deste repositório diverge do template.** Aqui quem grava é a regra Release
   Cut, então "written by the initialization wizards — do not edit by hand" seria falso. A divergência
   é só deste repositório, como a seção Persona do `AGENTS.md`.
3. **Posição do item nos wizards.** O marcador entrou como item 7, depois do `AGENTS.md`, e não ao
   lado da cópia de `methodology/` (item 3), para não renumerar a lista.
4. **Onde a regra de leitura é definida no `update-project`.** A definição fica uma única vez no STEP
   0, e o STEP 4 a aplica à versão escolhida. A mesma regra aparece também no `new-project`, no
   `onboard-existing` e no `overview.md`, porque os wizards precisam funcionar sem a metodologia
   carregada.
5. **Este relatório** é um arquivo auxiliar em `history/`, e não um arquivo no scratchpad (lição da
   018). Ele vai para `.archived/` quando a 025 fechar. Se não quiser guardá-lo, um `git rm` resolve.

## 8. Riscos e pontos em aberto (não corrigidos)

- **A regra da F2 é mais estreita que a S3.** Um setup parcial que copia um `methodology/` ausente mas
  mantém prompts de operação antigos grava a versão atual sobre prompts antigos. É um caso raro, que
  aceitei como risco; dá para ampliar a regra se você quiser.
- **O caminho de setup parcial do `onboard-existing` talvez nunca seja alcançado.** O STEP 1
  (`onboard-existing.md:52-55`) pula para o STEP 5 quando `.stateful-spec/` existe. A ambiguidade é
  anterior ao M1.
- **Árvore desatualizada no `project-definition.md`.** A árvore de `.stateful-spec/` não lista
  `persona.md` nem `persona-reference.md`, que vieram da 022. É anterior ao M1 e não mexi.
- **Avaliação.** A utilidade do marcador para o Workspace segue INCONCLUSIVA até o M7.

## 9. O que depende de você

1. **Aprovar o M1** e rodar o `end-session`, que fecha a 025 em `done`. Ou pedir mudanças, e a 025
   continua aberta.
2. **Decidir os pontos da seção 7** que quiser reverter e **a regra da F2** (seção 8).
3. **Push e PR** da `feat/multi-context`, que dependem da sua autorização.
4. **Próximo milestone:** o M2 (`methodology/workspace.md`, incluindo a R6, conforme a nota em
   Key Decisions do `memory.md`).
5. O corte e a tag da 3.0.0 continuam dependendo da sua decisão e só acontecem depois do M7.
