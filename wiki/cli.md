# CLI `redmine`

`redmine` é o entrypoint de linha de comando do `redmine-wrapper-rs`. Ele cobre
os 22 recursos e os 91 endpoints da biblioteca com uma superfície declarativa,
pensada para consultas rápidas, scripts e lotes.

O binário fica no crate `crates/redmine-cli` e não é publicado no crates.io.

## Instalação

### Release (recomendado)

```bash
curl --proto '=https' --tlsv1.2 -sSf \
  https://raw.githubusercontent.com/st-all-one/redmine-wrapper-rs/main/install.sh | bash
```

O instalador detecta o SO/arquitetura, baixa
`redmine-<versão>-<target>.{tar.gz,zip}` do GitHub Release, verifica o
`sha256sums.txt` e instala em `~/.local/bin`.

```bash
# versão específica e diretório alternativo
curl ... | REDMINE_VERSION=v0.2.1 bash
curl ... | REDMINE_INSTALL_DIR=/usr/local/bin bash
```

### A partir do source

```bash
./install.sh --from-source
# equivalente a:
cargo build --release --locked -p redmine-cli
```

### Desinstalar

```bash
./install.sh --uninstall
```

O binário anterior é movido para `${XDG_CACHE_HOME:-~/.cache}/redmine/trash`
(nada é apagado de forma irreversível). O instalador usa `REDMINE_*` para
configuração; não confunda com `REDMINE_URL`/`REDMINE_TOKEN` do CLI.

```bash
export REDMINE_URL=https://redmine.example.com
export REDMINE_TOKEN=seu-api-token
redmine issues list --params '{"status_id":"open"}'
```

---

## Autenticação e conexão

|Flag|Env|Default|Descrição|
|---|---|---|---|
|`--url <URL>`|`REDMINE_URL`|—|URL base do Redmine|
|`--token <TOKEN>`|`REDMINE_TOKEN`|—|Chave de API|
|`--token-file <ARQ>`|—|—|Lê a chave de um arquivo; precede `--token`|
|`--timeout <SEG>`|`REDMINE_TIMEOUT`|`30`|Timeout por requisição|
|`--max-rps <N>`|`REDMINE_MAX_RPS`|`10`|Limite de requisições por segundo|
|`--switch-user <LOGIN>`|—|—|Header `X-Redmine-Switch-User`|
|`--json`|—|—|Envelope JSON compacto de máquina|

O token é obrigatório em tempo de execução. `--token-file` vence `--token`, que
vence `REDMINE_TOKEN`. A URL e o token são validados antes de qualquer chamada
(exit 2 quando ausentes).

---

## Contrato de entrada

### Posicional é conteúdo

O único argumento posicional é a **consulta** de `search`:

```bash
redmine search "falha ao salvar"
redmine search -              # lê a consulta do stdin
```

Todos os metadados são flags nomeadas: `--id`, `--project`, `--title`,
`--user`, `--file`, `--filename`, `--description`, `--version`,
`--reassign-to`, `--method`, `--path`.

As flags de metadados não são marcadas como obrigatórias no `--help` porque um
lote pode fornecê-las por linha. A validação acontece ao resolver a requisição:
`issues get` sem `--id` falha com exit 2.

### `--params`

`--params '<JSON>'` é sempre um objeto JSON:

- leituras e remoções (`GET`/`DELETE`): filtros de query string;
- criações e atualizações (`POST`/`PUT`/`PATCH`): corpo da requisição;
- `--params -` lê o JSON do stdin;
- em escritas, sem `--params` e com stdin não-TTY, o corpo vem do stdin
  (heredoc/pipe). Stdin vazio é erro.

Query string é achatada: escalares viram `chave=valor`, arrays de escalares
viram `chave[]=valor`, objetos aninhados viram `chave[sub]=valor`, `null` é
omitido e booleanos viram `1`/`0`.

Listas paginadas (`list`, `search`, etc.) buscam todas as páginas por padrão.
Se `--params` incluir `limit` ou `offset`, a CLI faz **uma única requisição** e
devolve apenas aquela página. Use `limit` em buscas amplas:

```bash
redmine search teste --params '{"limit":5}'
redmine issues list --params '{"limit":5,"offset":10}'
```

```bash
# listar com filtros
redmine issues list --params '{"status_id":"open","assigned_to_id":"me"}'

# criar a partir de heredoc (o corpo é embrulhado em {"issue": ...})
redmine issues create <<'JSON'
{"project_id":1,"subject":"Falha","description":"Passos..."}
JSON

# atualizar lendo o corpo do stdin
printf '{"notes":"Resolvido"}' | redmine issues update --id 42
```

### `--batch`

`--batch <ARQUIVO|->` lê JSONL: uma linha, um objeto. Chaves reservadas:

|Chave|Tipo|Uso|
|---|---|---|
|`id`|string ou número|ID do recurso|
|`project`|string ou número|Projeto|
|`title`|string|Título de wiki|
|`user`|string ou número|Usuário|
|`path`|string|Caminho em `api`|
|`query`|string|Consulta em `search`|
|`params`|objeto|Filtros ou corpo|

Chaves desconhecidas são erro **daquela linha**. O lote é best-effort: todas as
linhas são tentadas, cada resultado vira uma linha NDJSON
`{"index":N,"ok":true,"data":...}` ou `{"index":N,"ok":false,"error":{...}}`, e
o processo termina com exit 1 se qualquer item falhar. Teto de 1000 linhas.
`--batch` é exclusivo com `--params`.

```bash
redmine issues get --batch - <<'JSONL'
{"id":1}
{"id":2}
{"id":3}
JSONL
```

### `--dry-run`

Não envia nada: imprime `{"dry_run":true,"request":{method,path,query,body}}`.
Em lote, cada linha traz seu próprio `request`.

---

## Saída e exit codes

- stdout = dados; stderr = erros e avisos;
- sem `--json`, dados em JSON pretty; `null` é impresso como `ok`;
- com `--json`, envelope compacto
  `{"success":true,"command":"issues.get","data":...}` ou
  `{"success":false,"command":"...","error":{...}}`;
- `--batch` sempre em NDJSON;
- `EPIPE` (`| head`) termina em 0.

|Exit|Significado|
|---|---|
|0|sucesso|
|1|algum item de lote falhou|
|2|uso, JSON ou configuração inválidos|
|3|recurso não encontrado (404)|
|4|conflito/validação (409/422)|
|5|IO, rede, timeout ou rate limit|
|6|autenticação/autorização (401/403)|
|70|erro interno ou resposta fora do contrato|

Quando `--id` é repetível (`issues get`, `projects get`, ...) e recebe mais de
um ID, a saída é um array com um resultado por ID; um único ID devolve o objeto.

`--project` e `--title` são percent-encoded como segmentos de caminho, então
identificadores com espaço, `/` ou acento funcionam:

```bash
redmine wiki get --project "my proj/x" --title "Guia de Uso"
```

---

## Grupos e ações

### Núcleo

|Grupo|Ação|HTTP|Caminho|Flags|
|---|---|---|---|---|
|`issues`|`list`|GET|`/issues.json`|—|
|`issues`|`get`|GET|`/issues/{id}.json`|`--id` (repetível)|
|`issues`|`create`|POST|`/issues.json`|—|
|`issues`|`update`|PUT|`/issues/{id}.json`|`--id`|
|`issues`|`delete`|DELETE|`/issues/{id}.json`|`--id`|
|`issues`|`add-watcher`|POST|`/issues/{id}/watchers.json`|`--id --user`|
|`issues`|`remove-watcher`|DELETE|`/issues/{id}/watchers/{user}.json`|`--id --user`|
|`issues`|`statuses`|GET|`/issues/{id}.json?include=allowed_statuses`|`--id`|
|`projects`|`list`|GET|`/projects.json`|—|
|`projects`|`get`|GET|`/projects/{id}.json`|`--id` (repetível)|
|`projects`|`create`|POST|`/projects.json`|—|
|`projects`|`update`|PUT|`/projects/{id}.json`|`--id`|
|`projects`|`delete`|DELETE|`/projects/{id}.json`|`--id`|
|`projects`|`archive`|PUT|`/projects/{id}/archive.json`|`--id`|
|`projects`|`unarchive`|PUT|`/projects/{id}/unarchive.json`|`--id`|
|`users`|`list`|GET|`/users.json`|—|
|`users`|`get`|GET|`/users/{id}.json`|`--id` (repetível)|
|`users`|`me`|GET|`/my/account.json`|—|
|`users`|`create`|POST|`/users.json`|—|
|`users`|`update`|PUT|`/users/{id}.json`|`--id`|
|`users`|`delete`|DELETE|`/users/{id}.json`|`--id`|
|`time-entries`|`list`|GET|`/time_entries.json`|—|
|`time-entries`|`get`|GET|`/time_entries/{id}.json`|`--id` (repetível)|
|`time-entries`|`create`|POST|`/time_entries.json`|—|
|`time-entries`|`update`|PUT|`/time_entries/{id}.json`|`--id`|
|`time-entries`|`delete`|DELETE|`/time_entries/{id}.json`|`--id`|

### Colaboração

|Grupo|Ação|HTTP|Caminho|Flags|
|---|---|---|---|---|
|`journals`|`update`|PUT|`/journals/{id}.json`|`--id`|
|`journals`|`remove`|PUT|`/journals/{id}.json`|`--id`|
|`relations`|`list`|GET|`/issues/{id}/relations.json`|`--id` (issue)|
|`relations`|`get`|GET|`/relations/{id}.json`|`--id` (repetível)|
|`relations`|`create`|POST|`/issues/{id}/relations.json`|`--id` (issue)|
|`relations`|`delete`|DELETE|`/relations/{id}.json`|`--id`|
|`attachments`|`get`|GET|`/attachments/{id}.json`|`--id` (repetível)|
|`attachments`|`update`|PATCH|`/attachments/{id}.json`|`--id`|
|`attachments`|`delete`|DELETE|`/attachments/{id}.json`|`--id`|
|`attachments`|`upload`|POST|`/uploads.json`|`--file --filename`|
|`wiki`|`list`|GET|`/projects/{project}/wiki/index.json`|`--project`|
|`wiki`|`get`|GET|`/projects/{project}/wiki/{title}.json`|`--project --title`|
|`wiki`|`get-version`|GET|`/projects/{project}/wiki/{title}/{version}.json`|`--project --title --version`|
|`wiki`|`upsert`|PUT|`/projects/{project}/wiki/{title}.json`|`--project --title`|
|`wiki`|`delete`|DELETE|`/projects/{project}/wiki/{title}.json`|`--project --title`|
|`versions`|`list`|GET|`/projects/{project}/versions.json`|`--project`|
|`versions`|`get`|GET|`/versions/{id}.json`|`--id` (repetível)|
|`versions`|`create`|POST|`/projects/{project}/versions.json`|`--project`|
|`versions`|`update`|PUT|`/versions/{id}.json`|`--id`|
|`versions`|`delete`|DELETE|`/versions/{id}.json`|`--id`|

### Metadados

|Grupo|Ação|HTTP|Caminho|Flags|
|---|---|---|---|---|
|`enumerations`|`priorities`|GET|`/enumerations/issue_priorities.json`|—|
|`enumerations`|`activities`|GET|`/enumerations/time_entry_activities.json`|—|
|`enumerations`|`document-categories`|GET|`/enumerations/document_categories.json`|—|
|`trackers`|`list`|GET|`/trackers.json`|—|
|`issue-statuses`|`list`|GET|`/issue_statuses.json`|—|
|`issue-categories`|`list`|GET|`/projects/{project}/issue_categories.json`|`--project`|
|`issue-categories`|`get`|GET|`/issue_categories/{id}.json`|`--id` (repetível)|
|`issue-categories`|`create`|POST|`/projects/{project}/issue_categories.json`|`--project`|
|`issue-categories`|`update`|PUT|`/issue_categories/{id}.json`|`--id`|
|`issue-categories`|`delete`|DELETE|`/issue_categories/{id}.json`|`--id --reassign-to`|
|`memberships`|`list`|GET|`/projects/{project}/memberships.json`|`--project`|
|`memberships`|`get`|GET|`/memberships/{id}.json`|`--id` (repetível)|
|`memberships`|`create`|POST|`/projects/{project}/memberships.json`|`--project`|
|`memberships`|`update`|PUT|`/memberships/{id}.json`|`--id`|
|`memberships`|`delete`|DELETE|`/memberships/{id}.json`|`--id`|
|`roles`|`list`|GET|`/roles.json`|—|
|`roles`|`get`|GET|`/roles/{id}.json`|`--id` (repetível)|
|`groups`|`list`|GET|`/groups.json`|—|
|`groups`|`get`|GET|`/groups/{id}.json`|`--id` (repetível)|
|`groups`|`create`|POST|`/groups.json`|—|
|`groups`|`update`|PUT|`/groups/{id}.json`|`--id`|
|`groups`|`delete`|DELETE|`/groups/{id}.json`|`--id`|
|`groups`|`add-user`|POST|`/groups/{id}/users.json`|`--id --user`|
|`groups`|`remove-user`|DELETE|`/groups/{id}/users/{user}.json`|`--id --user`|

### Diversos e topo

|Grupo|Ação|HTTP|Caminho|Flags|
|---|---|---|---|---|
|`custom-fields`|`list`|GET|`/custom_fields.json`|—|
|`queries`|`list`|GET|`/queries.json`|—|
|`files`|`list`|GET|`/projects/{project}/files.json`|`--project`|
|`files`|`upload`|POST|`/uploads.json` + `/projects/{project}/files.json`|`--project --file --filename --description`|
|`news`|`list`|GET|`/news.json` ou `/projects/{project}/news.json`|`--project` (opcional)|
|`news`|`get`|GET|`/news/{id}.json`|`--id` (repetível)|
|`news`|`create`|POST|`/projects/{project}/news.json`|`--project`|
|`news`|`update`|PUT|`/news/{id}.json`|`--id`|
|`news`|`delete`|DELETE|`/news/{id}.json`|`--id`|
|`my-account`|`get`|GET|`/my/account.json`|—|
|`search`|—|GET|`/search.json`|consulta posicional|
|`api`|—|qualquer|`--path`|`--method --path`|

Ações sem `--params`: `journals remove`, `projects archive`,
`projects unarchive`, `issues add-watcher`, `groups add-user`,
`attachments upload`, `files upload` e as listas estáticas sem lote
(`enumerations`, `trackers`, `issue-statuses`, `roles`, `custom-fields`,
`queries`, `my-account`). Listas estáticas continuam aceitando `--params` como
filtros.

---

## `search`

```bash
redmine search "bug crítico"
redmine search --params '{"scope":"all"}' - <<'TXT'
falha no login
TXT
```

A consulta vira `q`; `--params` são filtros adicionais e não podem conter `q`.
O resultado é o array `results`. Sem `limit`/`offset`, todas as páginas são
buscadas; para buscas amplas, passe `{"limit":N}` para uma única página.

## `api`

Via de escape para endpoints ainda não tabelados:

```bash
redmine api --method GET --path /custom_fields.json --params '{"limit":5}'
redmine api --method POST --path /issues/42/notes.json --params '{"notes":"ok"}'
```

`--method` é obrigatório; `--path` deve começar por `/` e não pode conter query
string. Em `GET`/`DELETE`, `--params` vira query; nos demais métodos, corpo.
Em lote, cada linha fornece `path` (e `params`), mantendo o `--method` da
invocação.

## Uploads

```bash
# passo 1: obter o token
redmine attachments upload --file relatorio.pdf --filename relatorio.pdf

# passo 2: associar a uma issue no corpo de create/update
redmine issues update --id 42 --params "{\"issue\":{\"uploads\":[{\"token\":\"...\"}]}}"

# upload + associação a um arquivo de projeto em um comando
redmine files upload --project demo --file relatorio.pdf \
  --filename relatorio.pdf --description "Entrega"
```

`files upload` executa duas requisições: `/uploads.json` e
`/projects/{project}/files.json`; o token e o nome são preenchidos
automaticamente.

---

## Exemplos rápidos

```bash
redmine issues list --params '{"status_id":"open"}' | jq '. | length'
redmine issues get --id 1,2,3 --json | jq '.data'
redmine projects list --params '{"limit":100}' > projetos.json
redmine wiki get --project demo --title "Home"
redmine search "deploy" --json | jq '.data[].title'
redmine api --method GET --path /users/current.json
redmine issues list --dry-run --params '{"status_id":"closed"}'
```
