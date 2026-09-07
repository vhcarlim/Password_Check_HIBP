# 🛡️ PassCheck CLI — Verificador Seguro de Exposição de Senhas

Uma ferramenta de linha de comando (CLI) desenvolvida em **Rust** para verificar se uma senha já foi exposta em vazamentos de dados públicos catalogados pelo serviço [Have I Been Pwned (HIBP)](https://haveibeenpwned.com/), utilizando o modelo de privacidade **k-Anonymity**.

---

## 🔒 Modelo de Ameaça e Privacidade (Zero Trust)

A maioria dos usuários hesita em checar credenciais em sites de terceiros por receio de captura ou tráfego indevido de dados sensíveis. Esta ferramenta foi desenhada sob os princípios de **Zero Knowledge** e **segurança na camada de transporte/aplicação**:

1. **A senha nunca sai da máquina:** A entrada é lida de forma oculta via terminal (sem eco visual na tela).
2. **Hashing Criptográfico Local:** A senha em texto plano é convertida em um digest SHA-1 localmente.
3. **Princípio de k-Anonymity:**
   * Apenas os **5 primeiros caracteres hexadecimais** do hash SHA-1 são enviados à API do HIBP via HTTPS.
   * O servidor responde com uma lista de milhares de sufixos de hashes conhecidos que compartilham o mesmo prefixo de 5 caracteres.
   * A comparação do sufixo (os 35 caracteres restantes) é realizada **estritamente em memória local**.
   * É matematicamente impossível para qualquer intermediário de rede ou para o próprio servidor do HIBP reconstruir a senha original a partir de apenas 5 caracteres do hash.

---

## 🚀 Tecnologias e Crates Utilizadas

* **[Rust](https://www.rust-lang.org/)**: Linguagem de sistemas com foco em segurança de memória e concorrência sem garbage collector.
* **`sha1`**: Implementação pura em Rust do algoritmo de hash SHA-1 para geração do digest de busca.
* **`reqwest`** *(blocking)*: Cliente HTTP moderno para consulta à API REST pública do *Pwned Passwords*.
* **`rpassword`**: Captura segura de senhas via terminal sem exibir os caracteres digitados no console.

---

## 📦 Como Clonar e Executar

### Pré-requisitos
* Ter o **Rust** e o gerenciador de pacotes **Cargo** instalados ([rustup.rs](https://rustup.rs/)).

### Instalação e Execução

1. Clone este repositório:
```
git clone https://github.com/vhcarlim/Password_Check_HIBP.git
cd Password_Check_HIBP
````
# Compile e execute a aplicação:
````
cargo run --release
````

🛡️ Auditoria de Dependências e Segurança
Este projeto adota controle estrito de Supply Chain Security. O arquivo Cargo.lock contém os hashes criptográficos exatos de todas as bibliotecas dependentes.

Para auditar as dependências contra vulnerabilidades conhecidas cadastradas no RustSec Advisory Database:

# Instalação do auditor oficial da comunidade Rust
````
cargo install cargo-audit
````
# Execução da análise de segurança
````
cargo audit
````
📄 Licença
Distribuído sob a licença MIT. Veja LICENSE para mais informações.
