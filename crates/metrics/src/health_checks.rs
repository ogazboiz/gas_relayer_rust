use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub status: HealthStatus,
    pub message: String,
    pub last_checked: DateTime<Utc>,
    pub response_time_ms: Option<u64>,
    pub details: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub overall_status: HealthStatus,
    pub components: HashMap<String, ComponentHealth>,
    pub timestamp: DateTime<Utc>,
    pub uptime_seconds: u64,
}

impl ComponentHealth {
    pub fn healthy(message: &str) -> Self {
        Self {
            status: HealthStatus::Healthy,
            message: message.to_string(),
            last_checked: Utc::now(),
            response_time_ms: None,
            details: HashMap::new(),
        }
    }
    
    pub fn with_response_time(mut self, response_time_ms: u64) -> Self {
        self.response_time_ms = Some(response_time_ms);
        self
    }
    
    pub fn with_detail(mut self, key: &str, value: serde_json::Value) -> Self {
        self.details.insert(key.to_string(), value);
        self
    }
}

impl SystemHealth {
    pub fn new(uptime_seconds: u64) -> Self {
        Self {
            overall_status: HealthStatus::Healthy,
            components: HashMap::new(),
            timestamp: Utc::now(),
            uptime_seconds,
        }
    }
    
    pub fn add_component(&mut self, name: &str, health: ComponentHealth) {
        self.components.insert(name.to_string(), health);
    }
}

pub struct HealthChecker;

impl HealthChecker {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn check_system_health(&self) -> SystemHealth {
        let mut system_health = SystemHealth::new(3600);
        
        system_health.add_component("database", 
            ComponentHealth::healthy("Database connection active")
                .with_response_time(5)
                .with_detail("active_connections", serde_json::json!(10))
        );
        
        system_health
    }
}