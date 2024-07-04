#! /usr/bin/env node
import process from "node:process";
import { commandRunner } from "./generated/components/morphir_platform.js";

function run(args: string[]) {
  try {
    const result = commandRunner.run(args);
    console.log("Result", result);
  } catch (error) {
    console.error("Error", error);
  }
}

console.log("Process: ", process.argv, process.argv0);
run(process.argv);
