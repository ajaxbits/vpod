use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YtDlpInfo {
    pub id: Option<String>,
    pub title: Option<String>,
    pub formats: Vec<Format>,
    pub thumbnails: Vec<Thumbnail>,
    pub thumbnail: Option<String>,
    pub description: Option<String>,
    pub uploader: Option<String>,
    pub uploader_id: Option<String>,
    pub uploader_url: Option<String>,
    pub channel_id: Option<String>,
    pub channel_url: Option<String>,
    pub duration: Option<i64>,
    pub view_count: Option<i64>,
    pub average_rating: Value,
    pub age_limit: Option<i64>,
    pub webpage_url: Option<String>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub playable_in_embed: bool,
    pub live_status: Value,
    pub release_timestamp: Value,
    pub automatic_captions: AutomaticCaptions,
    pub subtitles: Subtitles,
    pub comment_count: Value,
    pub chapters: Vec<Chapter>,
    pub like_count: Option<i64>,
    pub channel: Option<String>,
    pub channel_follower_count: Option<i64>,
    pub upload_date: Option<String>,
    pub availability: Option<String>,
    pub original_url: Option<String>,
    pub webpage_url_basename: Option<String>,
    pub webpage_url_domain: Option<String>,
    pub extractor: Option<String>,
    pub extractor_key: Option<String>,
    pub playlist: Value,
    pub playlist_index: Value,
    pub display_id: Option<String>,
    pub fulltitle: Option<String>,
    pub duration_string: Option<String>,
    pub requested_subtitles: Value,
    #[serde(rename = "_has_drm")]
    pub has_drm: Value,
    pub requested_formats: Option<Vec<RequestedFormat>>,
    pub format: Option<String>,
    pub format_id: Option<String>,
    pub ext: Option<String>,
    pub protocol: Option<String>,
    pub language: Value,
    pub format_note: Option<String>,
    pub filesize_approx: Option<i64>,
    pub tbr: f64,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub resolution: Option<String>,
    pub fps: Option<i64>,
    pub dynamic_range: Option<String>,
    pub vcodec: Option<String>,
    pub vbr: f64,
    pub stretched_ratio: Value,
    pub acodec: Option<String>,
    pub abr: f64,
    pub asr: Option<i64>,
    pub audio_channels: Option<i64>,
    pub epoch: Option<i64>,
    #[serde(rename = "_filename")]
    pub filename: Option<String>,
    #[serde(rename = "filename")]
    pub filename2: Option<String>,
    pub urls: Option<String>,
    #[serde(rename = "_type")]
    pub type_field: Option<String>,
    #[serde(rename = "_version")]
    pub version: Version,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Format {
    pub format_id: Option<String>,
    pub format_note: Option<String>,
    pub ext: Option<String>,
    pub protocol: Option<String>,
    pub acodec: Option<String>,
    pub vcodec: Option<String>,
    pub url: Option<String>,
    pub width: Option<Option<i64>>,
    pub height: Option<Option<i64>>,
    pub fps: Option<f64>,
    pub rows: Option<Option<i64>>,
    pub columns: Option<Option<i64>>,
    #[serde(default)]
    pub fragments: Vec<Fragment>,
    pub audio_ext: Option<String>,
    pub video_ext: Option<String>,
    pub format: Option<String>,
    pub resolution: Option<String>,
    pub http_headers: HttpHeaders,
    pub asr: Option<Option<i64>>,
    pub filesize: Option<Option<i64>>,
    pub source_preference: Option<Option<i64>>,
    pub audio_channels: Option<Option<i64>>,
    pub quality: Option<Option<i64>>,
    pub has_drm: Option<bool>,
    pub tbr: Option<f64>,
    pub language: Option<String>,
    pub language_preference: Option<Option<i64>>,
    pub preference: Option<Option<i64>>,
    pub dynamic_range: Option<String>,
    pub abr: Option<f64>,
    pub downloader_options: Option<DownloaderOptions>,
    pub container: Option<String>,
    pub vbr: Option<f64>,
    pub filesize_approx: Option<Option<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fragment {
    pub url: Option<String>,
    pub duration: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HttpHeaders {
    #[serde(rename = "User-Agent")]
    pub user_agent: Option<String>,
    #[serde(rename = "Accept")]
    pub accept: Option<String>,
    #[serde(rename = "Accept-Language")]
    pub accept_language: Option<String>,
    #[serde(rename = "Sec-Fetch-Mode")]
    pub sec_fetch_mode: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloaderOptions {
    pub http_chunk_size: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thumbnail {
    pub url: Option<String>,
    pub preference: Option<i64>,
    pub id: Option<String>,
    pub height: Option<Option<i64>>,
    pub width: Option<Option<i64>>,
    pub resolution: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
pub struct Af {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ak {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sq {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Am {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ar {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hy {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct A {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ay {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Az {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bn {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Eu {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Be {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bho {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct B {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bg {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct My {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ca {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ceb {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Han {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZhHant {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Co {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hr {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct C {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Da {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dv {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nl {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnOrig {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct En {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Eo {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Et {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ee {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fil {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fi {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fr {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gl {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lg {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ka {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct De {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct El {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gn {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gu {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ht {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ha {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Haw {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Iw {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hi {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hmn {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hu {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct I {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ig {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Id {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ga {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct It {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ja {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Jv {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Kn {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Kk {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Km {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rw {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ko {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Kri {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ku {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ky {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lo {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct La {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lv {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ln {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lt {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lb {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mk {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mg {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct M {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ml {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mt {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mi {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mr {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mn {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ne {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nso {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct No {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ny {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Or {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Om {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct P {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fa {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pl {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pt {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pa {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Qu {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ro {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ru {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sm {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sa {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gd {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sr {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sn {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sd {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Si {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sk {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sl {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct So {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct St {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct E {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Su {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sw {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sv {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tg {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Um {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tt {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Te {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Th {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ti {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct T {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tr {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tk {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uk {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Und {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ur {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ug {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uz {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vi {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cy {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fy {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Xh {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Yi {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Yo {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Zu {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subtitles {
    pub en: Vec<En2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct En2 {
    pub ext: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chapter {
    pub start_time: f64,
    pub title: Option<String>,
    pub end_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestedFormat {
    pub asr: Option<Option<i64>>,
    pub filesize: Option<i64>,
    pub format_id: Option<String>,
    pub format_note: Option<String>,
    pub source_preference: Option<i64>,
    pub fps: Option<Option<i64>>,
    pub audio_channels: Option<Option<i64>>,
    pub height: Option<Option<i64>>,
    pub quality: Option<i64>,
    pub has_drm: bool,
    pub tbr: f64,
    pub url: Option<String>,
    pub width: Option<Option<i64>>,
    pub language: Option<String>,
    pub language_preference: Option<i64>,
    pub preference: Value,
    pub ext: Option<String>,
    pub vcodec: Option<String>,
    pub acodec: Option<String>,
    pub dynamic_range: Option<String>,
    pub vbr: Option<f64>,
    pub downloader_options: DownloaderOptions2,
    pub container: Option<String>,
    pub protocol: Option<String>,
    pub video_ext: Option<String>,
    pub audio_ext: Option<String>,
    pub format: Option<String>,
    pub resolution: Option<String>,
    pub http_headers: HttpHeaders2,
    pub abr: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloaderOptions2 {
    pub http_chunk_size: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HttpHeaders2 {
    #[serde(rename = "User-Agent")]
    pub user_agent: Option<String>,
    #[serde(rename = "Accept")]
    pub accept: Option<String>,
    #[serde(rename = "Accept-Language")]
    pub accept_language: Option<String>,
    #[serde(rename = "Sec-Fetch-Mode")]
    pub sec_fetch_mode: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Version {
    pub version: Option<String>,
    pub current_git_head: Value,
    pub release_git_head: Option<String>,
    pub repository: Option<String>,
}
