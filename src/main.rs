use std::error::Error;
use std::io;

use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};
use log::debug;
use serde::Deserialize;
use serde_json::{value::Value as JValue, Map};
use url::Url;

#[derive(Debug, Deserialize, PartialEq)]
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
        
        // properties Location address
        let mut location = Map::new();
        location.insert(
            "address".to_string(),
            JValue::String(self.formatted_address),
        );
        properties.insert("Location".to_string(), JValue::Object(location));
        properties.insert("Title".to_string(), JValue::String(self.name));

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

fn get_place_details(record: &Record) -> Result<PlaceDetails, Box<dyn Error>> {
    let ftid = record_to_ftid(&record)?;
    get_place_details_by_ftid(ftid)
}

fn record_to_ftid(record: &Record) -> Result<&str, Box<dyn Error>> {
    url_to_ftid(&record.URL)
        .ok_or(format!("Unable to find ftid for record: {:?}", record))
        .map_err(|e| e.into())
}

fn url_to_ftid(url: &Url) -> Option<&str> {
    url.path_segments()
        .and_then(|p| p.last())
        // TODO - verify this covers all cases; if so, this can be improved
        .map(|d| d.trim_start_matches("data=!4m2!3m1!1s"))
        .filter(|f| !f.is_empty())
}

fn get_place_details_by_ftid(ftid: &str) -> Result<PlaceDetails, Box<dyn Error>> {
    // TODO - add cache, these requests can cost money

    let api_key = &std::env::var("GOOGLE_API_KEY")?;
    let request_url = build_place_details_url(api_key, ftid)?;

    reqwest::blocking::get(request_url)?
        .json()
        .map_err(|e| e.into())
}

fn build_place_details_url(api_key: &str, ftid: &str) -> Result<Url, url::ParseError> {
    Url::parse(&format!(
        // Reference https://developers.google.com/places/web-service/details
        "https://maps.googleapis.com/maps/api/place/details/json?key={}&ftid={}&fields=name,geometry,formatted_address",
        api_key, ftid
    ))
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
    print!("{}", serialized);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_SAVED_CSV: &str = "
Title,Note,URL,Comment
E&O Kitchen and Bar,,https://www.google.com/maps/place/E%26O+Kitchen+and+Bar/data=!4m2!3m1!1s0x8085808941977519:0x6a23af223bffdaf8,
Chubby Noodle North Beach,,https://www.google.com/maps/place/Chubby+Noodle+North+Beach/data=!4m2!3m1!1s0x808580f3f7a0ee1f:0x6cff0ef558beff7d,
    ";

    #[test]
    fn test_csv_record() {
        let expected = vec![
            Record {
                Title: "E&O Kitchen and Bar".to_string(),
                Note: None,
                URL: Url::parse("https://www.google.com/maps/place/E%26O+Kitchen+and+Bar/data=!4m2!3m1!1s0x8085808941977519:0x6a23af223bffdaf8").unwrap(),
                Comment: None,
            },
            Record {
                Title: "Chubby Noodle North Beach".to_string(),
                Note: None,
                URL: Url::parse("https://www.google.com/maps/place/Chubby+Noodle+North+Beach/data=!4m2!3m1!1s0x808580f3f7a0ee1f:0x6cff0ef558beff7d").unwrap(),
                Comment: None,
            },
        ];
        let mut reader = csv::Reader::from_reader(EXAMPLE_SAVED_CSV.trim().as_bytes());

        let actual: Vec<Record> = reader.deserialize().map(|r| r.unwrap()).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_ftid_exists_in_url() {
        let url = Url::parse("https://www.google.com/maps/place/E%26O+Kitchen+and+Bar/data=!4m2!3m1!1s0x8085808941977519:0x6a23af223bffdaf8").unwrap();
        let actual = url_to_ftid(&url);
        assert_eq!(Some("0x8085808941977519:0x6a23af223bffdaf8"), actual);
    }

    #[test]
    fn test_ftid_missing_when_no_data_in_url() {
        let url = Url::parse("https://www.google.com/maps/place/E%26O+Kitchen+and+Bar/").unwrap();
        let actual = url_to_ftid(&url);
        assert_eq!(None, actual);
    }

    #[test]
    fn test_build_place_details_url() {
        let api_key = "API_KEY";
        let ftid = "FTID";
        let expected = Url::parse(&format!("https://maps.googleapis.com/maps/api/place/details/json?key={:}&ftid={:}&fields=name,geometry,formatted_address", api_key, ftid)).unwrap();

        let actual = build_place_details_url(api_key, ftid).unwrap();

        assert_eq!(expected, actual);
    }

    // geojson
    const EXAMPLE_PLACE_DETAILS_RESPONSE_BODY: &str = r#"
    {
        "html_attributions" : [],
        "result" : {
           "formatted_address" : "Calle Cruz de Piedra, 4, 03015 Alicante (Alacant), Alicante, Spain",
           "geometry" : {
              "location" : {
                 "lat" : 38.3642128,
                 "lng" : -0.4620302
              },
              "viewport" : {
                 "northeast" : {
                    "lat" : 38.3654640802915,
                    "lng" : -0.460755769708498
                 },
                 "southwest" : {
                    "lat" : 38.3627661197085,
                    "lng" : -0.4634537302915021
                 }
              }
           },
           "name" : "Calle Cruz de Piedra, 4"
        },
        "status" : "OK"
     }     
    "#;

    #[test]
    fn test_places_response_body_place_details() -> Result<(), Box<dyn Error>> {
        let _: PlaceDetails = serde_json::from_str(EXAMPLE_PLACE_DETAILS_RESPONSE_BODY)?;
        Ok(())
    }

    #[test]
    fn test_place_details_result_to_feature() {
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
        let expected = {
            let mut properties = Map::new();
            let mut location = Map::new();
            location.insert(
                "Address".to_string(),
                JValue::String("123 Fake St, Springfield".to_string()),
            );
            properties.insert("Location".to_string(), JValue::Object(location));
            properties.insert("Title".to_string(), JValue::String("Paper Co".to_string()));

            Feature {
                bbox: None,
                geometry: Some(Geometry::new(Value::Point(vec![11.0, 10.0]))),
                id: None,
                properties: Some(properties),
                foreign_members: None,
            }
        };

        let actual = place_details.into();

        assert_eq!(expected, actual);
    }
}
