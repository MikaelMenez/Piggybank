use std::path::PathBuf;
use tokio::process::Command;
use tokio::{join, spawn};

#[tokio::main]
async fn main() {
    // Caminhos EXATOS seus
    let pasta_backend = std::env::current_dir().unwrap().join("backend");
    let pasta_frontend = std::env::current_dir()
        .unwrap()
        .join("frontend/piggybankfront");
    // ← Correto!

    println!("🚀 Iniciando Backend  -> {}", pasta_backend.display());
    println!("🚀 Iniciando Frontend -> {}", pasta_frontend.display());

    // Backend cargo run (porta 46000 do seu código)
    let backend_handle = spawn(async move {
        let status = Command::new("cargo")
            .current_dir(pasta_backend)
            .arg("run")
            .status()
            .await
            .expect("Backend falhou");

        println!("Backend terminou: {:?}", status);
        status
    });

    // Frontend bun dev
    let frontend_handle = spawn(async move {
        let status = Command::new("bun")
            .current_dir(pasta_frontend)
            .arg("dev")
            .status()
            .await
            .expect("Frontend falhou");

        println!("Frontend terminou: {:?}", status);
        status
    });

    // Roda os dois EM PARALELO (Ctrl+C para parar)
    let (backend_res, frontend_res) = join!(backend_handle, frontend_handle);

    println!("🎉 Tudo finalizado!");
}
