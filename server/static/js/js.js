document.addEventListener('DOMContentLoaded', () => {
  var websocket = new WebSocket('ws://' + window.location.hostname + ':' + window.location.port + '/ws');
  websocket.onmessage = myOnMessage;
  console.log(websocket);
});

function myOnMessage(message) {
  let li = document.createElement("li");
  li.innerText = message.data;
  document.getElementById("list").append(li);

}
