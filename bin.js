#!/usr/bin/env node
console.log('from cli package');

const { start } = require("./index.js");
start(1).catch(console.error);
