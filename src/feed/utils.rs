use scraper::{Html, Selector};

async fn get_html(url: &str) -> Result<Html, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?;
    let text = resp.text().await?;
    Ok(Html::parse_document(&text))
}

pub async fn get_channel_id(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let document = get_html(url).await?;
    let selector = Selector::parse(r#"body > link[rel="canonical"]"#).unwrap();
    let link = document
        .select(&selector)
        .next()
        .map(|el| el.value().attr("href").unwrap())
        .expect("could not find canonical link for channel");

    let id = link.split('/').rev().next().unwrap().to_string();

    Ok(id)
}

pub async fn get_feed_image(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let document = get_html(url).await?;
    let selector = Selector::parse(r#"body > meta[property="og:image"]"#).unwrap();
    let link = document
        .select(&selector)
        .next()
        .map(|el| el.value().attr("content").unwrap())
        .expect("could not find canonical link for channel");

    Ok(link.to_string())
}

pub async fn get_feed_description(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let document = get_html(url).await?;
    let selector = Selector::parse(r#"body > meta[property="og:description"]"#).unwrap();
    let description = document
        .select(&selector)
        .next()
        .map(|el| el.value().attr("content").unwrap());

    let description = match description {
        Some(description) => description.to_owned(),
        None => String::new(),
    };

    Ok(description)
}

#[cfg(test)]
mod tests {
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
