# Google Takeout Saved Places to GeoJSON

## Examples

### Example CSV

```
Title,Note,URL,Comment
E&O Kitchen and Bar,,https://www.google.com/maps/place/E%26O+Kitchen+and+Bar/data=!4m2!3m1!1s0x8085808941977519:0x6a23af223bffdaf8,
Chubby Noodle North Beach,,https://www.google.com/maps/place/Chubby+Noodle+North+Beach/data=!4m2!3m1!1s0x808580f3f7a0ee1f:0x6cff0ef558beff7d,
```

### Default Run
```sh
% GOOGLE_API_KEY=[REDACTED] cargo run < [SAVED_LIST_NAME].csv 
{"features":[{"geometry":{"coordinates":[-122.40585,37.789696],"type":"Point"},"properties":{"address":"314 Sutter St, San Francisco, CA 94108, USA","name":"E&O Kitchen and Bar"},"type":"Feature"},{"geometry":{"coordinates":[-122.4085676,37.7997609],"type":"Point"},"properties":{"address":"570 Green Street off, Columbus Ave, San Francisco, CA 94133, USA","name":"Chubby Noodle North Beach"},"type":"Feature"}],"type":"FeatureCollection"}
```

### Debug Run
```sh
% GOOGLE_API_KEY=[REDACTED] cargo run < [SAVED_LIST_NAME].csv 
   Compiling google-takeout-saved-places-to-geojson v0.1.0 ([SRC_PATH])
    Finished dev [unoptimized + debuginfo] target(s) in 3.99s
     Running `target/debug/google-takeout-saved-places-to-geojson`
[2021-01-16T00:13:41Z DEBUG google_takeout_places_to_geojson] record: Record { Title: "E&O Kitchen and Bar", Note: None, URL: Url { scheme: "https", host: Some(Domain("www.google.com")), port: None, path: "/maps/place/E%26O+Kitchen+and+Bar/data=!4m2!3m1!1s0x8085808941977519:0x6a23af223bffdaf8", query: None, fragment: None }, Comment: None }
[2021-01-16T00:13:41Z DEBUG reqwest::connect] starting new connection: https://maps.googleapis.com/
[2021-01-16T00:13:44Z DEBUG reqwest::async_impl::client] response '200 OK' for https://maps.googleapis.com/maps/api/place/details/json?key=[REDACTED]&ftid=0x8085808941977519:0x6a23af223bffdaf8&fields=name,geometry,formatted_address
[2021-01-16T00:13:44Z DEBUG google_takeout_places_to_geojson] place_details: PlaceDetails { result: PlaceDetailsResult { formatted_address: "314 Sutter St, San Francisco, CA 94108, USA", geometry: PlaceDetailsGeometry { location: PlaceDetailsLocation { lat: 37.789696, lng: -122.40585 } }, name: "E&O Kitchen and Bar" } }
[2021-01-16T00:13:44Z DEBUG google_takeout_places_to_geojson] feature: Feature { bbox: None, geometry: Some(Geometry { bbox: None, value: Point([-122.40585, 37.789696]), foreign_members: None }), id: None, properties: Some({"address": String("314 Sutter St, San Francisco, CA 94108, USA"), "name": String("E&O Kitchen and Bar")}), foreign_members: None }
[2021-01-16T00:13:44Z DEBUG google_takeout_places_to_geojson] record: Record { Title: "Chubby Noodle North Beach", Note: None, URL: Url { scheme: "https", host: Some(Domain("www.google.com")), port: None, path: "/maps/place/Chubby+Noodle+North+Beach/data=!4m2!3m1!1s0x808580f3f7a0ee1f:0x6cff0ef558beff7d", query: None, fragment: None }, Comment: None }
[2021-01-16T00:13:44Z DEBUG reqwest::connect] starting new connection: https://maps.googleapis.com/
[2021-01-16T00:13:45Z DEBUG reqwest::async_impl::client] response '200 OK' for https://maps.googleapis.com/maps/api/place/details/json?key=[REDACTED]&ftid=0x808580f3f7a0ee1f:0x6cff0ef558beff7d&fields=name,geometry,formatted_address
[2021-01-16T00:13:45Z DEBUG google_takeout_places_to_geojson] place_details: PlaceDetails { result: PlaceDetailsResult { formatted_address: "570 Green Street off, Columbus Ave, San Francisco, CA 94133, USA", geometry: PlaceDetailsGeometry { location: PlaceDetailsLocation { lat: 37.7997609, lng: -122.4085676 } }, name: "Chubby Noodle North Beach" } }
[2021-01-16T00:13:45Z DEBUG google_takeout_places_to_geojson] feature: Feature { bbox: None, geometry: Some(Geometry { bbox: None, value: Point([-122.4085676, 37.7997609]), foreign_members: None }), id: None, properties: Some({"address": String("570 Green Street off, Columbus Ave, San Francisco, CA 94133, USA"), "name": String("Chubby Noodle North Beach")}), foreign_members: None }
{"features":[{"geometry":{"coordinates":[-122.40585,37.789696],"type":"Point"},"properties":{"address":"314 Sutter St, San Francisco, CA 94108, USA","name":"E&O Kitchen and Bar"},"type":"Feature"},{"geometry":{"coordinates":[-122.4085676,37.7997609],"type":"Point"},"properties":{"address":"570 Green Street off, Columbus Ave, San Francisco, CA 94133, USA","name":"Chubby Noodle North Beach"},"type":"Feature"}],"type":"FeatureCollection"}
```