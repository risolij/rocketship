use crate::models::earthquake::Earthquake;


#[derive(Debug)]
pub struct QueryBuilder<'m> {
    from: &'m str,
    to: &'m str,
    magnitude: Option<i8>,
}


impl<'m> QueryBuilder<'m> {
    pub fn new(from: &'m str, to: &'m str, magnitude: Option<i8>) -> Self {
        Self {
            from,
            to,
            magnitude
        }
    }

    fn build_query(&self) -> String {
        let base = format!("https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson");

        match self.magnitude {
            Some(mag) => {
                format!("{}&starttime={}&endtime={}&minmagnitude={}", base, self.from, self.to, mag)
            },
            None => {
                format!("{}&starttime={}&endtime={}", base, self.from, self.to)
            }
        }
    }


    //https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson&starttime=2014-01-01&endtime=2014-01-30&minmagnitude=5"
    pub async fn build_quake(&mut self) -> Result<Earthquake, Box<dyn std::error::Error>> {
        let resp = reqwest::get(&self.build_query())
            .await?
            .json::<Earthquake>()
            .await?;

        Ok(resp)
    }
}

