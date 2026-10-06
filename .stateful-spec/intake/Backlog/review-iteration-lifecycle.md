---
status: draft
title: Ciclo de vida de sessão para uma iteração pausada em `review`
origin: iteração 024 (end-session fechou em `review`; start-session reabriu), 2026-10-02
---

As operações de sessão não definem o tratamento de uma iteração fechada em `review` (critérios
não atendidos). A iteração 024 decidiu cada ponto por julgamento (ver *Decisions Made* da 024):

- **`end-session`**: se a iteração em `review` fica em Active Work ou vai para Recent Completions.
- **`methodology/history-archiving.md`**: se `review` conta como iteração **fechada** para
  `RAW_HISTORY`. Se não contar e outra iteração for aberta, a operação de arquivamento moveria a
  iteração pausada para `.archived/`, já que ela não está nem fechada nem aberta.
- **`start-session`**: os STEPs 3–4 só criam um `NNN` novo; não há caminho para reabrir uma
  iteração em `review` como Open Session (a 024 foi reaberta sem número novo).
- **`methodology/history-archiving.md`** (auxiliares): a operação mantém só as auxiliares do
  **milestone corrente** da iteração aberta (`history-archiving.md:76`). Uma iteração sem milestones
  não tem milestone corrente; ao pé da letra, uma reabertura (que roda a operação) arquivaria uma
  auxiliar ainda em uso, como `024-multi-repo-workspace-analysis.md`. A operação também não reaponta
  o link da auxiliar no arquivo central, só a célula `File` do History Index. A 024 tratou a
  auxiliar como corrente enquanto a iteração estiver aberta (2026-10-04). No fechamento (2026-10-05),
  a auxiliar foi para `.archived/` antes da central, e o link dela de volta para a central só
  resolve quando a central também for arquivada.

**Para moldar antes de promover:** se o caminho de reabertura pertence ao `start-session` ou ao
`resume-session`; o que fazer com a linha de Engrama na reabertura (manter ou voltar a
`_In progress_`; a 024 a recompilou pelo map-reduce do `save-session`); e a regra de sync
(prompt-fonte + 3 ports).
