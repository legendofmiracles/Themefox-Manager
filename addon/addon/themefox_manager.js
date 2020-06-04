/*
On a click on the browser action, send the app a message.
*/
browser.browserAction.onClicked.addListener(async () => {
 /* function onGot(tabInfo) {
    console.log(tabInfo[0].url);
  }
  */
/*
  function onError(error) {
    console.log(`Error: ${error}`);
  }
  */

  const gettingCurrent = await browser.tabs.query({
    currentWindow: true,
    active: true,
  });
  
  //console.log(url);
  console.log("Sending request");
  var sending = browser.runtime.sendNativeMessage(
    "themefox_manager",
    "DO " + gettingCurrent[0].url
  );
  sending.then(onResponse, onError);
});

function onResponse(response) {
  console.log("Received stdout: " + response["output"]);
  console.log("Received stderr: " + response["error"]);
  console.log("Quitted with error code: " + response["status"]);
}

function onError(error) {
  console.log(`Error: ${error}`);
}
/*
function url(){
  var querying = browser.tabs.query({currentWindow: true, active: true});
  
  return querying.then((tabs) => {
    console.log(tabs[0].url);
    return tabs[0].url;
  });
}
*/
/*
async function url(){
  return (await browser.tabs.query({currentWindow: true, active: true}))[0].url
}
*/
