use rocket::serde::Serialize;
use rocket::serde::Deserialize;


#[derive(Debug, Deserialize, Serialize)]
pub struct Earthquake {
    r#type: Option<String>, 
    metadata: Option<Metadata>,
    features: Option<Vec<Quake>>,
    bbox: Option<Vec<f64>>,
}


impl Earthquake {
    pub fn new() -> Self {
        Self {
            r#type: None,
            metadata: None,
            features: None,
            bbox: None,
        }
    }
    pub async fn test_earthquake(&mut self) {
        self.request().await.unwrap();
    }

    pub async fn request(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get("https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson&starttime=2014-01-01&endtime=2014-01-30&minmagnitude=5")
            .await?
            .json::<Self>()
            .await?;

        println!("{:#?}", resp);

        Ok(())

    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    generated: Option<i64>,
    url: Option<String>,
    title: Option<String>,
    status: Option<i32>,
    api: Option<String>,
    count: Option<i32>
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Quake {
    r#type: Option<String>,
    properties: Properties,
    geometry: Geometry,
    id: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Properties {
    mag: Option<f32>,
    place: Option<String>,
    time: Option<i64>,
    updated: Option<i64>,
    tz: Option<i64>,
    url: Option<String>,
    detail: Option<String>,
    felt: Option<i64>,
    cdi: Option<f32>,
    mmi: Option<f32>,
    alert: Option<String>,
    status: Option<String>,
    tsunami: Option<i32>,
    sig: Option<i32>,
    net: Option<String>,
    code: Option<String>,
    ids: Option<String>,
    sources: Option<String>,
    types: Option<String>,
    nst: Option<i32>,
    dmin: Option<f32>,
    rms: Option<f32>,
    gap: Option<i32>,
    magType: Option<String>,
    r#type: Option<String>,
    title: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Geometry {
    r#type: Option<String>,
    coordinates: Vec<f64>,
}
