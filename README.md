# Google Takeout Saved Places to GeoJSON

### Example CSV

```
Title,Note,URL,Comment
E&O Kitchen and Bar,,https://www.google.com/maps/place/E%26O+Kitchen+and+Bar/data=!4m2!3m1!1s0x8085808941977519:0x6a23af223bffdaf8,
Chubby Noodle North Beach,,https://www.google.com/maps/place/Chubby+Noodle+North+Beach/data=!4m2!3m1!1s0x808580f3f7a0ee1f:0x6cff0ef558beff7d,
```

### Run
```sh
% GOOGLE_API_KEY=[REDACTED] cargo run < [SAVED_LIST_NAME].csv 
{"features":[{"geometry":{"coordinates":[-122.40585,37.789696],"type":"Point"},"properties":{"Location":{"Address":"314 Sutter St, San Francisco, CA 94108, USA"},"Title":"E&O Kitchen and Bar"},"type":"Feature"},{"geometry":{"coordinates":[-122.4085676,37.7997609],"type":"Point"},"properties":{"Location":{"Address":"570 Green Street off, Columbus Ave, San Francisco, CA 94133, USA"},"Title":"Chubby Noodle North Beach"},"type":"Feature"}],"type":"FeatureCollection"}
```