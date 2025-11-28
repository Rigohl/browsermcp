// Tool: security_testing_sandbox
// Descripción: Ambiente de testing seguro para aprender seguridad
// Propósito: Educación en seguridad web sin problemas legales

use wasm_bindgen::prelude::*;
use serde_json::json;

#[wasm_bindgen]
pub struct SecurityTestingSandbox;

#[wasm_bindgen]
impl SecurityTestingSandbox {
    /// Crear ambiente sandbox para testing
    #[wasm_bindgen]
    pub fn create_sandbox() -> String {
        json!({
            "name": "security_testing_sandbox",
            "description": "Ambiente aislado y legal para testing de seguridad",
            "environments": {
                "local": {
                    "url": "http://127.0.0.1:8888",
                    "purpose": "Testing local sin internet",
                    "risks": "None - completamente aislado"
                },
                "docker": {
                    "image": "owasp/juice-shop (público)",
                    "purpose": "App vulnerable para aprender",
                    "legal": "✅ Oficial OWASP"
                },
                "vulnerable_apps": [
                    "OWASP Juice Shop",
                    "DVWA (Damn Vulnerable Web Application)",
                    "WebGoat",
                    "Mutillidae"
                ]
            },
            "what_you_can_test": [
                "XSS (Cross-Site Scripting)",
                "SQL Injection",
                "CSRF (Cross-Site Request Forgery)",
                "Authentication bypass",
                "Authorization issues",
                "Sensitive data exposure",
                "Security misconfiguration"
            ],
            "what_you_cannot_test": [
                "Servidores de terceros sin permiso",
                "Datos personales reales",
                "Sistemas en producción",
                "Redes internas sin autorización"
            ]
        }).to_string()
    }

    /// Mapeo de vulnerabilidades educativas
    #[wasm_bindgen]
    pub fn vulnerability_mapping() -> String {
        json!({
            "name": "vulnerability_mapping",
            "description": "Mapeo educativo de tipos de vulnerabilidades",
            "vulnerabilities": {
                "injection": {
                    "types": ["SQL Injection", "Command Injection", "LDAP Injection"],
                    "how_to_learn": "En OWASP Juice Shop o DVWA",
                    "how_to_fix": "Usar parameterized queries, input validation",
                    "cvss_score": "7.5-9.8"
                },
                "authentication": {
                    "types": ["Weak passwords", "Session management", "Credential stuffing"],
                    "how_to_learn": "Quebrar contraseñas débiles en sandbox",
                    "how_to_fix": "Strong hashing, MFA, secure sessions",
                    "cvss_score": "7.5-9.9"
                },
                "sensitive_data": {
                    "types": ["Unencrypted data", "Exposed credentials", "PII"],
                    "how_to_learn": "Buscar en responses/storage del sandbox",
                    "how_to_fix": "Encryption, data masking, secure transmission",
                    "cvss_score": "7.5-9.1"
                },
                "xxe": {
                    "types": ["XML External Entity"],
                    "how_to_learn": "En OWASP WebGoat",
                    "how_to_fix": "Deshabilitar XML features peligrosas",
                    "cvss_score": "7.5-9.8"
                }
            }
        }).to_string()
    }

    /// Testing ético checklist
    #[wasm_bindgen]
    pub fn ethical_testing_checklist() -> String {
        json!({
            "name": "ethical_testing_checklist",
            "before_testing": [
                "✅ ¿Tengo autorización explícita escrita?",
                "✅ ¿Es ambiente controlado (local/sandbox)?",
                "✅ ¿No contiene datos reales de terceros?",
                "✅ ¿Respetaré rate limiting y robots.txt?",
                "✅ ¿No busco datos sensibles o PII?"
            ],
            "during_testing": [
                "✅ Documentar todo lo que hago",
                "✅ No acceder a datos de otros usuarios",
                "✅ No modificar datos sin permiso",
                "✅ No realizar DoS/DDoS",
                "✅ Respetar 'no robots' directives"
            ],
            "after_testing": [
                "✅ Reportar hallazgos responsablemente",
                "✅ Dar tiempo para que lo arreglen",
                "✅ Limpiar mis rastros",
                "✅ No compartir exploits públicamente",
                "✅ Destruir datos recolectados si es necesario"
            ]
        }).to_string()
    }

    /// Recursos para aprender seguridad legalmente
    #[wasm_bindgen]
    pub fn learning_resources() -> String {
        json!({
            "name": "learning_resources",
            "official_platforms": {
                "hack_the_box": "https://www.hackthebox.com - CTF legal",
                "try_hack_me": "https://tryhackme.com - Labs legales",
                "owasp": "https://owasp.org - Educación oficial",
                "portswigger": "https://portswigger.net/web-security - Labs gratis"
            },
            "vulnerable_apps": {
                "juice_shop": "OWASP app vulnerable oficial",
                "dvwa": "Damn Vulnerable Web App",
                "webgoat": "Aplicación educativa OWASP",
                "mutillidae": "Vulnerable para aprender"
            },
            "certifications": [
                "CEH (Certified Ethical Hacker)",
                "OSCP (Offensive Security Certified Professional)",
                "CompTIA Security+",
                "Elearnsecurity eJPT"
            ],
            "bug_bounty_legal": {
                "bugcrowd": "https://www.bugcrowd.com",
                "hackerone": "https://www.hackerone.com",
                "intigriti": "https://www.intigriti.com",
                "requirement": "Autorización explícita de la empresa"
            }
        }).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_creation() {
        let result = SecurityTestingSandbox::create_sandbox();
        assert!(result.contains("security_testing_sandbox"));
    }

    #[test]
    fn test_ethical_checklist() {
        let result = SecurityTestingSandbox::ethical_testing_checklist();
        assert!(result.contains("ethical_testing_checklist"));
    }
}
