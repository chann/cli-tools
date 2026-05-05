use anyhow::Result;
use owo_colors::OwoColorize;
use std::net::TcpStream;
use std::path::Path;
use std::fs;
use x509_parser::prelude::*;

pub async fn inspect_remote(host: &str, port: u16) -> Result<()> {
    println!("{} {}:{}", "Inspecting remote certificate for".bold(), host.cyan(), port.cyan());

    let connector = native_tls::TlsConnector::new()?;
    let stream = TcpStream::connect(format!("{}:{}", host, port))?;
    let tls_stream = connector.connect(host, stream).map_err(|e| anyhow::anyhow!("TLS connection failed: {}", e))?;

    let cert = tls_stream.peer_certificate()?;
    if let Some(cert) = cert {
        let der = cert.to_der()?;
        display_cert_info(&der)?;
    } else {
        println!("{}", "No certificate found.".red());
    }

    Ok(())
}

pub fn inspect_file(path: &Path) -> Result<()> {
    println!("{} {}", "Inspecting local certificate file:".bold(), path.display().cyan());

    let content = fs::read(path)?;
    
    // Try as DER first, then PEM
    if let Ok((_, x509)) = X509Certificate::from_der(&content) {
        display_x509_info(&x509)?;
    } else {
        // Try to decode PEM
        let p = ::pem::parse(&content)?;
        if p.tag() == "CERTIFICATE" {
            display_cert_info(p.contents())?;
        } else {
            anyhow::bail!("Unsupported PEM tag: {}", p.tag());
        }
    }

    Ok(())
}

fn display_cert_info(der: &[u8]) -> Result<()> {
    let (_, x509) = X509Certificate::from_der(der)?;
    display_x509_info(&x509)
}

fn display_x509_info(x509: &X509Certificate) -> Result<()> {
    println!("\n{}", "--- Certificate Information ---".bold().green());
    println!("{:<15}: {}", "Subject".bold(), x509.subject());
    println!("{:<15}: {}", "Issuer".bold(), x509.issuer());
    println!("{:<15}: {}", "Version".bold(), x509.version());
    println!("{:<15}: {}", "Serial".bold(), x509.tbs_certificate.raw_serial_as_string());
    
    let validity = x509.validity();
    println!("{:<15}: {}", "Not Before".bold(), validity.not_before);
    println!("{:<15}: {}", "Not After".bold(), validity.not_after);

    if validity.is_valid() {
        println!("{:<15}: {}", "Status".bold(), "VALID".green().bold());
    } else {
        println!("{:<15}: {}", "Status".bold(), "EXPIRED or NOT YET VALID".red().bold());
    }

    println!("\n{}", "--- Signature Algorithm ---".bold().blue());
    println!("{:<15}: {}", "Algorithm".bold(), x509.signature_algorithm.algorithm);

    println!("\n{}", "--- Subject Alternative Names ---".bold().blue());
    if let Ok(Some(ext)) = x509.subject_alternative_name() {
        for name in &ext.value.general_names {
            println!("  - {:?}", name);
        }
    } else {
        println!("  (None)");
    }

    println!("\n{}", "--- Public Key Information ---".bold().blue());
    let pki = &x509.tbs_certificate.subject_pki;
    println!("{:<15}: {}", "Algorithm".bold(), pki.algorithm.algorithm);

    Ok(())
}
