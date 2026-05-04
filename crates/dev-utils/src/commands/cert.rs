use anyhow::Result;
use owo_colors::OwoColorize;
use std::net::TcpStream;
use x509_parser::prelude::*;

pub async fn inspect(host: &str, port: u16) -> Result<()> {
    println!("{} {}:{}", "Inspecting certificate for".bold(), host.cyan(), port.cyan());

    // We use a synchronous connection here for simplicity with native-tls
    // in a separate thread or just use a sync implementation since it's a CLI tool.
    let connector = native_tls::TlsConnector::new()?;
    let stream = TcpStream::connect(format!("{}:{}", host, port))?;
    let tls_stream = connector.connect(host, stream).map_err(|e| anyhow::anyhow!("TLS connection failed: {}", e))?;

    let cert = tls_stream.peer_certificate()?;
    if let Some(cert) = cert {
        let der = cert.to_der()?;
        let (_, x509) = X509Certificate::from_der(&der)?;

        println!("\n{}", "--- Certificate Information ---".bold().green());
        println!("{}: {}", "Subject".bold(), x509.subject());
        println!("{}: {}", "Issuer".bold(), x509.issuer());
        println!("{}: {}", "Version".bold(), x509.version());
        println!("{}: {}", "Serial".bold(), x509.tbs_certificate.serial);
        
        let validity = x509.validity();
        println!("{}: {}", "Not Before".bold(), validity.not_before);
        println!("{}: {}", "Not After".bold(), validity.not_after);

        if validity.is_valid() {
            println!("{}: {}", "Status".bold(), "VALID".green().bold());
        } else {
            println!("{}: {}", "Status".bold(), "EXPIRED or NOT YET VALID".red().bold());
        }

        println!("\n{}", "--- Subject Alternative Names ---".bold().blue());
        if let Ok(Some(ext)) = x509.subject_alternative_name() {
            for name in &ext.value.general_names {
                println!("  - {:?}", name);
            }
        } else {
            println!("  (None)");
        }
    } else {
        println!("{}", "No certificate found.".red());
    }

    Ok(())
}
