Piggybank API

API HTTP construída com Rust + Axum + SQLite para gerenciamento completo de transações financeiras. Valores são armazenados em centavos no banco (1099 = R$ 10,99) e expostos em reais nas respostas JSON.
📋 Tabela transactions
Campo	Tipo	Descrição
id	INTEGER PRIMARY KEY AUTOINCREMENT	ID único gerado automaticamente
tipo	TEXT NOT NULL	Tipo da transação (entrada, lazer, saida, supermercado...)
valor	INTEGER NOT NULL	Valor em centavos (1099 = R$ 10,99)
data	TEXT NOT NULL	Data/hora (YYYY-MM-DD HH:MM:SS via datetime('now'))
🏗️ Structs

    CreateTransaction (input JSON): tipo: String, valor: f64

    Transaction (banco): id, tipo, data, valor: i64

    Transactionjson (resposta JSON): id, tipo, data, valor: f64

🚀 Rotas da API
POST /add_transaction

Cria nova transação. Data automática.

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

Filtra por tipo ("lazer", "entrada").

bash
curl http://localhost:3000/transactions/by_tipo/lazer

GET /transactions/by_date/{mes}/{ano}

Filtra por mês/ano (/by_date/1/2026 → "2026-01").

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
📊 Status Codes
Código	Descrição
200 OK	Sucesso (GET)
201 Created	Sucesso (POST/PUT/DELETE)
422 Unprocessable Entity	JSON inválido
500 Internal Server Error	Erro no banco
🌐 Endereço

text
http://127.0.0.1:3000/

🧪 Dados de Teste

sql
INSERT INTO transactions (tipo, valor, data) VALUES 
('entrada', 15000, '2026-01-08 12:00:00'),
('lazer', -2357, '2026-01-08 14:30:00'),
('saida', -8000, '2025-12-15 10:30:00');

📋 Exemplo de Resposta JSON

json
[
  {
    "id": 1,
    "tipo": "entrada",
    "valor": 150.00,
    "data": "2026-01-08 12:00:00"
  }
]

