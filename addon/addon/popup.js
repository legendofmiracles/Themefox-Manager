var myport;

window.addEventListener("DOMContentLoaded", async (event) => {
  var tab = await browser.tabs.query({
    currentWindow: true,
    active: true,
  });
  //var tab = tab.then(browserTabs);

  var input = document.getElementById("URL");
  input.value = tab[0].url;
  document.getElementById("install-button").onclick = install;

  // connects to bg script
  myport = browser.runtime.connect({ name: "lolsu@themefox.net" });
  //sends a message
  myport.postMessage({ output: "ping" });
  // on a message
  myport.onMessage.addListener(function (m) {
    //console.log("In content script, received message from background script: ");
    //console.log(m.message);
    if (m.message == "LOLSU") {

    } else if (m.message != "pong") {
      browser.browserAction.setPopup({ popup: "error.html" })
      console.log("Error: didn't get a anwser from the native application");
    }
    //if ()
  });


});

function install() {
  var mode = document.getElementById("select")
  var strMode = mode.options[mode.selectedIndex].text
  //console.log("mode: " + strMode);
  var url = document.getElementById("URL").value;
  //console.log("url: " + url)
  if (strMode == "Themefox") {
    strMode = "DO"
  } else if (strMode == "git") { }
  else {
    console.log("Weird, no correct mode.")
  }
  myport.postMessage({ output: strMode, url: url });

  if (!document.getElementById("textarea")) {
    var injection = document.createElement("textarea")
    document.getElementById("install-button").after(injection, { id: "textarea", readOnly: true });
  } else {
    console.log("Element exists.");
  }
}