package ttaomae.timecalc.util;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class ValueFormatterTest
{
    @Test
    public void testInputSameDigit()
    {
        var formatter = new ValueFormatter();
        assertEquals("0s", formatter.toString());
        assertEquals("0", formatter.inputCharacter(ValueFormatter.TOGGLE_TYPE_CHARACTER));
        assertEquals("0", formatter.toString());
        assertEquals("0s", formatter.inputCharacter(ValueFormatter.TOGGLE_TYPE_CHARACTER));
        assertEquals("0s", formatter.toString());

        assertInputAndDeleteCharacter("1s", "1", formatter, '1');
        assertInputAndDeleteCharacter("11s", "11", formatter, '1');
        assertInputAndDeleteCharacter("01:11", "111", formatter, '1');
        assertInputAndDeleteCharacter("11:11", "1111", formatter, '1');
        assertInputAndDeleteCharacter("1:11:11", "11111", formatter, '1');
        assertInputAndDeleteCharacter("11:11:11", "111111", formatter, '1');
        assertInputAndDeleteCharacter("111:11:11", "1111111", formatter, '1');
        assertInputAndDeleteCharacter("1111:11:11", "11111111", formatter, '1');
        assertInputAndDeleteCharacter("1111:11:11.0", "11111111.0", formatter, '.');
        assertInputAndDeleteCharacter("1111:11:11.1", "11111111.1", formatter, '1');
        assertInputAndDeleteCharacter("1111:11:11.11", "11111111.11", formatter, '1');
        assertInputAndDeleteCharacter("1111:11:11.111", "11111111.111", formatter, '1');
        assertInputAndDeleteCharacter("1111:11:11.1111", "11111111.1111", formatter, '1');
        assertInputAndDeleteCharacter("1111:11:11.11111", "11111111.11111", formatter, '1');
        assertInputAndDeleteCharacter("1111:11:11.111111", "11111111.111111", formatter, '1');
        assertInputAndDeleteCharacter("1111:11:11.1111111", "11111111.1111111", formatter, '1');
        assertInputAndDeleteCharacter("1111:11:11.11111111", "11111111.11111111", formatter, '1');
        assertInputAndDeleteCharacter("1111:11:11.111111111", "11111111.111111111", formatter, '1');
        assertInputCharacter("1111:11:11.111111111", "11111111.111111111", formatter, '1');
        assertInputCharacter("1111:11:11.111111111", "11111111.111111111", formatter, '1');
        assertInputCharacter("1111:11:11.111111111", "11111111.111111111", formatter, '1');

        formatter.clear();
        assertEquals("0s", formatter.toString());
    }

    @Test
    public void testInputDifferentDigits()
    {
        var formatter = new ValueFormatter();
        assertInputAndDeleteCharacter("9s", "9", formatter, '9');
        assertInputAndDeleteCharacter("98s", "98", formatter, '8');
        assertInputAndDeleteCharacter("09:87", "987", formatter, '7');
        assertInputAndDeleteCharacter("98:76", "9876", formatter, '6');
        assertInputAndDeleteCharacter("9:87:65", "98765", formatter, '5');
        assertInputAndDeleteCharacter("98:76:54", "987654", formatter, '4');
        assertInputAndDeleteCharacter("987:65:43", "9876543", formatter, '3');
        assertInputAndDeleteCharacter("9876:54:32", "98765432", formatter, '2');
        assertInputAndDeleteCharacter("98765:43:21", "987654321", formatter, '1');
        assertInputAndDeleteCharacter("987654:32:10", "9876543210", formatter, '0');
        assertInputAndDeleteCharacter("987654:32:10.0", "9876543210.0", formatter, '.');
        assertInputAndDeleteCharacter("987654:32:10.1", "9876543210.1",formatter, '1');
        assertInputAndDeleteCharacter("987654:32:10.12", "9876543210.12",formatter, '2');
        assertInputAndDeleteCharacter("987654:32:10.123", "9876543210.123",formatter, '3');
        assertInputAndDeleteCharacter("987654:32:10.1234", "9876543210.1234",formatter, '4');
        assertInputAndDeleteCharacter("987654:32:10.12345", "9876543210.12345",formatter, '5');
        assertInputAndDeleteCharacter("987654:32:10.123456", "9876543210.123456",formatter, '6');
        assertInputAndDeleteCharacter("987654:32:10.1234567", "9876543210.1234567",formatter, '7');
        assertInputAndDeleteCharacter("987654:32:10.12345678", "9876543210.12345678",formatter, '8');
        assertInputAndDeleteCharacter("987654:32:10.123456789", "9876543210.123456789",formatter, '9');

        formatter.clear();
        assertInputAndDeleteCharacter("1s", "1", formatter, '1');
        assertInputAndDeleteCharacter("10s", "10", formatter, '0');
        assertInputAndDeleteCharacter("01:02", "102", formatter, '2');
        assertInputAndDeleteCharacter("10:20", "1020", formatter, '0');
        assertInputAndDeleteCharacter("1:02:03", "10203", formatter, '3');
        assertInputAndDeleteCharacter("10:20:30", "102030",formatter, '0');
        assertInputAndDeleteCharacter("10:20:30.0", "102030.0", formatter, '.');
        assertInputAndDeleteCharacter("10:20:30.4", "102030.4", formatter, '4');
        assertInputAndDeleteCharacter("10:20:30.40", "102030.40", formatter, '0');
        assertInputAndDeleteCharacter("10:20:30.405", "102030.405", formatter, '5');
        assertInputAndDeleteCharacter("10:20:30.4050", "102030.4050", formatter, '0');

        formatter.clear();
        assertInputAndDeleteCharacter("0s", "0", formatter, '0');
        assertInputAndDeleteCharacter("0s", "0", formatter, '0');
        assertInputAndDeleteCharacter("0s", "0", formatter, '0');
        assertInputAndDeleteCharacter("0.0s", "0.0",formatter, '.');
        assertInputAndDeleteCharacter("0.0s", "0.0", formatter, '0');
        assertInputAndDeleteCharacter("0.02s", "0.02", formatter, '2');
        assertInputAndDeleteCharacter("0.024s", "0.024", formatter, '4');
        assertInputAndDeleteCharacter("0.0240s", "0.0240", formatter, '0');
        assertInputAndDeleteCharacter("0.02406s", "0.02406", formatter, '6');
        assertInputAndDeleteCharacter("0.024060s", "0.024060", formatter, '0');
        assertInputAndDeleteCharacter("0.0240600s", "0.0240600", formatter, '0');
        assertInputAndDeleteCharacter("0.02406008s", "0.02406008", formatter, '8');
        assertInputAndDeleteCharacter("0.024060080s", "0.024060080", formatter, '0');

        formatter.clear();
        assertInputAndDeleteCharacter("0.0s", "0.0", formatter, '.');
        assertInputAndDeleteCharacter("0.0s", "0.0", formatter, '0');
        assertInputAndDeleteCharacter("0.00s", "0.00", formatter, '0');
        assertInputAndDeleteCharacter("0.000s", "0.000", formatter, '0');
        assertInputAndDeleteCharacter("0.0000s", "0.0000", formatter, '0');
        assertInputAndDeleteCharacter("0.00000s", "0.00000", formatter, '0');
        assertInputAndDeleteCharacter("0.000000s", "0.000000", formatter, '0');
        assertInputAndDeleteCharacter("0.0000000s", "0.0000000", formatter, '0');
        assertInputAndDeleteCharacter("0.00000000s", "0.00000000", formatter, '0');
        assertInputAndDeleteCharacter("0.000000000s", "0.000000000", formatter, '0');
    }

    @Test
    public void testInputIgnoredCharacters()
    {
        var formatter = new ValueFormatter();
        assertInputCharacter("0s", "0", formatter, 'a');
        assertInputAndDeleteCharacter("1s", "1", formatter, '1');
        assertInputCharacter("1s", "1", formatter, 'b');
        assertInputAndDeleteCharacter("12s", "12", formatter, '2');
        assertInputCharacter("12s", "12", formatter, 'c');
        assertInputAndDeleteCharacter("01:23", "123", formatter, '3');
        assertInputAndDeleteCharacter("01:23.0", "123.0", formatter, '.');
        assertInputCharacter("01:23.0", "123.0", formatter, '.');
        assertInputCharacter("01:23.0", "123.0", formatter, '.');
        assertInputAndDeleteCharacter("01:23.4", "123.4", formatter, '4');
        assertInputCharacter("01:23.4", "123.4", formatter, 'x');
        assertInputAndDeleteCharacter("01:23.45", "123.45", formatter, '5');
        assertInputCharacter("01:23.45", "123.45", formatter, 'y');
        assertInputAndDeleteCharacter("01:23.456", "123.456", formatter, '6');
        assertInputCharacter("01:23.456", "123.456", formatter, 'z');
        assertInputAndDeleteCharacter("01:23.4567", "123.4567", formatter, '7');
        assertInputCharacter("01:23.4567", "123.4567", formatter, '.');
        assertInputAndDeleteCharacter("01:23.45678", "123.45678", formatter, '8');
        assertInputCharacter("01:23.45678", "123.45678", formatter, '+');
        assertInputAndDeleteCharacter("01:23.456789", "123.456789", formatter, '9');
    }

    @Test
    public void testIsEmpty()
    {
        var formatter = new ValueFormatter();
        assertTrue(formatter.isEmpty());

        // Zero is considered empty.
        formatter.inputCharacter('0'); // "0s"
        assertDeleteCharacterIsEmpty(formatter, true);
        formatter.clear();
        formatter.inputCharacter(ValueFormatter.TOGGLE_TYPE_CHARACTER); // "0"
        assertDeleteCharacterIsEmpty(formatter, true);

        // Empty after deleting character.
        formatter.clear();
        formatter.inputCharacter('1'); // "1s"
        assertDeleteCharacterIsEmpty(formatter, true); // "0s"

        // Empty after deleting time with fraction.
        formatter.clear();
        formatter.inputCharacter('2');
        formatter.inputCharacter('.');
        formatter.inputCharacter('3');
        assertEquals("2.3s", formatter.toString());
        assertDeleteCharacterIsEmpty(formatter, false); // "2.0s"
        assertDeleteCharacterIsEmpty(formatter, false); // "2s"
        assertDeleteCharacterIsEmpty(formatter, true); // "0s"

        // Empty after deleting number with fraction.
        formatter.clear();
        formatter.inputCharacter('2');
        formatter.inputCharacter('.');
        formatter.inputCharacter('3');
        formatter.inputCharacter(ValueFormatter.TOGGLE_TYPE_CHARACTER);
        assertEquals("2.3", formatter.toString());
        assertDeleteCharacterIsEmpty(formatter, false); // "2.0"
        assertDeleteCharacterIsEmpty(formatter, false); // "2"
        assertDeleteCharacterIsEmpty(formatter, true); // "0"
    }

    private static void assertInputCharacter(
            String time, String number, ValueFormatter formatter, char ch)
    {
        // Input character and check time.
        assertEquals(time, formatter.inputCharacter(ch));
        assertEquals(time, formatter.toString());

        // Toggle to number.
        assertEquals(number, formatter.inputCharacter(ValueFormatter.TOGGLE_TYPE_CHARACTER));
        assertEquals(number, formatter.toString());

        // Toggle back to time.
        assertEquals(time, formatter.inputCharacter(ValueFormatter.TOGGLE_TYPE_CHARACTER));
        assertEquals(time, formatter.toString());
    }

    private static void assertInputAndDeleteCharacter(
            String time, String number, ValueFormatter formatter, char ch)
    {
        String timeBefore = formatter.toString();

        // Input character and check time.
        assertEquals(time, formatter.inputCharacter(ch));
        assertEquals(time, formatter.toString());

        // Delete character and check that value is the same as before inputting character.
        assertEquals(timeBefore, formatter.deleteCharacter());
        assertEquals(timeBefore, formatter.toString());

        // Toggle to number.
        formatter.inputCharacter(ValueFormatter.TOGGLE_TYPE_CHARACTER);
        String numberBefore = formatter.toString();

        // Input character and check number.
        assertEquals(number, formatter.inputCharacter(ch));
        assertEquals(number, formatter.toString());

        // Delete character and check that value is the same as before inputting character.
        assertEquals(numberBefore, formatter.deleteCharacter());
        assertEquals(numberBefore, formatter.toString());

        // Input character and toggle back to time so that formatter is in expected state.
        formatter.inputCharacter(ch);
        assertEquals(time, formatter.inputCharacter(ValueFormatter.TOGGLE_TYPE_CHARACTER));
        assertEquals(time, formatter.toString());
    }

    private static void assertDeleteCharacterIsEmpty(ValueFormatter formatter, boolean isEmpty)
    {
        formatter.deleteCharacter();
        assertEquals(isEmpty, formatter.isEmpty());
    }
}
