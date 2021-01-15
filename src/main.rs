use std::error::Error;
use std::io;

use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};
use log::{debug, error};
use serde::Deserialize;
use serde_json::{to_value, Map};
use url::Url;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct Record {
    Title: String,
    Note: Option<String>,
    URL: Url,
    Comment: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PlaceDetails {
    result: PlaceDetailsResult,
}

#[derive(Debug, Deserialize)]
struct PlaceDetailsResult {
    formatted_address: String,
    geometry: PlaceDetailsGeometry,
    name: String,
}

#[derive(Debug, Deserialize)]
struct PlaceDetailsGeometry {
    location: PlaceDetailsLocation,
}

#[derive(Debug, Deserialize)]
struct PlaceDetailsLocation {
    lat: f64,
    lng: f64,
}

impl Into<Feature> for PlaceDetailsResult {
    fn into(self) -> Feature {
        let mut properties = Map::new();
        properties.insert(
            "address".to_string(),
            to_value(self.formatted_address).unwrap(),
        );
        properties.insert("name".to_string(), to_value(self.name).unwrap());

        let geometry = Geometry::new(Value::Point(vec![
            self.geometry.location.lng,
            self.geometry.location.lat,
        ]));

        Feature {
            bbox: None,
            geometry: Some(geometry),
            id: None,
            properties: Some(properties),
            foreign_members: None,
        }
    }
}

// TODO - create record iterator
// TODO - Record to Feature

fn get_place_details(record: Record, api_key: &str) -> Result<PlaceDetails, Box<dyn Error>> {
    let ftid = record
        .URL
        .path_segments()
        .and_then(|p| p.last())
        // TODO - verify this covers all cases; if so, this can be optimized
        .map(|d| d.trim_start_matches("data=!4m2!3m1!1s"))
        .ok_or(format!("Unable to find ftid for record: {:?}", record))?;
    debug!("ftid {:?}", ftid);

    // TODO - add cache
    // call google api
    let request_url = Url::parse(&format!(
        // Reference https://developers.google.com/places/web-service/details
        "https://maps.googleapis.com/maps/api/place/details/json?key={}&ftid={}&fields=name,geometry,formatted_address",
        api_key, ftid
    ))?;

    reqwest::blocking::get(request_url)?.json().map_err(|e| e.into())
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let api_key = &std::env::var("GOOGLE_API_KEY")?;

    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut features: Vec<Feature> = vec![];

    for result in rdr.deserialize() {
        let record: Record = result?;
        debug!("{:?}", record);

        let place_details = get_place_details(record, &api_key)?;
        debug!("body: {:?}", place_details);

        let feature: Feature = place_details.result.into();
        features.push(feature);
    }
    let feature_collection = FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    };

    let serialized = GeoJson::from(feature_collection).to_string();
    println!("{:}", serialized);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    // read csv to record
    #[test]
    fn test_csv_record() {
        unimplemented!();
    }

    // get ftid from record

    // build reqwest request

    // geojson
    #[test]
    fn test_places_response_body_place_details() -> Result<(), Box<dyn Error>> {
        let places_response_body = std::fs::read_to_string("./data/places_response.json")?;
        let _: PlaceDetails = serde_json::from_str(&places_response_body)?;
        Ok(())
    }

    #[test]
    fn test_place_details_result_to_feature() -> Result<(), Box<dyn Error>> {
        let place_details = PlaceDetailsResult {
            formatted_address: "123 Fake St, Springfield".to_string(),
            geometry: PlaceDetailsGeometry {
                location: PlaceDetailsLocation {
                    lat: 10.0,
                    lng: 11.0,
                },
            },
            name: "Paper Co".to_string(),
        };
        unimplemented!();
    }
}
