import type {Input, NsInput} from "../types";
import {NS_INPUT_LABELS} from "../types";

class ParserError extends Error {
}

const VALID_INPUTS = new Set(Object.keys(NS_INPUT_LABELS));

export function parseExpression(text: string): Input | null {
    if (!text.trim()) return null;

    // Tokenize
    const regex = /\(|\)|and|or|\w+/gi;
    const tokens = text.match(regex) || [];

    let current = 0;

    function parseOr(): Input {
        let left = parseAnd();

        while (current < tokens.length && tokens[current].toLowerCase() === "or") {
            current++;
            const right = parseAnd();
            left = {Binary: {left, right, operator: "Or"}};
        }

        return left;
    }

    function parseAnd(): Input {
        let left = parsePrimary();

        while (current < tokens.length && tokens[current].toLowerCase() === "and") {
            current++;
            const right = parsePrimary();
            left = {Binary: {left, right, operator: "And"}};
        }

        return left;
    }

    function parsePrimary(): Input {
        if (current >= tokens.length) {
            throw new ParserError("Unexpected end of expression");
        }

        const token = tokens[current];

        if (token === "(") {
            current++;
            const expr = parseOr();
            if (current >= tokens.length || tokens[current] !== ")") {
                throw new ParserError("Expected \")\"");
            }
            current++;
            return {Grouping: {input: expr}};
        }

        if (token.toLowerCase() === "and" || token.toLowerCase() === "or" || token === ")") {
            throw new ParserError(`Unexpected token '${token}'`);
        }

        if (!VALID_INPUTS.has(token)) {
            const valid = Array.from(VALID_INPUTS).find(
                (v) => v.toLowerCase() === token.toLowerCase()
            );
            if (valid) {
                current++;
                return {Value: {input: valid as NsInput}};
            }
            throw new ParserError(`Invalid input '${token}'`);
        }

        current++;
        return {Value: {input: token as NsInput}};
    }

    try {
        const result = parseOr();
        if (current < tokens.length) {
            throw new ParserError(`Unexpected token '${tokens[current]}' at end`);
        }
        return result;
    } catch (e) {
        if (e instanceof ParserError) {
            return null;
        }
        throw e;
    }
}

export function stringifyCondition(cond: Input): string {
    if ("Value" in cond) {
        return cond.Value.input;
    }
    if ("Grouping" in cond) {
        return `(${stringifyCondition(cond.Grouping.input)})`;
    }
    if ("Binary" in cond) {
        const op = cond.Binary.operator === "And" ? "and" : "or";
        return `${stringifyCondition(cond.Binary.left)} ${op} ${stringifyCondition(cond.Binary.right)}`;
    }
    return "";
}