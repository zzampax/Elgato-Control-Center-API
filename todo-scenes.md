### ECCDUMP - 10/2024
---
```http
GET /elgato/lights HTTP/1.1
Accept: application/json
User-Agent: CCW/1.7.2.624
Host: 192.168.17.11:9123
```

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 87
Connection: keep-alive

{"numberOfLights":1,"lights":[{"on":1,"hue":357.0,"saturation":78.0,"brightness":100}]}
```
---
```http
GET /elgato/accessory-info HTTP/1.1
Accept: application/json
User-Agent: CCW/1.7.2.624
Host: 192.168.17.11:9123
```

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 294
Connection: keep-alive

{"productName":"Elgato Light Strip","hardwareBoardType":70,"hardwareRevision":"1","macAddress":"3C:6A:9D:18:70:52","firmwareBuildNumber":233,"firmwareVersion":"1.0.4","serialNumber":"EW52J1A03730","displayName":"","features":["lights"],"wifi-info":{"ssid":"M28","frequencyMHz":2400,"rssi":-51}}
```
---
```http
GET /elgato/lights HTTP/1.1
Accept: application/json
User-Agent: CCW/1.7.2.624
Host: 192.168.17.11:9123
```

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 87
Connection: keep-alive

{"numberOfLights":1,"lights":[{"on":1,"hue":357.0,"saturation":78.0,"brightness":100}]}
```
---
```http
GET /elgato/accessory-info HTTP/1.1
Accept: application/json
User-Agent: CCW/1.7.2.624
Host: 192.168.17.11:9123
```

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 294
Connection: keep-alive

{"productName":"Elgato Light Strip","hardwareBoardType":70,"hardwareRevision":"1","macAddress":"3C:6A:9D:18:70:52","firmwareBuildNumber":233,"firmwareVersion":"1.0.4","serialNumber":"EW52J1A03730","displayName":"","features":["lights"],"wifi-info":{"ssid":"M28","frequencyMHz":2400,"rssi":-52}}
```
---
```http
GET /elgato/lights HTTP/1.1
Accept: application/json
User-Agent: CCW/1.7.2.624
Host: 192.168.17.11:9123
```

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 87
Connection: keep-alive

{"numberOfLights":1,"lights":[{"on":1,"hue":357.0,"saturation":78.0,"brightness":100}]}
```
---
```http
GET /elgato/accessory-info HTTP/1.1
Accept: application/json
User-Agent: CCW/1.7.2.624
Host: 192.168.17.11:9123
```

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 294
Connection: keep-alive

{"productName":"Elgato Light Strip","hardwareBoardType":70,"hardwareRevision":"1","macAddress":"3C:6A:9D:18:70:52","firmwareBuildNumber":233,"firmwareVersion":"1.0.4","serialNumber":"EW52J1A03730","displayName":"","features":["lights"],"wifi-info":{"ssid":"M28","frequencyMHz":2400,"rssi":-53}}
```
---
```http
GET /elgato/lights HTTP/1.1
Accept: application/json
User-Agent: CCW/1.7.2.624
Host: 192.168.17.11:9123
```

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 87
Connection: keep-alive

{"numberOfLights":1,"lights":[{"on":1,"hue":357.0,"saturation":78.0,"brightness":100}]}
```
---
```http
GET /elgato/accessory-info HTTP/1.1
Accept: application/json
User-Agent: CCW/1.7.2.624
Host: 192.168.17.11:9123
```

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 294
Connection: keep-alive

{"productName":"Elgato Light Strip","hardwareBoardType":70,"hardwareRevision":"1","macAddress":"3C:6A:9D:18:70:52","firmwareBuildNumber":233,"firmwareVersion":"1.0.4","serialNumber":"EW52J1A03730","displayName":"","features":["lights"],"wifi-info":{"ssid":"M28","frequencyMHz":2400,"rssi":-53}}
```
---
```http
GET /elgato/lights HTTP/1.1
Accept: application/json
User-Agent: CCW/1.7.2.624
Host: 192.168.17.11:9123
```

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 87
Connection: keep-alive

{"numberOfLights":1,"lights":[{"on":1,"hue":357.0,"saturation":78.0,"brightness":100}]}
```
---
```http
GET /elgato/accessory-info HTTP/1.1
Accept: application/json
User-Agent: CCW/1.7.2.624
Host: 192.168.17.11:9123
```

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 294
Connection: keep-alive

{"productName":"Elgato Light Strip","hardwareBoardType":70,"hardwareRevision":"1","macAddress":"3C:6A:9D:18:70:52","firmwareBuildNumber":233,"firmwareVersion":"1.0.4","serialNumber":"EW52J1A03730","displayName":"","features":["lights"],"wifi-info":{"ssid":"M28","frequencyMHz":2400,"rssi":-54}}
```
---
```http
PUT /elgato/lights HTTP/1.1
Accept: application/json
User-Agent: CCW/1.7.2.624
Content-Type: application/json; charset=utf-8
Host: 192.168.17.11:9123
Content-Length: 481

{"lights":[{"brightness":100,"id":"com.corsair.cc.scene.sunrise","name":"Sunrise","numberOfSceneElements":4,"on":1,"scene":[{"brightness":100,"durationMs":1000,"hue":60,"saturation":100,"transitionMs":10000},{"brightness":100,"durationMs":1000,"hue":30,"saturation":100,"transitionMs":10000},{"brightness":100,"durationMs":1000,"hue":0,"saturation":100,"transitionMs":10000},{"brightness":100,"durationMs":1000,"hue":30,"saturation":100,"transitionMs":10000}]}],"numberOfLights":1}
```

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 505
Connection: keep-alive

{"numberOfLights":1,"lights":[{"on":1,"id":"com.corsair.cc.scene.sunrise","name":"Sunrise","brightness":100.0,"numberOfSceneElements":4,"scene":[{"hue":60.0,"saturation":100.0,"brightness":100.0,"durationMs":1000,"transitionMs":10000},{"hue":30.0,"saturation":100.0,"brightness":100.0,"durationMs":1000,"transitionMs":10000},{"hue":0,"saturation":100.0,"brightness":100.0,"durationMs":1000,"transitionMs":10000},{"hue":30.0,"saturation":100.0,"brightness":100.0,"durationMs":1000,"transitionMs":10000}]}]}
```
---
```http
GET /elgato/lights HTTP/1.1
Accept: application/json
User-Agent: CCW/1.7.2.624
Host: 192.168.17.11:9123
```

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 505
Connection: keep-alive

{"numberOfLights":1,"lights":[{"on":1,"id":"com.corsair.cc.scene.sunrise","name":"Sunrise","brightness":100.0,"numberOfSceneElements":4,"scene":[{"hue":60.0,"saturation":100.0,"brightness":100.0,"durationMs":1000,"transitionMs":10000},{"hue":30.0,"saturation":100.0,"brightness":100.0,"durationMs":1000,"transitionMs":10000},{"hue":0,"saturation":100.0,"brightness":100.0,"durationMs":1000,"transitionMs":10000},{"hue":30.0,"saturation":100.0,"brightness":100.0,"durationMs":1000,"transitionMs":10000}]}]}
```
---
```http
GET /elgato/accessory-info HTTP/1.1
Accept: application/json
User-Agent: CCW/1.7.2.624
Host: 192.168.17.11:9123
```

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 294
Connection: keep-alive

{"productName":"Elgato Light Strip","hardwareBoardType":70,"hardwareRevision":"1","macAddress":"3C:6A:9D:18:70:52","firmwareBuildNumber":233,"firmwareVersion":"1.0.4","serialNumber":"EW52J1A03730","displayName":"","features":["lights"],"wifi-info":{"ssid":"M28","frequencyMHz":2400,"rssi":-53}}
```
---
```http
GET /elgato/lights HTTP/1.1
Accept: application/json
User-Agent: CCW/1.7.2.624
Host: 192.168.17.11:9123
```

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 505
Connection: keep-alive

{"numberOfLights":1,"lights":[{"on":1,"id":"com.corsair.cc.scene.sunrise","name":"Sunrise","brightness":100.0,"numberOfSceneElements":4,"scene":[{"hue":60.0,"saturation":100.0,"brightness":100.0,"durationMs":1000,"transitionMs":10000},{"hue":30.0,"saturation":100.0,"brightness":100.0,"durationMs":1000,"transitionMs":10000},{"hue":0,"saturation":100.0,"brightness":100.0,"durationMs":1000,"transitionMs":10000},{"hue":30.0,"saturation":100.0,"brightness":100.0,"durationMs":1000,"transitionMs":10000}]}]}
```
---
```http
GET /elgato/accessory-info HTTP/1.1
Accept: application/json
User-Agent: CCW/1.7.2.624
Host: 192.168.17.11:9123
```

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 294
Connection: keep-alive

{"productName":"Elgato Light Strip","hardwareBoardType":70,"hardwareRevision":"1","macAddress":"3C:6A:9D:18:70:52","firmwareBuildNumber":233,"firmwareVersion":"1.0.4","serialNumber":"EW52J1A03730","displayName":"","features":["lights"],"wifi-info":{"ssid":"M28","frequencyMHz":2400,"rssi":-53}}
```
---
```http
GET /elgato/lights HTTP/1.1
Accept: application/json
User-Agent: CCW/1.7.2.624
Host: 192.168.17.11:9123
```

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 505
Connection: keep-alive

{"numberOfLights":1,"lights":[{"on":1,"id":"com.corsair.cc.scene.sunrise","name":"Sunrise","brightness":100.0,"numberOfSceneElements":4,"scene":[{"hue":60.0,"saturation":100.0,"brightness":100.0,"durationMs":1000,"transitionMs":10000},{"hue":30.0,"saturation":100.0,"brightness":100.0,"durationMs":1000,"transitionMs":10000},{"hue":0,"saturation":100.0,"brightness":100.0,"durationMs":1000,"transitionMs":10000},{"hue":30.0,"saturation":100.0,"brightness":100.0,"durationMs":1000,"transitionMs":10000}]}]}
```
---
```http
GET /elgato/accessory-info HTTP/1.1
Accept: application/json
User-Agent: CCW/1.7.2.624
Host: 192.168.17.11:9123
```

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 294
Connection: keep-alive

{"productName":"Elgato Light Strip","hardwareBoardType":70,"hardwareRevision":"1","macAddress":"3C:6A:9D:18:70:52","firmwareBuildNumber":233,"firmwareVersion":"1.0.4","serialNumber":"EW52J1A03730","displayName":"","features":["lights"],"wifi-info":{"ssid":"M28","frequencyMHz":2400,"rssi":-54}}
```
