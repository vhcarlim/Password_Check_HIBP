use sha1::{Digest, Sha1};
use std::error::Error;

// 1. Função pura: transforma uma senha em hash SHA-1 em caixa alta
fn calcular_hash_sha1(senha: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(senha.as_bytes());
    let resultado = hasher.finalize();
    format!("{:X}", resultado) // :X formata os bytes em hexadecimal maiúsculo
}

// 2. Consulta a API e devolve quantas vezes o sufixo apareceu
fn consultar_vazamentos(prefixo: &str, sufixo: &str) -> Result<u32, Box<dyn Error>> {
    let url = format!("https://api.pwnedpasswords.com/range/{}", prefixo);

    // Cria um cliente HTTP com User-Agent (exigido pela API)
    let cliente = reqwest::blocking::Client::builder()
        .user_agent("ValidadorDeSenhas-Rust")
        .build()?;

    let resposta = cliente.get(&url).send()?.text()?;

    // Percorre cada linha retornada pelo servidor
    for linha in resposta.lines() {
        // As linhas chegam no formato: SUFIXO:QUANTIDADE
        let partes: Vec<&str> = linha.split(':').collect();

        if partes.len() == 2 && partes[0] == sufixo {
            let quantidade: u32 = partes[1].trim().parse().unwrap_or(0);
            return Ok(quantidade);
        }
    }

    // Se o sufixo não estiver na lista retornada, vazamentos = 0
    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Verificador Seguro de Exposição de Senhas ===");
    println!("Aviso: Sua senha completa NUNCA sairá deste computador.\n");

    // Lê a senha de forma oculta no terminal
    let senha = rpassword::prompt_password("Digite a senha que deseja testar: ")?;

    if senha.is_empty() {
        println!("Nenhuma senha foi informada.");
        return Ok(());
    }

    // Calcula o hash SHA-1 de 40 caracteres
    let hash_completo = calcular_hash_sha1(&senha);

    // Fatiamento (Slicing): pega os 5 primeiros e os 35 restantes
    let prefixo = &hash_completo[..5];
    let sufixo = &hash_completo[5..];

    println!("\n[Status] Prefixo enviado para consulta: {}", prefixo);
    println!("[Status] O sufixo ({}) permanece estritamente local.", sufixo);

    let vazamentos = consultar_vazamentos(prefixo, sufixo)?;

    println!("\n--- Resultado da Análise ---");
    if vazamentos > 0 {
        println!(
            "ALERTA: Esta senha já apareceu em vazamentos públicos {} vezes!",
            vazamentos
        );
        println!("Recomendação: Não utilize esta senha em nenhum serviço.");
    } else {
        println!("BOA NOTÍCIA: Nenhum vazamento público encontrado para esta senha.");
    }

    Ok(())
}
