import { Client } from "howl-network";

function test() {
    console.log(Client);
    const subscriber = Client.subscriber();
    console.log("subscriber", subscriber);
    subscriber.connect("ws://127.0.0.1:8000");
    console.log("Connected");
    console.log("starting to listen for data");
    const token = subscriber.listenForData(function(json) {
        console.log("listenForData cb -> ", json);
    });
    console.log("token: ", token);
}

test()