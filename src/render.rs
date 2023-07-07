use handlebars::Handlebars;
use once_cell::sync::Lazy;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use url::Url;

pub struct Renderer {
    inner: HashSet<String>,
}
impl Renderer {
    pub fn new(data: Vec<String>) -> Self {
        return Self { inner: HashSet::from_iter(data.iter().cloned()) };
    }

    pub fn into_html(&self) -> String {
        return TemplateData::new(self.inner.clone()).into_html();
    }
    pub fn into_grouped_json(&self) -> String {
        return TemplateData::new(self.inner.clone()).into_json();
    }
    pub fn into_json_array(&self) -> String {
        return serde_json::to_string(&self.inner).expect("could not convert to json array");
    }
    pub fn into_txt(&self) -> String {
        return self
            .inner
            .iter()
            .fold(String::new(), |acc, value| acc + value + "\n");
    }
}

#[derive(Serialize, Debug)]
pub struct TemplateData {
    row: Vec<DataRow>,
}

#[derive(Serialize, Debug)]
struct DataRow {
    hostname: String,
    urls: Vec<String>,
    size: usize,
}
impl TemplateData {
    pub fn new(links: HashSet<String>) -> Self {
        let mut data: HashMap<String, Vec<String>> = HashMap::new();
        for link in links.into_iter() {
            let url = Url::parse(&link).unwrap();
            let host = url.host_str().unwrap_or("Unknown hostname").to_owned();
            if data.contains_key(&host) {
                data.entry(host).and_modify(|v| v.push(link));
            } else {
                data.insert(host, vec![link]);
            }
        }
        return Self {
            row: data
                .into_iter()
                .map(|(key, data)| {
                    return DataRow {
                        size: data.len(),
                        hostname: key,
                        urls: data,
                    };
                })
                .collect(),
        };
    }
    pub fn into_html(&self) -> String {
        static HBS: Lazy<Handlebars<'_>> = Lazy::new(|| {
            let mut h = Handlebars::new();
            h.register_template_file("bookmarks", "./templates/bookmarks.hbs")
                .expect("could not register template");
            return h;
        });
        let html = HBS.render("bookmarks", self).unwrap();
        return html;
    }
    pub fn into_json(&self) -> String {
        return serde_json::to_string(&self.row).expect("could not convert to json");
    }
}
