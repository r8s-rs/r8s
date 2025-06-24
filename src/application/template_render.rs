use tera::{Context, Tera};
use serde_json::Value;


pub struct TemplateRender {
    tera: Tera,
}

impl TemplateRender {
    pub fn new() -> Self {
        Self { tera: Tera::default() }
    }

    pub fn render_str(&mut self, template: &str, context: &Value) -> Result<String, String> {
        let context = Context::from_value(context.clone())
            .map_err(|e| format!("Erro ao criar contexto: {}", e))?;
        
        self.tera.render_str(template, &context)
            .map_err(|e| format!("Erro ao renderizar template: {}", e))
    }
}