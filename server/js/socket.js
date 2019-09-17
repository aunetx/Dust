function change(message) {

    function callback(text) {
        document.getElementById("demo").innerHTML = text;
    }

    socket.send(message, function (text) {
        document.getElementById("demo").innerHTML = text;
    });
    console.log("Hello!");
}


var socket = {
    ip_prepend: "http://",
    ip_append: "/socket/message",
    server_url: location.host,

    send: async function (message, callback) {
        var request = new XMLHttpRequest(),
            ip = this.ip_prepend + this.server_url + this.ip_append;

        request.responseType = "json";
        request.open('PUT', ip, true);

        request.onreadystatechange = function () {
            if (request.readyState == 4) {
                if (request.status == 200) {
                    callback(request.response.res);
                } else {
                    console.log("Cannot retrieve socket answer : HTTP code " + request.status)
                }
            }
        }

        request.send();
    }
}