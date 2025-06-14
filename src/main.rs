// use actix_web::{App, HttpServer, Responder, get, web};

// #[get("/customerOnboarding")]
// async fn customer_onboarding(info: web::Query<serde_json::Value>) -> impl Responder {
//     let name_str = info
//         .get("name")
//         .and_then(|v| v.as_str())
//         .unwrap_or("Guest")
//         .to_string();

//     let engine = rhai::Engine::new();
//     let mut scope = rhai::Scope::new();
//     scope.push("final_name", name_str.to_uppercase());
//     let script_to_eval = "final_name";
//     let result: Result<String, _> = engine.eval_with_scope(&mut scope, script_to_eval);

//     match result {
//         Ok(processed_name) => {
//             format!(
//                 "Dear {}, thank you for your order at BigCorp.",
//                 processed_name
//             )
//         }
//         Err(e) => {
//             format!("An error occurred: {}", e)
//         }
//     }
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     println!("Listening on http://localhost:3000 ...");
//     HttpServer::new(|| App::new().service(customer_onboarding))
//         .bind(("127.0.0.1", 3000))?
//         .run()
//         .await
// }

// EXAMPLE with command injection since i encountered problems with rhai in my system
// It's a different flavor, but it belongs to the exact same family of vulnerabilities and highlights the same fundamental mistake:
// Mixing untrusted user data with executable code in a dynamically generated string.
// The ability to execute terminal commands is one of the most critical impacts of an injection vulnerability.
// The JavaScript eval example and the sh -c example are just two different paths to that same dangerous outcome.

use actix_web::{App, HttpServer, Responder, get, web};
use std::process::Command;

#[get("/")]
async fn index(info: web::Query<serde_json::Value>) -> impl Responder {
    let name = info.get("name").and_then(|v| v.as_str()).unwrap_or("Guest");

    // VULNERABLE LINE: The user input is no longer inside quotes,
    // so the shell is free to interpret special characters like ';'.
    let command_to_run = format!("echo Welcome, {}", name);
    let output = Command::new("sh").arg("-c").arg(&command_to_run).output();

    match output {
        Ok(o) => {
            if o.status.success() {
                String::from_utf8_lossy(&o.stdout).to_string()
            } else {
                String::from_utf8_lossy(&o.stderr).to_string()
            }
        }
        Err(e) => format!("Failed to execute command: {}", e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on http://localhost:3000 ...");
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}

// curl 'http://localhost:3000/?name=Edgar;whoami' # <username> will be printed
