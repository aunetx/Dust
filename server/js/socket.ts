function change() {
    let message = "Hello world!";
    let socket = new SocketNoRt();

    socket.onResponse(function (text) {
        console.log(text);
    });

    socket.send(message);
}

class SocketNoRt {
    ip: string;
    request: XMLHttpRequest;

    send(message: string) {
        this.request.send();
    }

    stateChanged(event) {
        let req: XMLHttpRequest = event.target
        console.log(this)
        if (req.readyState == 4) {
            if (req.status == 200) {
                console.log("Well done")
                //soThis.on_response(req.response.res);
            } else {
                console.log("Cannot retrieve socket answer : HTTP status code is " + req.status)
            }
        }
    }

    // Response function set by user, BEFORE sending with onResponse(function(t) {...})
    on_response: Function;
    onResponse(callback: Function) {
        this.on_response = callback;
    }

    constructor() {
        this.ip = "http://" + location.host + "/socket/message";
        // Request init
        this.request = new XMLHttpRequest();
        this.request.responseType = "json";
        this.request.open('PUT', this.ip, true);
        this.request.onreadystatechange = this.stateChanged;
    }
}

class SocketRt {
    server: {
        ip: string;
    }
    client: {
        id: number;
    }
    request: XMLHttpRequest;

    constructor() {
        
    }
}