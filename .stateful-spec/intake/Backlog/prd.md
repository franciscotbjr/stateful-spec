---
status: triaged
title: Gestão de contexto com multiplos repositórios.
origin: idea
destination: O-010 → iteration 024 (multi-repo-workspace)
---

# Problema

Na versão atual, a metodologia não permite que o Agente de IA enxerge mais de um repositório ao mesmo tempo.

Precisamos criar um plano detalhado para que o Agente de IA possa ser executado a partir de uma pasta/repositório que contenha uma relação de outros repositórios seguindo ainda assim a metodologia.

A ideia fundamental é permitir o compartilhamento de conhecimento para que o progresso, aprendizado, obtido em repositórios separados permita ganho para todos os quem compõem o workspace.

Como exemplo no projeto "D:\development\public\stand-in" em que vários aprendizados foram adicionados à memória e posteriormente retroalimentado comandos, skills e a própria metodologia.

Além de funcionar como base de conhecimento, nosso objetivo é criar uma indexação de memória mais simplificada: o repositório de contexto central deve conhecer também os indíces dos repositórios mapeados. Assim, não será necessária uma réplica completa da memória das pastas/repositórios, criando tipos ponteiros que o Agente de IA poderá seguir para buscar o conhecimento necessário.

Pesquise na internet para buscar referências sobre esse tipo de abordagem que possam agregar conhecimento já validado/certifica/aplicado ou de reconhecida eficiência e eficácia.

# Critérios de Aceite

- Não poderá haver quebra, se o agente for executado a partir do repositório que já está configurado, a metodologia se mantém como é.
- A lista de repositórios conhecidos poderá estar em YAML ou XML, ou mesmo um formato mais eficiente para a IA, de maneira a formar um Workspace.
- A partir de uma nova sessão nesta pasta/repositório (Workspace), será possível trabalhar em qualquer dos projetos do Workspace.
- A partir de uma nova sessão nesta pasta/repositório (Workspace), será possível criar um novo projeto (pasta/repositório).
- A partir de uma nova sessão nesta pasta/repositório (Workspace), será possível rascunhar uma ideia até que ela se transforme uma ação a ser executada. Criar um comando específico para o processo de rascunho.
- Deverá existir um novo comando para incluir novas pastas/repositórios no contexto. Ao ser executado, esse commando, além de atualizar o arquivo de mapeamento, irá indexar no Workspace os ponteiros para o conhecimento (memory) da pasta/repositório que está sendo incluído.
- Poderá haver, mais de uma sessão ativida, sendo no máximo uma por pasta/repositório que compõe o Workspace.
- O Workspace terá seu próprio AGENTS.md, assim como a configuração da própria metodologia.

# Atenção

Questione-me exaustivamente sobre todos os aspectos deste prompt até chegarmos a um entendimento comum. Percorra todas as seções e subseções e faça todas as perguntas que julgar necessárias para garantir que eu entendi completamente o que você deseja. Percorra cada ramo da árvore de decisções (design), resolvendo as dependências entre decisões uma a uma. Em cada decisão, documente todas as alternativas consideradas e os motivos para a escolha feita. Encerre o ciclo apenas quando tiver certeza de que todas as dúvidas foram sanadas e que chegamos a um entendimento comum.

Faça as perguntas uma de cada vez, aguardando minhas respostas antes de prosseguir para a próxima pergunta.

Se uma pergunta puder ser respondida explorando a base do repositório, faça-o e responda por conta própria, sem me perguntar. Caso contrário, pergunte.
