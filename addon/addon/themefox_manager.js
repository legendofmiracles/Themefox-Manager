/*
On a click on the browser action, send the app a message.
*/


function onResponse(response) {
  console.log("Received stdout: " + response["output"]);
  console.log("Received stderr: " + response["error"]);
  console.log("Quitted with error code: " + response["status"]);
}

function onError(error) {
  console.log(`Error: ${error}`);
}


function request(tabURL){
  console.log("Sending request");
  var sending = browser.runtime.sendNativeMessage(
    "themefox_manager",
    "DO " + tabURL
  );
  sending.then(onResponse, onError);
}

function onTestResponse(response) {
  if (response["output"] == "pong"){
    return true;
  } else {
    return false;
  }
}

function testConnection(){
  console.log("Sending test");
  var sending = browser.runtime.sendNativeMessage(
    "themefox_manager",
    "ping"
  );
  sending.then(onTestResponse, onTestError);
}
