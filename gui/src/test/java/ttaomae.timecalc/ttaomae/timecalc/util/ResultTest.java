package ttaomae.timecalc.util;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

public class ResultTest
{
    @Test
    public void testSuccess()
    {
        var stringResult = Result.success("success");
        assertTrue(stringResult.isSuccess());
        assertEquals("success", stringResult.getValue().get());
        assertFalse(stringResult.isFailure());
        assertFalse(stringResult.getError().isPresent());

        var intResult = Result.success(123);
        assertTrue(intResult.isSuccess());
        assertEquals(123, intResult.getValue().get());
        assertFalse(intResult.isFailure());
        assertFalse(intResult.getError().isPresent());
    }

    @Test
    public void testFailure()
    {
        var stringFailure = Result.failure("failure");
        assertTrue(stringFailure.isFailure());
        assertEquals("failure", stringFailure.getError().get());
        assertFalse(stringFailure.isSuccess());
        assertFalse(stringFailure.getValue().isPresent());

        var intFailure = Result.failure(987);
        assertTrue(intFailure.isFailure());
        assertEquals(987, intFailure.getError().get());
        assertFalse(intFailure.isSuccess());
        assertFalse(intFailure.getValue().isPresent());
    }
}
