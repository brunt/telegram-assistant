use spending_tracker::{SpentRequest, SpentResponse, SpentTotalResponse};

#[derive(Debug, Clone)]
pub(crate) struct SpendingAPI {
    pub(crate) spending_total_url: String,
    pub(crate) spending_reset_url: String,
    pub(crate) spending_add_url: String,
    pub(crate) budget_set_url: String,
}
impl Default for SpendingAPI {
    fn default() -> Self {
        let spending_base_url = "http://localhost:8001";
        Self {
            spending_add_url: format!("{}/spent", spending_base_url),
            spending_total_url: format!("{}/spent", spending_base_url),
            spending_reset_url: format!("{}/reset", spending_base_url),
            budget_set_url: format!("{}/budget", spending_base_url),
        }
    }
}

impl SpendingAPI {
    pub(crate) async fn spending_request(
        &self,
        req: SpentRequest,
    ) -> Result<SpentResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .post(&self.spending_add_url)
            .json(&req)
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }

    pub(crate) async fn spending_total_request(
        &self,
    ) -> Result<SpentTotalResponse, reqwest::Error> {
        let response: SpentTotalResponse = reqwest::get(&self.spending_total_url)
            .await?
            .json::<SpentTotalResponse>()
            .await?;
        Ok(response)
    }

    pub(crate) async fn spending_reset_request(
        &self,
    ) -> Result<SpentTotalResponse, reqwest::Error> {
        let response: SpentTotalResponse = reqwest::get(&self.spending_reset_url)
            .await?
            .json::<SpentTotalResponse>()
            .await?;
        Ok(response)
    }

    pub(crate) async fn budget_set_request(
        &self,
        req: SpentRequest,
    ) -> Result<SpentResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .post(&self.budget_set_url)
            .json(&req)
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }
}
