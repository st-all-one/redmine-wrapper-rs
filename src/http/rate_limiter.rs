// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Implementação de sliding window para controle de taxa de requisições.
///
/// Permite no máximo `max_requests` em uma janela de tempo (1 segundo).
/// Bloqueia a thread se o limite for atingido, aguardando até que
/// uma posição seja liberada.
#[derive(Debug)]
pub struct SlidingWindow {
    /// Máximo de requisições permitidas na janela.
    max_requests: u32,

    /// Duração da janela deslizante.
    window: Duration,

    /// Timestamps das requisições recentes.
    timestamps: VecDeque<Instant>,
}

impl SlidingWindow {
    /// Cria uma nova janela deslizante.
    pub fn new(max_requests: u32) -> Self {
        Self {
            max_requests,
            window: Duration::from_secs(1),
            timestamps: VecDeque::with_capacity(max_requests as usize),
        }
    }

    /// Aguarda até que uma requisição possa ser feita, respeitando o limite.
    ///
    /// Remove timestamps expirados e bloqueia se necessário.
    pub fn acquire(&mut self) {
        let now = Instant::now();
        let cutoff = now - self.window;

        // Remove timestamps expirados
        while let Some(&ts) = self.timestamps.front() {
            if ts < cutoff {
                self.timestamps.pop_front();
            } else {
                break;
            }
        }

        if self.timestamps.len() >= self.max_requests as usize {
            // Aguarda até o timestamp mais antigo expirar
            if let Some(&oldest) = self.timestamps.front() {
                let sleep_duration = cutoff - oldest + self.window;
                std::thread::sleep(sleep_duration);
            }
            // Após dormir, remove o timestamp mais antigo
            self.timestamps.pop_front();
        }

        self.timestamps.push_back(now);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allows_requests_under_limit() {
        let mut limiter = SlidingWindow::new(10);
        limiter.acquire();
        limiter.acquire();
        limiter.acquire();
        limiter.acquire();
        assert_eq!(limiter.timestamps.len(), 4);
    }

    #[test]
    fn test_evicts_expired_timestamps() {
        let mut limiter = SlidingWindow::new(5);
        let past = Instant::now() - Duration::from_secs(2);
        for _ in 0..5 {
            limiter.timestamps.push_back(past);
        }
        limiter.acquire();
        assert_eq!(limiter.timestamps.len(), 1);
    }

    #[test]
    fn test_blocks_when_over_limit() {
        let mut limiter = SlidingWindow::new(2);
        limiter.acquire();
        limiter.acquire();
        let start = Instant::now();
        limiter.acquire();
        // Deve ter bloqueado brevemente
        assert!(start.elapsed() < Duration::from_secs(2));
    }
}
