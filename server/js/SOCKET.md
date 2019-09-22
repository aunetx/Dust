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

- ### `socket.send()`

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

An incoming message from the server can be retrieved with differents ways :

- ### `socket.send().callback()`

Simplest and quickest way to write client/server one-way communication (intercept only the response of that actual message, not every message named the same) :

```javascript
socket.send("get_hour").callback((response) => {
    console.log("Hour is : ", response)
})
```

*Note : prevents `socket.on()`'s defined callback to be fired !*

- ### `socket.on()`

Intercepts any message that is not intercepted by `send().callback()`. Can also fire on server message (that are not a response to a client request) :

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