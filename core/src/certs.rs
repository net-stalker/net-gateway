use std::{io::{self, BufReader}, fs::File, iter};
use rustls_pemfile::{read_one, Item, rsa_private_keys};
use rustls_pki_types::{CertificateDer, PrivatePkcs1KeyDer};
use x509_parser::oid_registry::asn1_rs::FromDer;
use rustls::Certificate;


pub fn read_certificate_from_file(file_path: &str) -> io::Result<Certificate> {
    let der = std::fs::read(file_path)?;
    match x509_parser::certificate::X509Certificate::from_der(der.as_slice()) {
        Ok((bytes, _)) => Ok(Certificate(bytes.to_vec())),
        Err(err) => Err(io::Error::new(io::ErrorKind::InvalidData, err)),
    }
}

pub fn read_pem_cert(file_path: &str) -> io::Result<CertificateDer<'static>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    for item in iter::from_fn(|| read_one(&mut reader).transpose()) {
        match item.unwrap() {
            Item::X509Certificate(cert) => return Ok(cert),
            _ => println!("unhandled item"),
        }
    }
    Err(io::Error::new(io::ErrorKind::InvalidData, "no certificate found"))
}

pub fn read_pem_key(file_path: &str) -> io::Result<PrivatePkcs1KeyDer<'static>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let keys = rsa_private_keys(&mut reader);
    for key in keys {
        match key {
            Ok(pem) => {
                // Handle the Pem object
                return Ok(pem);
            }
            Err(e) => {
                // Handle the error
                return Err(io::Error::new(io::ErrorKind::InvalidData, e));
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::InvalidData, "no key found"))
}
