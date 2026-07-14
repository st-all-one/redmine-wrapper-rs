// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Cliente HTTP assincrono (reqwest + tokio) com rate limiting.
pub mod client;
/// Parametros e tipos de paginacao offset/limit.
pub mod pagination;
/// Rate limiter sliding window com `tokio::sync::Mutex`.
pub mod rate_limiter;
