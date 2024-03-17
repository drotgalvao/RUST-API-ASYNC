#!/bin/bash

# Caminho para o arquivo init.sql
INIT_SQL_FILE="/docker-entrypoint-initdb.d/init.sql"

# Verifica a data de modificação do arquivo
LAST_MODIFIED=$(stat -c %Y "$INIT_SQL_FILE")

while true; do
    sleep 1
    NEW_MODIFIED=$(stat -c %Y "$INIT_SQL_FILE")
    if [ "$NEW_MODIFIED" != "$LAST_MODIFIED" ]; then
        # Executa o comando psql
        psql -U postgres -d mydatabase -f "$INIT_SQL_FILE"
        # Atualiza a data de modificação
        LAST_MODIFIED=$NEW_MODIFIED
    fi
done
