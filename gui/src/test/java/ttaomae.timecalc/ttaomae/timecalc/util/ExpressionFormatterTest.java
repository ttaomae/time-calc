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
        assertInputCharacter.accept("0", '#');
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
        assertInputCharacter("2s", formatter, '2');
        assertInputCharacter("24s", formatter, '4');
        assertInputCharacter("02:46", formatter, '6');
        assertInputCharacter("02:46 +", formatter, '+');
        assertInputCharacter("02:46 + 9s", formatter, '9');
        assertInputCharacter("02:46 + 97s", formatter, '7');
        assertInputCharacter("02:46 + 09:75", formatter, '5');
        assertInputCharacter("02:46 + 97:53", formatter, '3');
        assertInputCharacter("02:46 + 9:75:31", formatter, '1');
        assertInputCharacter("02:46 + 97:53:10", formatter, '0');
        assertInputCharacter("02:46 + 97:53:10 *", formatter, '*');
        assertInputCharacter("02:46 + 97:53:10 * (", formatter, '(');
        assertInputCharacter("02:46 + 97:53:10 * ((", formatter, '(');
        assertInputCharacter("02:46 + 97:53:10 * ((5s", formatter, '5');
        assertInputCharacter("02:46 + 97:53:10 * ((5", formatter, '#');
        assertInputCharacter("02:46 + 97:53:10 * ((55", formatter, '5');
        assertInputCharacter("02:46 + 97:53:10 * ((55.0", formatter, '.');
        assertInputCharacter("02:46 + 97:53:10 * ((55.5", formatter, '5');
        assertInputCharacter("02:46 + 97:53:10 * ((55.5 -", formatter, '-');
        assertInputCharacter("02:46 + 97:53:10 * ((55.5 - 0", formatter, '#');
        assertInputCharacter("02:46 + 97:53:10 * ((55.5 - 1", formatter, '1');
        assertInputCharacter("02:46 + 97:53:10 * ((55.5 - 18", formatter, '8');
        assertInputCharacter("02:46 + 97:53:10 * ((55.5 - 18)", formatter, ')');
        assertInputCharacter("02:46 + 97:53:10 * ((55.5 - 18) *", formatter, '*');
        assertInputCharacter("02:46 + 97:53:10 * ((55.5 - 18) * 3s", formatter, '3');
        assertInputCharacter("02:46 + 97:53:10 * ((55.5 - 18) * 36s", formatter, '6');
        assertInputCharacter("02:46 + 97:53:10 * ((55.5 - 18) * 36s /", formatter, '/');
        assertInputCharacter("02:46 + 97:53:10 * ((55.5 - 18) * 36s / 2s", formatter, '2');
        assertInputCharacter("02:46 + 97:53:10 * ((55.5 - 18) * 36s / 21s", formatter, '1');
        assertInputCharacter("02:46 + 97:53:10 * ((55.5 - 18) * 36s / 02:15", formatter, '5');
        assertInputCharacter("02:46 + 97:53:10 * ((55.5 - 18) * 36s / 02:15)", formatter, ')');

        formatter.clear();
        assertEquals("", formatter.toString());

        // Input an invalid expression.
        assertInputCharacter("(", formatter, '(');
        assertInputCharacter("((", formatter, '(');
        assertInputCharacter("((0", formatter, '#');
        assertInputCharacter("((2", formatter, '2');
        assertInputCharacter("((24", formatter, '4');
        assertInputCharacter("((248", formatter, '8');
        assertInputCharacter("((248.0", formatter, '.');
        assertInputCharacter("((248.9", formatter, '9');
        assertInputCharacter("((248.93", formatter, '3');
        assertInputCharacter("((248.931", formatter, '1');
        assertInputCharacter("((248.931 +", formatter, '+');
        assertInputCharacter("((248.931 + 5s", formatter, '5');
        assertInputCharacter("((248.931 + 57s", formatter, '7');
        assertInputCharacter("((248.931 + 05:74", formatter, '4');
        assertInputCharacter("((248.931 + 05:74)", formatter, ')');
        assertInputCharacter("((248.931 + 05:74) /", formatter, '/');
        assertInputCharacter("((248.931 + 05:74) / (", formatter, '(');
        assertInputCharacter("((248.931 + 05:74) / (6s", formatter, '6');
        assertInputCharacter("((248.931 + 05:74) / (6s -", formatter, '-');
        assertInputCharacter("((248.931 + 05:74) / (6s - 2s", formatter, '2');
        assertInputCharacter("((248.931 + 05:74) / (6s - 2", formatter, '#');
        assertInputCharacter("((248.931 + 05:74) / (6s - 2.0", formatter, '.');
        assertInputCharacter("((248.931 + 05:74) / (6s - 2.2", formatter, '2');
        assertInputCharacter("((248.931 + 05:74) / (6s - 2.2)", formatter, ')');
        assertInputCharacter("((248.931 + 05:74) / (6s - 2.2))", formatter, ')');
        assertInputCharacter("((248.931 + 05:74) / (6s - 2.2)) *", formatter, '*');
        assertInputCharacter("((248.931 + 05:74) / (6s - 2.2)) * (", formatter, '(');
        assertInputCharacter("((248.931 + 05:74) / (6s - 2.2)) * (9s", formatter, '9');
        assertInputCharacter("((248.931 + 05:74) / (6s - 2.2)) * (99s", formatter, '9');
        assertInputCharacter("((248.931 + 05:74) / (6s - 2.2)) * (99.0s", formatter, '.');
        assertInputCharacter("((248.931 + 05:74) / (6s - 2.2)) * (99.9s", formatter, '9');
        assertInputCharacter("((248.931 + 05:74) / (6s - 2.2)) * (99.99s", formatter, '9');


    }

    @Test
    public void testInputIgnoredCharacters()
    {
        var formatter = new ExpressionFormatter();

        assertInputCharacter("(", formatter, '(');

        // Cannot close an empty parenthesss.
        assertInputCharacter("(", formatter, ')');

        assertInputCharacter("(3s", formatter, '3');
        assertInputCharacter("(3", formatter, '#');
        assertInputCharacter("(3 *", formatter, '*');

        // Cannot close a parentheses after operator
        assertInputCharacter("(3 *", formatter, ')');

        assertInputCharacter("(3 * 1s", formatter, '1');
        assertInputCharacter("(3 * 12s", formatter, '2');
        assertInputCharacter("(3 * 01:20", formatter, '0');
        assertInputCharacter("(3 * 12:05", formatter, '5');
        assertInputCharacter("(3 * 12:05)", formatter, ')');

        // Cannot input closing parenthesis without matching opening parenthesis.
        assertInputCharacter("(3 * 12:05)", formatter, ')');

        // Cannot input number after closing parenthesis.
        assertInputCharacter("(3 * 12:05)", formatter, '7');
        assertInputCharacter("(3 * 12:05)", formatter, '.');
        assertInputCharacter("(3 * 12:05)", formatter, '#');

        assertInputCharacter("(3 * 12:05) +", formatter, '+');
        assertInputCharacter("(3 * 12:05) + 4s", formatter, '4');
        assertInputCharacter("(3 * 12:05) + 49s", formatter, '9');
        assertInputCharacter("(3 * 12:05) + 04:93", formatter, '3');
        assertInputCharacter("(3 * 12:05) + 49:36", formatter, '6');

        // Cannot input closing parenthesis without matching opening parenthesis.
        assertInputCharacter("(3 * 12:05) + 49:36", formatter, ')');
    }

    private static void assertInputCharacter(String expected, ExpressionFormatter formatter, char ch)
    {
        assertEquals(expected, formatter.inputCharacter(ch));
        assertEquals(expected, formatter.toString());
    }
}
