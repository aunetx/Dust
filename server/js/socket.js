function change(message) {
    send_message(message)
}

function send_message(message) {
    var ip = "http://" + location.host + "/socket/message",
        request = new XMLHttpRequest();

    request.responseType = "json";
    request.open('PUT', ip, true);

    request.onreadystatechange = function () {
        if (request.readyState == 4 && request.status == 200) {

            document.getElementById("demo").innerHTML = request.response.res;

        }
    }

    request.send();
}