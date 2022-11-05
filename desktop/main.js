const { app, BrowserWindow } = require("electron");
const join_path = require("path").join;

// console.log("MAIN");
// console.log(__dirname);

function createWindow() {
    const win = new BrowserWindow({
        webPreferences: {contextIsolation:false,nodeIntegration:true},
    });
    win.loadFile(join_path(__dirname, "front/index.html"));
}

app.whenReady().then(() => {
    createWindow();
});

app.on("window-all-closed", () => {
    app.quit();
});