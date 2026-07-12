// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Timeout padrão para requisições HTTP (30 segundos).
pub(crate) const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Máximo de requisições por segundo (rate limiting).
/// Valor padrão: 10 requisições/s.
pub(crate) const DEFAULT_MAX_RPS: u32 = 10;

/// Limite padrão de itens por página na paginação.
/// Valor padrão: 100 itens/página.
pub(crate) const DEFAULT_PAGINATION_LIMIT: u32 = 100;
