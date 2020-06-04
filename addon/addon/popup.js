window.addEventListener('DOMContentLoaded', async (event) => {
    var tab = await browser.tabs.query({
        currentWindow: true,
        active: true,
      });
      //var tab = tab.then(browserTabs);
      
    document.getElementById("URL").value = tab[0].url;
});

function browserTabs(e){
    return e;
}