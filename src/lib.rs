pub mod yt_xml;

use scraper::{Html, Selector};

async fn get_html(url: &str) -> Result<Html, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?;
    let text = resp.text().await?;
    Ok(Html::parse_document(&text))
}

pub async fn get_channel_id(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let document = get_html(&url).await?;
    let selector = Selector::parse(r#"body > link[rel="canonical"]"#).unwrap();
    let link = document
        .select(&selector)
        .next()
        .map(|el| el.value().attr("href").unwrap())
        .expect("could not find canonical link for channel");

    let id = link.split('/').rev().next().unwrap().to_string();

    Ok(id)
}

pub async fn get_channel_image(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let document = get_html(&url).await?;
    let selector = Selector::parse(r#"body > meta[property="og:image"]"#).unwrap();
    let link = document
        .select(&selector)
        .next()
        .map(|el| el.value().attr("content").unwrap())
        .expect("could not find canonical link for channel");

    Ok(link.to_string())
}

pub async fn get_channel_description(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let document = get_html(&url).await?;
    let selector = Selector::parse(r#"body > meta[property="og:description"]"#).unwrap();
    let description = document
        .select(&selector)
        .next()
        .map(|el| el.value().attr("content").unwrap())
        .expect("could not find description for channel");

    Ok(description.to_string())
}

pub async fn get_video_length(url: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?;
    let text = resp.text().await?;
    let length = text.find("lengthSeconds");
    match length {
        Some(i) => {
            let text = &text[i + 16..];
            let end = text.find('"').unwrap();
            let text = &text[..end];
            Ok(text
                .parse::<u32>()
                .expect("could not parse duration as u32!"))
        }
        None => Ok(1800),
    }
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

    #[tokio::test]
    async fn test_image() {
        let image = "https://yt3.ggpht.com/ytc/AMLnZu_ZK-GvsGbsEaBYo0q_u3NvSSDT__vlljY7nJohDg=s900-c-k-c0x00ffffff-no-rj";
        assert_eq!(
            get_channel_image(&format!(
                "https://www.youtube.com/channel/{}",
                get_channel_id("https://www.youtube.com/c/OstonCodeCypher")
                    .await
                    .unwrap()
            ))
            .await
            .unwrap(),
            image
        );
    }

    #[tokio::test]
    async fn test_video_length() {
        let url = "https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1";
        let length = 5603;
        assert_eq!(get_video_length(&url).await.unwrap(), length);
    }
}
