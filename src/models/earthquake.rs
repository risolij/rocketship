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
    pub fn get_props(&self) {
        match &self.features {
            Some(quake) => {
                for q in quake {
                    println!("{} - {} - {}", 
                        q.properties.mag.unwrap(),
                        q.properties.place.as_ref().unwrap(),
                        q.properties.time.unwrap(),
                    );
                }
            },
            None => {
                println!("No properties found");
            }
        }
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
    #[serde(rename = "camelCase")]
    mag_type: Option<String>,
    r#type: Option<String>,
    title: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Geometry {
    r#type: Option<String>,
    coordinates: Vec<f64>,
}
