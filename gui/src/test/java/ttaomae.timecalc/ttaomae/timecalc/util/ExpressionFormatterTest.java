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

        assertInputCharacter.accept("0:00:00", '0');
        assertInputCharacter.accept("0:00:01", '1');
        assertInputCharacter.accept("0:00:02", '2');
        assertInputCharacter.accept("0:00:03", '3');
        assertInputCharacter.accept("0:00:04", '4');
        assertInputCharacter.accept("0:00:05", '5');
        assertInputCharacter.accept("0:00:06", '6');
        assertInputCharacter.accept("0:00:07", '7');
        assertInputCharacter.accept("0:00:08", '8');
        assertInputCharacter.accept("0:00:09", '9');
        assertInputCharacter.accept("0:00:00.0", '.');
        assertInputCharacter.accept("0n", 'n');
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
        assertInputCharacter("0:00:02", formatter, '2');
        assertInputCharacter("0:00:24", formatter, '4');
        assertInputCharacter("0:02:46", formatter, '6');
        assertInputCharacter("0:02:46 +", formatter, '+');
        assertInputCharacter("0:02:46 + 0:00:09", formatter, '9');
        assertInputCharacter("0:02:46 + 0:00:97", formatter, '7');
        assertInputCharacter("0:02:46 + 0:09:75", formatter, '5');
        assertInputCharacter("0:02:46 + 0:97:53", formatter, '3');
        assertInputCharacter("0:02:46 + 9:75:31", formatter, '1');
        assertInputCharacter("0:02:46 + 97:53:10", formatter, '0');
        assertInputCharacter("0:02:46 + 97:53:10 *", formatter, '*');
        assertInputCharacter("0:02:46 + 97:53:10 * (", formatter, '(');
        assertInputCharacter("0:02:46 + 97:53:10 * ((", formatter, '(');
        assertInputCharacter("0:02:46 + 97:53:10 * ((0:00:05", formatter, '5');
        assertInputCharacter("0:02:46 + 97:53:10 * ((5n", formatter, 'n');
        assertInputCharacter("0:02:46 + 97:53:10 * ((55n", formatter, '5');
        assertInputCharacter("0:02:46 + 97:53:10 * ((55.0n", formatter, '.');
        assertInputCharacter("0:02:46 + 97:53:10 * ((55.5n", formatter, '5');
        assertInputCharacter("0:02:46 + 97:53:10 * ((55.5n -", formatter, '-');
        assertInputCharacter("0:02:46 + 97:53:10 * ((55.5n - 0n", formatter, 'n');
        assertInputCharacter("0:02:46 + 97:53:10 * ((55.5n - 1n", formatter, '1');
        assertInputCharacter("0:02:46 + 97:53:10 * ((55.5n - 18n", formatter, '8');
        assertInputCharacter("0:02:46 + 97:53:10 * ((55.5n - 18n)", formatter, ')');
        assertInputCharacter("0:02:46 + 97:53:10 * ((55.5n - 18n) *", formatter, '*');
        assertInputCharacter("0:02:46 + 97:53:10 * ((55.5n - 18n) * 0:00:03", formatter, '3');
        assertInputCharacter("0:02:46 + 97:53:10 * ((55.5n - 18n) * 0:00:36", formatter, '6');
        assertInputCharacter("0:02:46 + 97:53:10 * ((55.5n - 18n) * 0:00:36 /", formatter, '/');
        assertInputCharacter("0:02:46 + 97:53:10 * ((55.5n - 18n) * 0:00:36 / 0:00:02", formatter, '2');
        assertInputCharacter("0:02:46 + 97:53:10 * ((55.5n - 18n) * 0:00:36 / 0:00:21", formatter, '1');
        assertInputCharacter("0:02:46 + 97:53:10 * ((55.5n - 18n) * 0:00:36 / 0:02:15", formatter, '5');
        assertInputCharacter("0:02:46 + 97:53:10 * ((55.5n - 18n) * 0:00:36 / 0:02:15)", formatter, ')');

        formatter.clear();
        assertEquals("", formatter.toString());

        // Input an invalid expression.
        assertInputCharacter("(", formatter, '(');
        assertInputCharacter("((", formatter, '(');
        assertInputCharacter("((0n", formatter, 'n');
        assertInputCharacter("((2n", formatter, '2');
        assertInputCharacter("((24n", formatter, '4');
        assertInputCharacter("((248n", formatter, '8');
        assertInputCharacter("((248.0n", formatter, '.');
        assertInputCharacter("((248.9n", formatter, '9');
        assertInputCharacter("((248.93n", formatter, '3');
        assertInputCharacter("((248.931n", formatter, '1');
        assertInputCharacter("((248.931n +", formatter, '+');
        assertInputCharacter("((248.931n + 0:00:05", formatter, '5');
        assertInputCharacter("((248.931n + 0:00:57", formatter, '7');
        assertInputCharacter("((248.931n + 0:05:74", formatter, '4');
        assertInputCharacter("((248.931n + 0:05:74)", formatter, ')');
        assertInputCharacter("((248.931n + 0:05:74) /", formatter, '/');
        assertInputCharacter("((248.931n + 0:05:74) / (", formatter, '(');
        assertInputCharacter("((248.931n + 0:05:74) / (0:00:06", formatter, '6');
        assertInputCharacter("((248.931n + 0:05:74) / (0:00:06 -", formatter, '-');
        assertInputCharacter("((248.931n + 0:05:74) / (0:00:06 - 0:00:02", formatter, '2');
        assertInputCharacter("((248.931n + 0:05:74) / (0:00:06 - 2n", formatter, 'n');
        assertInputCharacter("((248.931n + 0:05:74) / (0:00:06 - 2.0n", formatter, '.');
        assertInputCharacter("((248.931n + 0:05:74) / (0:00:06 - 2.2n", formatter, '2');
        assertInputCharacter("((248.931n + 0:05:74) / (0:00:06 - 2.2n)", formatter, ')');
        assertInputCharacter("((248.931n + 0:05:74) / (0:00:06 - 2.2n))", formatter, ')');
        assertInputCharacter("((248.931n + 0:05:74) / (0:00:06 - 2.2n)) *", formatter, '*');
        assertInputCharacter("((248.931n + 0:05:74) / (0:00:06 - 2.2n)) * (", formatter, '(');
        assertInputCharacter("((248.931n + 0:05:74) / (0:00:06 - 2.2n)) * (0:00:09", formatter, '9');
        assertInputCharacter("((248.931n + 0:05:74) / (0:00:06 - 2.2n)) * (0:00:99", formatter, '9');
        assertInputCharacter("((248.931n + 0:05:74) / (0:00:06 - 2.2n)) * (0:00:99.0", formatter, '.');
        assertInputCharacter("((248.931n + 0:05:74) / (0:00:06 - 2.2n)) * (0:00:99.9", formatter, '9');
        assertInputCharacter("((248.931n + 0:05:74) / (0:00:06 - 2.2n)) * (0:00:99.99", formatter, '9');


    }

    @Test
    public void testInputIgnoredCharacters()
    {
        var formatter = new ExpressionFormatter();

        assertInputCharacter("(", formatter, '(');

        // Cannot close an empty parenthesss.
        assertInputCharacter("(", formatter, ')');

        assertInputCharacter("(0:00:03", formatter, '3');
        assertInputCharacter("(3n", formatter, 'n');
        assertInputCharacter("(3n *", formatter, '*');

        // Cannot close a parenthesss after operator
        assertInputCharacter("(3n *", formatter, ')');

        assertInputCharacter("(3n * 0:00:01", formatter, '1');
        assertInputCharacter("(3n * 0:00:12", formatter, '2');
        assertInputCharacter("(3n * 0:01:20", formatter, '0');
        assertInputCharacter("(3n * 0:12:05", formatter, '5');
        assertInputCharacter("(3n * 0:12:05)", formatter, ')');

        // Cannot input closing parenthesis without matching opening parenthesis.
        assertInputCharacter("(3n * 0:12:05)", formatter, ')');

        // Cannot input number after closing parenthesis.
        assertInputCharacter("(3n * 0:12:05)", formatter, '7');
        assertInputCharacter("(3n * 0:12:05)", formatter, '.');
        assertInputCharacter("(3n * 0:12:05)", formatter, 'n');

        assertInputCharacter("(3n * 0:12:05) +", formatter, '+');
        assertInputCharacter("(3n * 0:12:05) + 0:00:04", formatter, '4');
        assertInputCharacter("(3n * 0:12:05) + 0:00:49", formatter, '9');
        assertInputCharacter("(3n * 0:12:05) + 0:04:93", formatter, '3');
        assertInputCharacter("(3n * 0:12:05) + 0:49:36", formatter, '6');

        // Cannot input closing parenthesis without matching opening parenthesis.
        assertInputCharacter("(3n * 0:12:05) + 0:49:36", formatter, ')');
    }

    private static void assertInputCharacter(String expected, ExpressionFormatter formatter, char ch)
    {
        assertEquals(expected, formatter.inputCharacter(ch));
        assertEquals(expected, formatter.toString());
    }
}
