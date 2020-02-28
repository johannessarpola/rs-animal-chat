(function(){function r(e,n,t){function o(i,f){if(!n[i]){if(!e[i]){var c="function"==typeof require&&require;if(!f&&c)return c(i,!0);if(u)return u(i,!0);var a=new Error("Cannot find module '"+i+"'");throw a.code="MODULE_NOT_FOUND",a}var p=n[i]={exports:{}};e[i][0].call(p.exports,function(r){var n=e[i][1][r];return o(n||r)},p,p.exports,r,e,n,t)}return n[i].exports}for(var u="function"==typeof require&&require,i=0;i<t.length;i++)o(t[i]);return o}return r})()({1:[function(require,module,exports){
// https://github.com/browserify/browserify#usage
// $sudo npm install -g browserify
// $browserify index.js > bundle.js

const socket = new WebSocket("ws://127.0.0.1:8080");
let userId = "";
let open = false;
let userInputs = [];
let server = []

function newMessageEntry(msg) {
  return {
    msg: msg,
    timestamp: new Date(),
    id: userId
  }
}

function removeMessages() {
  const messages = document.getElementById("messages");
  while (messages.firstChild) {
    messages.removeChild(messages.firstChild);
  }
}


socket.addEventListener('open', function (event) {
  console.log("Start to chat");
});

const clear = document.getElementById("clear");
clear.onclick = removeMessages;

const exit = document.getElementById("exit");
exit.onclick = function () {
  socket.close();
};

const form = document.getElementById("form");

form.onsubmit = function (event) {
  event.preventDefault();
  const input = document.getElementById("msg");

  if (input.value === "") {
    return;
  }

  const me = newMessageEntry(input.value);
  const meJson = JSON.stringify(me);
  userInputs.push(meJson);
  socket.send(meJson);
  input.value = "";
  setTimeout(() => window.scrollTo({ top: window.innerHeight, behavior: "auto" }), 10);
};

socket.onmessage = function (event) {
  
  console.log("onmessage");
  console.log(event.data);
  
  const messageEntry = JSON.parse(event.data);
  if(messageEntry.msg == null && messageEntry.timestamp == null) {
    userId = messageEntry.id;
  }
  console.log(messageEntry);
  const msg = messageEntry.msg;
  server.push({ origin : event.origin, timestamp: new Date(), entry: messageEntry});

  if (userInputs[userInputs.length - 1] === "!warn") {
    alert("You sent warning to the other users");
  }

  if (msg.includes("!clearall")) {
    removeMessages();
    return;
  }

  if (msg.includes("!exitall")) {
    socket.close();
    return;
  }

  if (!open) {
    // to give id to user and verify the maximum number, only work once

    const messages = document.getElementById("messages");
    const li = document.createElement("li");
    const p = document.createElement("p");

    open = true;

    p.textContent = `Your id is ${userId} and "You" will be used in this page instead`;
    p.className = "blue";
    li.append(p)
    messages.append(li);
    return;
  } else {
    const authorOfMessage = messageEntry.id;

    const messages = document.getElementById("messages");
    const li = document.createElement("li");

    let displayMessage = '';
    if (authorOfMessage === userId) {
      li.className = "red-white";
      displayMessage += 'You';
    } else {
      displayMessage += authorOfMessage;
    }

    displayMessage += ": "
    displayMessage += msg;

    const p = document.createElement("p");
    p.append(displayMessage)
    li.append(p);
    messages.append(li);
    return;
  }
};

socket.onclose = function (event) {
  const closeMessage = event.data === undefined ? "Server, You or another user closed the connection." : "WebSocket is closed now."
  const messages = document.getElementById("messages");

  const li = document.createElement("li");
  li.append(closeMessage)
  li.className = "blue";
  messages.append(li);

  localStorage.setItem("userInputs", `[${userInputs}]`);
  localStorage.setItem("server", `[${server}]`);
};

},{}]},{},[1]);
