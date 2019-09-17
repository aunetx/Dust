function change(message) {
    let socket = new Socket()
    socket.send(message, function (text) {
        document.getElementById("demo").innerHTML = text;
    });
}

class Socket {
    ip: string;

    send(message, callback) {
        var request = new XMLHttpRequest();

        request.responseType = "json";
        request.open('PUT', this.ip, true);

        request.onreadystatechange = function () {
            if (request.readyState == 4) {
                if (request.status == 200) {
                    callback(request.response.res);
                } else {
                    console.log("Cannot retrieve socket answer : HTTP status code is " + request.status)
                }
            }
        }

        request.send();
    }

    constructor() {
        this.ip = "http://" + location.host + "/socket/message";
    }
}