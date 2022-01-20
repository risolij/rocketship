use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use std::cmp::Ordering;

#[derive(Debug, Deserialize, Serialize)]
pub struct Earthquake {
    r#type: Option<String>,
    metadata: Option<Metadata>,
    pub features: Option<Vec<Quake>>,
    bbox: Option<Vec<f64>>,
}

impl Earthquake {
    pub fn count(&self) -> usize {
        self.features.as_ref().unwrap().iter().count()
    }

    //pub fn sort(&self) {
    //    let items = self
    //        .features
    //        .as_ref()
    //        .unwrap()
    //        .iter()
    //        .map(|p| p.properties.mag.unwrap())
    //        .filter(|f| !f.is_nan())
    //        .collect::<Vec<f32>>()
    //        .sort_by(Earthquake::cmp_f32);

    //    println!("{:?}", items)

    //}

    //pub fn cmp_f32(a: &f32, b: &f32) -> Ordering {
    //    if a < b {
    //        return Ordering::Less;
    //    }

    //    if a < b {
    //        return Ordering::Greater;
    //    }

    //    return Ordering::Equal;
    //}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    generated: Option<i64>,
    url: Option<String>,
    title: Option<String>,
    status: Option<i32>,
    api: Option<String>,
    count: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Quake {
    r#type: Option<String>,
    pub properties: Properties,
    geometry: Geometry,
    id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Properties {
    pub mag: Option<f32>,
    pub place: Option<String>,
    pub time: Option<i64>,
    pub updated: Option<i64>,
    pub tz: Option<i64>,
    pub url: Option<String>,
    pub detail: Option<String>,
    pub felt: Option<i64>,
    pub cdi: Option<f32>,
    pub mmi: Option<f32>,
    pub alert: Option<String>,
    pub status: Option<String>,
    pub tsunami: Option<i32>,
    pub sig: Option<i32>,
    pub net: Option<String>,
    pub code: Option<String>,
    pub ids: Option<String>,
    pub sources: Option<String>,
    pub types: Option<String>,
    pub nst: Option<i32>,
    pub dmin: Option<f32>,
    pub rms: Option<f32>,
    pub gap: Option<i32>,
    #[serde(rename = "camelCase")]
    pub mag_type: Option<String>,
    pub r#type: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Geometry {
    r#type: Option<String>,
    coordinates: Vec<f64>,
}

//pub fn get_props(&self) {
//    match &self.features {
//        Some(quake) => {
//            for q in quake {
//                println!(
//                    "{} - {} - {}",
//                    q.properties.mag.unwrap(),
//                    q.properties.place.as_ref().unwrap(),
//                    q.properties.time.unwrap(),
//                );
//            }
//        }
//        None => {
//            println!("No properties found");
//        }
//    }
//}
