use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub id: String,
    pub title: String,
    pub formats: Vec<Format>,
    pub thumbnails: Vec<Thumbnail>,
    pub thumbnail: String,
    pub description: String,
    pub uploader: String,
    #[serde(rename = "uploader_id")]
    pub uploader_id: String,
    #[serde(rename = "uploader_url")]
    pub uploader_url: String,
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    #[serde(rename = "channel_url")]
    pub channel_url: String,
    pub duration: i64,
    #[serde(rename = "view_count")]
    pub view_count: i64,
    #[serde(rename = "average_rating")]
    pub average_rating: Value,
    #[serde(rename = "age_limit")]
    pub age_limit: i64,
    #[serde(rename = "webpage_url")]
    pub webpage_url: String,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    #[serde(rename = "playable_in_embed")]
    pub playable_in_embed: bool,
    #[serde(rename = "live_status")]
    pub live_status: Value,
    #[serde(rename = "release_timestamp")]
    pub release_timestamp: Value,
    #[serde(rename = "automatic_captions")]
    pub automatic_captions: AutomaticCaptions,
    pub subtitles: Subtitles,
    #[serde(rename = "comment_count")]
    pub comment_count: Value,
    pub chapters: Vec<Chapter>,
    #[serde(rename = "like_count")]
    pub like_count: i64,
    pub channel: String,
    #[serde(rename = "channel_follower_count")]
    pub channel_follower_count: i64,
    #[serde(rename = "upload_date")]
    pub upload_date: String,
    pub availability: String,
    #[serde(rename = "original_url")]
    pub original_url: String,
    #[serde(rename = "webpage_url_basename")]
    pub webpage_url_basename: String,
    #[serde(rename = "webpage_url_domain")]
    pub webpage_url_domain: String,
    pub extractor: String,
    #[serde(rename = "extractor_key")]
    pub extractor_key: String,
    pub playlist: Value,
    #[serde(rename = "playlist_index")]
    pub playlist_index: Value,
    #[serde(rename = "display_id")]
    pub display_id: String,
    pub fulltitle: String,
    #[serde(rename = "duration_string")]
    pub duration_string: String,
    #[serde(rename = "requested_subtitles")]
    pub requested_subtitles: Value,
    #[serde(rename = "_has_drm")]
    pub has_drm: Value,
    pub asr: i64,
    pub filesize: i64,
    #[serde(rename = "format_id")]
    pub format_id: String,
    #[serde(rename = "format_note")]
    pub format_note: String,
    #[serde(rename = "source_preference")]
    pub source_preference: i64,
    pub fps: Value,
    #[serde(rename = "audio_channels")]
    pub audio_channels: i64,
    pub height: Value,
    pub quality: i64,
    #[serde(rename = "has_drm")]
    pub has_drm2: bool,
    pub tbr: f64,
    pub url: String,
    pub width: Value,
    pub language: String,
    #[serde(rename = "language_preference")]
    pub language_preference: i64,
    pub preference: Value,
    pub ext: String,
    pub vcodec: String,
    pub acodec: String,
    #[serde(rename = "dynamic_range")]
    pub dynamic_range: Value,
    pub abr: f64,
    #[serde(rename = "downloader_options")]
    pub downloader_options: DownloaderOptions2,
    pub container: String,
    pub protocol: String,
    #[serde(rename = "audio_ext")]
    pub audio_ext: String,
    #[serde(rename = "video_ext")]
    pub video_ext: String,
    pub format: String,
    pub resolution: String,
    #[serde(rename = "http_headers")]
    pub http_headers: HttpHeaders2,
    pub epoch: i64,
    #[serde(rename = "_filename")]
    pub filename: String,
    #[serde(rename = "filename")]
    pub filename2: String,
    pub urls: String,
    #[serde(rename = "_type")]
    pub type_field: String,
    #[serde(rename = "_version")]
    pub version: Version,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    #[serde(rename = "format_id")]
    pub format_id: String,
    #[serde(rename = "format_note")]
    pub format_note: String,
    pub ext: String,
    pub protocol: String,
    pub acodec: String,
    pub vcodec: String,
    pub url: String,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub fps: Option<f64>,
    pub rows: Option<i64>,
    pub columns: Option<i64>,
    #[serde(default)]
    pub fragments: Vec<Fragment>,
    #[serde(rename = "audio_ext")]
    pub audio_ext: String,
    #[serde(rename = "video_ext")]
    pub video_ext: String,
    pub format: String,
    pub resolution: String,
    #[serde(rename = "http_headers")]
    pub http_headers: HttpHeaders,
    pub asr: Option<i64>,
    pub filesize: Option<i64>,
    #[serde(rename = "source_preference")]
    pub source_preference: Option<i64>,
    #[serde(rename = "audio_channels")]
    pub audio_channels: Option<i64>,
    pub quality: Option<i64>,
    #[serde(rename = "has_drm")]
    pub has_drm: Option<bool>,
    pub tbr: Option<f64>,
    pub language: Option<String>,
    #[serde(rename = "language_preference")]
    pub language_preference: Option<i64>,
    pub preference: Option<i64>,
    #[serde(rename = "dynamic_range")]
    pub dynamic_range: Option<String>,
    pub abr: Option<f64>,
    #[serde(rename = "downloader_options")]
    pub downloader_options: Option<DownloaderOptions>,
    pub container: Option<String>,
    pub vbr: Option<f64>,
    #[serde(rename = "filesize_approx")]
    pub filesize_approx: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fragment {
    pub url: String,
    pub duration: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpHeaders {
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(rename = "Accept")]
    pub accept: String,
    #[serde(rename = "Accept-Language")]
    pub accept_language: String,
    #[serde(rename = "Sec-Fetch-Mode")]
    pub sec_fetch_mode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloaderOptions {
    #[serde(rename = "http_chunk_size")]
    pub http_chunk_size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub url: String,
    pub preference: i64,
    pub id: String,
    pub height: Option<i64>,
    pub width: Option<i64>,
    pub resolution: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomaticCaptions {
    pub af: Vec<Af>,
    pub ak: Vec<Ak>,
    pub sq: Vec<Sq>,
    pub am: Vec<Am>,
    pub ar: Vec<Ar>,
    pub hy: Vec<Hy>,
    #[serde(rename = "as")]
    pub as_field: Vec<A>,
    pub ay: Vec<Ay>,
    pub az: Vec<Az>,
    pub bn: Vec<Bn>,
    pub eu: Vec<Eu>,
    pub be: Vec<Be>,
    pub bho: Vec<Bho>,
    pub bs: Vec<B>,
    pub bg: Vec<Bg>,
    pub my: Vec<My>,
    pub ca: Vec<Ca>,
    pub ceb: Vec<Ceb>,
    #[serde(rename = "zh-Hans")]
    pub zh_hans: Vec<Han>,
    #[serde(rename = "zh-Hant")]
    pub zh_hant: Vec<ZhHant>,
    pub co: Vec<Co>,
    pub hr: Vec<Hr>,
    pub cs: Vec<C>,
    pub da: Vec<Da>,
    pub dv: Vec<Dv>,
    pub nl: Vec<Nl>,
    #[serde(rename = "en-orig")]
    pub en_orig: Vec<EnOrig>,
    pub en: Vec<En>,
    pub eo: Vec<Eo>,
    pub et: Vec<Et>,
    pub ee: Vec<Ee>,
    pub fil: Vec<Fil>,
    pub fi: Vec<Fi>,
    pub fr: Vec<Fr>,
    pub gl: Vec<Gl>,
    pub lg: Vec<Lg>,
    pub ka: Vec<Ka>,
    pub de: Vec<De>,
    pub el: Vec<El>,
    pub gn: Vec<Gn>,
    pub gu: Vec<Gu>,
    pub ht: Vec<Ht>,
    pub ha: Vec<Ha>,
    pub haw: Vec<Haw>,
    pub iw: Vec<Iw>,
    pub hi: Vec<Hi>,
    pub hmn: Vec<Hmn>,
    pub hu: Vec<Hu>,
    pub is: Vec<I>,
    pub ig: Vec<Ig>,
    pub id: Vec<Id>,
    pub ga: Vec<Ga>,
    pub it: Vec<It>,
    pub ja: Vec<Ja>,
    pub jv: Vec<Jv>,
    pub kn: Vec<Kn>,
    pub kk: Vec<Kk>,
    pub km: Vec<Km>,
    pub rw: Vec<Rw>,
    pub ko: Vec<Ko>,
    pub kri: Vec<Kri>,
    pub ku: Vec<Ku>,
    pub ky: Vec<Ky>,
    pub lo: Vec<Lo>,
    pub la: Vec<La>,
    pub lv: Vec<Lv>,
    pub ln: Vec<Ln>,
    pub lt: Vec<Lt>,
    pub lb: Vec<Lb>,
    pub mk: Vec<Mk>,
    pub mg: Vec<Mg>,
    pub ms: Vec<M>,
    pub ml: Vec<Ml>,
    pub mt: Vec<Mt>,
    pub mi: Vec<Mi>,
    pub mr: Vec<Mr>,
    pub mn: Vec<Mn>,
    pub ne: Vec<Ne>,
    pub nso: Vec<Nso>,
    pub no: Vec<No>,
    pub ny: Vec<Ny>,
    pub or: Vec<Or>,
    pub om: Vec<Om>,
    pub ps: Vec<P>,
    pub fa: Vec<Fa>,
    pub pl: Vec<Pl>,
    pub pt: Vec<Pt>,
    pub pa: Vec<Pa>,
    pub qu: Vec<Qu>,
    pub ro: Vec<Ro>,
    pub ru: Vec<Ru>,
    pub sm: Vec<Sm>,
    pub sa: Vec<Sa>,
    pub gd: Vec<Gd>,
    pub sr: Vec<Sr>,
    pub sn: Vec<Sn>,
    pub sd: Vec<Sd>,
    pub si: Vec<Si>,
    pub sk: Vec<Sk>,
    pub sl: Vec<Sl>,
    pub so: Vec<So>,
    pub st: Vec<St>,
    pub es: Vec<E>,
    pub su: Vec<Su>,
    pub sw: Vec<Sw>,
    pub sv: Vec<Sv>,
    pub tg: Vec<Tg>,
    pub ta: Vec<Um>,
    pub tt: Vec<Tt>,
    pub te: Vec<Te>,
    pub th: Vec<Th>,
    pub ti: Vec<Ti>,
    pub ts: Vec<T>,
    pub tr: Vec<Tr>,
    pub tk: Vec<Tk>,
    pub uk: Vec<Uk>,
    pub und: Vec<Und>,
    pub ur: Vec<Ur>,
    pub ug: Vec<Ug>,
    pub uz: Vec<Uz>,
    pub vi: Vec<Vi>,
    pub cy: Vec<Cy>,
    pub fy: Vec<Fy>,
    pub xh: Vec<Xh>,
    pub yi: Vec<Yi>,
    pub yo: Vec<Yo>,
    pub zu: Vec<Zu>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Af {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ak {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sq {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Am {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ar {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hy {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct A {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ay {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Az {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bn {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eu {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Be {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bho {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct B {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bg {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct My {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ca {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ceb {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Han {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZhHant {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Co {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hr {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct C {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Da {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dv {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nl {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnOrig {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct En {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eo {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Et {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ee {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fil {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fi {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fr {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gl {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lg {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ka {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct De {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct El {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gn {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gu {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ht {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ha {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Haw {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Iw {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hi {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hmn {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hu {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ig {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Id {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ga {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct It {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ja {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Jv {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kn {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kk {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Km {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rw {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ko {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kri {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ku {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ky {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lo {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct La {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lv {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ln {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lt {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lb {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mk {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mg {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct M {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ml {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mt {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mi {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mr {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mn {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ne {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nso {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct No {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ny {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Or {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Om {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fa {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pl {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pt {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pa {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Qu {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ro {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ru {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sm {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sa {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gd {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sr {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sn {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sd {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Si {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sk {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sl {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct So {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct St {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct E {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Su {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sw {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sv {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tg {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Um {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tt {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Te {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Th {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ti {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct T {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tr {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tk {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uk {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Und {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ur {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ug {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uz {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vi {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cy {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fy {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Xh {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Yi {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Yo {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zu {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subtitles {
    pub en: Vec<En2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct En2 {
    pub ext: String,
    pub url: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chapter {
    #[serde(rename = "start_time")]
    pub start_time: f64,
    pub title: String,
    #[serde(rename = "end_time")]
    pub end_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloaderOptions2 {
    #[serde(rename = "http_chunk_size")]
    pub http_chunk_size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpHeaders2 {
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(rename = "Accept")]
    pub accept: String,
    #[serde(rename = "Accept-Language")]
    pub accept_language: String,
    #[serde(rename = "Sec-Fetch-Mode")]
    pub sec_fetch_mode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub version: String,
    #[serde(rename = "current_git_head")]
    pub current_git_head: Value,
    #[serde(rename = "release_git_head")]
    pub release_git_head: String,
    pub repository: String,
}
