In **k6**, you can pass an **authentication token** (such as a **JWT**) when establishing a **WebSocket** connection using the **`headers`** option in the `ws.connect()` function.

### Example: Passing an Authentication Token in k6 WebSocket
```javascript
import ws from 'k6/ws';
import { check } from 'k6';

export const options = {
  vus: 1,  // Number of Virtual Users
  duration: '10s', // Test duration
};

export default function () {
  const url = 'wss://example.com/socket';
  const token = 'your-jwt-token'; // Replace with actual token

  const params = {
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  };

  const res = ws.connect(url, params, function (socket) {
    socket.on('open', function open() {
      console.log('Connected to WebSocket');
      socket.send(JSON.stringify({ event: 'subscribe', channel: 'updates' }));
    });

    socket.on('message', function (message) {
      console.log('Received:', message);
    });

    socket.on('close', function () {
      console.log('Disconnected from WebSocket');
    });

    socket.on('error', function (err) {
      console.error('WebSocket Error:', err);
    });
  });

  check(res, { 'Connected successfully': (r) => r && r.status === 101 });
}
```

### Key Points:
- Use **`headers`** inside the `params` object to pass the `Authorization` token.
- `Authorization: Bearer <token>` is the common format for JWT-based authentication.
- Check the WebSocket connection status with `check()`.

Let me know if you need any modifications!
