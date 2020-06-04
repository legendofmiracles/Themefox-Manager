var myport;
window.addEventListener("DOMContentLoaded", async (event) => {
  var tab = await browser.tabs.query({
    currentWindow: true,
    active: true,
  });
  //var tab = tab.then(browserTabs);

  var input = document.getElementById("URL") 
  input.value = tab[0].url;
  
  // connects to bg script
  myPort = browser.runtime.connect({ name: "lolsu@themefox.net" });
  //sends a message
  myPort.postMessage({ output: "ping" });
  // on a message
  myPort.onMessage.addListener(function (m) {
    //console.log("In content script, received message from background script: ");
    console.log(m.message);
    if (m.message != "pong"){
        browser.browserAction.setPopup()
    }
    //if ()
  });

  
});