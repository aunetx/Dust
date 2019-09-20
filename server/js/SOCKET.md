# Socket documentation

## Creating a realtime socket connection

This creates a new `Socket` object :

```javascript
socket = new Socket()
```

You can also create that object with custom server ip and port.
Note that the port is a `string` and not a `number` :

```javascript
socket = new Socket("https://custom.ip", "8080")
```

## Sending a message to server

A message is composed by a `name` and an optional `content`, that is a JSON object.
To send a message, nothing simpler :

```javascript
socket = new Socket()

// without content
socket.send("get_hour");

// with JSON content
socket.send("direct_message", {
    target: "user1",
    message: "Hello world!"
})
```

## Intercepting incoming message

An incoming message from the server can be retrieved with `socket.on()`

```javascript
// unamed function
socket.on("user_data", function(data) {
    console.log(data);
})
// OR : more modern
socket.on("user_data", (data) => {
    console.log(data)
});

// named function
function showData(data) {
    console.log(data);
}
socket.on("user_data", (data) => showData(data));
```
