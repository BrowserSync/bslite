import { start } from "../../index.js";

start(1, (arg1, value) => {
    console.log("incoming...", JSON.parse(value))
}).then((x) => console.log("all done!")).catch(e => console.error("nope!"))