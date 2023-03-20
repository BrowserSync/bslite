import { start } from "../../index.js";

start(process.argv.slice(2)).then((x) => console.log("all done!")).catch(e => console.error("nope!"))