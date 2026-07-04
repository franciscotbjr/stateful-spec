---
status: draft
title: Avaliação ex post da persona SDD (naturalista, ~5 iterações após a adoção)
origin: iteration 022 (sdd-persona-adoption, O-009) — decisão D8
---

A persona adotada na iteração 022 (`.stateful-spec/persona.md`) declara, pela própria régua
(§8 do documento de referência), que sua eficácia é **INCONCLUSIVO — hipótese ex ante com
fundamentação citada, aguardando avaliação naturalista**. Este item é o caminho de resolução
desse INCONCLUSIVO: sem ele, a declaração viraria permanente silenciosa.

**Questão:** após ~5 iterações operando sob a persona (≈ iteração 027), os sinais ex post
naturalistas do §8 indicam que ela paga o próprio custo?

**Sinais a colher (do §8 do `persona-reference.md`):**

- **Sensor de confusão (C4) em tendência** — as regiões onde o agente hesitou/errou repetidamente
  diminuíram após intervenções de curadoria, ou só mudaram de lugar?
- **Custo do contexto** — tokens por sessão para o mesmo tipo de tarefa (a curadoria por subtração
  deveria reduzi-los).
- **Retrabalho por spec drift** — iterações reabertas por spec obsoleta.
- **Sobrevivência das decisões** — decisões registradas (Decisions Made) que um leitor futuro
  entendeu e manteve/reverteu sem perguntar ao autor.
- **O Sensor disparou?** — os checklists §6 registrados nos Session Logs das iterações 022+
  existem e reprovaram algo (um checklist que nunca reprova é suspeito de não estar sendo aplicado).

**Próximo passo:** permanece `draft` até existirem ≥5 iterações de sinal (≈027); então shape
(flip para `ready`) e triagem — o desfecho pode ser manter, ajustar ou remover a persona
(curadoria é subtração tanto quanto adição).
