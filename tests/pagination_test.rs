// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use redmine_wrapper::http::pagination::PaginationParams;

#[test]
fn test_pagination_params_default() {
    let params = PaginationParams::default();
    assert!(params.offset.is_none());
    assert!(params.limit.is_none());
}

#[test]
fn test_pagination_params_new() {
    let params = PaginationParams::new(10, 50);
    assert_eq!(params.offset, Some(10));
    assert_eq!(params.limit, Some(50));
}

#[test]
fn test_pagination_params_to_query() {
    let params = PaginationParams::new(0, 100);
    let query = params.to_query();
    assert!(query.contains(&("offset", "0".to_string())));
    assert!(query.contains(&("limit", "100".to_string())));
}

#[test]
fn test_pagination_params_to_query_default() {
    let params = PaginationParams::default();
    let query = params.to_query();
    assert!(query.is_empty());
}
