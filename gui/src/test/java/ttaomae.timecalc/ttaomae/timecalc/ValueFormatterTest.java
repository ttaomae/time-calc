package ttaomae.timecalc;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.Test;

public class ValueFormatterTest
{
    @Test
    public void testInputSameDigit()
    {
        var formatter = new ValueFormatter();
        assertEquals("0:00:00", formatter.toString());
        assertEquals("0n", formatter.inputCharacter('n'));
        assertEquals("0n", formatter.toString());
        assertEquals("0:00:00", formatter.inputCharacter('n'));
        assertEquals("0:00:00", formatter.toString());

        assertInputCharacter("0:00:01", "1n", formatter, '1');
        assertInputCharacter("0:00:11", "11n", formatter, '1');
        assertInputCharacter("0:01:11", "111n", formatter, '1');
        assertInputCharacter("0:11:11", "1111n", formatter, '1');
        assertInputCharacter("1:11:11", "11111n", formatter, '1');
        assertInputCharacter("11:11:11", "111111n", formatter, '1');
        assertInputCharacter("111:11:11", "1111111n", formatter, '1');
        assertInputCharacter("1111:11:11", "11111111n", formatter, '1');
        assertInputCharacter("1111:11:11.0", "11111111.0n", formatter, '.');
        assertInputCharacter("1111:11:11.1", "11111111.1n", formatter, '1');
        assertInputCharacter("1111:11:11.11", "11111111.11n", formatter, '1');
        assertInputCharacter("1111:11:11.111", "11111111.111n", formatter, '1');
        assertInputCharacter("1111:11:11.1111", "11111111.1111n", formatter, '1');
        assertInputCharacter("1111:11:11.11111", "11111111.11111n", formatter, '1');
        assertInputCharacter("1111:11:11.111111", "11111111.111111n", formatter, '1');
        assertInputCharacter("1111:11:11.1111111", "11111111.1111111n", formatter, '1');
        assertInputCharacter("1111:11:11.11111111", "11111111.11111111n", formatter, '1');
        assertInputCharacter("1111:11:11.111111111", "11111111.111111111n", formatter, '1');
        assertInputCharacter("1111:11:11.111111111", "11111111.111111111n", formatter, '1');
        assertInputCharacter("1111:11:11.111111111", "11111111.111111111n", formatter, '1');
        assertInputCharacter("1111:11:11.111111111", "11111111.111111111n", formatter, '1');

        formatter.clear();
        assertEquals("0:00:00", formatter.toString());
    }

    @Test
    public void testInputDifferentDigits()
    {
        var formatter = new ValueFormatter();
        assertInputCharacter("0:00:09", "9n", formatter, '9');
        assertInputCharacter("0:00:98", "98n", formatter, '8');
        assertInputCharacter("0:09:87", "987n", formatter, '7');
        assertInputCharacter("0:98:76", "9876n", formatter, '6');
        assertInputCharacter("9:87:65", "98765n", formatter, '5');
        assertInputCharacter("98:76:54", "987654n", formatter, '4');
        assertInputCharacter("987:65:43", "9876543n", formatter, '3');
        assertInputCharacter("9876:54:32", "98765432n", formatter, '2');
        assertInputCharacter("98765:43:21", "987654321n", formatter, '1');
        assertInputCharacter("987654:32:10", "9876543210n", formatter, '0');
        assertInputCharacter("987654:32:10.0", "9876543210.0n", formatter, '.');
        assertInputCharacter("987654:32:10.1", "9876543210.1n",formatter, '1');
        assertInputCharacter("987654:32:10.12", "9876543210.12n",formatter, '2');
        assertInputCharacter("987654:32:10.123", "9876543210.123n",formatter, '3');
        assertInputCharacter("987654:32:10.1234", "9876543210.1234n",formatter, '4');
        assertInputCharacter("987654:32:10.12345", "9876543210.12345n",formatter, '5');
        assertInputCharacter("987654:32:10.123456", "9876543210.123456n",formatter, '6');
        assertInputCharacter("987654:32:10.1234567", "9876543210.1234567n",formatter, '7');
        assertInputCharacter("987654:32:10.12345678", "9876543210.12345678n",formatter, '8');
        assertInputCharacter("987654:32:10.123456789", "9876543210.123456789n",formatter, '9');

        formatter.clear();
        assertInputCharacter("0:00:01", "1n", formatter, '1');
        assertInputCharacter("0:00:10", "10n", formatter, '0');
        assertInputCharacter("0:01:02", "102n", formatter, '2');
        assertInputCharacter("0:10:20", "1020n", formatter, '0');
        assertInputCharacter("1:02:03", "10203n", formatter, '3');
        assertInputCharacter("10:20:30", "102030n",formatter, '0');
        assertInputCharacter("10:20:30.0", "102030.0n", formatter, '.');
        assertInputCharacter("10:20:30.4", "102030.4n", formatter, '4');
        assertInputCharacter("10:20:30.40", "102030.40n", formatter, '0');
        assertInputCharacter("10:20:30.405", "102030.405n", formatter, '5');
        assertInputCharacter("10:20:30.4050", "102030.4050n", formatter, '0');

        formatter.clear();
        assertInputCharacter("0:00:00", "0n", formatter, '0');
        assertInputCharacter("0:00:00", "0n", formatter, '0');
        assertInputCharacter("0:00:00", "0n", formatter, '0');
        assertInputCharacter("0:00:00.0", "0.0n",formatter, '.');
        assertInputCharacter("0:00:00.0", "0.0n", formatter, '0');
        assertInputCharacter("0:00:00.02", "0.02n", formatter, '2');
        assertInputCharacter("0:00:00.024", "0.024n", formatter, '4');
        assertInputCharacter("0:00:00.0240", "0.0240n", formatter, '0');
        assertInputCharacter("0:00:00.02406", "0.02406n", formatter, '6');
        assertInputCharacter("0:00:00.024060", "0.024060n", formatter, '0');
        assertInputCharacter("0:00:00.0240600", "0.0240600n", formatter, '0');
        assertInputCharacter("0:00:00.02406008", "0.02406008n", formatter, '8');
        assertInputCharacter("0:00:00.024060080", "0.024060080n", formatter, '0');

        formatter.clear();
        assertInputCharacter("0:00:00.0", "0.0n", formatter, '.');
        assertInputCharacter("0:00:00.0", "0.0n", formatter, '0');
        assertInputCharacter("0:00:00.00", "0.00n", formatter, '0');
        assertInputCharacter("0:00:00.000", "0.000n", formatter, '0');
        assertInputCharacter("0:00:00.0000", "0.0000n", formatter, '0');
        assertInputCharacter("0:00:00.00000", "0.00000n", formatter, '0');
        assertInputCharacter("0:00:00.000000", "0.000000n", formatter, '0');
        assertInputCharacter("0:00:00.0000000", "0.0000000n", formatter, '0');
        assertInputCharacter("0:00:00.00000000", "0.00000000n", formatter, '0');
        assertInputCharacter("0:00:00.000000000", "0.000000000n", formatter, '0');
    }

    @Test
    public void testInputIgnoredCharacters()
    {
        var formatter = new ValueFormatter();
        assertInputCharacter("0:00:00", "0n", formatter, 'a');
        assertInputCharacter("0:00:01", "1n", formatter, '1');
        assertInputCharacter("0:00:01", "1n", formatter, 'b');
        assertInputCharacter("0:00:12", "12n", formatter, '2');
        assertInputCharacter("0:00:12", "12n", formatter, 'c');
        assertInputCharacter("0:01:23", "123n", formatter, '3');
        assertInputCharacter("0:01:23.0", "123.0n", formatter, '.');
        assertInputCharacter("0:01:23.0", "123.0n", formatter, '.');
        assertInputCharacter("0:01:23.0", "123.0n", formatter, '.');
        assertInputCharacter("0:01:23.4", "123.4n", formatter, '4');
        assertInputCharacter("0:01:23.4", "123.4n", formatter, 'x');
        assertInputCharacter("0:01:23.45", "123.45n", formatter, '5');
        assertInputCharacter("0:01:23.45", "123.45n", formatter, 'y');
        assertInputCharacter("0:01:23.456", "123.456n", formatter, '6');
        assertInputCharacter("0:01:23.456", "123.456n", formatter, 'z');
        assertInputCharacter("0:01:23.4567", "123.4567n", formatter, '7');
        assertInputCharacter("0:01:23.4567", "123.4567n", formatter, '.');
        assertInputCharacter("0:01:23.45678", "123.45678n", formatter, '8');
        assertInputCharacter("0:01:23.45678", "123.45678n", formatter, '+');
        assertInputCharacter("0:01:23.456789", "123.456789n", formatter, '9');
    }

    private static void assertInputCharacter(String time, String number, ValueFormatter formatter, char ch)
    {
        assertEquals(time, formatter.inputCharacter(ch));
        assertEquals(time, formatter.toString());
        assertEquals(number, formatter.inputCharacter('n'));
        assertEquals(number, formatter.toString());
        assertEquals(time, formatter.inputCharacter('n'));
        assertEquals(time, formatter.toString());
    }
}
