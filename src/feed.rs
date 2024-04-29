use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use axum_extra::body::AsyncReadBody;
use chrono::DateTime;
use quick_xml::{
    events::{attributes::Attribute, BytesCData, BytesDecl, BytesText, Event},
    Reader,
};
use scraper::{Html, Selector};
use std::borrow::Cow;
use std::collections::HashMap;

fn move_head(n: u32, reader: &mut quick_xml::Reader<&[u8]>) {
    for _ in 0..n {
        let _ = reader.read_event();
    }
}

fn get_tag_content<'a>(reader: &mut Reader<&'a [u8]>) -> Cow<'a, str> {
    let start = match reader.read_event().unwrap() {
        Event::Start(bytes) => bytes,
        _ => panic!("Expected start event"),
    };
    let end = start.to_end().into_owned();
    reader.read_text(end.name()).unwrap()
}

fn get_attr_val(attr_byte_str: &[u8], reader: &mut Reader<&[u8]>) -> String {
    if let Event::Empty(tag) = reader.read_event().unwrap() {
        let attr = tag
            .attributes()
            .filter_map(Result::ok)
            .find(|attr| attr.key.as_ref() == attr_byte_str)
            .map(|attr| attr.decode_and_unescape_value(reader).unwrap());
        match attr {
            Some(val) => return val.to_string(),
            None => panic!("attribute not found"),
        }
    } else {
        panic!("Expected empty event")
    };
}

fn extract_html_data(html_content: &str) -> String {
    let document = Html::parse_document(html_content);
    let image_selector = Selector::parse(r#"body > meta[property="og:image"]"#).unwrap();

    let image_url = document
        .select(&image_selector)
        .next()
        .unwrap()
        .value()
        .attr("content")
        .unwrap();

    image_url.to_string() // TODO: get rid of this alloc
}

async fn get_channel_id(html_content: &str) -> String {
    let document = Html::parse_document(html_content);
    let selector = Selector::parse(r#"body > link[rel="canonical"]"#).unwrap();
    let link = document
        .select(&selector)
        .next()
        .map(|el| el.value().attr("href").unwrap())
        .expect("Could not find canonical link");

    link.split('/').last().unwrap().to_string()
}

#[tracing::instrument]
pub async fn serve_feed(
    Path(youtube_path): Path<String>,
    Query(query): Query<HashMap<String, String>>,
    State(state): State<crate::AppState>,
    request: axum::extract::Request,
) -> impl IntoResponse {
    let channel_html = state
        .client
        .get(format!("https://www.youtube.com/{youtube_path}"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let channel_id = &get_channel_id(&channel_html).await;

    let xml = state
        .client
        .get(format!(
            "https://www.youtube.com/feeds/videos.xml?channel_id={channel_id}"
        ))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let mut cursor: quick_xml::Reader<&[u8]> = quick_xml::Reader::from_str(&xml);

    let (mut asyncwriter, asyncreader) = tokio::io::duplex(256 * 1024);
    let mut writer = quick_xml::Writer::new_with_indent(&mut asyncwriter, b' ', 4);

    let decl = Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None));
    writer.write_event_async(decl).await.unwrap();
    writer
        .create_element("rss")
        .with_attribute(("version", "2.0"))
        .with_attribute(("xmlns:atom", "http://www.w3.org/2005/Atom"))
        .with_attribute(("xmlns:podcast", "https://podcastindex.org/namespace/1.0"))
        .with_attribute(("xmlns:itunes", "http://www.itunes.com/dtds/podcast-1.0.dtd"))
        .with_attribute(("xmlns:content", "http://purl.org/rss/1.0/modules/content/"))
        .write_inner_content_async::<_, _, quick_xml::Error>(|writer| async move {
            writer
                .create_element("channel")
                .write_inner_content_async::<_, _, quick_xml::Error>(|writer| async move {
                    move_head(14, &mut cursor);
                    let title = get_tag_content(&mut cursor);
                    move_head(1, &mut cursor);
                    let yt_url = get_attr_val(b"href", &mut cursor);

                    writer
                        .create_element("atom:link")
                        .with_attribute(Attribute::from((
                            "href".as_bytes(),
                            format!("https://{}", request.uri()).as_bytes(),
                        )))
                        .with_attribute(("rel", "self"))
                        .with_attribute(("type", "application/rss+xml"))
                        .write_empty_async()
                        .await
                        .unwrap();
                    writer
                        .create_element("title")
                        .write_text_content_async(BytesText::new(&title))
                        .await
                        .unwrap();
                    writer
                        .create_element("link")
                        .write_text_content_async(BytesText::new(&yt_url))
                        .await
                        .unwrap();
                    writer
                        .create_element("podcast:guid")
                        .write_text_content_async(BytesText::new(&channel_id))
                        .await
                        .unwrap();
                    writer
                        .create_element("podcast:locked")
                        .write_text_content_async(BytesText::new("yes"))
                        .await
                        .unwrap();
                    writer
                        .create_element("itunes:author")
                        .write_text_content_async(BytesText::new(&title))
                        .await
                        .unwrap();

                    let image_url = extract_html_data(&channel_html);
                    writer
                        .create_element("itunes:image")
                        .write_text_content_async(BytesText::new(&image_url))
                        .await
                        .unwrap();
                    writer
                        .create_element("copyright")
                        .write_text_content_async(BytesText::new(&title))
                        .await
                        .unwrap();
                    writer
                        .create_element("description")
                        .write_text_content_async(BytesText::new("lmao")) // TODO: need description
                        .await
                        .unwrap();

                    move_head(17, &mut cursor); // cursor now at the blank text field before the first <entry>

                    loop {
                        match cursor.read_event() {
                            Ok(Event::End(_)) => break,
                            Ok(_) => {
                                let episode_url = state.episode_url.clone();
                                let (id, title, url, published, thumbnail, description) =
                                    parse_video(&mut cursor);

                                let generic_feed_items = [("title", title), ("url", url)];

                                writer
                                    .create_element("item")
                                    .write_inner_content_async::<_, _, quick_xml::Error>(
                                        |writer| async move {
                                            writer
                                                .create_element("guid")
                                                .with_attribute(("isPermaLink", "false"))
                                                .write_text_content_async(BytesText::new(&format!(
                                                    "{}-{}",
                                                    channel_id, id
                                                )))
                                                .await
                                                .unwrap();

                                            writer
                                                .create_element("pubDate")
                                                .write_text_content_async(BytesText::new(
                                                    &DateTime::parse_from_rfc3339(&published)
                                                        .unwrap()
                                                        .to_rfc2822(),
                                                ))
                                                .await
                                                .unwrap();

                                            for (key, val) in generic_feed_items.iter() {
                                                writer
                                                    .create_element(*key)
                                                    .write_text_content_async(BytesText::new(val))
                                                    .await
                                                    .unwrap();
                                            }

                                            writer
                                                .create_element("itunes:image")
                                                .with_attribute((
                                                    "href",
                                                    thumbnail.to_string().as_ref(),
                                                ))
                                                .write_empty_async()
                                                .await
                                                .unwrap();

                                            writer
                                                .create_element("itunes:block")
                                                .write_text_content_async(BytesText::new("yes"))
                                                .await
                                                .unwrap();

                                            writer
                                                .create_element("enclosure")
                                                .with_attribute(("length", "1337")) // TODO: populate
                                                .with_attribute(("type", "audio/x-m4a"))
                                                .with_attribute(Attribute::from((
                                                    "url".as_bytes(),
                                                    format!(
                                                        "{}ep/{}/{}",
                                                        episode_url, channel_id, id
                                                    )
                                                    .as_bytes(),
                                                )))
                                                .write_empty_async()
                                                .await
                                                .unwrap();

                                            writer
                                                .create_element("description")
                                                .write_cdata_content_async(BytesCData::new(
                                                    description,
                                                ))
                                                .await
                                                .unwrap();

                                            Ok(writer)
                                        },
                                    )
                                    .await
                                    .unwrap();
                            }
                            Err(e) => panic!("Error: {:?}", e),
                        }
                    }

                    Ok(writer)
                })
                .await
                .unwrap();
            Ok(writer)
        })
        .await
        .unwrap();

    AsyncReadBody::new(asyncreader).into_response()
}

fn parse_video<'a>(
    cursor: &mut quick_xml::Reader<&'a [u8]>,
) -> (
    Cow<'a, str>,
    Cow<'a, str>,
    Cow<'a, str>,
    Cow<'a, str>,
    Cow<'a, str>,
    Cow<'a, str>,
) {
    move_head(5, cursor);
    let id = get_tag_content(cursor);
    move_head(5, cursor);
    let title = get_tag_content(cursor);
    move_head(1, cursor);
    let url = get_attr_val(b"href", cursor);
    move_head(13, cursor);
    let published = get_tag_content(cursor);
    move_head(13, cursor);
    let thumbnail = get_attr_val(b"url", cursor);
    move_head(1, cursor);
    let description = get_tag_content(cursor);

    move_head(13, cursor); // leave cursor on empty text field after </entry>

    (
        id,
        title,
        url.into(),
        published,
        thumbnail.into(),
        description,
    )
}
