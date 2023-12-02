import dotenv from "dotenv";
import path from "path";

const getInput = async ({ year, day, session }) => {
  const resp = await fetch(
    `https://www.adventofcode.com/${year}/day/${day}/input`,
    {
      method: "GET",
      headers: { session: session },
    }
  );
  return resp.text();
};

export default function (plop) {
  plop.setGenerator("rust", {
    description: "Create a new day's Puzzle for Rust",
    prompts: [
      {
        type: "input",
        name: "day",
        message: "What Day of the puzzle to initialize?",
      },
    ],

    actions: (data) => {
      data.day_raw = Number(data.day); // the raw day with zero padding
      data.day = data.day.padStart(2, "0");
      const actions = [];
      const env = dotenv.config({
        path: path.join(plop.getPlopfilePath(), ".env"),
      }).parsed;
      const input = getInput({ ...data, ...env });
      actions.push(
        async (data) => {
          data.input = await getInput({ ...data, ...env });
          return "Downloaded Puzzle Input!";
        },
        {
          type: "addMany",
          destination: "rust/day_{{day}}",
          base: `templates/rust`,
          templateFiles: `templates/**/*.hbs`,
        },
        {
          type: "add",
          path: "rust/day_{{day}}/input.txt",
          template: "{{input}}",
        },
        {
          type: "append",
          path: "Cargo.toml",
          pattern: "#plop-members-prefix",
          template: '\t"rust/day_{{day}}",',
        },
        () =>
          `Find Today's Puzzle at https://adventofcode.com/${env.year}/day/${data.day_raw}`
      );
      return actions;
    },
  });
}
