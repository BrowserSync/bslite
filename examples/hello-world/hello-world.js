import { start } from "../../index.js";

start(1).then((x) => console.log("all done!")).catch(e => console.error("nope!"))