# kekchat
Sala de chat utilizando protocolo WebSocket.

![Tela do Chat](/kekchat.png "kekchat")

Este projeto foi feito como avaliação para uma disciplina de Sistemas Distribuídos e visa o aprendizado de arquiteturas cliente-servidor, protocolo WebSocket e programação parelela.

## Requisitos
1. [Node.js](https://nodejs.org/en/) versão 16 ou superior
2. Yarn versão 1.22 ou superior. Basta executar o seguinte comando após ter o Node.js instalado:
`npm install --global yarn`
3. [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) versão 1.59 ou superior.

## Instruções
1. Tendo os requisitos acima, acesse a pasta raiz do projeto e utilize o Yarn para instalação das dependências
`yarn`
2. Após isso, configure o IP onde deseja abrir o servidor no arquivo
`server/util/ip.json`
3. Execute o seguinte comando para abrir o servidor WebSocket
`yarn s-run`
4. Em uma segunta aba ou janela do terminal, execute os seguintes comandos para compilar e servir a aplicação do cliente
`yarn c-build`
`yarn c-start`
5. Se tudo ocorreu corretamente, você deve conseguir acessar o chat através do seu navegador usando o IP configurado e porta 8080. Caso tenha configurado um endereço aberto à rede local, outras máquinas também conseguirão acessar o chat.
