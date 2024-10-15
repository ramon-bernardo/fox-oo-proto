# Fox Object-Oriented Proto

Um projeto projetado para demonstrar uma abordagem orientada a objetos para gerenciar tipos de tiles. Ele define traits e implementações-chave para diferentes tipos de tiles, como HouseTile, permitindo uma interação eficiente, mutabilidade e casting de tipos seguro. Este sistema estabelece a base para estruturas mais complexas, expandindo além de tiles para outros objetos de jogo no futuro.

## Conceito
Este projeto é inspirado nos princípios de design orientado a objetos encontrados em [The Forgotten Server](https://github.com/otland/forgottenserver), particularmente seu gerenciamento de entidades de jogo, como criaturas, tiles e contêineres. Semelhante ao sistema para gerenciar "things" este projeto explora como os traits do Rust podem substituir hierarquias tradicionais de objetos, tornando o sistema mais flexível e eficiente.

Ao aproveitar as abstrações de custo zero do Rust, o projeto minimiza a sobrecarga em tempo de execução enquanto mantém garantias estritas em tempo de compilação. Ele substitui o uso tradicional de herança por traits.