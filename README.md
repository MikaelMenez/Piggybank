# Piggybank
Esta API HTTP, construída com Axum e SQLite, permite criar, listar, atualizar e deletar transações financeiras simples.
Cada transação possui um tipo (tipo) e um valor (valor), armazenado em centavos no banco, mas exposto em reais (ex.: 10.99).
Modelo de dados

Tabela transactions no SQLite:

    id: INTEGER PRIMARY KEY AUTOINCREMENT

    tipo: TEXT NOT NULL

    valor: INTEGER NOT NULL (valor em centavos, ex.: R$ 10,99 → 1099)

Structs em Rust:

    Transaction (interno / banco):

        id: Option<i64>

        tipo: String

        valor: i64 (centavos)

    Transactionjson (JSON da API):

        id: Option<i64>

        tipo: String

        valor: f64 (reais, ex.: 10.99)

Rotas
GET /

    Descrição: Inicializa a tabela transactions (se ainda não existir) e responde com uma mensagem simples.

    Resposta:

        200 OK com corpo: "hello world" em texto puro.

POST /add_transaction

    Descrição: Cria uma nova transação.

    Body (JSON - Transactionjson):

    json
    {
      "tipo": "string",
      "valor": 10.99
    }

        id é opcional e ignorado na inserção.

        valor é recebido em reais e convertido para centavos (valor * 100.0 → i64) antes de salvar.

    Comportamento:

        Garante a existência da tabela com create_table_transactions.

        Insere: INSERT INTO transactions (tipo, valor) VALUES (?, ?).

    Respostas:

        201 CREATED em caso de sucesso.

        500 INTERNAL_SERVER_ERROR com mensagem "Erro ao criar membro: {erro}" em caso de falha no banco.

GET /transactions

    Descrição: Lista todas as transações.

    Comportamento:

        Consulta: SELECT * FROM transactions com sqlx::query_as!(Transaction, ...).

        Converte cada Transaction em Transactionjson, dividindo o valor por 100:

            valor: x.valor as f64 / 100.0 (centavos → reais).

    Resposta:

        200 OK com JSON:

        json
        [
          {
            "id": 1,
            "tipo": "entrada",
            "valor": 10.99
          },
          {
            "id": 2,
            "tipo": "saida",
            "valor": 5.50
          }
        ]

        Em caso de erro de banco:

            500 INTERNAL_SERVER_ERROR com mensagem "Erro ao buscar membros {erro}".

PUT /modify_transaction/{id}

    Descrição: Atualiza uma transação existente pelo id.

    Parâmetros:

        Path: {id} (inteiro, ex.: /modify_transaction/1).

    Body (JSON - Transactionjson):

json
{
  "tipo": "string",
  "valor": 10.99
}

Comportamento:

    Garante a existência da tabela.

    Atribui o id do path ao corpo: membro.id = Some(path).

    Converte valor de reais para centavos (valor *= 100.0, depois as i64).

    Executa:

        sql
        UPDATE transactions
        SET tipo = ?, valor = ?
        WHERE id = ?

        com os campos tipo, valorint, id.

    Respostas:

        201 CREATED em caso de atualização bem-sucedida.

        500 INTERNAL_SERVER_ERROR com mensagem "Erro ao criar membro: {erro}" em caso de erro.

DELETE /delete_transaction/{id}

    Descrição: Remove uma transação pelo id.

    Parâmetros:

        Path: {id} (ex.: /delete_transaction/1).

    Comportamento:

        Garante a existência da tabela.

        Executa: DELETE FROM transactions WHERE id = ?.

    Respostas:

        201 CREATED em caso de remoção bem-sucedida (semântica de status simples mas funcional).

        500 INTERNAL_SERVER_ERROR com mensagem "Erro ao criar membro: {erro}" em caso de falha.

Execução do servidor

    Endereço de escuta: 0.0.0.0:3000.

    URL de acesso exibida no console: http://127.0.0.1:{porta}/ (por padrão, http://127.0.0.1:3000/).

    Inicialização do pool SQLite: SqlitePool::connect("sqlite:app.db").
