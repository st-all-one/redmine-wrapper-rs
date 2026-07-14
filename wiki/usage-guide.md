# Guia de Uso — Todos os Resources

Exemplos práticos de como usar cada um dos 22 resources do `redmine-wrapper-rs`.

> **Convenções:** Todos os métodos retornam `Result<T, RedmineError>`.
> Métodos de listagem retornam `Vec<T>` (coletados eager).
> Use `?` para propagar erros ou `match` para tratamento granular.

---

## Índice

1. [Issues](#issues)
2. [Projects](#projects)
3. [Users](#users)
4. [Time Entries](#time-entries)
5. [Journals](#journals)
6. [Relations](#relations)
7. [Attachments](#attachments)
8. [Wiki](#wiki)
9. [Versions](#versions)
10. [Enumerations](#enumerations)
11. [Trackers](#trackers)
12. [Issue Statuses](#issue-statuses)
13. [Issue Categories](#issue-categories)
14. [Memberships](#memberships)
15. [Roles](#roles)
16. [Groups](#groups)
17. [Custom Fields](#custom-fields)
18. [Queries](#queries)
19. [Files](#files)
20. [Search](#search)
21. [News](#news)
22. [My Account](#my-account)

---

## Issues

```rust,ignore
use redmine_wrapper::RedmineClient;
use redmine_wrapper::types::issue::*;

let client = RedmineClient::new(
    RedmineConfigBuilder::default()
        .base_url("https://redmine.exemplo.com")
        .token("sua-chave")
        .build()?,
)?;

// Listar issues abertas atribuídas a mim
let filter = IssueFilter {
    assigned_to_id: Some("me".into()),
    status_id: Some("open".into()),
    ..Default::default()
};
let issues = client.issues.list(Some(&filter))?;
for issue in &issues {
    println!("#{}: {} — {}", issue.id, issue.subject.as_deref().unwrap_or(""), issue.status.as_ref().map(|s| &s.name).unwrap_or(&"N/A".into()));
}

// Buscar issue com journals e attachments
let issue = client.issues.get_with_includes(42, &["journals", "attachments"])?;
println!("Assunto: {}", issue.subject.as_deref().unwrap_or(""));
if let Some(journals) = &issue.journals {
    for j in journals {
        println!("  Comentário: {}", j.notes.as_deref().unwrap_or(""));
    }
}

// Status permitidos para transição
let statuses = client.issues.get_allowed_statuses(42)?;
for s in &statuses {
    println!("Pode transicionar para: {} (#{})", s.name, s.id);
}

// Criar issue
let new_issue = client.issues.create(&CreateIssuePayload {
    project_id: 1,
    subject: "Bug crítico no login".into(),
    description: Some("Usuário não consegue autenticar com SSO".into()),
    priority_id: Some(4),
    tracker_id: Some(1),
    assigned_to_id: Some(5),
    ..Default::default()
})?;
println!("Issue criada: #{}", new_issue.id);

// Atualizar (comentar + mudar status)
client.issues.update(42, &UpdateIssuePayload {
    notes: Some("Corrigido na versão 2.1".into()),
    status_id: Some(3),
    done_ratio: Some(100),
    ..Default::default()
})?;

// Adicionar watcher
client.issues.add_watcher(42, 10)?;

// Excluir
client.issues.delete(99)?;
```

## Projects

```rust,ignore
use redmine_wrapper::*;
use redmine_wrapper::types::project::*;

// Listar todos os projetos
let projects = client.projects.list()?;
for p in &projects {
    println!("{}: {} ({})", p.id, p.name.as_deref().unwrap_or(""), p.identifier.as_deref().unwrap_or(""));
}

// Buscar projeto com associações
let project = client.projects.get_with_includes(1, &["trackers", "issue_categories"])?;
println!("Módulos habilitados: {:?}", project.enabled_modules);

// Criar projeto
let new_project = client.projects.create(&CreateProjectPayload {
    name: "App Mobile".into(),
    identifier: "app-mobile".into(),
    description: Some("Aplicativo mobile da empresa".into()),
    is_public: Some(false),
    ..Default::default()
})?;

// Arquivar/desarquivar
client.projects.archive(1)?;
client.projects.unarchive(1)?;

// Atualizar
client.projects.update(1, &UpdateProjectPayload {
    description: Some("Descrição atualizada".into()),
    ..Default::default()
})?;

// Excluir
client.projects.delete(2)?;
```

## Users

```rust,ignore
use redmine_wrapper::*;
use redmine_wrapper::types::user::*;

// Dados do usuário autenticado
let me = client.my_account.get()?;
println!("Logado como: {} {}", me.firstname.as_deref().unwrap_or(""), me.lastname.as_deref().unwrap_or(""));

// Listar usuários ativos
let users = client.users.list(Some(&UserFilter {
    status: Some(UserStatus::Active),
    ..Default::default()
}))?;
for u in &users {
    println!("{} — {} {}", u.login.as_deref().unwrap_or(""), u.firstname.as_deref().unwrap_or(""), u.lastname.as_deref().unwrap_or(""));
}

// Buscar usuário com associações
let user = client.users.get_with_includes(5, &["memberships", "groups"])?;

// Criar usuário (requer admin)
let new_user = client.users.create(&CreateUserPayload {
    login: "joao.silva".into(),
    firstname: "João".into(),
    lastname: "Silva".into(),
    mail: "joao@example.com".into(),
    password: Some("senha123".into()),
    ..Default::default()
})?;

// Atualizar
client.users.update(5, &UpdateUserPayload {
    mail: Some("novo@example.com".into()),
    ..Default::default()
})?;

// Excluir
client.users.delete(5)?;
```

## Time Entries

```rust,ignore
use redmine_wrapper::types::time_entry::*;

// Listar apontamentos de hoje
let entries = client.time_entries.list(Some(&TimeEntryFilter {
    spent_on: Some("2026-07-11".into()),
    ..Default::default()
}))?;

for e in &entries {
    println!("{}h — {} (issue #{})", e.hours.unwrap_or(0.0), e.comments.as_deref().unwrap_or(""), e.issue.as_ref().map(|i| i.id).unwrap_or(0));
}

// Criar apontamento
let entry = client.time_entries.create(&CreateTimeEntryPayload {
    issue_id: Some(42),
    hours: 3.5,
    activity_id: 9, // Desenvolvimento
    comments: Some("Implementação do módulo de relatórios".into()),
    spent_on: Some("2026-07-11".into()),
    ..Default::default()
})?;
println!("Apontamento #{} criado", entry.id);
```

## Journals

> Journals **não** possuem endpoint GET standalone. São obtidos exclusivamente
> via `GET /issues/{id}.json?include=journals`.

```rust,ignore
use redmine_wrapper::types::journal::*;

// Obter journals de uma issue
let issue = client.issues.get_with_includes(42, &["journals"])?;
for journal in issue.journals.unwrap_or_default() {
    println!("[#{}] {}: {}", journal.id, journal.user.as_ref().map(|u| &u.name).unwrap_or(&"?".into()), journal.notes.as_deref().unwrap_or(""));
}

// Atualizar anotações
client.journals.update(123, &UpdateJournalPayload {
    notes: "Comentário atualizado".into(),
    private_notes: Some(true),
})?;

// Remover anotações (limpar)
client.journals.remove(123)?;
```

## Relations

```rust,ignore
use redmine_wrapper::types::relation::*;

// Listar relações de uma issue
let relations = client.relations.list_by_issue(42)?;
for r in &relations {
    println!("Issue #{} → #{} ({:?})", r.issue_id.unwrap_or(0), r.issue_to_id.unwrap_or(0), r.relation_type);
}

// Criar relação (bloqueio)
let rel = client.relations.create_on_issue(42, &CreateRelationPayload {
    issue_to_id: 50,
    relation_type: RelationType::Blocks,
    delay: None,
})?;

// Excluir
client.relations.delete(rel.id)?;
```

## Attachments

```rust,ignore
use redmine_wrapper::types::attachment::*;

// Upload em 2 passos:
// Passo 1: Upload do arquivo → token
let data = std::fs::read("relatorio.pdf")?;
let token = client.attachments.upload("relatorio.pdf", &data)?;
println!("Token de upload: {}", token);

// Passo 2: Associar token a uma issue
client.issues.update(42, &UpdateIssuePayload {
    notes: Some("Relatório anexado".into()),
    uploads: Some(vec![UploadPayload {
        token: token.clone(),
        filename: Some("relatorio.pdf".into()),
        content_type: Some("application/pdf".into()),
        description: Some("Relatório mensal".into()),
    }]),
    ..Default::default()
})?;

// Obter detalhes do anexo
let attachment = client.attachments.get(5)?;
println!("Arquivo: {} ({} bytes)", attachment.filename.as_deref().unwrap_or(""), attachment.filesize.unwrap_or(0));

// Excluir anexo
client.attachments.delete(5)?;
```

## Wiki

```rust,ignore
use redmine_wrapper::types::wiki::*;

// Listar páginas wiki de um projeto
let pages = client.wiki.list(1)?;
for p in &pages {
    println!("Página: {} (v{})", p.title, p.version);
}

// Obter página
let page = client.wiki.get(1, "Home", None)?;
println!("{}", page.text.as_deref().unwrap_or(""));

// Obter versão específica
let old = client.wiki.get_version(1, "Home", 3)?;

// Criar/atualizar página
client.wiki.create_or_update(1, "Guia-de-Uso", &CreateWikiPagePayload {
    text: "# Guia de Uso\n\nBem-vindo ao guia...".into(),
    comments: Some("Versão inicial".into()),
    parent_title: None,
})?;

// Excluir página
client.wiki.delete(1, "Pagina-Antiga")?;
```

## Versions

```rust,ignore
use redmine_wrapper::types::version::*;

// Listar versões de um projeto
let versions = client.versions.list_by_project(1)?;
for v in &versions {
    println!("{} — entrega: {}", v.name.as_deref().unwrap_or(""), v.due_date.as_deref().unwrap_or("sem data"));
}

// Criar versão
let version = client.versions.create_on_project(1, &CreateVersionPayload {
    name: "v2.0.0".into(),
    description: Some("Release principal do semestre".into()),
    status: Some(VersionStatus::Open),
    due_date: Some("2026-12-20".into()),
    ..Default::default()
})?;

// Atualizar
client.versions.update(version.id, &UpdateVersionPayload {
    status: Some(VersionStatus::Locked),
    ..Default::default()
})?;

// Excluir
client.versions.delete(version.id)?;
```

## Enumerations

```rust,ignore
// Prioridades disponíveis
let priorities = client.enumerations.list_issue_priorities()?;
for p in &priorities {
    println!("#{}: {} (padrão: {})", p.id, p.name.as_deref().unwrap_or(""), p.is_default.unwrap_or(false));
}

// Atividades de apontamento de horas
let activities = client.enumerations.list_time_entry_activities()?;
for a in &activities {
    println!("#{}: {}", a.id, a.name.as_deref().unwrap_or(""));
}

// Categorias de documento
let categories = client.enumerations.list_document_categories()?;
```

## Trackers

```rust,ignore
let trackers = client.trackers.list()?;
for t in &trackers {
    println!("#{}: {} — status padrão: {}", t.id, t.name.as_deref().unwrap_or(""), t.default_status.as_ref().map(|s| &s.name).unwrap_or(&"N/A".into()));
}
```

## Issue Statuses

```rust,ignore
let statuses = client.issue_statuses.list()?;
for s in &statuses {
    println!("#{}: {} (fechado: {})", s.id, s.name.as_deref().unwrap_or(""), s.is_closed.unwrap_or(false));
}
```

## Issue Categories

```rust,ignore
use redmine_wrapper::types::issue_category::*;

// Listar categorias de um projeto
let categories = client.issue_categories.list_by_project(1)?;
for c in &categories {
    println!("#{}: {}", c.id, c.name.as_deref().unwrap_or(""));
}

// Criar categoria
let cat = client.issue_categories.create(1, &CreateIssueCategoryPayload {
    name: "Suporte N2".into(),
    assigned_to_id: Some(5),
})?;

// Excluir com reassign
client.issue_categories.delete(cat.id, Some(10))?;
```

## Memberships

```rust,ignore
use redmine_wrapper::types::membership::*;

// Listar membros de um projeto
let members = client.memberships.list_by_project(1)?;
for m in &members {
    let name = m.user.as_ref().map(|u| &u.name).or_else(|| m.group.as_ref().map(|g| &g.name));
    println!("Membro: {}", name.unwrap_or(&"N/A".into()));
}

// Adicionar membro
let membership = client.memberships.create(1, &CreateMembershipPayload {
    user_id: Some(10),
    group_id: None,
    role_ids: vec![3], // Desenvolvedor
})?;

// Atualizar papéis
client.memberships.update(membership.id, &UpdateMembershipPayload {
    role_ids: vec![3, 4],
})?;

// Remover
client.memberships.delete(membership.id)?;
```

## Roles

```rust,ignore
// Listar todos os papéis
let roles = client.roles.list()?;
for r in &roles {
    println!("#{}: {} — permissões: {}", r.id, r.name.as_deref().unwrap_or(""), r.permissions.as_ref().map(|p| p.len().to_string()).unwrap_or("0".into()));
}

// Obter papel com permissões
let role = client.roles.get(3)?;
println!("Permissões: {:?}", role.permissions);
```

## Groups

```rust,ignore
use redmine_wrapper::types::group::*;

// Listar grupos
let groups = client.groups.list()?;
for g in &groups {
    println!("#{}: {}", g.id, g.name.as_deref().unwrap_or(""));
}

// Criar grupo
let group = client.groups.create(&CreateGroupPayload {
    name: "Equipe Backend".into(),
    user_ids: Some(vec![5, 10]),
})?;

// Adicionar/remover usuário
client.groups.add_user(group.id, 15)?;
client.groups.remove_user(group.id, 5)?;

// Excluir
client.groups.delete(group.id)?;
```

## Custom Fields

```rust,ignore
let fields = client.custom_fields.list()?;
for f in &fields {
    println!("#{}: {} ({:?}) — obrigatório: {}", f.id, f.name.as_deref().unwrap_or(""), f.field_format, f.is_required.unwrap_or(false));
}
```

## Queries

```rust,ignore
let queries = client.queries.list()?;
for q in &queries {
    println!("#{}: {} (pública: {})", q.id, q.name.as_deref().unwrap_or(""), q.is_public.unwrap_or(false));
}

// Usar query_id no filtro de issues
let filter = IssueFilter {
    query_id: Some(5),
    ..Default::default()
};
let issues = client.issues.list(Some(&filter))?;
```

## Files

```rust,ignore
use redmine_wrapper::types::file::*;

// Listar arquivos de um projeto
let files = client.files.list_by_project(1)?;
for f in &files {
    println!("{} ({} bytes) — {}", f.filename.as_deref().unwrap_or(""), f.filesize.unwrap_or(0), f.created_on.as_deref().unwrap_or(""));
}

// Anexar arquivo a projeto (requer token de upload)
let data = std::fs::read("documento.pdf")?;
let token = client.attachments.upload("documento.pdf", &data)?;
let file = client.files.attach_to_project(1, &CreateFilePayload {
    token,
    filename: "documento.pdf".into(),
    content_type: Some("application/pdf".into()),
    description: Some("Contrato assinado".into()),
    version_id: None,
})?;
```

## Search

```rust,ignore
use redmine_wrapper::types::search::*;

// Busca textual
let results = client.search.search(&SearchFilter {
    query: "bug crítico".into(),
    issues: Some(true),
    wiki_pages: Some(true),
    ..Default::default()
})?;

for r in &results {
    println!("[{}] {} — {}", r.result_type.as_deref().unwrap_or(""), r.title.as_deref().unwrap_or(""), r.url.as_deref().unwrap_or(""));
}
```

## News

```rust,ignore
use redmine_wrapper::types::news::*;

// Listar notícias globais
let news = client.news.list()?;
for n in &news {
    println!("{} — {}", n.title.as_deref().unwrap_or(""), n.author.as_ref().map(|a| &a.name).unwrap_or(&"N/A".into()));
}

// Notícias de um projeto
let project_news = client.news.list_by_project(1)?;

// Criar notícia
let news_item = client.news.create(1, &CreateNewsPayload {
    title: "Release v2.0".into(),
    summary: Some("Novas funcionalidades do sistema".into()),
    description: Some("Detalhes da release...".into()),
})?;

// Atualizar
client.news.update(news_item.id, &UpdateNewsPayload {
    title: Some("Release v2.0 — Atualização".into()),
    ..Default::default()
})?;

// Excluir
client.news.delete(news_item.id)?;
```

## My Account

```rust,ignore
use redmine_wrapper::types::my_account::MyAccount;

let account = client.my_account.get()?;
println!("Usuário: {} {} ({})", account.firstname.as_deref().unwrap_or(""), account.lastname.as_deref().unwrap_or(""), account.login.as_deref().unwrap_or(""));
println!("Admin: {}", account.admin.unwrap_or(false));
println!("API Key: {}", account.api_key.as_deref().map(|_| "***").unwrap_or("N/A"));
```

---

## Resumo de Métodos por Resource

| Resource | list | get | create | update | delete | Métodos Extras |
|---|---|---|---|---|---|---|
| Issues | ✅ | ✅ | ✅ | ✅ | ✅ | `get_with_includes`, `get_allowed_statuses`, `add_watcher`, `remove_watcher` |
| Projects | ✅ | ✅ | ✅ | ✅ | ✅ | `get_with_includes`, `archive`, `unarchive` |
| Users | ✅ | ✅ | ✅ | ✅ | ✅ | `get_with_includes`, `get_current` |
| Time Entries | ✅ | ✅ | ✅ | ✅ | ✅ | — |
| Journals | — | — | — | ✅ | ✅ (remove) | journals via `?include=journals` em Issues |
| Relations | ✅ | ✅ | ✅ | — | ✅ | `list_by_issue`, `create_on_issue` |
| Attachments | — | ✅ | — | — | ✅ | `upload` (2-passos) |
| Wiki | ✅ | ✅ | ✅ | ✅ | ✅ | `get_version`, `create_or_update` |
| Versions | ✅ | ✅ | ✅ | ✅ | ✅ | `list_by_project`, `create_on_project` |
| Enumerations | ✅ | — | — | — | — | `list_issue_priorities`, `list_time_entry_activities`, `list_document_categories` |
| Trackers | ✅ | — | — | — | — | — |
| Issue Statuses | ✅ | — | — | — | — | — |
| Issue Categories | ✅ | ✅ | ✅ | ✅ | ✅ | `list_by_project`, delete com `reassign_to_id` |
| Memberships | ✅ | ✅ | ✅ | ✅ | ✅ | `list_by_project` |
| Roles | ✅ | ✅ | — | — | — | — |
| Groups | ✅ | ✅ | ✅ | ✅ | ✅ | `get_with_includes`, `add_user`, `remove_user` |
| Custom Fields | ✅ | — | — | — | — | — |
| Queries | ✅ | — | — | — | — | — |
| Files | ✅ | — | — | — | — | `list_by_project`, `attach_to_project` |
| Search | ✅ | — | — | — | — | — |
| News | ✅ | ✅ | ✅ | ✅ | ✅ | `list_by_project` |
| My Account | — | ✅ | — | — | — | — |
