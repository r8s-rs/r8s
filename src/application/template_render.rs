use liquid::{Object, ParserBuilder, Parser, model::Value as LiquidValue};
use serde_json::Value;

pub struct TemplateRender {
    parser: Parser,
}

impl TemplateRender {
    pub fn new() -> Self {
        let parser = ParserBuilder::with_stdlib()
            .build()
            .expect("Failed to build Liquid parser");

        Self { parser }
    }

    fn json_to_liquid(value: serde_json::Value) -> LiquidValue {
        match value {
            serde_json::Value::Null => LiquidValue::Nil,
            serde_json::Value::Bool(b) => LiquidValue::scalar(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    LiquidValue::scalar(i)
                } else if let Some(f) = n.as_f64() {
                    LiquidValue::scalar(f)
                } else {
                    LiquidValue::scalar(n.to_string())
                }
            }
            serde_json::Value::String(s) => LiquidValue::scalar(s),
            serde_json::Value::Array(arr) => {
                LiquidValue::Array(arr.into_iter().map(Self::json_to_liquid).collect())
            }
            serde_json::Value::Object(obj) => {
                let mut liquid_obj = Object::new();
                for (key, val) in obj {
                    liquid_obj.insert(key.into(), Self::json_to_liquid(val));
                }
                LiquidValue::Object(liquid_obj)
            }
        }
    }

    pub fn render_str(&mut self, template: &str, context: Value) -> Result<String, String> {
        dbg!(&context);
        let context = Self::json_to_liquid(context);

        let mut globals = Object::new();

        if let LiquidValue::Object(obj) = context {
            globals = obj;
        }

        dbg!(&globals);

        match self.parser.parse(template) {
            Ok(template) => {
                let rendered = template.render(&globals)
                    .map_err(|e| format!("Erro ao renderizar template: {}", e))?;

                Ok(rendered)
            }
            Err(e) => Err(format!("Erro ao analisar template: {}", e)),
        }
    }
}