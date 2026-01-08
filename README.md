Piggybank API

API HTTP construída com Rust + Axum + SQLite para gerenciamento completo de transações financeiras. Valores são armazenados em centavos no banco (1099 = R$ 10,99) e expostos em reais nas respostas JSON.
Tabela transactions

text
id              INTEGER PRIMARY KEY AUTOINCREMENT
tipo            TEXT NOT NULL              (entrada, lazer, saida, supermercado...)
valor           INTEGER NOT NULL           (centavos: 1099 = R$ 10,99)
data            TEXT NOT NULL              (YYYY-MM-DD HH:MM:SS via datetime('now'))

Structs

    CreateTransaction: tipo: String, valor: f64 (input JSON)

    Transaction: id, tipo, data, valor: i64 (banco)

    Transactionjson: id, tipo, data, valor: f64 (resposta JSON)

Rotas da API
POST /add_transaction

Cria nova transação. Data é automática (datetime('now')).

bash
curl -X POST http://localhost:3000/add_transaction \
  -H "Content-Type: application/json" \
  -d '{"tipo": "lazer", "valor": -23.57}'

Status: 201 Created
GET /transactions

Lista todas transações.

bash
curl http://localhost:3000/transactions

GET /transactions/by_tipo/{tipo}

Filtra por tipo (ex: "lazer", "entrada").

bash
curl http://localhost:3000/transactions/by_tipo/lazer

GET /transactions/by_date/{mes}/{ano}

Filtra por mês/ano (ex: /by_date/1/2026 → "2026-01").

bash
curl http://localhost:3000/transactions/by_date/1/2026

PUT /modify_transaction/{id}

Atualiza transação por ID.

bash
curl -X PUT http://localhost:3000/modify_transaction/1 \
  -H "Content-Type: application/json" \
  -d '{"tipo": "entrada", "valor": 150.00}'

DELETE /delete_transaction/{id}

Remove transação por ID.

bash
curl -X DELETE http://localhost:3000/delete_transaction/1

GET /

Health check.

bash
curl http://localhost:3000/

Resposta: "hello world"
Status Codes

    200 OK: Sucesso (GET)

    201 Created: Sucesso (POST/PUT/DELETE)

    422 Unprocessable Entity: JSON inválido

    500 Internal Server Error: Erro no banco

Endereço

text
http://127.0.0.1:3000/

Dados de Teste

sql
INSERT INTO transactions (tipo, valor, data) VALUES 
('entrada', 15000, '2026-01-08 12:00:00'),
('lazer', -2357, '2026-01-08 14:30:00'),
('saida', -8000, '2025-12-15 10:30:00');

Exemplo de Resposta JSON

json
[
  {
    "id": 1,
    "tipo": "entrada", 
    "valor": 150.00,
    "data": "2026-01-08 12:00:00"
  }
]
