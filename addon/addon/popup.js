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

  if (document.getElementsByTagName("textarea")) {
    var injection = document.createElement("textarea")
    document.getElementById("install-button").after(injection, { id: "textarea"});
    let textarea = document.getElementsByTagName("textarea");
    textarea[0].toggleAttribute("readonly");
    textarea[0].id = "textarea"
    textarea[0].rows = 20;
    textarea[0].placeholder = "The output from the native application will appear here."
    textarea[0].value = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut facilisis, arcu vitae adipiscing placerat, nisl lectus accumsan nisi, vitae iaculis sem neque vel lectus. Praesent tristique commodo lorem quis fringilla. Sed ac tellus eros. Sed consectetur eleifend felis vitae luctus. Praesent sagittis, est eget bibendum tincidunt, ligula diam tincidunt augue, a fermentum odio velit eget mi. Phasellus mattis, elit id fringilla semper, orci magna cursus ligula, non venenatis lacus augue sit amet dui. Pellentesque lacinia odio id nisi pulvinar commodo tempus at odio. Ut consectetur eros porttitor nunc mollis ultrices. Aenean porttitor, purus sollicitudin viverra auctor, neque erat blandit sapien, sit amet tincidunt massa mi ac nibh. Proin nibh sem, bibendum ut placerat nec, cursus et lacus. Phasellus vel augue turpis. Nunc eu mauris eu leo blandit mollis interdum eget lorem. ";
  } else {
    console.log("Element exists.");
  }
}