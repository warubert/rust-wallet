# 💰 Rust Wallet

Uma aplicação web de carteira digital construída em Rust, permitindo que usuários gerenciem seus ativos financeiros — comprem, acompanhem e visualizem a variação de valor dos seus investimentos.
Construido para o deasfio do bootcamp Santander 2026 - Rust AI Developer

---

## 📌 O que o projeto faz

A **Rust Wallet** é uma aplicação full-stack que permite:

- **Cadastro e autenticação** de usuários com login automático no primeiro acesso
- **Listagem de ativos** disponíveis para compra (ex: Bitcoin, Ethereum)
- **Compra de ativos** com registro do preço e quantidade no momento da aquisição
- **Visualização da carteira** com histórico detalhado de compras e cálculo de variação de valor em relação ao preço atual
- **Gerenciamento de ativos** (criação e edição) exclusivamente por usuários com role **admin**, com verificação do role no JWT
- **API REST** para integração externa com os ativos cadastrados

---

## 🚀 Como executar a aplicação

### Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) (edição 2024)
- [Docker](https://docs.docker.com/get-docker/) e Docker Compose
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli): `cargo install sqlx-cli`

### Passo a passo

**1. Clone o repositório**

```bash
git clone https://github.com/warubert/rust-wallet.git
cd rust-wallet
```

**2. Suba o banco de dados**

```bash
docker compose up -d
```

**3. Configure as variáveis de ambiente**

Crie um arquivo `.env` na raiz do projeto:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
```

**4. Execute as migrations**

```bash
sqlx migrate run
```

**5. Inicie a aplicação**

```bash
cargo run
```

A aplicação estará disponível em: **http://localhost:3000**

---

## 🔐 Usuário administrador

A aplicação possui um sistema de roles: `user` (padrão) e `admin`.

Somente usuários com role **admin** podem **criar** e **editar** ativos. Para usuários comuns, esses botões aparecem desabilitados na interface.

Um usuário administrador é criado automaticamente na primeira inicialização da aplicação:

| Campo    | Valor   |
| -------- | ------- |
| Username | `admin` |
| Senha    | `admin` |

> ⚠️ Recomenda-se alterar a senha do usuário `admin` em ambientes de produção.

O role do usuário é armazenado no JWT e verificado nos endpoints de criação e edição de ativos — tanto nas rotas da interface web quanto na API REST.

---

## 🛠️ Tecnologias utilizadas

| Tecnologia        | Uso                                                            |
| ----------------- | -------------------------------------------------------------- |
| **Rust**          | Linguagem principal                                            |
| **Axum**          | Framework web assíncrono                                       |
| **Tokio**         | Runtime assíncrono                                             |
| **SQLx**          | Acesso ao banco com queries verificadas em tempo de compilação |
| **PostgreSQL**    | Banco de dados relacional                                      |
| **Askama**        | Renderização de templates HTML                                 |
| **JWT Simple**    | Autenticação via JSON Web Tokens                               |
| **password-auth** | Hash seguro de senhas                                          |
| **axum-extra**    | Suporte a cookies assinados                                    |
| **Docker**        | Containerização do banco de dados                              |
| **Insta**         | Snapshot testing                                               |
| **tracing**       | Logging e instrumentação                                       |

---

## ✨ Melhorias implementadas

Foi implementada a **experiência de criação e edição de ativos via modais** na interface web.

Em vez de redirecionar o usuário para páginas separadas, as ações de criar um novo ativo e editar um existente são realizadas diretamente na página `/assets` por meio de janelas modais. Isso torna o fluxo mais fluido e evita recarregamentos desnecessários de página, melhorando a experiência do usuário sem abrir mão da simplicidade do HTML + formulários nativos.

Também foi implementado um **sistema de roles** para controle de acesso:

- Todo usuário criado automaticamente recebe o role `user`, que permite visualizar e comprar ativos, mas não criar ou editá-los.
- O role `admin` é necessário para criar e editar ativos — tanto pelos endpoints da API REST quanto pela interface web.
- O role é embutido no JWT no momento do login e verificado nos endpoints protegidos, sem necessidade de consulta adicional ao banco.
- Na interface, os botões de criação e edição ficam visualmente desabilitados para usuários sem permissão.

---

## 🧪 Como testar

Os testes utilizam **`sqlx::test`** (banco isolado por teste) e **Insta** para snapshot testing.

**Executar todos os testes:**

```bash
cargo test
```

**Revisar ou atualizar snapshots:**

```bash
cargo insta review
```

Os testes cobrem:

- `test_create_asset` — criação de um ativo via handler
- `test_list_assets` — listagem de ativos com fixture
- `test_update_asset` — atualização de nome e valor de ativo

Os snapshots ficam em `src/routes/snapshots/` e garantem que o JSON retornado pela API não mude de forma inesperada.

---

## 📚 O que aprendi durante o desafio

- **Ownership e borrow checker na prática**: Trabalhar com Rust forçou um entendimento profundo de como o compilador gerencia memória, principalmente ao lidar com extractors do Axum que consomem ou emprestam dados da requisição.

- **Async/await com Tokio**: Aprendi como estruturar código assíncrono em Rust, usando `try_join!` para paralelizar queries ao banco e entendendo como o runtime do Tokio gerencia as tasks.

- **SQLx com verificação em tempo de compilação**: As queries são verificadas contra o schema real do banco durante a compilação — o que elimina uma categoria inteira de erros em runtime, mas exige que o banco esteja disponível no ambiente de desenvolvimento.

- **Extratores customizados no Axum**: Implementar `FromRequestParts` para os tipos `User`, `Option<User>`, `Admin` e `Repository` ensinou como o Axum desacopla a extração de dados da lógica de negócio dos handlers.

- **Template rendering com Askama**: Como integrar templates HTML fortemente tipados com filtros customizados (como `human_datetime`) diretamente na aplicação Rust.

- **Snapshot testing com Insta**: Uma abordagem eficiente para testar saídas JSON de APIs sem escrever assertions manuais para cada campo.
