function change() {
    let message = "Hello world!";
    let socket = new SocketNoRt();
    socket.onResponse(function (text) {
        console.log(text);
    });
    socket.send(message);
}
class SocketNoRt {
    constructor() {
        this.ip = "http://" + location.host + "/socket/message";
        // Request init
        this.request = new XMLHttpRequest();
        this.request.responseType = "json";
        this.request.open('PUT', this.ip, true);
        this.request.onreadystatechange = this.stateChanged;
    }
    send(message) {
        this.request.send();
    }
    stateChanged(event) {
        let req = event.target;
        console.log(this);
        if (req.readyState == 4) {
            if (req.status == 200) {
                console.log("Well done");
                //soThis.on_response(req.response.res);
            }
            else {
                console.log("Cannot retrieve socket answer : HTTP status code is " + req.status);
            }
        }
    }
    onResponse(callback) {
        this.on_response = callback;
    }
}
class SocketRt {
    constructor() {
    }
}
//# sourceMappingURL=socket.js.map