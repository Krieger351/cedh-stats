import { Command } from "commander";
const program = new Command();

import { registerCommands } from "./commands";

program.name("edh util").description("cli to some edh utils").version("0.8.0");

registerCommands(program);

program.parse();
