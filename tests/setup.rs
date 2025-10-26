// // tests/setup.rs
// use once_cell::sync::Lazy;
// use std::process::Command;
// use std::sync::Mutex;

// static RAY_GUARD: Lazy<Mutex<Option<RayGuard>>> = Lazy::new(|| Mutex::new(None));
// pub static RAY_DASHBOARD_URL: &str = "http://127.0.0.1:8265";

// pub struct RayGuard;

// impl RayGuard {
//     /// Start Ray cluster if not already started
//     pub fn start() {
//         let mut guard = RAY_GUARD.lock().unwrap();

//         if guard.is_none() {
//             println!("Starting Ray via docker-compose...");
//             Command::new("docker")
//                 .args(["compose", "-f", "docker-compose.yaml", "up", "-d"])
//                 .status()
//                 .expect("Failed to start docker-compose");

//             *guard = Some(RayGuard);
//         }
//     }
// }

// impl Drop for RayGuard {
//     fn drop(&mut self) {
//         println!("Stopping Ray via docker-compose...");
//         Command::new("docker")
//             .args(["compose", "-f", "docker-compose.yaml", "down"])
//             .status()
//             .expect("Failed to stop docker-compose");
//     }
// }
