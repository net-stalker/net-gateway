pub fn get_addr_from_host(host_name: &str) -> String {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("getent hosts {} | awk '{{ print $1 }}'", host_name))
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[macro_export]
macro_rules! set_ips {
    ($config:expr, $callback:expr, $($field:ident),*) => {
        use std::str::FromStr;
        $(
            let ip = $callback(&$config.$field.host_name);

            if let Err(e) = std::net::IpAddr::from_str(&ip) {
                panic!("Error parsing {} ip: {}", stringify!($field), e);
            }
            $config.$field.addr = format!("{}:{}", ip, &$config.$field.port);
        )*
    };
}
#[cfg(test)]
mod tests {
    struct FirstField {
        host_name: String,
        addr: String,
        port: u16,
    }

    struct SecondField {
        host_name: String,
        addr: String,
        port: u16,
    }

    struct Config {
        field1: FirstField,
        field2: SecondField,
    }

    fn mock_get_addr_from_host(host_name: &str) -> String {
        host_name.to_string()
    }

    #[test]
    fn test_set_ips() {
        let mut config = Config {
            field1: FirstField {
                host_name: "0.0.0.0".to_string(),
                addr: "".to_string(),
                port: 8080,
            },
            field2: SecondField {
                host_name: "1.1.1.1".to_string(),
                addr: "".to_string(),
                port: 8081,
            },
        };

        set_ips!(config, mock_get_addr_from_host, field1, field2);

        assert!(!config.field1.addr.is_empty());
        assert!(!config.field2.addr.is_empty());

        assert_eq!(config.field1.addr, format!("{}:{}", config.field1.host_name, config.field1.port));
        assert_eq!(config.field2.addr, format!("{}:{}", config.field2.host_name, config.field2.port));
    }
}