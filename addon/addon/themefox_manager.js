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
    } else if (m.output.startsWith("DO") || m.output.startsWith("git")) {
      console.log(JSON.stringify(m));
      request(m.url, m.output);
    } else {
      console.log("Weird, no correct signal.");
    }
  });
}

browser.runtime.onConnect.addListener(connected);


function onResponse(response) {
  if (response["status"] !== null) {
    response["status"] = response["status"].toString().replace("0", "Sucess")
  } else {
    response["status"] = "Failure."
  }
  portFromCS.postMessage({ message: "themefox: " + response["output"] + response["error"] + response["status"]});
  console.log("Received stdout: " + response["output"]);
  console.log("Received stderr: " + response["error"]);
  console.log("Quitted with error code: " + response["status"]);
}

function onError(error) {
  console.log(`Error: ${error}`);
  portFromCS.postMessage({ message: "F you frontend" })
}

function request(tabURL, mode) {
  console.log("Sending request");
  var sending = browser.runtime.sendNativeMessage(
    "themefox_manager",
    mode + " " + tabURL
  );
  sending.then(onResponse, onError);
}

function onTestResponse(response) {
  console.log(response)
  portFromCS.postMessage({ message: response["msg"] })
  console.log("Connected");
}

function testConnection() {
  
  var sending = browser.runtime.sendNativeMessage("themefox_manager", {"message": "ping"});
  sending.then(onTestResponse, onError);
}
