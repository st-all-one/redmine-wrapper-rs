// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod attachments;
pub mod custom_fields;
pub mod enumerations;
pub mod files;
pub mod groups;
pub mod issue_categories;
pub mod issue_statuses;
pub mod issues;
pub mod journals;
pub mod memberships;
pub mod my_account;
pub mod news;
pub mod projects;
pub mod queries;
pub mod relations;
pub mod roles;
pub mod search;
pub mod time_entries;
pub mod trackers;
pub mod users;
pub mod versions;
pub mod wiki;

pub use attachments::AttachmentsResource;
pub use custom_fields::CustomFieldsResource;
pub use enumerations::EnumerationsResource;
pub use files::FilesResource;
pub use groups::GroupsResource;
pub use issue_categories::IssueCategoriesResource;
pub use issue_statuses::IssueStatusesResource;
pub use issues::IssuesResource;
pub use journals::JournalsResource;
pub use memberships::MembershipsResource;
pub use my_account::MyAccountResource;
pub use news::NewsResource;
pub use projects::ProjectsResource;
pub use queries::QueriesResource;
pub use relations::RelationsResource;
pub use roles::RolesResource;
pub use search::SearchResource;
pub use time_entries::TimeEntriesResource;
pub use trackers::TrackersResource;
pub use users::UsersResource;
pub use versions::VersionsResource;
pub use wiki::WikiResource;
