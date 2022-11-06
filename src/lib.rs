use scraper::{Html, Selector};
pub async fn get_channel_id(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?;
    let text = resp.text().await?;
    let document = Html::parse_document(&text);
    let selector = Selector::parse(r#"body > link[rel="canonical"]"#).unwrap();
    let link = document
        .select(&selector)
        .next()
        .map(|el| el.value().attr("href").unwrap())
        .expect("could not find canonical link for channel");

    let id = link.split('/').rev().next().unwrap().to_string();

    Ok(id)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[tokio::test]
    async fn test_grim_beard_id() {
        let grim_beard = "UCNmv1Cmjm3Hk8Vc9kIgv0AQ";
        assert_eq!(
            get_channel_id("https://www.youtube.com/c/GrimBeard")
                .await
                .unwrap(),
            grim_beard
        );
    }
    #[tokio::test]
    async fn test_vihart_id() {
        let vihart = "UCOGeU-1Fig3rrDjhm9Zs_wg";
        assert_eq!(
            get_channel_id("https://www.youtube.com/user/vihart")
                .await
                .unwrap(),
            vihart
        );
    }
}
