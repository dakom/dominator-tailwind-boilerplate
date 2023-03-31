/* this is just needed to patch the github pages deployment
 * because github pages does not support relative paths
 */

const fs = require("fs");
const path = require("path");

(async () => {
    const appConfig = await readAsync("./app.config.json");
    const pkgConfig = await readAsync("./package.json");
    const ROOTS = ["css", "media", pkgConfig.name];

    const {uri_root} = JSON.parse(appConfig);
    if(!uri_root) {
        throw new Error("`uri_root` not found in app.config.json");
    }

    let data = await readAsync("./dist/index.html");
    if(uri_root !== "") {
        for(const root of ROOTS) {
            data = data.replaceAll(`"/${root}`, `"/${uri_root}/${root}`);
            data = data.replaceAll(`'/${root}`, `'/${uri_root}/${root}`);
        }
    }

    await writeAsync("./dist/index.html", data);
    await writeAsync("./dist/404.html", data);
})();

function readAsync(src) {
    return new Promise((resolve, reject) => {
        fs.readFile(path.resolve(src), "utf-8", (err, data) => {
            if(err) {
                reject(err);
            } else {
                resolve(data);
            }
        });
    });
}

function writeAsync(dest, data) {
    return new Promise((resolve, reject) => {
        fs.writeFile(path.resolve(dest), data, { encoding: "utf8", flag: "w", }, (err) => {
            if(err) {
                reject(err);
            } else {
                resolve();
            }
        });
    });
}