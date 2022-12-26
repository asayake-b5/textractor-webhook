document.addEventListener('DOMContentLoaded', () => {
  var websocket = new WebSocket('ws://' + window.location.hostname + ':' + window.location.port + '/ws');
  websocket.onmessage = myOnMessage;
  console.log(websocket);
});

function myOnMessage(message) {
  let p = document.createElement("p");
  p.innerText = message.data;
  document.body.append(p);

}
