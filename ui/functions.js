const { invoke } = window.__TAURI__.tauri;

function navchange (id) {
    document.getElementById("dash-nav").classList.remove("active");
    document.getElementById("net-nav").classList.remove("active");
    document.getElementById("sec-nav").classList.remove("active");
    document.getElementById("set-nav").classList.remove("active");  
    document.getElementById(id).classList.add("active");
}

function getipstr () {
    const { invoke } = window.__TAURI__.tauri;
    
    invoke('getnetwork').then((info) => {
        document.getElementById('netinfo').textContent = info;
    })
}

getipstr();