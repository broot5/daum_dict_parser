use anyhow::Result;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
struct Tmp {
    q: String,
    items: Vec<String>,
}

struct DaumDict {
    q: String,
    items: Vec<String>,
    data: HashMap<String, String>,
}

impl DaumDict {
    fn new() -> Self {
        Self {
            q: String::new(),
            items: Vec::new(),
            data: HashMap::new(),
        }
    }
    async fn search(&mut self, keyword: String) -> Result<String> {
        self.q = keyword;

        let url = format!("https://suggest-bar.daum.net/suggest?mod=json&code=utf_in_out&enc=utf&id=language&cate=eng&q={}", self.q);
        let resp = reqwest::get(&url).await?.json::<Tmp>().await?;

        self.q = resp.q;
        self.items = resp.items;

        self.refine_items();

        Ok(self
            .data
            .get(self.q.as_str())
            .unwrap_or(&"Failed to search word".to_string())
            .to_owned())
    }

    fn refine_items(&mut self) {
        for i in &self.items {
            let a: Vec<&str> = i.split('|').collect();
            self.data
                .insert(a.get(1).unwrap().to_string(), a.get(2).unwrap().to_string());
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut daum_dict = DaumDict::new();

    loop {
        let a = daum_dict.search(get_input()).await?;

        println!("{}", a);
    }
    Ok(())
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().into()
}
