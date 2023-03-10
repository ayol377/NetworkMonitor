const invoke = window.__TAURI__.invoke

export async function invoke_getnetwork() {
    console.log("Bridged");
    return await invoke("getnetwork");
}