use std::error::Error;
use std::io;

use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};
use log::debug;
use serde::Deserialize;
use serde_json::{Map, value::Value as JValue};
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
            JValue::String(self.formatted_address),
        );
        properties.insert("name".to_string(), JValue::String(self.name));

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

fn get_place_details(record: &Record) -> Result<PlaceDetails, Box<dyn Error>> {
    let ftid = record_to_ftid(&record)?;

    // TODO - add cache
    get_place_details_by_ftid(ftid)
}

fn record_to_ftid(record: &Record) -> Result<&str, Box<dyn Error>> {
    record
        .URL
        .path_segments()
        .and_then(|p| p.last())
        // TODO - verify this covers all cases; if so, this can be optimized
        .map(|d| d.trim_start_matches("data=!4m2!3m1!1s"))
        .ok_or(format!("Unable to find ftid for record: {:?}", record))
        .map_err(|e| e.into())
}

fn get_place_details_by_ftid(ftid: &str) -> Result<PlaceDetails, Box<dyn Error>> {
    let api_key = &std::env::var("GOOGLE_API_KEY")?;
    let request_url = Url::parse(&format!(
        // Reference https://developers.google.com/places/web-service/details
        "https://maps.googleapis.com/maps/api/place/details/json?key={}&ftid={}&fields=name,geometry,formatted_address",
        api_key, ftid
    ))?;

    reqwest::blocking::get(request_url)?.json().map_err(|e| e.into())
}

fn record_to_feature(record: &Record) -> Result<Feature, Box<dyn Error>> {
    let place_details = get_place_details(record)?;
    debug!("place_details: {:?}", place_details);

    Ok(place_details.result.into())
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut features: Vec<Feature> = vec![];

    for result in rdr.deserialize() {
        let record: Record = result?;
        debug!("record: {:?}", record);

        let feature = record_to_feature(&record)?;
        debug!("feature: {:?}", feature);

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

    #[test]
    fn test_csv_record() {
        unimplemented!();
    }

    // TODO - get ftid from record

    // TODO - build reqwest request

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
