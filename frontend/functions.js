const { invoke } = window.__TAURI__.tauri;

function navchange (id) {
    document.getElementById("dash-nav").classList.remove("active");
    document.getElementById("net-nav").classList.remove("active");
    document.getElementById("sec-nav").classList.remove("active");
    document.getElementById("set-nav").classList.remove("active");  
    document.getElementById(id).classList.add("active");

    switch (id) {
        case "dash-nav":
            document.getElementById("dashboard-div").style.visibility="initial";
            break;
        
        case "net-nav":
            document.getElementById("dashboard-div").style.visibility="hidden";
            break;

        case "sec-nav":
            document.getElementById("dashboard-div").style.visibility="hidden";
            break;

        case "set-nav":
            document.getElementById("dashboard-div").style.visibility="hidden";
            break;

        default:
            break;
    }
}

function getipstr () {
    invoke('getnetwork').then((info) => {
        document.getElementById('netinfo').textContent = info;
    });
}

function devlistgen(){
    const t = "dev_";
    invoke('getdevs').then((devices) =>{
        var list_div = document.getElementById("dev-list");
        list_div.innerHTML = '';
        var html = '';
        for (var i in devices) {
            console.log(devices);
            var dev = devices[i];
            var idname = t.concat(dev[0]);
            var payload = "<li style='cursor:pointer;' class='list-group-item' id='dev_id' onclick= devicenavupdate('div_id')><div class='fw-bold'>Device Name <span class='badge bg_color '> status_text </span></div>MAC: dev_mac <br> IP: ip_addr </li>";
            payload = payload.replace("div_id", idname);
            payload = payload.replace("dev_id", idname);
            payload = payload.replace("dev_mac", dev[0].toUpperCase());
            payload = payload.replace("ip_addr", dev[1]);
            if (dev[2] == "up") {
                payload = payload.replace("bg_color", "bg-success");
                payload = payload.replace("status_text", "Online");
            }else{
                payload = payload.replace("bg_color", "bg-secondary");
                payload = payload.replace("status_text", "Offline");
            }
            html = html.concat(payload);
        }
        console.log(html);
        list_div.innerHTML = html;
    });
}

async function refresh(){
    while (true) {
        devlistgen();
    }
}

getipstr();
devlistgen();


function devicenavupdate(idname){
    var deac_item = document.getElementsByClassName("list-group-item active");
    if (deac_item.item(0) != null){
        deac_item.item(0).classList.remove("active");
    }
    var act_item = document.getElementById(idname);
    act_item.classList.add("active");
    console.log("updated!");
}