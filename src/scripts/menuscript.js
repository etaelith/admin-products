const { getTauriVersion } = window.__TAURI__.app;
function normalizePathname(pathname) {
    if (pathname.startsWith("./")) {
        return pathname.substring(2);
    }
    if (pathname.startsWith("/")) {
        return pathname.substring(1);
    }
    return pathname;
}
document.addEventListener("DOMContentLoaded", async function () {
    const links = document.getElementsByClassName("linkActive")
    const dato = await window.location.href
    console.log(dato)
    const currentUrl = window.location.pathname;
    const normalize = normalizePathname(currentUrl)
    for (let i = 0; i < links.length; i++) {
        const link = links[i];
        if (normalizePathname(link.getAttribute("href")) === normalize) {
            link.classList.add("active")
        } else {
            link.classList.remove("active")
        }
    }

})
