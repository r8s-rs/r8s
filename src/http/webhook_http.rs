use crate::domain::entities::{HttpRequest as HttpRequestEntity, HttpMethod};
use crate::infrastructure::repositories::WebhookRepository;
use std::{collections::HashMap, net::IpAddr};
use serde_json::{Value, json};
use futures_util::StreamExt;
use tracing::{info, error};
use std::str::FromStr;
use nanoid::nanoid;
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
pub async fn webhook_http(
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

    let method = HttpMethod::from_str(method.as_str());

    if method.is_err() {
        return Ok(HttpResponse::BadGateway().body("method not found"));
    }

    let method = method.unwrap();

    let host = req.connection_info();

    let host = host.host();

    let mut ip: Option<IpAddr> = None;

    if let Some(peer_addr) = req.peer_addr() {
        ip = Some(peer_addr.ip());
    }

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
        dbg!(res);
        return Ok(HttpResponse::NotFound().body("not found"));
    }

    // Extrair parâmetros de consulta (query params)
    let query_params: Value = json!(query.into_inner());

    let query_params = match query_params {
        Value::Null => None,
        Value::Object(ref map) if map.is_empty() => None,
        Value::Array(ref arr) if arr.is_empty() => None,
        Value::String(ref s) if s.is_empty() => None,
        value => Some(value),
    };

    // Extrair dados de formulário, se houver
    let form_data: Option<Value> = match form {
        Some(form_data) => Some(json!(form_data.into_inner())),
        None => None,
    };
    
    // Extrair corpo da requisição (JSON body)
    let mut body_bytes = BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body_bytes.extend_from_slice(&chunk);
    }
    
    // Tentar converter o corpo para JSON
    let body: Option<Value> = if !body_bytes.is_empty() {
        match serde_json::from_slice(&body_bytes) {
            Ok(json_body) => json_body,
            Err(_) => {
                // Se não for JSON válido, retornar como string
                Some(
                    json!(
                        String::from_utf8_lossy(&body_bytes).to_string()
                    )
                )
            }
        }
    } else {
        None
    };
    
    // Extrair headers
    let mut headers = HashMap::new();
    for (key, value) in req.headers() {
        headers.insert(
            key.as_str().to_string(),
            value.to_str().unwrap_or_default().to_string(),
        );
    }

    let headers = if headers.is_empty() {
        None
    } else {
        Some(headers)
    };

    let wh = res.unwrap();

    let http_request = HttpRequestEntity {
        host: host.into(),
        ip,
        path: path.into(),
        method: method.to_string(),
        headers,
        form_data,
        query_params,
        body,
    };

    let id = format!("{}/{}", wh.workflow_id, nanoid!(10));

    let id = id.as_str();

    let insert = state.partitions.webhook_v1_pendings.insert(
        id,
        serde_json::to_vec(&http_request).unwrap(),
    );

    match insert {
        Ok(()) => {
            info!("Sucesso ao inserir webhook na partition");
        }
        Err(e) => {
            error!("Erro ao inserir webhook na partition: {e}");
        }
    }

    Ok(HttpResponse::Ok().json(json!({"success": true})))
}