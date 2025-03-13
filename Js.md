Here’s a simple HTML file with inline CSS and JavaScript to connect to a WebSocket server, send messages, and receive messages.  

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WebSocket Chat</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        #chat { width: 100%; height: 300px; border: 1px solid #ddd; padding: 10px; overflow-y: auto; margin-bottom: 10px; }
        input, button { padding: 10px; font-size: 16px; }
    </style>
</head>
<body>

    <div id="chat"></div>
    <input type="text" id="message" placeholder="Type a message..." />
    <button onclick="sendMessage()">Send</button>

    <script>
        let ws = new WebSocket("wss://yourserver.com/chat"); // Change to your WebSocket URL

        ws.onopen = function() {
            logMessage("Connected to WebSocket server");
        };

        ws.onmessage = function(event) {
            logMessage("Server: " + event.data);
        };

        ws.onerror = function(error) {
            logMessage("Error: " + error.message);
        };

        ws.onclose = function() {
            logMessage("Connection closed");
        };

        function sendMessage() {
            let input = document.getElementById("message");
            let msg = input.value.trim();
            if (msg) {
                ws.send(msg);
                logMessage("You: " + msg);
                input.value = "";
            }
        }

        function logMessage(msg) {
            let chat = document.getElementById("chat");
            let div = document.createElement("div");
            div.textContent = msg;
            chat.appendChild(div);
            chat.scrollTop = chat.scrollHeight;
        }
    </script>

</body>
</html>
```

### How it Works:
- **WebSocket connection** opens to `wss://yourserver.com/chat` (replace with your actual WebSocket server URL).
- **Messages** are sent when the "Send" button is clicked.
- **Incoming messages** from the server are displayed in the chat box.
- **Errors and connection status** are logged to the chat window.

Let me know if you need modifications!
