package ttaomae.timecalc;

import static org.junit.jupiter.api.Assertions.assertEquals;
import org.junit.jupiter.api.Test;

public class TimeFormatterTest
{
    @Test
    public void testInputSameDigit()
    {
        var formatter = new TimeFormatter();
        assertEquals("0:00:00", formatter.toString());

        assertInputCharacter("0:00:01", formatter, '1');
        assertInputCharacter("0:00:11", formatter, '1');
        assertInputCharacter("0:01:11", formatter, '1');
        assertInputCharacter("0:11:11", formatter, '1');
        assertInputCharacter("1:11:11", formatter, '1');
        assertInputCharacter("11:11:11", formatter, '1');
        assertInputCharacter("111:11:11", formatter, '1');
        assertInputCharacter("1111:11:11", formatter, '1');
        assertInputCharacter("1111:11:11.0", formatter, '.');
        assertInputCharacter("1111:11:11.1", formatter, '1');
        assertInputCharacter("1111:11:11.11", formatter, '1');
        assertInputCharacter("1111:11:11.111", formatter, '1');
        assertInputCharacter("1111:11:11.1111", formatter, '1');
        assertInputCharacter("1111:11:11.11111", formatter, '1');
        assertInputCharacter("1111:11:11.111111", formatter, '1');
        assertInputCharacter("1111:11:11.1111111", formatter, '1');
        assertInputCharacter("1111:11:11.11111111", formatter, '1');
        assertInputCharacter("1111:11:11.111111111", formatter, '1');
        assertInputCharacter("1111:11:11.111111111", formatter, '1');
        assertInputCharacter("1111:11:11.111111111", formatter, '1');
        assertInputCharacter("1111:11:11.111111111", formatter, '1');

        formatter.clear();
        assertEquals("0:00:00", formatter.toString());
    }

    @Test
    public void testInputDifferentDigits()
    {
        var formatter = new TimeFormatter();
        assertInputCharacter("0:00:09", formatter, '9');
        assertInputCharacter("0:00:98", formatter, '8');
        assertInputCharacter("0:09:87", formatter, '7');
        assertInputCharacter("0:98:76", formatter, '6');
        assertInputCharacter("9:87:65", formatter, '5');
        assertInputCharacter("98:76:54", formatter, '4');
        assertInputCharacter("987:65:43", formatter, '3');
        assertInputCharacter("9876:54:32", formatter, '2');
        assertInputCharacter("98765:43:21", formatter, '1');
        assertInputCharacter("987654:32:10", formatter, '0');
        assertInputCharacter("987654:32:10.0", formatter, '.');
        assertInputCharacter("987654:32:10.1", formatter, '1');
        assertInputCharacter("987654:32:10.12", formatter, '2');
        assertInputCharacter("987654:32:10.123", formatter, '3');
        assertInputCharacter("987654:32:10.1234", formatter, '4');
        assertInputCharacter("987654:32:10.12345", formatter, '5');
        assertInputCharacter("987654:32:10.123456", formatter, '6');
        assertInputCharacter("987654:32:10.1234567", formatter, '7');
        assertInputCharacter("987654:32:10.12345678", formatter, '8');
        assertInputCharacter("987654:32:10.123456789", formatter, '9');

        formatter.clear();
        assertInputCharacter("0:00:01", formatter, '1');
        assertInputCharacter("0:00:10", formatter, '0');
        assertInputCharacter("0:01:02", formatter, '2');
        assertInputCharacter("0:10:20", formatter, '0');
        assertInputCharacter("1:02:03", formatter, '3');
        assertInputCharacter("10:20:30", formatter, '0');
        assertInputCharacter("10:20:30.0", formatter, '.');
        assertInputCharacter("10:20:30.4", formatter, '4');
        assertInputCharacter("10:20:30.40", formatter, '0');
        assertInputCharacter("10:20:30.405", formatter, '5');
        assertInputCharacter("10:20:30.4050", formatter, '0');

        formatter.clear();
        assertInputCharacter("0:00:00", formatter, '0');
        assertInputCharacter("0:00:00", formatter, '0');
        assertInputCharacter("0:00:00", formatter, '0');
        assertInputCharacter("0:00:00.0", formatter, '.');
        assertInputCharacter("0:00:00.0", formatter, '0');
        assertInputCharacter("0:00:00.02", formatter, '2');
        assertInputCharacter("0:00:00.024", formatter, '4');
        assertInputCharacter("0:00:00.0240", formatter, '0');
        assertInputCharacter("0:00:00.02406", formatter, '6');
        assertInputCharacter("0:00:00.024060", formatter, '0');
        assertInputCharacter("0:00:00.0240600", formatter, '0');
        assertInputCharacter("0:00:00.02406008", formatter, '8');
        assertInputCharacter("0:00:00.024060080", formatter, '0');

        formatter.clear();
        assertInputCharacter("0:00:00.0", formatter, '.');
        assertInputCharacter("0:00:00.0", formatter, '0');
        assertInputCharacter("0:00:00.00", formatter, '0');
        assertInputCharacter("0:00:00.000", formatter, '0');
        assertInputCharacter("0:00:00.0000", formatter, '0');
        assertInputCharacter("0:00:00.00000", formatter, '0');
        assertInputCharacter("0:00:00.000000", formatter, '0');
        assertInputCharacter("0:00:00.0000000", formatter, '0');
        assertInputCharacter("0:00:00.00000000", formatter, '0');
        assertInputCharacter("0:00:00.000000000", formatter, '0');
    }

    @Test
    public void testInputIgnoredCharacters()
    {
        var formatter = new TimeFormatter();
        assertInputCharacter("0:00:00", formatter, 'a');
        assertInputCharacter("0:00:01", formatter, '1');
        assertInputCharacter("0:00:01", formatter, 'b');
        assertInputCharacter("0:00:12", formatter, '2');
        assertInputCharacter("0:00:12", formatter, 'c');
        assertInputCharacter("0:01:23", formatter, '3');
        assertInputCharacter("0:01:23.0", formatter, '.');
        assertInputCharacter("0:01:23.0", formatter, '.');
        assertInputCharacter("0:01:23.0", formatter, '.');
        assertInputCharacter("0:01:23.4", formatter, '4');
        assertInputCharacter("0:01:23.4", formatter, 'x');
        assertInputCharacter("0:01:23.45", formatter, '5');
        assertInputCharacter("0:01:23.45", formatter, 'y');
        assertInputCharacter("0:01:23.456", formatter, '6');
        assertInputCharacter("0:01:23.456", formatter, 'z');
        assertInputCharacter("0:01:23.4567", formatter, '7');
        assertInputCharacter("0:01:23.4567", formatter, '.');
        assertInputCharacter("0:01:23.45678", formatter, '8');
        assertInputCharacter("0:01:23.45678", formatter, '+');
        assertInputCharacter("0:01:23.456789", formatter, '9');

    }

    private static void assertInputCharacter(String expected, TimeFormatter formatter, char ch)
    {
        assertEquals(expected, formatter.inputCharacter(ch));
        assertEquals(expected, formatter.toString());
    }
}
