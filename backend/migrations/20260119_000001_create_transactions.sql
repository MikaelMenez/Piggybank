-- Up (cria tabela)
CREATE TABLE IF NOT EXISTS transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tipo TEXT NOT NULL,
    valor INTEGER NOT NULL,
    data TEXT NOT NULL
);

-- Down (remove tabela)
DROP TABLE IF EXISTS transactions;
