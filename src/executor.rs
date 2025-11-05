// Async executor module - handles blocking operations without freezing UI

use crate::dns::providers::DNSProvider;
use crate::error::{DnsError, DnsResult};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Speed test result with async execution
#[derive(Clone, Default)]
pub struct AsyncSpeedTestResult {
    pub provider: String,
    pub primary_ping: Option<f64>,
    pub secondary_ping: Option<f64>,
    pub avg_ping: Option<f64>,
}

/// State of speed test execution
#[derive(Clone)]
pub enum SpeedTestState {
    Idle,
    Running { progress: usize, total: usize },
    Completed(Vec<AsyncSpeedTestResult>),
    Failed(String),
}

/// Async executor for DNS operations
/// Prevents UI freezing by running operations in background thread
pub struct AsyncExecutor {
    speed_test_state: Arc<Mutex<SpeedTestState>>,
}

impl AsyncExecutor {
    pub fn new() -> Self {
        Self {
            speed_test_state: Arc::new(Mutex::new(SpeedTestState::Idle)),
        }
    }

    /// Start speed test in background thread
    pub fn start_speed_test(&self, providers: Vec<DNSProvider>) {
        let state = self.speed_test_state.clone();
        
        // Update state to Running
        if let Ok(mut s) = state.lock() {
            *s = SpeedTestState::Running {
                progress: 0,
                total: providers.len(),
            };
        }

        thread::spawn(move || {
            let mut results = Vec::new();
            let total = providers.len();

            for (idx, provider) in providers.iter().enumerate() {
                // Update progress
                if let Ok(mut s) = state.lock() {
                    *s = SpeedTestState::Running {
                        progress: idx + 1,
                        total,
                    };
                }

                // Perform ping with timeout
                let primary_ping = Self::ping_with_timeout(&provider.primary, Duration::from_secs(2));
                let secondary_ping = Self::ping_with_timeout(&provider.secondary, Duration::from_secs(2));

                // Calculate average
                let mut pings = Vec::new();
                if let Some(p) = primary_ping {
                    pings.push(p);
                }
                if let Some(p) = secondary_ping {
                    pings.push(p);
                }

                let avg_ping = if !pings.is_empty() {
                    Some(pings.iter().sum::<f64>() / pings.len() as f64)
                } else {
                    None
                };

                results.push(AsyncSpeedTestResult {
                    provider: provider.name.clone(),
                    primary_ping,
                    secondary_ping,
                    avg_ping,
                });
            }

            // Sort by average ping
            results.sort_by(|a, b| match (a.avg_ping, b.avg_ping) {
                (Some(a_ping), Some(b_ping)) => a_ping.partial_cmp(&b_ping).unwrap_or(std::cmp::Ordering::Equal),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            });

            // Update state to Completed
            if let Ok(mut s) = state.lock() {
                *s = SpeedTestState::Completed(results);
            }
        });
    }

    /// Get current speed test state
    pub fn get_speed_test_state(&self) -> SpeedTestState {
        self.speed_test_state
            .lock()
            .map(|s| s.clone())
            .unwrap_or(SpeedTestState::Idle)
    }

    /// Reset speed test state
    pub fn reset_speed_test(&self) {
        if let Ok(mut s) = self.speed_test_state.lock() {
            *s = SpeedTestState::Idle;
        }
    }

    /// Ping with timeout to prevent hanging
    fn ping_with_timeout(ip: &str, timeout: Duration) -> Option<f64> {
        // Validate IP first
        if crate::validation::validate_ip_address(ip).is_err() {
            return None;
        }

        let ip = ip.to_string();
        let result = Arc::new(Mutex::new(None));
        let result_clone = result.clone();

        let handle = thread::spawn(move || {
            if let Some(ping) = Self::ping_internal(&ip) {
                if let Ok(mut r) = result_clone.lock() {
                    *r = Some(ping);
                }
            }
        });

        // Wait with timeout
        thread::sleep(timeout);

        // Try to get result
        if let Ok(r) = result.lock() {
            *r
        } else {
            None
        }
    }

    /// Internal ping implementation (blocking)
    fn ping_internal(ip: &str) -> Option<f64> {
        use std::process::Command;

        let command = format!(
            "Test-Connection -ComputerName {} -Count 1 | Select-Object -ExpandProperty ResponseTime",
            ip
        );

        match Command::new(r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe")
            .arg("-Command")
            .arg(&command)
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    stdout.trim().parse::<f64>().ok()
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Default for AsyncExecutor {
    fn default() -> Self {
        Self::new()
    }
}
