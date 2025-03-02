#[cfg(windows)]
mod windows {
    use winreg::enums::*;
    use winreg::RegKey;

    pub fn listar_softwares() {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let path = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall";

        if let Ok(subkey) = hklm.open_subkey(path) {
            for item in subkey.enum_keys().filter_map(Result::ok) {
                let app_key = subkey.open_subkey(&item);
                if let Ok(app_key) = app_key {
                    if let Ok(name) = app_key.get_value::<String, _>("DisplayName") {
                        println!("{}", name);
                    }
                }
            }
        }
    }
}

#[cfg(unix)]
mod linux {
    use std::process::Command;

    pub fn listar_softwares() {
        let output = if Command::new("which").arg("dpkg").output().is_ok() {
            Command::new("dpkg").arg("-l").output()
        } else if Command::new("which").arg("rpm").output().is_ok() {
            Command::new("rpm").arg("-qa").output()
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Nenhum gerenciador de pacotes compatível encontrado"))
        };

        match output {
            Ok(output) => {
                let list = String::from_utf8_lossy(&output.stdout);
                for line in list.lines().skip(5) {
                    println!("{}", line);
                }
            }
            Err(e) => eprintln!("Erro ao obter a lista de softwares: {:?}", e),
        }
    }
}

fn main() {
    #[cfg(windows)]
    {
        println!("Softwares instalados no Windows:");
        windows::listar_softwares();
    }

    #[cfg(unix)]
    {
        println!("Softwares instalados no Linux:");
        linux::listar_softwares();
    }
}
