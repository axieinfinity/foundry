use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::process::Command;
use std::str;
use actix_cors::Cors;
use std::time::Duration;

async fn run_cast_command(tx: web::Path<String>) -> Result<String, String> {
    let output = Command::new("cast")
        .args([
            "run",
            tx.as_str(),
            "--no-rate-limit",
            "-r",
            "https://api-archived.roninchain.com/rpc",
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        match str::from_utf8(&output.stdout) {
            Ok(stdout) => Ok(stdout.to_string()),
            Err(_) => Err("Output was not valid UTF-8".to_string()),
        }
    } else {
        match str::from_utf8(&output.stderr) {
            Ok(stderr) => Err(stderr.to_string()),
            Err(_) => Err("Error output was not valid UTF-8".to_string()),
        }
    }
}

#[get("/run_cast/{tx}")]
async fn cast_command_handler(tx: web::Path<String>) -> impl Responder {
    match run_cast_command(tx).await {
        Ok(output) => HttpResponse::Ok().content_type("text/plain").body(output),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error executing command: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new().wrap(cors).service(cast_command_handler)
    })
    .keep_alive(Duration::from_secs(600))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}