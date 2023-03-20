#!/usr/bin/env node
console.log('from cli package');

const { start } = require("./index.js");
start(process.argv.slice(2)).catch(console.error);
