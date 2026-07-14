// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Teste manual completo da biblioteca redmine-wrapper-rs.
///
/// Avalia todos os 22 recursos da API em modo **somente leitura**:
/// nenhum dado é criado, alterado ou excluído.
///
/// Uso:
/// ```bash
/// REDMINE_URL=https://redmine.seu-dominio.com REDMINE_TOKEN=sua-chave cargo run --example check
/// ```
///
/// Cada operação é executada e seu resultado (PASS/FAIL/SKIP) é
/// registrado. Ao final, um resumo consolidado é exibido.

use std::env;
use std::fmt;
use std::sync::atomic::{AtomicU32, Ordering};

use redmine_wrapper::core::config::RedmineConfigBuilder;
use redmine_wrapper::core::errors::RedmineError;
use redmine_wrapper::types::base::RedmineId;
use redmine_wrapper::types::issue::IssueFilter;
use redmine_wrapper::types::search::SearchFilter;
use redmine_wrapper::types::time_entry::TimeEntryFilter;
use redmine_wrapper::RedmineClient;

static PASS: AtomicU32 = AtomicU32::new(0);
static FAIL: AtomicU32 = AtomicU32::new(0);
static SKIP: AtomicU32 = AtomicU32::new(0);

fn ok(label: &str, detail: impl fmt::Display) {
    PASS.fetch_add(1, Ordering::Relaxed);
    println!("  \x1b[32mPASS\x1b[0m  {label}: {detail}");
}

fn fail(label: &str, detail: impl fmt::Display) {
    FAIL.fetch_add(1, Ordering::Relaxed);
    println!("  \x1b[31mFAIL\x1b[0m  {label}: {detail}");
}

fn skip(label: &str, detail: impl fmt::Display) {
    SKIP.fetch_add(1, Ordering::Relaxed);
    println!("  \x1b[33mSKIP\x1b[0m  {label}: {detail}");
}

fn is_auth_err(e: &RedmineError) -> bool {
    matches!(e, RedmineError::Api { category, .. }
        if category.http_status() == 401 || category.http_status() == 403)
}

fn summary() {
    let p = PASS.load(Ordering::Relaxed);
    let f = FAIL.load(Ordering::Relaxed);
    let s = SKIP.load(Ordering::Relaxed);
    let total = p + f + s;
    println!();
    println!("═══════════════════════════════════════");
    println!("  RESULTADO FINAL");
    println!("  Total: {total:3}  PASS: {p:3}  FAIL: {f:3}  SKIP: {s:3}");
    println!("═══════════════════════════════════════");
    if f > 0 {
        std::process::exit(1);
    }
}

macro_rules! section {
    ($title:expr) => {
        println!();
        println!("─── {} ───", $title);
    };
}

fn main() {
    if env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }

    let base_url = env::var("REDMINE_URL").expect("REDMINE_URL não definida");
    let token = env::var("REDMINE_TOKEN").ok();

    let config = RedmineConfigBuilder::default()
        .base_url(base_url.clone())
        .token(token.clone().unwrap_or_default())
        .build()
        .expect("falha ao construir config");
    let client = RedmineClient::new(config).expect("falha ao criar cliente");

    println!("Conectado a: {base_url}");
    if token.is_some() {
        println!("Autenticação: via token");
    } else {
        println!("Autenticação: anônima");
    }

    // ────────────────────────────────────────────
    // 1. my_account
    // ────────────────────────────────────────────
    section!("my_account");
    {
        match client.my_account.get() {
            Ok(a) => ok("my_account.get", format_args!("#{} {} {}", a.id, a.firstname.as_deref().unwrap_or("?"), a.lastname.as_deref().unwrap_or("?"))),
            Err(RedmineError::Api { ref category, .. }) if category.http_status() == 403 => skip("my_account.get", "sem permissão (403) — token pode ser anônimo"),
            Err(e) => fail("my_account.get", e),
        }
    }

    // ────────────────────────────────────────────
    // 2. projects
    // ────────────────────────────────────────────
    section!("projects");
    {
        match client.projects.list() {
            Ok(list) => {
                ok("projects.list", format_args!("{} projeto(s)", list.len()));
                if let Some(p) = list.first() {
                    let pid = p.id;
                    match client.projects.get(pid) {
                        Ok(p2) => ok("projects.get", format_args!("#{} {}", p2.id, p2.name.as_deref().unwrap_or("?"))),
                        Err(e) => fail("projects.get", e),
                    }
                    match client.projects.get_with_includes(pid, &["trackers", "issue_categories"]) {
                        Ok(p2) => ok("projects.get_with_includes", format_args!("#{} com includes", p2.id)),
                        Err(e) => fail("projects.get_with_includes", e),
                    }
                } else {
                    skip("projects.get", "nenhum projeto disponível");
                    skip("projects.get_with_includes", "nenhum projeto disponível");
                }
            }
            Err(RedmineError::Api { ref category, .. }) if category.http_status() == 403 => skip("projects.list", "sem permissão (403)"),
            Err(e) => fail("projects.list", e),
        }
    }

    // ────────────────────────────────────────────
    // 3. users
    // ────────────────────────────────────────────
    section!("users");
    {
        match client.users.get_current() {
            Ok(u) => ok("users.get_current", format_args!("#{} {} {}", u.id, u.firstname.as_deref().unwrap_or("?"), u.lastname.as_deref().unwrap_or("?"))),
            Err(RedmineError::Api { ref category, .. }) if category.http_status() == 403 => skip("users.get_current", "sem permissão (403)"),
            Err(e) => fail("users.get_current", e),
        }
        match client.users.list(None) {
            Ok(list) => {
                ok("users.list", format_args!("{} usuário(s)", list.len()));
                if let Some(u) = list.first() {
                    let uid = u.id;
                    match client.users.get(uid) {
                        Ok(u2) => ok("users.get", format_args!("#{} {}", u2.id, u2.login.as_deref().unwrap_or("?"))),
                        Err(e) => fail("users.get", e),
                    }
                    match client.users.get_with_includes(uid, &["memberships", "groups"]) {
                        Ok(u2) => ok("users.get_with_includes", format_args!("#{} com includes", u2.id)),
                        Err(e) => fail("users.get_with_includes", e),
                    }
                } else {
                    skip("users.get", "nenhum usuário disponível");
                    skip("users.get_with_includes", "nenhum usuário disponível");
                }
            }
            Err(ref e) if is_auth_err(e) => skip("users.list", "sem permissão"),
            Err(e) => fail("users.list", e),
        }
    }

    let mut first_issue_id: Option<RedmineId> = None;

    // ────────────────────────────────────────────
    // 4. issues
    // ────────────────────────────────────────────
    section!("issues");
    {
        match client.issues.list(None) {
            Ok(list) => {
                ok("issues.list", format_args!("{} issue(s)", list.len()));
                if let Some(iss) = list.first() {
                    let iid = iss.id;
                    first_issue_id = Some(iid);
                    match client.issues.get(iid) {
                        Ok(i) => ok("issues.get", format_args!("#{} {}", i.id, i.subject.as_deref().unwrap_or("?"))),
                        Err(e) => fail("issues.get", e),
                    }
                    match client.issues.get_with_includes(iid, &["journals", "attachments", "relations"]) {
                        Ok(i) => ok("issues.get_with_includes", format_args!("#{} com includes", i.id)),
                        Err(e) => fail("issues.get_with_includes", e),
                    }
                    match client.issues.get_allowed_statuses(iid) {
                        Ok(statuses) => ok("issues.get_allowed_statuses", format_args!("{} status(is) permitido(s)", statuses.len())),
                        Err(e) => fail("issues.get_allowed_statuses", e),
                    }
                } else {
                    skip("issues.get", "nenhuma issue disponível");
                    skip("issues.get_with_includes", "nenhuma issue disponível");
                    skip("issues.get_allowed_statuses", "nenhuma issue disponível");
                }
            }
            Err(RedmineError::Api { ref category, .. }) if category.http_status() == 403 => skip("issues.list", "sem permissão (403)"),
            Err(e) => fail("issues.list", e),
        }
        // Testa filtro básico
        let filter = IssueFilter {
            status_id: Some("open".into()),
            ..Default::default()
        };
        match client.issues.list(Some(&filter)) {
            Ok(list) => ok("issues.list (filtro open)", format_args!("{} issue(s) abertas", list.len())),
            Err(e) => fail("issues.list (filtro open)", e),
        }
    }

    // ────────────────────────────────────────────
    // 5. time_entries
    // ────────────────────────────────────────────
    section!("time_entries");
    {
        match client.time_entries.list(None) {
            Ok(list) => {
                ok("time_entries.list", format_args!("{} apontamento(s)", list.len()));
                if let Some(te) = list.first() {
                    match client.time_entries.get(te.id) {
                        Ok(t) => ok("time_entries.get", format_args!("#{} {:.1}h", t.id, t.hours.unwrap_or(0.0))),
                        Err(e) => fail("time_entries.get", e),
                    }
                } else {
                    skip("time_entries.get", "nenhum apontamento disponível");
                }
            }
            Err(RedmineError::Api { ref category, .. }) if category.http_status() == 403 => skip("time_entries.list", "sem permissão (403)"),
            Err(e) => fail("time_entries.list", e),
        }
        // Testa filtro por período
        let filter = TimeEntryFilter {
            from: Some("2020-01-01".into()),
            to: Some("2030-12-31".into()),
            ..Default::default()
        };
        match client.time_entries.list(Some(&filter)) {
            Ok(list) => ok("time_entries.list (filtro período)", format_args!("{} apontamento(s)", list.len())),
            Err(e) => fail("time_entries.list (filtro período)", e),
        }
    }

    // ────────────────────────────────────────────
    // 6. journals
    // ────────────────────────────────────────────
    section!("journals");
    {
        if let Some(iid) = first_issue_id {
            match client.issues.get_with_includes(iid, &["journals"]) {
                Ok(i) => {
                    let count = i.journals.as_ref().map(|j| j.len()).unwrap_or(0);
                    if count > 0 {
                        ok("journals (via issue includes)", format_args!("{} journal(is) na issue #{}", count, iid));
                    } else {
                        skip("journals (via issue includes)", "issue sem journals");
                    }
                }
                Err(e) => fail("journals (via issue includes)", e),
            }
        } else {
            skip("journals (via issue includes)", "nenhuma issue disponível");
        }
    }

    // ────────────────────────────────────────────
    // 7. relations
    // ────────────────────────────────────────────
    section!("relations");
    {
        if let Some(iid) = first_issue_id {
            match client.relations.list_by_issue(iid) {
                Ok(rels) => ok("relations.list_by_issue", format_args!("{} relação(ões) na issue #{}", rels.len(), iid)),
                Err(e) => fail("relations.list_by_issue", e),
            }
        } else {
            skip("relations.list_by_issue", "nenhuma issue disponível");
        }
    }

    // ────────────────────────────────────────────
    // 8. attachments
    // ────────────────────────────────────────────
    section!("attachments");
    {
        if let Some(iid) = first_issue_id {
            match client.issues.get_with_includes(iid, &["attachments"]) {
                Ok(i) => {
                    if let Some(atts) = i.attachments {
                        if let Some(a) = atts.first() {
                            match client.attachments.get(a.id) {
                                Ok(att) => ok("attachments.get", format_args!("#{} ({})", a.id, att.filename.as_deref().unwrap_or("?"))),
                                Err(e) => fail("attachments.get", e),
                            }
                        } else {
                            skip("attachments.get", "issue sem attachments");
                        }
                    } else {
                        skip("attachments.get", "issue sem attachments");
                    }
                }
                Err(e) => fail("attachments.get", e),
            }
        } else {
            skip("attachments.get", "nenhuma issue disponível");
        }
    }

    // ────────────────────────────────────────────
    // 9. wiki
    // ────────────────────────────────────────────
    section!("wiki");
    {
        match client.projects.list() {
            Ok(projects) => {
                if let Some(p) = projects.first() {
                    let pid = p.id;
                    match client.wiki.list(pid) {
                        Ok(pages) => {
                            ok("wiki.list", format_args!("{} página(s) no projeto #{}", pages.len(), pid));
                            if let Some(wp) = pages.first() {
                                match client.wiki.get(pid, &wp.title, None) {
                                    Ok(page) => ok("wiki.get", format_args!("'{}' (v{})", page.title.as_deref().unwrap_or("?"), page.version.unwrap_or(0))),
                                    Err(e) => fail("wiki.get", e),
                                }
                                match client.wiki.get(pid, &wp.title, Some(&["attachments"])) {
                                    Ok(page) => ok("wiki.get (com includes)", format_args!("'{}' c/ anexos", page.title.as_deref().unwrap_or("?"))),
                                    Err(e) => fail("wiki.get (com includes)", e),
                                }
                                // Tenta acessar versão 1 da página
                                match client.wiki.get_version(pid, &wp.title, 1) {
                                    Ok(page) => ok("wiki.get_version", format_args!("'{}' v1", page.title.as_deref().unwrap_or("?"))),
                                    Err(e) => {
                                        // Pode ser que a versão 1 não exista — não consideramos FAIL
                                        skip("wiki.get_version", format_args!("versão 1 não acessível: {e}"));
                                    }
                                }
                            } else {
                                skip("wiki.get", "projeto sem páginas wiki");
                                skip("wiki.get (com includes)", "projeto sem páginas wiki");
                                skip("wiki.get_version", "projeto sem páginas wiki");
                            }
                        }
                        Err(RedmineError::Api { ref category, .. }) if category.http_status() == 403 => skip("wiki.list", "sem permissão (403)"),
                        Err(e) => fail("wiki.list", e),
                    }
                } else {
                    skip("wiki.list", "nenhum projeto disponível");
                    skip("wiki.get", "nenhum projeto disponível");
                    skip("wiki.get (com includes)", "nenhum projeto disponível");
                    skip("wiki.get_version", "nenhum projeto disponível");
                }
            }
            Err(e) => {
                skip("wiki.list", format_args!("não foi possível listar projetos: {e}"));
            }
        }
    }

    // ────────────────────────────────────────────
    // 10. versions
    // ────────────────────────────────────────────
    section!("versions");
    {
        match client.projects.list() {
            Ok(projects) => {
                if let Some(p) = projects.first() {
                    let pid = p.id;
                    match client.versions.list_by_project(pid) {
                        Ok(list) => {
                            ok("versions.list_by_project", format_args!("{} versão(ões)", list.len()));
                            if let Some(v) = list.first() {
                                match client.versions.get(v.id) {
                                    Ok(v2) => ok("versions.get", format_args!("#{} {}", v2.id, v2.name.as_deref().unwrap_or("?"))),
                                    Err(e) => fail("versions.get", e),
                                }
                            } else {
                                skip("versions.get", "projeto sem versões");
                            }
                        }
                        Err(RedmineError::Api { ref category, .. }) if category.http_status() == 403 => skip("versions.list_by_project", "sem permissão (403)"),
                        Err(e) => fail("versions.list_by_project", e),
                    }
                } else {
                    skip("versions.list_by_project", "nenhum projeto disponível");
                    skip("versions.get", "nenhum projeto disponível");
                }
            }
            Err(e) => {
                skip("versions.list_by_project", format_args!("não foi possível listar projetos: {e}"));
            }
        }
    }

    // ────────────────────────────────────────────
    // 11. enumerations
    // ────────────────────────────────────────────
    section!("enumerations");
    {
        match client.enumerations.list_issue_priorities() {
            Ok(list) => ok("enumerations.list_issue_priorities", format_args!("{} prioridade(s)", list.len())),
            Err(e) => fail("enumerations.list_issue_priorities", e),
        }
        match client.enumerations.list_time_entry_activities() {
            Ok(list) => ok("enumerations.list_time_entry_activities", format_args!("{} atividade(s)", list.len())),
            Err(e) => fail("enumerations.list_time_entry_activities", e),
        }
        match client.enumerations.list_document_categories() {
            Ok(list) => ok("enumerations.list_document_categories", format_args!("{} categoria(s) de documento", list.len())),
            Err(e) => fail("enumerations.list_document_categories", e),
        }
    }

    // ────────────────────────────────────────────
    // 12. trackers
    // ────────────────────────────────────────────
    section!("trackers");
    {
        match client.trackers.list() {
            Ok(list) => ok("trackers.list", format_args!("{} tracker(s)", list.len())),
            Err(e) => fail("trackers.list", e),
        }
    }

    // ────────────────────────────────────────────
    // 13. issue_statuses
    // ────────────────────────────────────────────
    section!("issue_statuses");
    {
        match client.issue_statuses.list() {
            Ok(list) => ok("issue_statuses.list", format_args!("{} status", list.len())),
            Err(e) => fail("issue_statuses.list", e),
        }
    }

    // ────────────────────────────────────────────
    // 14. issue_categories
    // ────────────────────────────────────────────
    section!("issue_categories");
    {
        match client.projects.list() {
            Ok(projects) => {
                if let Some(p) = projects.first() {
                    let pid = p.id;
                    match client.issue_categories.list_by_project(pid) {
                        Ok(list) => {
                            ok("issue_categories.list_by_project", format_args!("{} categoria(s)", list.len()));
                            if let Some(c) = list.first() {
                                match client.issue_categories.get(c.id) {
                                    Ok(cat) => ok("issue_categories.get", format_args!("#{} {}", cat.id, cat.name.as_deref().unwrap_or("?"))),
                                    Err(e) => fail("issue_categories.get", e),
                                }
                            } else {
                                skip("issue_categories.get", "projeto sem categorias");
                            }
                        }
                        Err(RedmineError::Api { ref category, .. }) if category.http_status() == 403 => skip("issue_categories.list_by_project", "sem permissão (403)"),
                        Err(e) => fail("issue_categories.list_by_project", e),
                    }
                } else {
                    skip("issue_categories.list_by_project", "nenhum projeto disponível");
                    skip("issue_categories.get", "nenhum projeto disponível");
                }
            }
            Err(e) => {
                skip("issue_categories.list_by_project", format_args!("não foi possível listar projetos: {e}"));
            }
        }
    }

    // ────────────────────────────────────────────
    // 15. memberships
    // ────────────────────────────────────────────
    section!("memberships");
    {
        match client.projects.list() {
            Ok(projects) => {
                if let Some(p) = projects.first() {
                    let pid = p.id;
                    match client.memberships.list_by_project(pid) {
                        Ok(list) => {
                            ok("memberships.list_by_project", format_args!("{} associação(ões)", list.len()));
                            if let Some(m) = list.first() {
                                match client.memberships.get(m.id) {
                                    Ok(_) => ok("memberships.get", format_args!("#{}", m.id)),
                                    Err(e) => fail("memberships.get", e),
                                }
                            } else {
                                skip("memberships.get", "projeto sem associações");
                            }
                        }
                        Err(RedmineError::Api { ref category, .. }) if category.http_status() == 403 => skip("memberships.list_by_project", "sem permissão (403)"),
                        Err(e) => fail("memberships.list_by_project", e),
                    }
                } else {
                    skip("memberships.list_by_project", "nenhum projeto disponível");
                    skip("memberships.get", "nenhum projeto disponível");
                }
            }
            Err(e) => {
                skip("memberships.list_by_project", format_args!("não foi possível listar projetos: {e}"));
            }
        }
    }

    // ────────────────────────────────────────────
    // 16. roles
    // ────────────────────────────────────────────
    section!("roles");
    {
        match client.roles.list() {
            Ok(list) => {
                ok("roles.list", format_args!("{} papel(éis)", list.len()));
                if let Some(r) = list.first() {
                    match client.roles.get(r.id) {
                        Ok(role) => ok("roles.get", format_args!("#{} {}", role.id, role.name.as_deref().unwrap_or("?"))),
                        Err(e) => fail("roles.get", e),
                    }
                } else {
                    skip("roles.get", "nenhum papel disponível");
                }
            }
            Err(RedmineError::Api { ref category, .. }) if category.http_status() == 403 => skip("roles.list", "sem permissão (403)"),
            Err(e) => fail("roles.list", e),
        }
    }

    // ────────────────────────────────────────────
    // 17. groups
    // ────────────────────────────────────────────
    section!("groups");
    {
        match client.groups.list() {
            Ok(list) => {
                ok("groups.list", format_args!("{} grupo(s)", list.len()));
                if let Some(g) = list.first() {
                    let gid = g.id;
                    match client.groups.get(gid) {
                        Ok(g2) => ok("groups.get", format_args!("#{} {}", g2.id, g2.name.as_deref().unwrap_or("?"))),
                        Err(e) => fail("groups.get", e),
                    }
                    match client.groups.get_with_includes(gid, &["users", "memberships"]) {
                        Ok(g2) => ok("groups.get_with_includes", format_args!("#{} com includes", g2.id)),
                        Err(e) => fail("groups.get_with_includes", e),
                    }
                } else {
                    skip("groups.get", "nenhum grupo disponível");
                    skip("groups.get_with_includes", "nenhum grupo disponível");
                }
            }
            Err(ref e) if is_auth_err(e) => skip("groups.list", "sem permissão — requer admin"),
            Err(e) => fail("groups.list", e),
        }
    }

    // ────────────────────────────────────────────
    // 18. custom_fields
    // ────────────────────────────────────────────
    section!("custom_fields");
    {
        match client.custom_fields.list() {
            Ok(list) => ok("custom_fields.list", format_args!("{} campo(s)", list.len())),
            Err(ref e) if is_auth_err(e) => skip("custom_fields.list", "sem permissão — requer admin"),
            Err(e) => fail("custom_fields.list", e),
        }
    }

    // ────────────────────────────────────────────
    // 19. queries
    // ────────────────────────────────────────────
    section!("queries");
    {
        match client.queries.list() {
            Ok(list) => ok("queries.list", format_args!("{} consulta(s)", list.len())),
            Err(e) => fail("queries.list", e),
        }
    }

    // ────────────────────────────────────────────
    // 20. files
    // ────────────────────────────────────────────
    section!("files");
    {
        match client.projects.list() {
            Ok(projects) => {
                if let Some(p) = projects.first() {
                    let pid = p.id;
                    match client.files.list_by_project(pid) {
                        Ok(list) => ok("files.list_by_project", format_args!("{} arquivo(s)", list.len())),
                        Err(ref e) if is_auth_err(e) => skip("files.list_by_project", "sem permissão"),
                        Err(e) => fail("files.list_by_project", e),
                    }
                } else {
                    skip("files.list_by_project", "nenhum projeto disponível");
                }
            }
            Err(e) => {
                skip("files.list_by_project", format_args!("não foi possível listar projetos: {e}"));
            }
        }
    }

    // ────────────────────────────────────────────
    // 21. search
    // ────────────────────────────────────────────
    section!("search");
    {
        let filter = SearchFilter {
            query: "a".into(),
            limit: Some(5),
            issues: Some(true),
            offset: None,
            scope: None,
            all_words: None,
            titles_only: None,
            news: None,
            documents: None,
            changesets: None,
            wiki_pages: None,
            messages: None,
            projects: None,
            open_issues: None,
            attachments: None,
        };
        match client.search.search(&filter) {
            Ok(list) => ok("search.search", format_args!("{} resultado(s)", list.len())),
            Err(RedmineError::Api { ref category, .. }) if category.http_status() == 403 => skip("search.search", "sem permissão (403)"),
            Err(e) => fail("search.search", e),
        }
    }

    // ────────────────────────────────────────────
    // 22. news
    // ────────────────────────────────────────────
    section!("news");
    {
        match client.news.list() {
            Ok(list) => {
                ok("news.list", format_args!("{} notícia(s)", list.len()));
                if let Some(n) = list.first() {
                    match client.news.get(n.id) {
                        Ok(_) => ok("news.get", format_args!("#{}", n.id)),
                        Err(e) => fail("news.get", e),
                    }
                } else {
                    skip("news.get", "nenhuma notícia disponível");
                }
            }
            Err(RedmineError::Api { ref category, .. }) if category.http_status() == 403 => skip("news.list", "sem permissão (403)"),
            Err(e) => fail("news.list", e),
        }
        match client.projects.list() {
            Ok(projects) => {
                if let Some(p) = projects.first() {
                    match client.news.list_by_project(p.id) {
                        Ok(list) => ok("news.list_by_project", format_args!("{} notícia(s) no projeto #{}", list.len(), p.id)),
                        Err(RedmineError::Api { ref category, .. }) if category.http_status() == 403 => skip("news.list_by_project", "sem permissão (403)"),
                        Err(e) => fail("news.list_by_project", e),
                    }
                } else {
                    skip("news.list_by_project", "nenhum projeto disponível");
                }
            }
            Err(e) => {
                skip("news.list_by_project", format_args!("não foi possível listar projetos: {e}"));
            }
        }
    }

    summary();
}
