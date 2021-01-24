use lazy_static::*;
use regex::Regex;
use spending_tracker::{Category, SpentRequest, SpentTotalResponse, SpentResponse};

#[derive(Debug, Clone)]
pub(crate) struct SpendingAPI {
    pub(crate) spending_total_url: String,
    pub(crate) spending_reset_url: String,
    pub(crate) spending_add_url: String,
    pub(crate) budget_set_url: String,
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

    //determine if request was for total, reset, or addition, and perform that action, return a formatted string of the results.
    pub(crate) async fn parse_spent_request(
        &self,
        input: &str,
        category: Option<Category>,
    ) -> String {
        let split: Vec<&str> = input.split(' ').collect();
        match split[0] {
            "budget" | "Budget" => match split[1].parse::<f64>() {
                Ok(amount) => match &self
                    .budget_set_request(SpentRequest { amount, category })
                    .await
                {
                    Ok(s) => s.to_string(),
                    Err(_) => "error calling api".to_string(),
                },
                Err(_) => "cannot parse that value as float".to_string(),
            },
            _ => match split[1] {
                "reset" => match &self.spending_reset_request().await {
                    Ok(s) => s.to_string(),
                    Err(_) => "error calling api".to_string(),
                },
                "total" => match &self.spending_total_request().await {
                    Ok(s) => s.to_string(),
                    Err(_) => "error calling api".to_string(),
                },
                _ => match split[1].parse::<f64>() {
                    Ok(amount) => match &self
                        .spending_request(SpentRequest { amount, category })
                        .await
                    {
                        Ok(s) => s.to_string(),
                        Err(_) => "error calling api".to_string(),
                    },
                    Err(_) => "cannot parse that value as float".to_string(),
                },
            },
        }
    }
}

pub(crate) fn is_spent_request(text: &str) -> bool {
    lazy_static! {
        static ref NSRE: Regex =
            Regex::new(r"(budget|Budget|spent|Spent)\s(total|reset|-?[0-9]+\.?[0-9]+)").unwrap();
    }
    NSRE.is_match(text)
}

pub(crate) fn is_spent_category_request(text: &str) -> bool {
    lazy_static! {
        static ref NSREC: Regex =
            Regex::new(r"(spent|Spent)\s(total|reset|-?[0-9]+\.?[0-9]+)\s(dining|travel|merchandise|entertainment|grocery|other)").unwrap();
    }
    NSREC.is_match(text)
}

pub(crate) fn help_spending() -> &'static str {
    "Spending Tracker:\nspent total\nspent reset\nspent 10.67"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_spent_request() {
        assert_eq!(is_spent_request("spent total"), true);
        assert_eq!(is_spent_request("spent reset"), true);
        assert_eq!(is_spent_request("spent 0.01"), true);
        assert_eq!(is_spent_request("spent 1000"), true);
        assert_eq!(is_spent_request("spent -4"), false);
        assert_eq!(is_spent_request("spent 10.00 travel"), true);
    }

    #[test]
    fn test_is_spent_category_request() {
        assert_eq!(is_spent_category_request("spent 10.00 dining"), true);
        assert_eq!(is_spent_category_request("spent 10.00 entertainment"), true);
        assert_eq!(is_spent_category_request("spent 10.00 merchandise"), true);
        assert_eq!(is_spent_category_request("spent 10.00 travel"), true);
        assert_eq!(is_spent_category_request("spent 10.00 other"), true);
        assert_eq!(is_spent_category_request("spent 10.00 grocery"), true);
        assert_eq!(is_spent_category_request("spent 10.00 something"), false);
        assert_eq!(is_spent_category_request("spent 10.00"), false);
    }
}
