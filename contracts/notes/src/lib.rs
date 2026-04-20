#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data untuk sertifikat
#[contracttype]
#[derive(Clone, Debug)]
pub struct Certificate {
    pub id: u64,
    pub name: String,
    pub course: String,
}

// Key untuk menyimpan data di blockchain
const CERT_DATA: Symbol = symbol_short!("CERT_DATA");

#[contract]
pub struct CertificateContract;

#[contractimpl]
impl CertificateContract {
    // READ: Melihat daftar semua sertifikat yang diterbitkan
    pub fn get_certs(env: Env) -> Vec<Certificate> {
        env.storage()
            .instance()
            .get(&CERT_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // CREATE: Menerbitkan sertifikat baru
    pub fn issue_cert(env: Env, name: String, course: String) -> String {
        let mut certs = Self::get_certs(env.clone());
        
        // Auto-increment ID sederhana
        let new_id = (certs.len() as u64) + 1;
        
        let new_cert = Certificate {
            id: new_id,
            name,
            course,
        };
        
        certs.push_back(new_cert);
        env.storage().instance().set(&CERT_DATA, &certs);
        
        String::from_str(&env, "Certificate successfully issued!")
    }

    // DELETE: Menarik ulang (mencabut) sertifikat jika ada kesalahan
    pub fn revoke_cert(env: Env, id: u64) -> String {
        let certs = Self::get_certs(env.clone());
        let mut valid_certs = Vec::new(&env);
        let mut found = false;

        for cert in certs.iter() {
            if cert.id != id {
                valid_certs.push_back(cert);
            } else {
                found = true; // Sertifikat ditemukan dan tidak dimasukkan ke list baru
            }
        }

        if found {
            env.storage().instance().set(&CERT_DATA, &valid_certs);
            String::from_str(&env, "Certificate officially revoked!")
        } else {
            String::from_str(&env, "Certificate ID not found!")
        }
    }
}