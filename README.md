# echo Weather Proxy

A proxy service in front of the Yr weather API to provide simplified weather data and caching for use in our applications.

## Response

You can hit the `/weather` endpoint to get the current weather data in a simplified format.

```json
{
  "temperature": 13.0,
  "condition": "cloudy", // can be null
  "wind_speed": 2.5
}
```
