function change() {
    socket.send("Hello")
}

class Socket {
    server: {
        ip?: string
    }
    request: XMLHttpRequest;

    send(message_name: string, content?: {}) {
        // open request and set Header 'Message-Name'
        this.request.open('PUT', this.server.ip, true)
        this.request.setRequestHeader('Message-Name', message_name)
        // send request
        if (content) {
            this.request.send(JSON.stringify(content))
        } else {
            this.request.send()
        }

        this.request.onload = function () {
            console.log(this.status)
            console.log(this.response)
        }
    }

    constructor(domain?: string, port?: string) {
        this.server = {}
        this.server.ip = "http://" + (domain ? domain : location.hostname) + ':' + (port ? port : location.port) + "/socket/message"

        this.request = new XMLHttpRequest()
        this.request.responseType = "json"
    }
}

let socket = new Socket();

/*class SocketNoRt {
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
}*/