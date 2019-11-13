package ttaomae.timecalc.util;

import org.junit.jupiter.api.Test;

import java.util.function.BiConsumer;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class ExpressionFormatterTest
{
    @Test
    public void testInputFirstCharacter()
    {
        var formatter = new ExpressionFormatter();
        assertEquals("", formatter.toString());

        BiConsumer<String, Character> assertInputCharacter = (expected, input) -> {
            assertEquals(expected, formatter.inputCharacter(input));
            formatter.clear();
        };

        assertInputCharacter.accept("0s", '0');
        assertInputCharacter.accept("1s", '1');
        assertInputCharacter.accept("2s", '2');
        assertInputCharacter.accept("3s", '3');
        assertInputCharacter.accept("4s", '4');
        assertInputCharacter.accept("5s", '5');
        assertInputCharacter.accept("6s", '6');
        assertInputCharacter.accept("7s", '7');
        assertInputCharacter.accept("8s", '8');
        assertInputCharacter.accept("9s", '9');
        assertInputCharacter.accept("0.0s", '.');
        assertInputCharacter.accept("0", ExpressionFormatter.TOGGLE_TYPE_CHARACTER);
        assertInputCharacter.accept("(", '(');

        // These characters should be ignored.
        assertEquals("", formatter.inputCharacter('+'));
        assertEquals("", formatter.inputCharacter('-'));
        assertEquals("", formatter.inputCharacter('*'));
        assertEquals("", formatter.inputCharacter('/'));
        assertEquals("", formatter.inputCharacter(')'));
    }

    @Test
    public void testInputValidExpression()
    {
        var formatter = new ExpressionFormatter();
        assertEquals("", formatter.toString());

        // Input a valid expression.
        assertInputAndDeleteCharacter("2s", "0s", formatter, '2');
        assertInputAndDeleteCharacter("24s", formatter, '4');
        assertInputAndDeleteCharacter("02:46", formatter, '6');
        assertInputAndDeleteCharacter("02:46 +", formatter, '+');
        assertInputAndDeleteCharacter("02:46 + 9s", "02:46 + 0s", formatter, '9');
        assertInputAndDeleteCharacter("02:46 + 97s", formatter, '7');
        assertInputAndDeleteCharacter("02:46 + 09:75", formatter, '5');
        assertInputAndDeleteCharacter("02:46 + 97:53", formatter, '3');
        assertInputAndDeleteCharacter("02:46 + 9:75:31", formatter, '1');
        assertInputAndDeleteCharacter("02:46 + 97:53:10", formatter, '0');
        assertInputAndDeleteCharacter("02:46 + 97:53:10 *", formatter, '*');
        assertInputAndDeleteCharacter("02:46 + 97:53:10 * (", formatter, '(');
        assertInputAndDeleteCharacter("02:46 + 97:53:10 * ((", formatter, '(');
        assertInputAndDeleteCharacter(
                "02:46 + 97:53:10 * ((5s",
                "02:46 + 97:53:10 * ((0s", formatter, '5');
        assertInputCharacter("02:46 + 97:53:10 * ((5", formatter, ExpressionFormatter.TOGGLE_TYPE_CHARACTER);
        assertInputAndDeleteCharacter("02:46 + 97:53:10 * ((55", formatter, '5');
        assertInputAndDeleteCharacter("02:46 + 97:53:10 * ((55.0", formatter, '.');
        assertInputAndDeleteCharacter("02:46 + 97:53:10 * ((55.5", formatter, '5');
        assertInputAndDeleteCharacter("02:46 + 97:53:10 * ((55.5 -", formatter, '-');
        assertInputCharacter("02:46 + 97:53:10 * ((55.5 - 0", formatter, ExpressionFormatter.TOGGLE_TYPE_CHARACTER);
        assertInputAndDeleteCharacter("02:46 + 97:53:10 * ((55.5 - 1", formatter, '1');
        assertInputAndDeleteCharacter("02:46 + 97:53:10 * ((55.5 - 18", formatter, '8');
        assertInputAndDeleteCharacter("02:46 + 97:53:10 * ((55.5 - 18)", formatter, ')');
        assertInputAndDeleteCharacter("02:46 + 97:53:10 * ((55.5 - 18) *", formatter, '*');
        assertInputAndDeleteCharacter(
                "02:46 + 97:53:10 * ((55.5 - 18) * 3s",
                "02:46 + 97:53:10 * ((55.5 - 18) * 0s", formatter, '3');
        assertInputAndDeleteCharacter("02:46 + 97:53:10 * ((55.5 - 18) * 36s", formatter, '6');
        assertInputAndDeleteCharacter("02:46 + 97:53:10 * ((55.5 - 18) * 36s /", formatter, '/');
        assertInputAndDeleteCharacter(
                "02:46 + 97:53:10 * ((55.5 - 18) * 36s / 2s",
                "02:46 + 97:53:10 * ((55.5 - 18) * 36s / 0s", formatter, '2');
        assertInputAndDeleteCharacter("02:46 + 97:53:10 * ((55.5 - 18) * 36s / 21s", formatter, '1');
        assertInputAndDeleteCharacter("02:46 + 97:53:10 * ((55.5 - 18) * 36s / 02:15", formatter, '5');
        assertInputAndDeleteCharacter("02:46 + 97:53:10 * ((55.5 - 18) * 36s / 02:15)", formatter, ')');

        formatter.clear();
        assertEquals("", formatter.toString());

        // Input an invalid expression.
        assertInputAndDeleteCharacter("(", formatter, '(');
        assertInputAndDeleteCharacter("((", formatter, '(');
        assertInputAndDeleteCharacter("((0", formatter, ExpressionFormatter.TOGGLE_TYPE_CHARACTER);
        assertInputAndDeleteCharacter("((2", formatter, '2');
        assertInputAndDeleteCharacter("((24", formatter, '4');
        assertInputAndDeleteCharacter("((248", formatter, '8');
        assertInputAndDeleteCharacter("((248.0", formatter, '.');
        assertInputAndDeleteCharacter("((248.9", formatter, '9');
        assertInputAndDeleteCharacter("((248.93", formatter, '3');
        assertInputAndDeleteCharacter("((248.931", formatter, '1');
        assertInputAndDeleteCharacter("((248.931 +", formatter, '+');
        assertInputAndDeleteCharacter("((248.931 + 5s", "((248.931 + 0s", formatter, '5');
        assertInputAndDeleteCharacter("((248.931 + 57s", formatter, '7');
        assertInputAndDeleteCharacter("((248.931 + 05:74", formatter, '4');
        assertInputAndDeleteCharacter("((248.931 + 05:74)", formatter, ')');
        assertInputAndDeleteCharacter("((248.931 + 05:74) /", formatter, '/');
        assertInputAndDeleteCharacter("((248.931 + 05:74) / (", formatter, '(');
        assertInputAndDeleteCharacter(
                "((248.931 + 05:74) / (6s",
                "((248.931 + 05:74) / (0s", formatter, '6');
        assertInputAndDeleteCharacter("((248.931 + 05:74) / (6s -", formatter, '-');
        assertInputAndDeleteCharacter(
                "((248.931 + 05:74) / (6s - 2s",
                "((248.931 + 05:74) / (6s - 0s", formatter, '2');
        assertInputCharacter("((248.931 + 05:74) / (6s - 2", formatter, ExpressionFormatter.TOGGLE_TYPE_CHARACTER);
        assertInputAndDeleteCharacter("((248.931 + 05:74) / (6s - 2.0", formatter, '.');
        assertInputAndDeleteCharacter("((248.931 + 05:74) / (6s - 2.2", formatter, '2');
        assertInputAndDeleteCharacter("((248.931 + 05:74) / (6s - 2.2)", formatter, ')');
        assertInputAndDeleteCharacter("((248.931 + 05:74) / (6s - 2.2))", formatter, ')');
        assertInputAndDeleteCharacter("((248.931 + 05:74) / (6s - 2.2)) *", formatter, '*');
        assertInputAndDeleteCharacter("((248.931 + 05:74) / (6s - 2.2)) * (", formatter, '(');
        assertInputAndDeleteCharacter(
                "((248.931 + 05:74) / (6s - 2.2)) * (9s",
                "((248.931 + 05:74) / (6s - 2.2)) * (0s", formatter, '9');
        assertInputAndDeleteCharacter("((248.931 + 05:74) / (6s - 2.2)) * (99s", formatter, '9');
        assertInputAndDeleteCharacter("((248.931 + 05:74) / (6s - 2.2)) * (99.0s", formatter, '.');
        assertInputAndDeleteCharacter("((248.931 + 05:74) / (6s - 2.2)) * (99.9s", formatter, '9');
        assertInputAndDeleteCharacter("((248.931 + 05:74) / (6s - 2.2)) * (99.99s", formatter, '9');
    }

    @Test
    public void testInputIgnoredCharacters()
    {
        var formatter = new ExpressionFormatter();

        assertInputAndDeleteCharacter("(", formatter, '(');

        // Cannot close an empty parentheses.
        assertInputCharacter("(", formatter, ')');

        assertInputCharacter("(3s", formatter, '3');
        assertInputCharacter("(3", formatter, ExpressionFormatter.TOGGLE_TYPE_CHARACTER);
        assertInputAndDeleteCharacter("(3 *", formatter, '*');

        // Cannot close a parentheses after operator
        assertInputCharacter("(3 *", formatter, ')');

        assertInputCharacter("(3 * 1s", formatter, '1');
        assertInputAndDeleteCharacter("(3 * 12s", formatter, '2');
        assertInputAndDeleteCharacter("(3 * 01:20", formatter, '0');
        assertInputAndDeleteCharacter("(3 * 12:05", formatter, '5');
        assertInputAndDeleteCharacter("(3 * 12:05)", formatter, ')');

        // Cannot input closing parenthesis without matching opening parenthesis.
        assertInputCharacter("(3 * 12:05)", formatter, ')');

        // Cannot input number after closing parenthesis.
        assertInputCharacter("(3 * 12:05)", formatter, '7');
        assertInputCharacter("(3 * 12:05)", formatter, '.');
        assertInputCharacter("(3 * 12:05)", formatter, ExpressionFormatter.TOGGLE_TYPE_CHARACTER);

        assertInputAndDeleteCharacter("(3 * 12:05) +", formatter, '+');
        assertInputCharacter("(3 * 12:05) + 4s", formatter, '4');
        assertInputAndDeleteCharacter("(3 * 12:05) + 49s", formatter, '9');
        assertInputAndDeleteCharacter("(3 * 12:05) + 04:93", formatter, '3');
        assertInputAndDeleteCharacter("(3 * 12:05) + 49:36", formatter, '6');

        // Cannot input closing parenthesis without matching opening parenthesis.
        assertInputCharacter("(3 * 12:05) + 49:36", formatter, ')');
    }

    @Test
    public void testDeleteCharacter()
    {
        var formatter = new ExpressionFormatter();

        // Delete empty formatter.
        assertEquals("", formatter.deleteCharacter());

        // Delete 0s.
        assertEquals("0s", formatter.inputCharacter('0'));
        assertEquals("", formatter.deleteCharacter());

        // Delete 0.
        assertEquals("0", formatter.inputCharacter(ExpressionFormatter.TOGGLE_TYPE_CHARACTER));
        assertEquals("", formatter.deleteCharacter());

        // Delete a non-zero time.
        assertEquals("1s", formatter.inputCharacter('1'));
        assertEquals("0s", formatter.deleteCharacter());
        assertEquals("", formatter.deleteCharacter());

        // Delete a non-zero number.
        assertEquals("2s", formatter.inputCharacter('2'));
        assertEquals("2", formatter.inputCharacter(ExpressionFormatter.TOGGLE_TYPE_CHARACTER));
        assertEquals("0", formatter.deleteCharacter());
        assertEquals("", formatter.deleteCharacter());
    }

    private static void assertInputAndDeleteCharacter(String expected, ExpressionFormatter formatter, char ch)
    {
        String beforeInput = formatter.toString();
        assertInputAndDeleteCharacter(expected, beforeInput, formatter, ch);
    }

    private static void assertInputAndDeleteCharacter(
            String expected, String expectedDelete, ExpressionFormatter formatter, char ch)
    {
        // Input character and check expression.
        assertEquals(expected, formatter.inputCharacter(ch));
        assertEquals(expected, formatter.toString());

        // Delete character and check that expression is the same as before inputting character.
        assertEquals(expectedDelete, formatter.deleteCharacter());
        assertEquals(expectedDelete, formatter.toString());

        // Input character to return formatter to expected state.
        formatter.inputCharacter(ch);
    }

    private static void assertInputCharacter(String expected, ExpressionFormatter formatter, char ch)
    {
        assertEquals(expected, formatter.inputCharacter(ch));
        assertEquals(expected, formatter.toString());
    }
}
