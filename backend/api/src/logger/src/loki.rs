use log::{
    Record,
    kv::{Source, Visitor},
};
use reqwest::Client;
use serde::Serialize;
use std::{
    collections::HashMap,
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Serialize)]
struct LokiRequest {
    streams: Vec<LokiStream>,
}

#[derive(Serialize)]
struct LokiStream {
    stream: HashMap<String, String>,
    values: Vec<[String; 2]>,
}

struct LokiVisitor<'a> {
    values: HashMap<String, String>,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> LokiVisitor<'a> {
    fn from_source(source: &'a dyn Source) -> Result<HashMap<String, String>, log::kv::Error> {
        let mut visitor = Self {
            values: HashMap::with_capacity(source.count()),
            _marker: std::marker::PhantomData,
        };
        source.visit(&mut visitor)?;
        Ok(visitor.values)
    }
}

impl<'a> Visitor<'a> for LokiVisitor<'a> {
    fn visit_pair(
        &mut self,
        key: log::kv::Key<'a>,
        value: log::kv::Value<'a>,
    ) -> Result<(), log::kv::Error> {
        self.values.insert(key.to_string(), value.to_string());
        Ok(())
    }
}

#[derive(Clone)]
pub struct LokiLogger {
    url: String,
    labels: HashMap<String, String>,
    client: Client,
}

impl LokiLogger {
    pub fn new(url: &str, labels: Option<HashMap<String, String>>) -> Self {
        Self {
            url: url.to_string(),
            labels: labels.unwrap_or_default(),
            client: Client::new(),
        }
    }

    pub fn log_record(&self, record: &Record) -> Result<(), Box<dyn Error>> {
        let kv_labels = LokiVisitor::from_source(record.key_values())?;
        let mut labels = self.labels.clone();
        labels.extend(kv_labels);
        labels.insert(
            "level".into(),
            record.level().to_string().to_ascii_lowercase(),
        );

        let message = format!("{}", record.args());
        let request = make_request(message, labels)?;

        let client = self.client.clone();
        let url = self.url.clone();
        tokio::spawn(async move {
            if let Err(e) = client.post(url).json(&request).send().await {
                eprintln!("Loki error: {:?}", e);
            }
        });

        Ok(())
    }
}

fn make_request(
    message: String,
    labels: HashMap<String, String>,
) -> Result<LokiRequest, Box<dyn Error>> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_nanos()
        .to_string();

    Ok(LokiRequest {
        streams: vec![LokiStream {
            stream: labels,
            values: vec![[timestamp, message]],
        }],
    })
}
