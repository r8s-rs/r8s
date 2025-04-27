use crate::infrastructure::repositories::WebhookRepository;
use std::collections::HashMap;
use serde_json::{Value, json};
use futures_util::StreamExt;
use crate::State;
use actix_web::{
    HttpResponse,
    HttpRequest,
    Result,
    Error,
    web::{
        BytesMut,
        Payload,
        Query,
        Form,
        Data,
        Path,
    },
};


// Handler que processa todos os tipos de requisições HTTP
pub async fn webhook(
    path: Path<String>,
    req: HttpRequest,
    query: Query<HashMap<String, String>>,
    form: Option<Form<HashMap<String, String>>>,
    mut payload: Payload,
    state: Data<State>,
) -> Result<HttpResponse, Error> {
    let path = path.into_inner();

    let path = path.as_str();

    let method = req.method()
        .to_string()
        .to_lowercase();

    dbg!(path, &method);

    let res = WebhookRepository::get_by_path(
        &state.db,
        path,
        &method,
    ).await;

    if let Err(e) = res {
        return Ok(
            HttpResponse::InternalServerError()
                .body(e.to_string())
        );
    }

    let res = res.unwrap();

    if res.is_none() {
        return Ok(HttpResponse::NotFound().body("not found"));
    }

    // Extrair parâmetros de consulta (query params)
    let query: Value = json!(query.into_inner());
    
    // Extrair dados de formulário, se houver
    let form_data: Value = match form {
        Some(form_data) => json!(form_data.into_inner()),
        None => json!({}),
    };
    
    // Extrair corpo da requisição (JSON body)
    let mut body_bytes = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body_bytes.extend_from_slice(&chunk);
    }
    
    // Tentar converter o corpo para JSON
    let body: Value = if !body_bytes.is_empty() {
        match serde_json::from_slice(&body_bytes) {
            Ok(json_body) => json_body,
            Err(_) => {
                // Se não for JSON válido, retornar como string
                json!(String::from_utf8_lossy(&body_bytes).to_string())
            }
        }
    } else {
        json!({})
    };
    
    // Extrair headers
    let mut headers = HashMap::new();
    for (key, value) in req.headers() {
        headers.insert(
            key.as_str().to_string(),
            value.to_str().unwrap_or_default().to_string(),
        );
    }
    
    // Montar a resposta
    let response = json!({
        "success": true,
        "request": {
            "method": method,
            "path": path,
            "query": query,
            "form_data": form_data,
            "body": body,
            "headers": headers
        }
    });
    
    // Retornar resposta JSON
    Ok(HttpResponse::Ok().json(response))
}