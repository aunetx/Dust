class Socket {
    constructor(domain, port) {
        this.callback_list = {};
        this.server = {};
        this.server.ip = "http://" + (domain ? domain : location.hostname) + ':' + (port ? port : location.port) + "/socket/message";
        this.request = new XMLHttpRequest();
        this.request.responseType = "json";
    }
    // * Send message
    send(message_name, content) {
        // open request and set Header 'Message-Name'
        this.request.open('PUT', this.server.ip, true);
        this.request.setRequestHeader('Message-Name', message_name);
        // send request
        if (content) {
            this.request.send(JSON.stringify(content));
        }
        else {
            this.request.send();
        }
        // get response and launch user-defined function
        var socketClass = this;
        this.request.onload = (d) => {
            if (this.request.status == 200) {
                socketClass.callback_list[message_name](this.request.response);
            }
        };
        return this;
    }
    callback(fun) {
        this.request.onload = () => {
            fun(this.request.response);
        };
    }
    // TODO implement socket.on() firing on server message
    // * Set response for message
    on(name, callback) {
        this.callback_list[name] = callback;
    }
}
//# sourceMappingURL=socket.js.map