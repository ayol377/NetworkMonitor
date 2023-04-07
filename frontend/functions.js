const { invoke } = window.__TAURI__.tauri;

var selected_dev = "";

function navchange (id) {
    document.getElementById("dash-nav").classList.remove("active");
    document.getElementById("net-nav").classList.remove("active");
    document.getElementById("sec-nav").classList.remove("active");
    document.getElementById("set-nav").classList.remove("active");  
    document.getElementById(id).classList.add("active");

    switch (id) {
        case "dash-nav":
            document.getElementById("dashboard-div").style.visibility="visible";
            document.getElementById("security-div").style.visibility="hidden";
            document.getElementById("settings-div").style.visibility="hidden";
            break;
        
        case "net-nav":
            document.getElementById("dashboard-div").style.visibility="hidden";
            document.getElementById("security-div").style.visibility="hidden";
            document.getElementById("settings-div").style.visibility="hidden";
            break;

        case "sec-nav":
            document.getElementById("dashboard-div").style.visibility="hidden";
            document.getElementById("security-div").style.visibility="visible";
            document.getElementById("settings-div").style.visibility="hidden";
            break;

        case "set-nav":
            document.getElementById("dashboard-div").style.visibility="hidden";
            document.getElementById("security-div").style.visibility="hidden";
            document.getElementById("settings-div").style.visibility="visible";
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
        var html = '';
        for (var i in devices) {
            var dev = devices[i];
            var idname = t.concat(dev[0]);
            var payload = "<li style='cursor:pointer;' class='list-group-item act_p' id='dev_id' onclick= devicenavupdate('div_id')><div class='fw-bold'>Device_Name <span class='badge bg_color '> status_text </span></div>MAC: dev_mac <br> IP: ip_addr </li>";
            payload = payload.replace("div_id", idname);
            payload = payload.replace("dev_id", idname);
            payload = payload.replace("dev_mac", dev[0].toUpperCase());
            payload = payload.replace("ip_addr", dev[1]);
            if (dev[3] == "up") {
                payload = payload.replace("bg_color", "bg-success");
                payload = payload.replace("status_text", "Online");
            }else{
                payload = payload.replace("bg_color", "bg-secondary");
                payload = payload.replace("status_text", "Offline");
            }
            if (selected_dev == dev[0]){
                console.log(dev[1], "is selected. setting as active");
                payload = payload.replace("act_p", "active");
            }else{
                payload = payload.replace("act_p", "");
            }
            payload = payload.replace("Device_Name", dev[2]);

            html = html.concat(payload);
        }
        console.log("updated!");
        if (html != ""){
            list_div.innerHTML = html;
        }
    });
}

async function refresh(){
    while (true) {
        devlistgen();
    }
}

function devicenavupdate(idname){
    var deac_item = document.getElementsByClassName("list-group-item active");
    if (deac_item.item(0) != null){
        deac_item.item(0).classList.remove("active");
    }
    var act_item = document.getElementById(idname);
    act_item.classList.add("active");
    selected_dev = idname.replace("dev_", "");
    console.log(selected_dev, "set as dev selected!");
    var mac = idname.replace("dev_", "");
    setDetails(mac);
    console.log("updated!");
}

function setDetails(mac){
    var devName = document.getElementById("dev_name");
    var ipAdd = document.getElementById("ip_add");
    var MacAdd = document.getElementById("mac_add");
    var manf = document.getElementById("man_name");
    invoke('getdev', { mac: mac }).then((dev) => {
        devName.innerHTML = dev[0];
        ipAdd.innerHTML = dev[1];
        MacAdd.innerHTML = dev[2];
        manf.innerHTML = dev[3];
    });
}

function updateSettings(setting){
    invoke('update_setting', {setting: setting});
    console.log(setting);
    setTimeout(settings, 1000);
}

function settings(){
    var dns_btn = document.getElementById("DNS-BTN");
    var mit_btn = document.getElementById("MIT-BTN");
    var evt_btn = document.getElementById("EVT-BTN");
    var cld_btn = document.getElementById("CLD-BTN");
    invoke('get_settings').then((settings) => {
        console.log(settings);
        if (settings[0] == "1"){
            dns_btn.innerHTML = "Disable";
            dns_btn.classList.remove("btn-primary");
            dns_btn.classList.add("btn-danger");
        }else{
            dns_btn.innerHTML = "Enable";
            dns_btn.classList.add("btn-primary");
            dns_btn.classList.remove("btn-danger");
        }
        if (settings[1] == "1"){
            mit_btn.innerHTML = "Disable";
            mit_btn.classList.remove("btn-primary");
            mit_btn.classList.add("btn-danger");
        }else{
            mit_btn.innerHTML = "Enable";
            mit_btn.classList.add("btn-primary");
            mit_btn.classList.remove("btn-danger");
        }
        if (settings[2] == "1"){
            evt_btn.innerHTML = "Disable";
            evt_btn.classList.remove("btn-primary");
            evt_btn.classList.add("btn-danger");
        }else{
            evt_btn.innerHTML = "Enable";
            evt_btn.classList.add("btn-primary");
            evt_btn.classList.remove("btn-danger");
        }
        if (settings[3] == "1"){
            cld_btn.innerHTML = "Disable";
            cld_btn.classList.remove("btn-primary");
            cld_btn.classList.add("btn-danger");
        }else{
            cld_btn.innerHTML = "Enable";
            cld_btn.classList.add("btn-primary");
            cld_btn.classList.remove("btn-danger");
        }
    });
}

function alert_gen(){
    invoke('get_alert_list').then((alerts) => {
        var html = "";
        for (i in alerts){
            alert = alerts[i];
            var payload = '<div class="alert alert-type m-0" role="alert">time date desc</div>';
            payload = payload.replace("type", alert[2]);
            payload = payload.replace("time", alert[0]);
            payload = payload.replace("date", alert[1]);
            payload = payload.replace("desc", alert[3]);
            var html = html.concat(payload);
        }
        var list = document.getElementById("notifications");
        list.innerHTML = html;
    });
}

function logout(){
    invoke('logout');
    setTimeout(getaccount, 1000);
}

function signup(){
    var success = document.getElementById("acct-success");
    var fail = document.getElementById("acct-err");
    fail.style.visibility="hidden";
    success.style.visibility="hidden";
    var email = document.getElementById("email").value;
    var passwd = document.getElementById("password").value;

    invoke('signup', {email: email, passwd: passwd}).then((msg) => {
        console.log(msg);
        if (msg != "Ok"){
            fail.style.visibility="inherit";
            fail.innerHTML = msg;
        }else {
            success.style.visibility="inherit";
            success.innerHTML = "Logged in";
        }
    });
    setTimeout(getaccount, 2000);
}

function getaccount(){
    invoke('getaccount').then((email) => {
        var acct_div = document.getElementById("acct-login-info");
        if (email != ""){
            var msg = "Logged in as: ".concat(email);
            acct_div.innerText = msg;
        }else {
            var msg = "Not Logged in".concat(email);
            acct_div.innerText = msg;
        }
    });
}

function login(){
    var success = document.getElementById("acct-success");
    var fail = document.getElementById("acct-err");
    var email = document.getElementById("email").value;
    var passwd = document.getElementById("password").value;

    invoke('login', {email: email, passwd: passwd}).then((msg) => {
        console.log(msg);
        if (msg != "Ok"){
            fail.style.visibility="inherit";
            fail.innerHTML = msg;
        }else {
            success.style.visibility="inherit";
            success.innerHTML = "Logged in";
        }
    });
    setTimeout(getaccount, 2000);
}

getaccount();
setInterval(alert_gen, 5000);
setTimeout(settings, 1000);
getipstr();
setInterval(devlistgen, 5000);