import { definePlugin } from "oxlint";

export default definePlugin({
  meta: {
    name: "test-plugin-options",
  },
  rules: {
    "check-options": {
      meta: {
        messages: {
          wrongValue: "Expected value to be {{expected}}, got {{actual}}",
          noOptions: "No options provided",
        },
      },
      create(context) {
        const { options } = context;

        // Check if options were passed correctly
        if (!options || options.length === 0) {
          context.report({
            message: "No options provided",
            loc: { start: { line: 1, column: 0 }, end: { line: 1, column: 1 } },
          });
        }

        return {
          DebuggerStatement(node) {
            // First option is a boolean
            const shouldReport = options[0];

            if (shouldReport) {
              context.report({
                message: JSON.stringify(context.options, null, 2),
                node,
              });
            }
          },
        };
      },
    },
  },
});
