console.log("#test123");

const load = function() {
  const elem = document.getElementById("app");
  if(elem){
    elem.innerHTML = "<h1>Hello</h1>";
  }
}
load();