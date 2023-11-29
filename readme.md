# Vitrine API

**Vitrine API** é uma aplicação REST desenvolvida em Rust, Rocket e Diesel para gerenciar produtos, departamentos e marcas.

## Estrutura do Projeto

O código está organizado em diversos módulos, cada um desempenhando um papel específico:

- **router**: Define as rotas da API.
- **models**: Contém os modelos de dados.
- **schema**: Define o esquema do banco de dados.
- **ops**: Engloba as operações de banco de dados.
- **args**: Define os argumentos para as operações de banco de dados.

## Como Executar

Siga os passos abaixo para executar o projeto localmente:

1. Clone o repositório.
2. Certifique-se de ter Rust, Rocket e Diesel instalados.
3. Navegue até o diretório do projeto.
4. Crie um arquivo chamado `.env` na raiz do projeto e adicione a seguinte linha, substituindo as informações apropriadas: DATABASE_URL=postgresql://postgres:sua_senha@localhost:5432/seu_banco_de_dados
5. Execute `diesel setup` para configurar o banco de dados.
6. Execute `diesel migration run` para aplicar as migrações do banco de dados.
7. Execute `cargo run` para iniciar o servidor.

## Rotas da API

A API oferece diversas rotas para interação. Aqui estão algumas delas:

- **GET /**: Retorna a página inicial.
- **GET /products**: Obtém todos os produtos.
- **GET /products/:id**: Obtém informações sobre um produto específico.
- **POST /department**: Cria um novo departamento.
- **PUT /department**: Atualiza um departamento existente.
- **DELETE /department**: Exclui um departamento.
- **GET /department**: Retorna informações sobre um departamento específico.
- **GET /departments**: Obtém todos os departamentos.
- **POST /brand**: Cria uma nova marca.
- **PUT /brand**: Atualiza uma marca existente.
- **DELETE /brand**: Exclui uma marca.
- **GET /brand**: Retorna informações sobre uma marca específica.
- **GET /brands**: Obtém todas as marcas.

Sinta-se à vontade para explorar e utilizar essas rotas para gerenciar os dados da aplicação.