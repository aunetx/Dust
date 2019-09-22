let socket = new Socket();

function change() {
    socket.on("Hello", (data) => {
        console.log("Hello", data)
    });

    socket.send("Hello")
}
