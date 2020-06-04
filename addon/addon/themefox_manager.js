/*
On a click on the browser action, send the app a message.
*/
var portFromCS;

function connected(p) {
  portFromCS = p;
  //portFromCS.postMessage({ greeting: "hi there content script!" });
  portFromCS.onMessage.addListener(function (m) {
    //console.log("In background script, received message from content script");
    if (m.output == "ping") {
      testConnection();
    } else if (m.output.startsWith("DO")) {
      console.log(JSON.stringify(m));
      request(m.url);
    }
  });
}

browser.runtime.onConnect.addListener(connected);


function onResponse(response) {
  console.log("Received stdout: " + response["output"]);
  console.log("Received stderr: " + response["error"]);
  console.log("Quitted with error code: " + response["status"]);
}

function onError(error) {
  console.log(`Error: ${error}`);
}

function request(tabURL) {
  console.log("Sending request");
  var sending = browser.runtime.sendNativeMessage(
    "themefox_manager",
    "DO " + tabURL
  );
  sending.then(onResponse, onError);
}

function onTestResponse(response) {
  portFromCS.postMessage({ message: response["msg"] })
}

function testConnection() {
  console.log("Connected");
  var sending = browser.runtime.sendNativeMessage("themefox_manager", "ping");
  sending.then(onTestResponse, onError);
}
