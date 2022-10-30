use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub(crate) struct NewsAPI {
    pub(crate) sources: Vec<String>,
    pub(crate) api_key: String,
    pub(crate) url: String,
    pub(crate) page_size: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct NewsAPIResponse {
    pub(crate) status: String,
    #[serde(rename = "totalResults")]
    pub(crate) total_results: i32,
    pub(crate) articles: Vec<Article>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Article {
    source: Source,
    author: Option<String>,
    title: String,
    description: String,
    url: String,
    #[serde(rename = "urlToImage")]
    url_to_image: String,
    #[serde(rename = "publishedAt")]
    published_at: String, // "2022-10-30T05:07:30Z",
    content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Source {
    id: String,
    name: String,
}

impl Default for NewsAPI {
    fn default() -> Self {
        Self {
            sources: vec![
                "abc-news",
                "al-jazeera-english",
                "ars-technica",
                "associated-press",
                "axios",
                // "buzzfeed",
                "cnn",
                // "hacker-news",
                "medical-news-today",
                "nbc-news",
                "newsweek",
                "politico",
                "recode",
                "reuters",
                "techcrunch",
                "vice-news",
                "wired",
            ]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>(),
            api_key: env::var("NEWS_API_KEY").expect("missing NEWS_API_KEY"),
            url: "https://newsapi.org/v2/top-headlines".to_string(),
            page_size: 5,
        }
    }
}

impl NewsAPI {
    pub(crate) async fn request_data(&self) -> Result<NewsAPIResponse, reqwest::Error> {
        let sources = self.sources.to_vec().join(",");
        let endpoint = format!(
            "{url}?sources={sources}&pageSize={page_size}&apiKey={api_key}",
            url = self.url,
            sources = sources,
            page_size = self.page_size,
            api_key = self.api_key
        );
        let data = reqwest::Client::builder()
            .user_agent("telegram-bot")
            .build()?
            .get(&endpoint)
            .send()
            .await?
            .json()
            .await?;
        Ok(data)
    }
}

//TODO: look into richer telegram responses to use images
impl Display for NewsAPIResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut resp_str = String::new();
        for article in &self.articles {
            resp_str.push_str(&format!(
                r#"Title: {}
{}
{}

"#,
                article.title, article.description, article.url
            ));
        }
        write!(f, "{}", resp_str)
    }
}
