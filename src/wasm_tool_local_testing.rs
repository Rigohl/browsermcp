// Tool: local_server_testing
// Descripción: Inicia servidor web local para testing seguro
// Propósito: Testing de formularios, CAPTCHA, navegación sin problemas legales

use wasm_bindgen::prelude::*;
use serde_json::json;

#[wasm_bindgen]
pub struct LocalServerTesting;

#[wasm_bindgen]
impl LocalServerTesting {
    /// Crear servidor local con formularios de prueba
    #[wasm_bindgen]
    pub fn create_test_server() -> String {
        json!({
            "name": "local_server_testing",
            "description": "Inicia servidor web local en http://127.0.0.1:8888 con formularios de prueba",
            "features": [
                "Formularios simples y complejos",
                "CAPTCHA simulado (no real)",
                "Validación de datos",
                "Autenticación local",
                "Rate limiting simulado",
                "Bot detection simulado"
            ],
            "endpoints": {
                "/": "Landing page",
                "/login": "Formulario de login",
                "/register": "Formulario de registro",
                "/forms/simple": "Formulario simple",
                "/forms/complex": "Formulario complejo",
                "/api/data": "Endpoint para extraer datos",
                "/captcha": "CAPTCHA simulado",
                "/protected": "Página protegida"
            },
            "usage": {
                "start": "local_server_testing.start_server()",
                "stop": "local_server_testing.stop_server()",
                "reset": "local_server_testing.reset_data()"
            }
        }).to_string()
    }

    /// Generar datos ficticios para testing
    #[wasm_bindgen]
    pub fn generate_test_data(count: usize) -> String {
        json!({
            "name": "generate_test_data",
            "description": format!("Genera {} registros ficticios para testing", count),
            "data": {
                "users": count,
                "fields": [
                    "name",
                    "email",
                    "phone",
                    "address",
                    "company",
                    "website"
                ],
                "formats": ["json", "csv", "sql"],
                "usage": "Usar para poblar servidor local o base de datos de prueba"
            }
        }).to_string()
    }

    /// Simular CAPTCHA para testing
    #[wasm_bindgen]
    pub fn create_fake_captcha() -> String {
        json!({
            "name": "create_fake_captcha",
            "description": "Crea CAPTCHA simulado SOLO para testing local (no real)",
            "types": [
                "image_text", // Texto en imagen
                "math_puzzle", // Puzzle matemático
                "click_squares", // Clickear cuadrados
                "audio_captcha" // CAPTCHA de audio
            ],
            "warning": "⚠️ SOLO PARA TESTING LOCAL - NO USAR EN PRODUCCIÓN",
            "usage": {
                "create": "create_fake_captcha()",
                "verify": "verify_captcha(token)",
                "solve_time": "0-3 segundos (simulado)"
            }
        }).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_server() {
        let result = LocalServerTesting::create_test_server();
        assert!(result.contains("local_server_testing"));
    }

    #[test]
    fn test_fake_captcha() {
        let result = LocalServerTesting::create_fake_captcha();
        assert!(result.contains("TESTING LOCAL"));
    }
}
