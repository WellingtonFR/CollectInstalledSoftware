# Listar Softwares Instalados

Este projeto em Rust exibe a lista de softwares instalados tanto no Windows quanto no Linux.

## 📌 Funcionalidades

- **Windows**: Obtém os softwares instalados acessando o Registro do Windows.
- **Linux**: Lista os pacotes instalados usando `dpkg -l` (Debian/Ubuntu) ou `rpm -qa` (Fedora/RHEL).
- Suporte multiplataforma.

## 🚀 Instalação

### 1. Instale o Rust

Certifique-se de que o Rust está instalado:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verifique a instalação:

```sh
rustc --version
cargo --version
```

### 2. Clone o repositório

```sh
git clone https://github.com/seu-usuario/listar-softwares.git
cd listar-softwares
```

### 3. Compile e execute

Para Windows:

```sh
cargo run
```

Para Linux:

```sh
cargo run
```

## 📦 Dependências

As dependências são definidas no `Cargo.toml`:

```toml
[target.'cfg(windows)'.dependencies]
winreg = "0.52"

[target.'cfg(unix)'.dependencies]
which = "4.4.2"
```

## ⚙️ Como Funciona

### Windows

O código acessa o Registro do Windows e exibe os softwares instalados.

### Linux

O código usa comandos do sistema para listar pacotes instalados:

- `dpkg -l` para sistemas baseados em Debian/Ubuntu.
- `rpm -qa` para sistemas baseados em Fedora/RHEL.

## 🛠️ Tecnologias Utilizadas

- **Rust**
- **Cargo** (gerenciador de pacotes do Rust)
- **winreg** (para acessar o Registro do Windows)
- **which** (para verificar a disponibilidade de comandos no Linux)
